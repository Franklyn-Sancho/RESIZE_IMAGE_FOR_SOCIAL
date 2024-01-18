//router.rs
use crate::app::handlers;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/get-image").route(web::post().to(handlers::process_image_handler)),
    );
}
