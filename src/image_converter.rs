//JPEG, PNG, GIF, SVG, TIFF, BMP, PDF, HEIF e HEIC

use crate::utils::read_input::read_input;

pub fn ask_to_convert() -> bool {
    loop {
        let to_convert = read_input(
            "The image will be saved in jpg, would you like to change the format? (yes/no): ",
        );
        match to_convert.to_lowercase().as_str() {
            "yes" => return true,
            "no" => return false,
            _ => eprintln!("Invalid input. Please enter 'yes' or 'no'."),
        }
    }
}

pub fn ask_conversion_format() -> &'static str {
    loop {
        let format =
            read_input("Choose the conversion format: \n1 - Convert to JPEG\n2 - Convert to PNG");
        match format.as_str() {
            "1" => return ".jpeg",
            "2" => return ".png",
            _ => eprintln!("Invalid conversion option. Please choose '1' or '2'."),
        }
    }
}

pub fn select_output_name_with_extension() -> String {
    let mut output_name =
        read_input("Enter the name of the output file (it will be saved in the output folder): ");

    if ask_to_convert() {
        output_name.push_str(ask_conversion_format());
    } else {
        output_name.push_str(".jpg");
    }

    output_name
}
