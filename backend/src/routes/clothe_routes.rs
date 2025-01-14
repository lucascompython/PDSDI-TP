use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/clothes")
            .route(
                "/upload",
                web::post().to(crate::handlers::clothe_handlers::upload),
            )
            .route(
                "/get",
                web::get().to(crate::handlers::clothe_handlers::get_clothes),
            ),
    );
}
