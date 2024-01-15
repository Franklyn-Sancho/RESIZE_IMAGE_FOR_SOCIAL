use crate::{
    image_adjust::{self},
    image_converter::{ask_conversion_format, ask_to_convert},
    image_resizer::{read_output_path, ImageResizer},
    image_rotate::{self},
    social_plataform::input_social_plataform,
    utils::file_utils::select_file_from_dir, io_operations::IOOperator,
};

pub fn run_cli() {
    let input_file = select_input_file().expect("Failed to select input file");
    let output_path = select_output_path_with_extension();
    let social_platform = select_social_platform();
    let input_data = read_input_data(&input_file).expect("Failed to read input data");

    let img = image::load_from_memory(&input_data).unwrap();
    let resizer = ImageResizer::new(/* &input_data, &output_path,  */&social_platform).unwrap();
    let resized_img = resizer.resize(&img);

    let adjusted_img = image_adjust::adjust_image_effects(&resized_img);
    let rotated_img = image_rotate::rotate_if_desired(&adjusted_img);
    let io_operator = IOOperator::new(&input_data, &output_path);
    io_operator.save_output_image(&rotated_img);
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
