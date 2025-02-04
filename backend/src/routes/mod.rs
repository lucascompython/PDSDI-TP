use actix_web::web;

pub mod clothe_routes;
pub mod outfit_routes;
pub mod user_routes;

pub fn init(cfg: &mut web::ServiceConfig) {
    user_routes::init(cfg);
    clothe_routes::init(cfg);
    outfit_routes::init(cfg);
}
