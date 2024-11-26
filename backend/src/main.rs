use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:1234");
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 1234))?
        .run()
        .await
}
