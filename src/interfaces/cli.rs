use crate::{
    file_utils::select_file_from_dir,
    image_resizer::{read_output_path, ImageResizer},
    image_rotate::{self, rotate_image},
    social_plataform::input_social_plataform, image_converter::{ask_to_convert, ask_conversion_format},
};


pub fn run_cli() {
    let input_file = select_input_file().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    println!("Selected file: {}", input_file);

    let mut output_path = select_output_path();
    let social_platform = select_social_platform();
    let input_data = read_input_data(&input_file).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    if ask_to_convert() {
        let file_extension = ask_conversion_format().1;
        output_path.push_str(file_extension);
        println!("Image converted successfully")
    } else {
        output_path.push_str(".jpg");
    }

    let img = image::load_from_memory(&input_data).unwrap();
    let resizer = ImageResizer::new(&input_data, &output_path, &social_platform).unwrap();

    let resized_img = resizer.resize(&img);
    if image_rotate::ask_to_rotate() {
        let rotation = image_rotate::ask_rotation();
        let rotated_img = rotate_image(&resized_img, rotation);
        resizer.save_output_image(&rotated_img);
        println!("Image Rotated Successfully")
    } else {
        resizer.save_output_image(&resized_img);
    }
}


fn select_input_file() -> Result<String, Box<dyn std::error::Error>> {
    let dir_path = ".";
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



