use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use ahash::RandomState;
use papaya::HashMap;
use tokio::{fs, io::AsyncReadExt, task::JoinSet};

use tokio_postgres::types::ToSql;

use crate::{
    db::ClotheCache,
    models::clothe::{Category, Color},
    utils::session_utils::validate_session,
    State,
};

pub async fn upload(state: web::Data<State>, session: Session, data: web::Bytes) -> impl Responder {
    let user_id = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let clothes = cbf::deserialize_clothes(&mut data.as_ref()).unwrap();

    let mut query = String::from(
        "INSERT INTO clothes (name, color, category, user_id, is_for_hot_weather) VALUES ",
    );
    let length = clothes.len();
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::with_capacity(length * 5);
    let mut values: Vec<(&str, i16, i16, i16, bool)> = Vec::with_capacity(length);

    for (i, clothe) in clothes.iter().enumerate() {
        if i != 0 {
            query.push_str(", ");
        }

        query.push_str(&format!(
            "(${}, ${}, ${}, ${}, ${})",
            i * 5 + 1,
            i * 5 + 2,
            i * 5 + 3,
            i * 5 + 4,
            i * 5 + 5
        ));

        values.push((
            &clothe.name,
            clothe.color as i16,
            clothe.category as i16,
            clothe.user_id as i16,
            clothe.is_for_hot_weather,
        ));
    }

    for value in &values {
        params.push(&value.0);
        params.push(&value.1);
        params.push(&value.2);
        params.push(&value.3);
        params.push(&value.4);
    }

    query.push_str(" RETURNING clothe_id");

    // TODO: Write the files to the disk and possibly only handle the db later
    match state.db.client.query(&query, &params).await {
        Ok(rows) => {
            for (i, row) in rows.iter().enumerate() {
                state
                    .cache
                    .users
                    .pin()
                    .get(&(user_id as i16))
                    .unwrap()
                    .clothes
                    .pin()
                    .insert(
                        row.get(0),
                        ClotheCache {
                            name: values[i].0.to_string(),
                            color: Color::from(values[i].1),
                            category: Category::from(values[i].2),
                            is_hot_weather: values[i].4,
                        },
                    );
            }
        }
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    HttpResponse::Ok().finish()
}

async fn read_file(id: i16, path: std::path::PathBuf) -> (i16, Vec<u8>) {
    let file = fs::File::open(path).await.unwrap();

    let mut buffer = Vec::with_capacity(10 * 1024); // 10KB
    let mut reader = tokio::io::BufReader::new(file);
    reader.read_to_end(&mut buffer).await.unwrap();

    (id, buffer)
}

pub async fn get_clothes(state: web::Data<State>, session: Session) -> impl Responder {
    let user_id = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let users = &state.cache.users.pin();
    let users_clothes = users.get(&(user_id as i16)).unwrap().clothes.pin();
    let clothes_len = users_clothes.len();

    if clothes_len == 0 {
        return HttpResponse::NotFound().finish();
    }

    let mut clothes = Vec::with_capacity(clothes_len);

    let mut entries = tokio::fs::read_dir("uploads").await.unwrap();

    let mut set = JoinSet::new();

    while let Some(entry) = entries.next_entry().await.unwrap() {
        let file_name = entry.file_name().into_string().unwrap();
        if file_name.chars().next().unwrap() == '.' {
            continue;
        }
        let clothe_id = file_name.split('-').next().unwrap().parse::<i16>().unwrap();

        if let Some(_) = users_clothes.get(&clothe_id) {
            set.spawn(read_file(clothe_id, entry.path()));
        }
    }

    let results = HashMap::builder()
        .hasher(RandomState::new())
        .capacity(clothes_len)
        .build();

    while let Some(result) = set.join_next().await {
        let result = result.unwrap();
        results.pin().insert(result.0, result.1);
    }

    for (clothe_id, clothe) in users_clothes.iter() {
        // There is not much difference between doing this and doing it sequentially, I would need something like io-uring but it sucks for rust right now
        let mut buffer = Vec::with_capacity(16384);
        let file_name = format!("uploads/{}-.png", clothe_id);
        let file = fs::File::open(file_name).await.unwrap();
        let mut reader = tokio::io::BufReader::new(file);
        reader.read_to_end(&mut buffer).await.unwrap();

        clothes.push(cbf::Clothe {
            id: *clothe_id as u16,
            name: clothe.name.clone(),
            color: clothe.color.clone() as u8,
            category: clothe.category.clone() as u8,
            user_id: user_id as u16,
            is_for_hot_weather: clothe.is_hot_weather,
            file_name: "".to_string(),
            file: buffer,
        });
    }

    // TODO: can optimize this by setting the capacity of the vector
    let mut response = Vec::new();
    cbf::serialize_clothes(&clothes, &mut response).unwrap();

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(response)
}
