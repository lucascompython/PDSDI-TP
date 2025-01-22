// TODO: See compression middleware
// TODO: If check isn't successfull, then redirect directly from the backend
// TODO: Later remove the check api route because we won't need it since every page will call the
// api

use std::io::BufReader;

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
use db::{Cache, Db};
use rustls::{pki_types::PrivateKeyDer, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

const SECS_IN_WEEK: i64 = 60 * 60 * 24 * 7;

struct State {
    db: Db,
    cache: Cache,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (db, cache) = Db::new().await.unwrap();
    let state = web::Data::new(State { db, cache });

    if cfg!(debug_assertions) {
        println!("Development Server running at http://127.0.0.1:1234");
    } else {
        println!("Production Server running at https://0.0.0.0:1234");
    }

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
                .app_data(state.clone())
        })
        .bind(("0.0.0.0", 1234))?
        .run()
        .await
    } else {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .unwrap();

        let config = ServerConfig::builder().with_no_client_auth();

        let cert_file = &mut BufReader::new(include_bytes!("../certs/cert.pem") as &[u8]);
        let key_file = &mut BufReader::new(include_bytes!("../certs/key.pem") as &[u8]);

        let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>().unwrap();

        let mut keys = pkcs8_private_keys(key_file)
            .map(|key| key.map(PrivateKeyDer::Pkcs8))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        if keys.is_empty() {
            eprintln!("No keys found in key file");
            std::process::exit(1);
        }

        let config = config.with_single_cert(cert_chain, keys.remove(0)).unwrap();

        HttpServer::new(move || {
            App::new()
                .configure(routes::init)
                .wrap(
                    Cors::permissive(), // TODO: Change this to a more secure configuration
                )
                .wrap(
                    SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                        .cookie_secure(true)
                        .cookie_http_only(true)
                        .cookie_same_site(actix_web::cookie::SameSite::None)
                        .session_lifecycle(
                            PersistentSession::default()
                                .session_ttl(Duration::seconds(SECS_IN_WEEK)),
                        )
                        .build(),
                )
                .app_data(state.clone())
        })
        .bind_rustls_0_23("0.0.0.0:1234", config)?
        .run()
        .await
    }
}
