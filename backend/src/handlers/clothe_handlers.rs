use std::path::Path;

use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use futures::{StreamExt as _, TryStreamExt};
use sonic_rs::{JsonValueTrait, Value};
use tokio::io::AsyncWriteExt;

use crate::{db::Db, utils::session_utils::validate_session};

pub async fn upload(db: web::Data<Db>, session: Session, mut payload: Multipart) -> impl Responder {
    let user_id = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let mut clothe_name = String::new();
    let mut clothe_data: Option<Value> = None;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition().unwrap();
        let name = content_disposition.get_name().unwrap();

        if name == "clothe" {
            let mut json_data = Vec::new();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                json_data.extend_from_slice(&data);
            }
            let clothe: Value = sonic_rs::from_slice(&json_data).unwrap();

            clothe_name = clothe["name"].as_str().unwrap().to_string();
            clothe_data = Some(clothe);
        } else if name == "image" {
            let filename = content_disposition.get_filename().unwrap();
            let extension = Path::new(filename)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("");
            let final_filename = format!("{}-{}.{}", user_id, clothe_name, extension);
            let path = format!("uploads/{}", final_filename);
            let mut file = tokio::fs::File::create(&path).await.unwrap();

            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                file.write_all(&data).await.unwrap();
            }

            if let Some(ref clothe) = clothe_data {
                let color_name = clothe["color"].as_str().unwrap();
                let category_name = clothe["category"].as_str().unwrap();
                let is_hot_weather = clothe["isForHotWeather"].as_bool().unwrap_or(false);

                let color_id: i32 = db
                    .client
                    .query_one(&db.statements.get_color_id_by_name, &[&color_name])
                    .await
                    .unwrap()
                    .get(0);

                let category_id: i32 = db
                    .client
                    .query_one(&db.statements.get_category_id_by_name, &[&category_name])
                    .await
                    .unwrap()
                    .get(0);

                db.client
                    .execute(
                        &db.statements.insert_clothe,
                        &[
                            &clothe_name,
                            &color_id,
                            &category_id,
                            &user_id,
                            &is_hot_weather,
                        ],
                    )
                    .await
                    .unwrap();
            }
        }
    }

    HttpResponse::Ok().finish()
}
