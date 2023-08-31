//JPEG, PNG, GIF, SVG, TIFF, BMP, PDF, HEIF e HEIC

use std::io;

use image::ImageFormat;

pub fn ask_to_convert() -> bool {
    loop {
        let mut to_convert = String::new();
        println!("Do you want to convert this image (yes/no): ");
        io::stdin().read_line(&mut to_convert).unwrap();
        let to_convert = to_convert.trim();
        match to_convert {
            "yes" => return true,
            "no" => return false,
            _ => eprintln!("Invalid input. Please enter 'yes' or 'no'."),
        }
    }
}

pub fn ask_conversion_format() -> (ImageFormat, &'static str) {
    loop {
        let mut format = String::new();
        println!("Choose the conversion format: ");
        println!("1 - Convert to JPEG");
        println!("2 - Convert to PNG");
        io::stdin().read_line(&mut format).unwrap();
        let format = format.trim();

        match format {
            "1" => return (ImageFormat::Jpeg, ".jpeg"),
            "2" => return (ImageFormat::Png, ".png"),
            _ => eprintln!("Invalid conversion option"),
        }
    }
}

