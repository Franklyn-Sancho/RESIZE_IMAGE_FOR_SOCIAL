mod image_resizer;
mod rotate;
mod social_plataform;

use std::path::Path;

use image_resizer::ImageResizer;
use social_plataform::input_social_plataform;

use crate::{
    image_resizer::{read_input_path, read_output_path},
    rotate::rotate_image,
};

fn main() {
    let input_path = read_input_path();
    let output_path = read_output_path();
    let social_plataform = input_social_plataform();

    let resizer = ImageResizer::new(&input_path, &output_path, &social_plataform).unwrap();
    /* let img = match image::open(&Path::new(resizer.get_input_path())) {
        Ok(img) => img,
        Err(_) => {
            eprintln!("Could not open input image '{}'", resizer.get_input_path());
            std::process::exit(1)
        }
    }; */

    let resized_img = resizer.resize();
    let rotated_img = rotate_image(&resized_img);

    resizer.save_output_image(&rotated_img)
}


