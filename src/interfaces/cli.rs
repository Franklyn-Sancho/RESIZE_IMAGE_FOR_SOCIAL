use image::DynamicImage;

use crate::{
    
    image_resizer::{read_output_path, ImageResizer},
    image_rotate::{self, rotate_image},
    social_plataform::input_social_plataform, image_converter::{ask_to_convert, ask_conversion_format}, utils::file_utils::select_file_from_dir, image_config::{ menu_adjust_image, ask_to_adjust_effects},
};


pub fn run_cli() {
    let input_file = select_input_file().expect("Failed to select input file");
    let output_path = select_output_path_with_extension();
    let social_platform = select_social_platform();
    let input_data = read_input_data(&input_file).expect("Failed to read input data");

    let img = image::load_from_memory(&input_data).unwrap();
    let resizer = ImageResizer::new(&input_data, &output_path, &social_platform).unwrap();
    let resized_img = resizer.resize(&img);

    let adjusted_img = adjust_image_effects(&resized_img);
    rotate_and_save_image(&adjusted_img, &resizer);
}


fn select_output_path_with_extension() -> String {
    let mut output_path = select_output_path();
    if ask_to_convert() {
        let file_extension = ask_conversion_format().1;
        output_path.push_str(file_extension);
    } else {
        output_path.push_str(".jpg");
    }
    output_path
}

fn adjust_image_effects(img: &DynamicImage) -> DynamicImage {
    if ask_to_adjust_effects() {
        menu_adjust_image(img)
    } else {
        img.clone()
    }
}

fn rotate_and_save_image(img: &DynamicImage, resizer: &ImageResizer) {
    if image_rotate::ask_to_rotate() {
        let rotation = image_rotate::ask_rotation();
        let rotated_img = rotate_image(img, rotation);
        resizer.save_output_image(&rotated_img);
    } else {
        resizer.save_output_image(img);
    }
}

fn select_input_file() -> Result<String, Box<dyn std::error::Error>> {
    let dir_path = "input";
    select_file_from_dir(dir_path).map_err(|e| e.into())
}


fn read_input_data(input_file: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    std::fs::read(input_file).map_err(|e| e.into())
}


fn select_output_path() -> String {
    read_output_path()
}

fn select_social_platform() -> String {
    input_social_plataform()
}



