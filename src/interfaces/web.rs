use actix_files::Files;
use actix_web::{HttpServer, App};

use crate::app::router;

#[actix_web::main]
pub async fn run_web() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(router::init_routes)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}