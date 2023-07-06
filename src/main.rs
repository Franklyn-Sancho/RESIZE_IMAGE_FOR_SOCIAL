/* mod file_utils; */
mod image_resizer;
/* mod image_rotate; */
mod app;
mod social_plataform;

use std::env;

/* use file_utils::{select_file_from_dir};
use image_resizer::ImageResizer;
use social_plataform::input_social_plataform; */
use crate::app::router;
use actix_files::Files;
use actix_web::{App, HttpServer};

use crate::image_resizer::read_output_path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--web".to_string()) {
        // Iniciar a versão para web do programa
        run_web();
    } else {
        // Iniciar a versão CLI do programa
    }
}

#[actix_web::main]
async fn run_web() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(router::init_routes)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/* fn run_cli() {
    let dir_path = "."; //pasta atual -> diretório raiz da aplicação
    match select_file_from_dir(dir_path) { //retorna e controla apenas os arquivos válidos
        Ok(input_path) => {
            println!("Selected file: {}", input_path);
            let output_path = read_output_path();
            let social_plataform = input_social_plataform();
            let resizer = ImageResizer::new(&input_path, &output_path, &social_plataform).unwrap();
            let resized_img = resizer.resize();
            let rotated_img = rotate_image(&resized_img);
            resizer.save_output_image(&rotated_img)
        }
        Err(e) => {
            eprintln!("{}", e)
        }
    };
} */
