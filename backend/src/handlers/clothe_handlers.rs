use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use tokio_postgres::types::ToSql;

use crate::{db::Db, utils::session_utils::validate_session};

pub async fn upload(db: web::Data<Db>, session: Session, data: web::Bytes) -> impl Responder {
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
        ))
    }

    for value in &values {
        params.push(&value.0);
        params.push(&value.1);
        params.push(&value.2);
        params.push(&value.3);
        params.push(&value.4);
    }

    // Execute the query
    match db.client.execute(&query, &params).await {
        Ok(_) => println!("Clothes uploaded successfully"),
        Err(e) => println!("Clothes upload failed: {}", e),
    }

    // save to db
    HttpResponse::Ok().finish()
}

// get_clothes function that returns all the clothes a user has uploaded
// do this using multipart form data

pub async fn get_clothes(db: web::Data<Db>, session: Session) -> impl Responder {
    let user_id = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    HttpResponse::Ok().finish()
}
