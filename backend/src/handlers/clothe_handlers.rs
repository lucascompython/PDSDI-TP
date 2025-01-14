use std::path::Path;

use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use futures::{StreamExt as _, TryStreamExt};
use sonic_rs::{JsonValueTrait, Value};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{
    db::Db,
    models::clothe::{Clothe, Color},
    utils::session_utils::validate_session,
};

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

// get_clothes function that returns all the clothes a user has uploaded
// do this using multipart form data

pub async fn get_clothes(db: web::Data<Db>, session: Session) -> impl Responder {
    let user_id = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    match db
        .client
        .query(&db.statements.get_clothe_by_user_id, &[&user_id])
        .await
    {
        Ok(rows) => {
            let mut clothes = Vec::new();
            for row in rows {
                let id: i32 = row.get(0);
                let name: String = row.get(1);
                let color_id: i32 = row.get(2);
                let category_id: i32 = row.get(3);
                let user_id: i32 = row.get(4);
                let is_hot_weather: bool = row.get(5);

                let color_name: String = db
                    .client
                    .query_one(&db.statements.get_color_name_by_id, &[&color_id])
                    .await
                    .unwrap()
                    .get(0);

                let category_name: String = db
                    .client
                    .query_one(&db.statements.get_category_name_by_id, &[&category_id])
                    .await
                    .unwrap()
                    .get(0);

                clothes.push(Clothe {
                    id,
                    name,
                    color: color_name.parse().unwrap(),
                    category: category_name.parse().unwrap(),
                    user_id,
                    is_hot_weather,
                });
            }

            // get the files from the uploads folder

            let mut files = Vec::new();
            let mut entries = tokio::fs::read_dir("uploads").await.unwrap();
            while let Some(entry) = entries.next_entry().await.unwrap() {
                let path = entry.path();
                let filename = path.file_name().unwrap().to_str().unwrap();
                let parts: Vec<&str> = filename.split('-').collect();
                if parts.len() != 2 {
                    continue;
                }
                let file_user_id = parts[0].parse::<i32>().unwrap();
                if file_user_id == user_id {
                    // Read the file and push it to the files vector
                    let mut file = tokio::fs::File::open(path.clone()).await.unwrap();
                    let mut contents = Vec::new();
                    file.read_to_end(&mut contents).await.unwrap();
                    files.push((filename.to_string(), contents));
                }
            }

            let mut body = actix_web::web::BytesMut::new();
            // Create a multipart response
            for clothe in clothes {
                body.extend_from_slice(b"--boundary\r\n");
                body.extend_from_slice(b"Content-Disposition: form-data; name=\"clothe\"\r\n");
                body.extend_from_slice(b"Content-Type: application/json\r\n\r\n");
                body.extend_from_slice(sonic_rs::to_string(&clothe).unwrap().as_bytes());
                body.extend_from_slice(b"\r\n");
            }

            for (filename, contents) in files {
                body.extend_from_slice(b"--boundary\r\n");
                body.extend_from_slice(
                    format!(
                        "Content-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\n",
                        filename
                    )
                    .as_bytes(),
                );
                body.extend_from_slice(b"Content-Type: application/octet-stream\r\n\r\n");
                body.extend_from_slice(&contents);
                println!("{} {} bytes", filename, contents.len());

                body.extend_from_slice(b"\r\n");
            }
            body.extend_from_slice(b"--boundary--\r\n");

            HttpResponse::Ok()
                .insert_header((
                    actix_web::http::header::CONTENT_TYPE,
                    "multipart/form-data; boundary=boundary",
                ))
                .body(body)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
