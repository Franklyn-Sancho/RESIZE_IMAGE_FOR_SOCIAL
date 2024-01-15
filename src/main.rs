mod app;
mod utils;
mod image_resizer;
mod image_rotate;
mod social_plataform;
mod interfaces;
mod image_converter;
mod image_adjust;
mod io_operations;

use std::env;
use interfaces::cli::run_cli;
use interfaces::web::run_web;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--web".to_string()) {
        // Iniciar a versão web da aplicação
        let _ = run_web();
    } else {
        //iniciar a versão CLI
        run_cli();
    }
}



