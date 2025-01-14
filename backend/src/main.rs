// TODO: Restructure the backend
// TODO: See compression middleware
// TODO: If check isn't successfull, then redirect directly from the backend
// TODO: Later remove the check api route because we won't need it since every page will call the
// api

use actix_cors::Cors;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod db;
mod handlers;
mod models;
mod routes;
mod utils;

use actix_web::{
    cookie::{time::Duration, Key},
    web, App, HttpServer,
};
use db::Db;

const SECS_IN_WEEK: i64 = 60 * 60 * 24 * 7;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = web::Data::new(Db::new().await.unwrap());

    println!("Server running at http://127.0.0.1:1234");

    let key = Key::generate();

    if cfg!(debug_assertions) {
        #[cfg(feature = "log")]
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
        HttpServer::new(move || {
            App::new()
                .configure(routes::init)
                .wrap(Cors::permissive()) // TODO: Change this to a more secure configuration
                .wrap(actix_web::middleware::Logger::default())
                .wrap(
                    SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                        .cookie_secure(false) // Change to true in production
                        .cookie_http_only(false)
                        .cookie_same_site(actix_web::cookie::SameSite::Strict)
                        .session_lifecycle(
                            PersistentSession::default()
                                .session_ttl(Duration::seconds(SECS_IN_WEEK)),
                        )
                        .build(),
                )
                .app_data(client.clone())
        })
        .bind(("127.0.0.1", 1234))?
        .run()
        .await
    } else {
        HttpServer::new(move || {
            App::new()
                .configure(routes::init)
                .wrap(
                    Cors::permissive(), // TODO: Change this to a more secure configuration
                )
                .wrap(
                    SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                        .cookie_secure(false) // Change to true in production
                        .cookie_http_only(false)
                        .cookie_same_site(actix_web::cookie::SameSite::Strict)
                        .session_lifecycle(
                            PersistentSession::default()
                                .session_ttl(Duration::seconds(SECS_IN_WEEK)),
                        )
                        .build(),
                )
                .app_data(client.clone())
        })
        .bind(("127.0.0.1", 1234))?
        .run()
        .await
    }
}
