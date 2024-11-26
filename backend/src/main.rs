use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
    age: u8,
}

#[get("/")]
async fn index() -> impl Responder {
    let data = MyObj {
        name: "Rust".to_string(),
        age: 8,
    };

    // HttpResponse::Ok().json(data)
    web::Json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:1234");
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 1234))?
        .run()
        .await
}
