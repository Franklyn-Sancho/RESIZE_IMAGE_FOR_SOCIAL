mod app;
mod utils;
mod image_resizer;
mod image_rotate;
mod social_plataform;
mod interfaces;
mod image_converter;
mod image_adjust;

use std::env;
use interfaces::cli::run_cli;
use interfaces::web::run_web;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--web".to_string()) {
        // Iniciar a versão para web do programa
        let _ = run_web();
    } else {
        run_cli();
    }
}



