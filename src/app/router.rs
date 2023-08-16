//router.rs
use crate::app::handlers;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/resize_rotate")
            .route(web::post().to(handlers::resize_rotate_handler)),
    );

    cfg.service(
        web::resource("/download/{filename}")
            .route(web::get().to(handlers::download_image)),
    );
}
