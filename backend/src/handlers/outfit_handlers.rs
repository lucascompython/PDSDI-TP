use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncReadExt};

use crate::{
    utils::{bool_pack::BoolPack, json_utils::Json, session_utils::validate_session},
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
