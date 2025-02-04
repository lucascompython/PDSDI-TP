use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncReadExt};
use tokio_postgres::types::ToSql;

use crate::{
    utils::{
        bool_pack::BoolPack,
        json_utils::{json_response, Json},
        session_utils::validate_session,
    },
    State,
};

#[derive(Deserialize)]
struct GenerateOutfitRequest {
    bool_pack: u16,
}

#[derive(Deserialize, Serialize)]
struct ClotheForPython {
    id: i32,
    name: String,
    color: u8,
    category: u8,
    user_id: u16,
    is_for_hot_weather: bool,
}

pub async fn generate_outfit(
    state: web::Data<State>,
    session: Session,
    data: web::Bytes,
) -> impl Responder {
    let user_id = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let Json(req): Json<GenerateOutfitRequest> = Json::from_bytes(data).unwrap();

    let bool_pack = BoolPack::from_u16(req.bool_pack);
    let bool_pack_clone = bool_pack.clone();

    let users = state.cache.users.pin();
    let users = users.get(&(user_id as i16)).unwrap();

    let clothes = users.clothes.pin();

    let mut clothes_json = Vec::with_capacity(clothes.len());

    for (clothe_id, clothe) in clothes.iter() {
        clothes_json.push(ClotheForPython {
            id: *clothe_id as i32,
            name: clothe.name.clone(),
            color: clothe.color.clone() as u8,
            category: clothe.category.clone() as u8,
            user_id: user_id as u16,
            is_for_hot_weather: clothe.is_hot_weather,
        });
    }

    let json_string = sonic_rs::to_string(&clothes_json).unwrap();

    let mut generated_outfit: Vec<ClotheForPython> = vec![];

    Python::with_gil(|py| {
        let model = PyModule::import(py, "model").unwrap();
        let gen_outfit = model.getattr("gen_outfit").unwrap();
        let clothes: String = gen_outfit
            .call1((json_string, bool_pack_clone.to_u16()))
            .unwrap()
            .extract()
            .unwrap();

        generated_outfit = sonic_rs::from_str(&clothes).unwrap();
    });

    let mut outfit = Vec::with_capacity(generated_outfit.len());

    for clothe in generated_outfit {
        let mut buffer = Vec::with_capacity(16384);
        let file_name = format!("uploads/{}.png", clothe.id);
        let file = fs::File::open(file_name).await.unwrap();
        let mut reader = tokio::io::BufReader::new(file);
        reader.read_to_end(&mut buffer).await.unwrap();

        outfit.push(cbf::Clothe {
            id: clothe.id as u16,
            name: clothe.name,
            color: clothe.color,
            category: clothe.category,
            user_id: clothe.user_id,
            is_for_hot_weather: clothe.is_for_hot_weather,
            file_name: "".to_string(),
            file: buffer,
        });
    }

    let mut response = Vec::new();

    cbf::serialize_clothes(&outfit, &mut response).unwrap();

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(response)
}

#[derive(Deserialize)]
struct SaveOutfitRequest {
    name: String,
    outfit_type: i32,
    clothes: Vec<i16>,
}

pub async fn save_outfit(
    state: web::Data<State>,
    session: Session,
    data: web::Bytes,
) -> impl Responder {
    let user_id = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let Json(req): Json<SaveOutfitRequest> = Json::from_bytes(data).unwrap();

    let outfit = state
        .db
        .client
        .query_one(
            &state.db.statements.insert_outfit,
            &[
                &req.name,
                &(user_id as i16),
                &&0_i16,
                &(req.outfit_type as i16),
            ],
        )
        .await
        .unwrap();

    let outfit_id: i16 = outfit.get(0);

    let mut query = String::from("INSERT INTO outfit_clothes (outfit_id, clothe_id) VALUES ");

    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::with_capacity(req.clothes.len() * 2);

    for (i, clothe_id) in req.clothes.iter().enumerate() {
        if i != 0 {
            query.push_str(", ");
        }

        query.push_str(&format!("(${}, ${})", i * 2 + 1, i * 2 + 2));

        params.push(&outfit_id);
        params.push(clothe_id);
    }

    state.db.client.execute(&query, &params).await.unwrap();

    HttpResponse::Ok().finish()
}

pub async fn get_last_outfit(state: web::Data<State>, session: Session) -> impl Responder {
    let user_id = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let outfit = match state
        .db
        .client
        .query_one(
            &state.db.statements.get_last_outfit_by_user,
            &[&(user_id as i16)],
        )
        .await
    {
        Ok(outfit) => outfit,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let outfit_id: i16 = outfit.get(0);

    let clothes = state
        .db
        .client
        .query(&state.db.statements.get_last_clothes_by_user, &[&outfit_id])
        .await
        .unwrap();

    let mut outfit_clothes: Vec<cbf::Clothe> = Vec::with_capacity(clothes.len());

    for clothe in clothes {
        let clothe_id: i16 = clothe.get(0);
        let mut buffer = Vec::with_capacity(16384);
        let file_name = format!("uploads/{}.png", clothe_id);
        let file = fs::File::open(file_name).await.unwrap();
        let mut reader = tokio::io::BufReader::new(file);
        reader.read_to_end(&mut buffer).await.unwrap();

        outfit_clothes.push(cbf::Clothe {
            id: clothe_id as u16,
            name: clothe.get(1),
            color: clothe.get::<usize, i16>(2) as u8,
            category: clothe.get::<usize, i16>(3) as u8,
            user_id: clothe.get::<usize, i16>(4) as u16,
            is_for_hot_weather: clothe.get(5),
            file_name: "".to_string(),
            file: buffer,
        });
    }

    let mut response = Vec::new();

    cbf::serialize_clothes(&outfit_clothes, &mut response).unwrap();

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(response)
}

#[derive(Serialize)]
struct Outfit {
    id: i16,
    name: String,
    outfit_type: i16,
    created_at: chrono::NaiveDateTime,
}

pub async fn get_outfits(state: web::Data<State>, session: Session) -> impl Responder {
    let user_id = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let outfits = state
        .db
        .client
        .query(
            &state.db.statements.get_outfits_by_user,
            &[&(user_id as i16)],
        )
        .await
        .unwrap();

    let mut outfits_json = Vec::with_capacity(outfits.len());

    for outfit in outfits {
        outfits_json.push(Outfit {
            id: outfit.get(0),
            name: outfit.get(1),
            outfit_type: outfit.get(2),
            created_at: outfit.get(3),
        });
    }

    json_response(&outfits_json)
}

pub async fn get_outfit_image(
    state: web::Data<State>,
    session: Session,
    outfit_id: web::Path<i16>,
) -> impl Responder {
    let _ = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let outfit_id = outfit_id.into_inner();

    let clothe = state
        .db
        .client
        .query_one(
            &state.db.statements.get_clothe_id_by_outfit,
            &[&(outfit_id as i16)],
        )
        .await
        .unwrap();

    let clothe_id: i16 = clothe.get(0);

    let mut buffer = Vec::with_capacity(16384);
    let file_name = format!("uploads/{}.png", clothe_id);
    let file = fs::File::open(file_name).await.unwrap();
    let mut reader = tokio::io::BufReader::new(file);
    reader.read_to_end(&mut buffer).await.unwrap();

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(buffer)
}
