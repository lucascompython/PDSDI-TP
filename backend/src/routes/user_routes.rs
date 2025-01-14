use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route(
                "/register",
                web::post().to(crate::handlers::user_handlers::register),
            )
            .route(
                "/login",
                web::post().to(crate::handlers::user_handlers::login),
            )
            .route(
                "/logout",
                web::post().to(crate::handlers::user_handlers::logout),
            )
            .route(
                "/check",
                web::get().to(crate::handlers::user_handlers::check),
            )
            .route(
                "/protected",
                web::get().to(crate::handlers::user_handlers::protected),
            ), // .route(
               //     "/update",
               //     web::put().to(crate::handlers::user_handlers::update),
               // )
               // .route(
               //     "/delete",
               //     web::delete().to(crate::handlers::user_handlers::delete),
               // ),
    );
}
