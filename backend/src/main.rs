use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use actix_web::{get, web, App, HttpServer, Responder};
mod json_utils;
use json_utils::{json_response, Json};
use serde::{Deserialize, Serialize};

const DB_SCHEMA: &str = include_str!("../sql/schema.sql");

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    age: u8,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=pdsdi dbname=clothe_match password=pdsdi",
        tokio_postgres::NoTls,
    )
    .await
    .unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.batch_execute(DB_SCHEMA).await.unwrap();

    println!("Database schema applied!");
    println!("Server running at http://127.0.0.1:1234");

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 1234))?
        .run()
        .await
}
