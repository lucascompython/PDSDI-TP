use std::sync::Arc;

use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod db;
mod json_utils;

use actix_web::{cookie::Key, get, post, web, App, HttpResponse, HttpServer, Responder};
use db::{DbClient, RegistrationError};
use json_utils::{json_response, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LoggedIn {
    id: i32,
    is_admin: bool,
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

fn validate_session(session: &Session) -> Result<i32, HttpResponse> {
    let user_id = session.get::<i32>("user_id").unwrap_or(None);

    match user_id {
        Some(id) => {
            session.renew();
            Ok(id)
        }
        None => Err(HttpResponse::Unauthorized().finish()),
    }
}

#[get("/")]
async fn index(session: Session) -> impl Responder {
    let user_id = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    json_response(&LoggedIn {
        id: user_id,
        is_admin: session.get::<bool>("is_admin").unwrap().unwrap(),
    })
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

#[post("/login")]
async fn login(
    db: web::Data<Arc<DbClient>>,
    login_data: web::Bytes,
    session: Session,
) -> impl Responder {
    let Json(data): Json<LoginRequest> = Json::from_bytes(login_data).unwrap();

    match db.login_user(&data.email, &data.password).await {
        Ok(user) => {
            session.insert("user_id", user.user_id).unwrap();
            session.insert("is_admin", user.is_admin).unwrap();
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

#[get("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.clear();
    HttpResponse::Ok().finish()
}

#[get("/protected")]
async fn protected(session: Session) -> impl Responder {
    let user_id = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let is_admin = session.get::<bool>("is_admin").unwrap().unwrap();

    if !is_admin {
        return HttpResponse::Forbidden().finish();
    }

    HttpResponse::Ok().body(format!(
        "User logged in with id: {}; And is admin: {}",
        user_id,
        session.get::<bool>("is_admin").unwrap().unwrap()
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Arc::new(DbClient::new().await.unwrap());

    println!("Server running at http://127.0.0.1:1234");

    let key = Key::generate();

    if cfg!(debug_assertions) {
        #[cfg(feature = "log")]
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        HttpServer::new(move || {
            App::new()
                .wrap(Cors::permissive()) // TODO: Change this to a more secure configuration
                .wrap(actix_web::middleware::Logger::default())
                .wrap(
                    SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                        .cookie_secure(false)
                        .cookie_http_only(false)
                        .build(),
                )
                .app_data(web::Data::new(client.clone()))
                .service(index)
                .service(register)
                .service(login)
                .service(logout)
                .service(protected)
        })
        .bind(("127.0.0.1", 1234))?
        .run()
        .await
    } else {
        HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::permissive(), // TODO: Change this to a more secure configuration
                )
                .wrap(
                    SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                        .cookie_secure(false)
                        .cookie_http_only(true)
                        .cookie_same_site(actix_web::cookie::SameSite::Strict)
                        .build(),
                )
                .app_data(web::Data::new(client.clone()))
                .service(index)
                .service(register)
                .service(login)
                .service(logout)
                .service(protected)
        })
        .bind(("127.0.0.1", 1234))?
        .run()
        .await
    }
}
