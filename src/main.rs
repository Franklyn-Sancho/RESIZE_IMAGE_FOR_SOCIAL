mod image_resizer;
mod social_plataform;

use image_resizer::ImageResizer;
use std::io;

fn main() {
    let mut input = String::new();
    println!("enter the path of the image you want to resize:");
    io::stdin().read_line(&mut input).unwrap();
    let input_path = input.trim();

    let mut output = String::new();
    println!("Enter the name of the output file (it will be saved in the output folder): ");
    io::stdin().read_line(&mut output).unwrap();
    let output_path = output.trim();

    let mut platform = String::new();
    println!("Enter the name of the social media platform: ");
    io::stdin().read_line(&mut platform).unwrap();
    let social_plataform = platform.trim();

    let resizer = match ImageResizer::new(input_path, output_path, social_plataform) {
        Some(resizer) => resizer,
        None => {
            eprintln!("social network not supported");
            std::process::exit(1)
        }
    };

    if let Err(e) = resizer.resize() {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}
