//router.rs
use crate::app::handlers;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/resize")
            .route(web::get().to(handlers::resize_image)),
    );
}
