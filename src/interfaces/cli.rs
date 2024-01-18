use crate::{
    image_adjust::{self},
    image_converter::select_output_name_with_extension,
    image_resizer::ImageResizer,
    image_rotate::{self},
    io_operations::IOOperator,
    social_plataform::input_social_plataform,
    utils::file_utils::select_file_from_dir,
};

pub fn run_cli() {
    let input_file = select_input_file().expect("Failed to select input file");
    let output_path = select_output_name_with_extension();
    let social_platform = input_social_plataform();
    let input_data = read_input_data(&input_file).expect("Failed to read input data");

    let img = image::load_from_memory(&input_data).unwrap();
    let resizer = ImageResizer::new(&social_platform).unwrap();
    let resized_img = resizer.resize(&img);

    let adjusted_img = image_adjust::adjust_image_effects(&resized_img);
    let rotated_img = image_rotate::rotate_if_desired(&adjusted_img);
    let io_operator = IOOperator::new(&output_path);
    io_operator.save_output_image(&rotated_img);
}


fn select_input_file() -> Result<String, Box<dyn std::error::Error>> {
    let dir_path = "input";
    select_file_from_dir(dir_path).map_err(|e| e.into())
}

fn read_input_data(input_file: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    std::fs::read(input_file).map_err(|e| e.into())
}


