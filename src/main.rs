mod file_utils;
mod image_resizer;
mod image_rotate;
mod social_plataform;

use file_utils::{select_file_from_dir};
use image_resizer::ImageResizer;
use social_plataform::input_social_plataform;

use crate::{
    image_resizer::{read_output_path},
    image_rotate::rotate_image,
};

fn main() {
    let dir_path = ".";
    match select_file_from_dir(dir_path) {
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
    }
}
