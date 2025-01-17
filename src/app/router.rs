//router.rs
use crate::app::handlers;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/resize-image")
                .route(web::post().to(handlers::resize_handler)),
        )
        .service(
            web::resource("/rotate-image")
                .route(web::post().to(handlers::rotate_handler)),
        )
        .service(
            web::resource("/adjust-image")
                .route(web::post().to(handlers::adjust_handler)),
        )
        .service(
            web::resource("/convert-image") // Rota para conversão de imagem
                .route(web::post().to(handlers::convert_handler)),
        )
        .service(
            web::resource("/download/{id}")
                .route(web::get().to(handlers::download_image)),
        );
}


