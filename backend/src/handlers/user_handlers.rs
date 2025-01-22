use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    models::user::User,
    utils::{
        hashing_utils::{hash, verify},
        json_utils::{json_response, Json},
        session_utils::{admin_only, validate_session},
    },
    State,
};

#[derive(Serialize, Deserialize)]
struct LoggedIn {
    id: i32,
    is_admin: bool,
}

pub async fn check(session: Session) -> impl Responder {
    let user_id = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    json_response(&LoggedIn {
        id: user_id,
        is_admin: session.get::<bool>("is_admin").unwrap().unwrap(),
    })
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
    admin: bool,
}

pub async fn register(
    state: web::Data<State>,
    session: Session,
    request_data: web::Bytes,
) -> impl Responder {
    let Json(user): Json<RegisterRequest> = Json::from_bytes(request_data).unwrap();

    if let Err(response) = admin_only(&session) {
        return response;
    }

    match state
        .db
        .client
        .query(&state.db.statements.check_user_exists, &[&user.email])
        .await
    {
        Ok(rows) => {
            if rows.len() > 0 {
                return HttpResponse::Conflict().finish();
            }
        }
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    let password_bytes = hash(&user.password);

    match state
        .db
        .client
        .query(
            &state.db.statements.insert_user,
            &[
                &user.username,
                &user.email,
                &&password_bytes[..],
                &user.admin,
            ],
        )
        .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

pub async fn login(
    state: web::Data<State>,
    login_data: web::Bytes,
    session: Session,
) -> impl Responder {
    let Json(user_request): Json<LoginRequest> = Json::from_bytes(login_data).unwrap();

    let user = match state
        .db
        .client
        .query_one(
            &state.db.statements.get_user_by_email,
            &[&user_request.email],
        )
        .await
    {
        Ok(row) => {
            let password_slice: &[u8] = row.get(3);
            let mut password_bytes = [0u8; 48];
            password_bytes.copy_from_slice(password_slice);
            User {
                user_id: row.get(0),
                username: row.get(1),
                email: row.get(2),
                password: password_bytes,
                is_admin: row.get(4),
            }
        }
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match verify(&user_request.password, &user.password) {
        true => {
            session.insert("user_id", user.user_id).unwrap();
            session.insert("is_admin", user.is_admin).unwrap();
            HttpResponse::Ok().finish()
        }
        false => HttpResponse::Unauthorized().finish(),
    }
}

pub async fn logout(session: Session) -> impl Responder {
    session.clear();
    HttpResponse::Ok().finish()
}

pub async fn protected(session: Session) -> impl Responder {
    let user_id = match validate_session(&session) {
        Ok(id) => id,
        Err(response) => return response,
    };

    if let Err(response) = admin_only(&session) {
        return response;
    }

    HttpResponse::Ok().body(format!(
        "User logged in with id: {}; And is admin: {}",
        user_id,
        session.get::<bool>("is_admin").unwrap().unwrap()
    ))
}
