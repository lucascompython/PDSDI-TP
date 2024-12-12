use std::sync::Arc;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod db;
mod json_utils;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use db::{DbClient, RegistrationError};
use json_utils::{json_response, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    age: u8,
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[get("/")]
async fn index(data: web::Bytes) -> impl Responder {
    let Json(data): Json<MyObj> = Json::from_bytes(data).unwrap();
    println!("data.name: {}", data.name);
    println!("data.age: {}", data.age);

    let data2 = MyObj {
        name: "Rust".to_string(),
        age: 8,
    };

    json_response(&data2)
}

#[post("/register")]
async fn register(db: web::Data<Arc<DbClient>>, request_data: web::Bytes) -> impl Responder {
    let Json(request_data): Json<RegisterRequest> = Json::from_bytes(request_data).unwrap();

    match db
        .register_user(
            &request_data.username,
            &request_data.email,
            &request_data.password,
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(RegistrationError::UserAlreadyExists) => HttpResponse::BadRequest().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// #[post("/login")]
// async fn login(data: web::Bytes) -> impl Responder {
//     let Json(data): Json<User> = Json::from_bytes(data).unwrap();
//     println!("data.name: {}", data.name);
//     println!("data.age: {}", data.age);

//     let data2 = MyObj {
//         name: "Rust".to_string(),
//         age: 8,
//     };

//     json_response(&data2)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Arc::new(DbClient::new().await.unwrap());

    let first_color = client.get_color_by_id(1).await.unwrap();

    println!("first_color: {}", first_color.color_name);

    println!("Server running at http://127.0.0.1:1234");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(index)
            .service(register)
    })
    .bind(("127.0.0.1", 1234))?
    .run()
    .await
}
