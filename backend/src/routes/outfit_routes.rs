use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/outfits").route(
        "/generate",
        web::post().to(crate::handlers::outfit_handlers::generate_outfit),
    ));
}
