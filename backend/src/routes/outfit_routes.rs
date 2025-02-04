use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/outfits")
            .route(
                "/generate",
                web::post().to(crate::handlers::outfit_handlers::generate_outfit),
            )
            .route(
                "/save",
                web::post().to(crate::handlers::outfit_handlers::save_outfit),
            )
            .route(
                "/last",
                web::get().to(crate::handlers::outfit_handlers::get_last_outfit),
            )
            .route(
                "/get",
                web::get().to(crate::handlers::outfit_handlers::get_outfits),
            )
            .route(
                "/image/{outfit_id}",
                web::get().to(crate::handlers::outfit_handlers::get_outfit_image),
            ),
    );
}
