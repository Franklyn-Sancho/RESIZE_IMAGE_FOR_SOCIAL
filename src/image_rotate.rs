use image::DynamicImage;
use serde_derive::Deserialize;
use std::io;

use crate::utils::read_input::read_input;

#[derive(Deserialize, Clone)]
pub enum Rotation {
    None,
    Right90,
    Left90,
    HalfCircle,
}

pub fn rotate_image(img: &DynamicImage, rotation: Rotation) -> DynamicImage {
    apply_rotation(img, rotation)
}

pub fn ask_to_rotate() -> bool {
    loop {
        let to_rotate = read_input("Do you want to rotate this image (yes/no): ");
        match to_rotate.to_lowercase().as_str() {
            "yes" => return true,
            "no" => return false,
            _ => eprintln!("Invalid input. Please enter 'yes' or 'no'."),
        }
    }
}

pub fn ask_rotation() -> Rotation {
    println!("Choose the rotation option: ");
    println!("1 - Rotate 90 degrees to the right");
    println!("2 - Rotate 90 degrees to the left");
    println!("3 - Rotate HalfCircle");
    
    let rotation = read_input("Enter the rotation option: ");

    match rotation.trim() {
        "1" => Rotation::Right90,
        "2" => Rotation::Left90,
        "3" => Rotation::HalfCircle,
        _ => {
            eprintln!("Invalid rotation option");
            std::process::exit(1)
        }
    }
}

//o método que aplica a rotação
fn apply_rotation(img: &DynamicImage, rotation: Rotation) -> DynamicImage {
    match rotation {
        Rotation::None => img.clone(),
        Rotation::Right90 => img.rotate90(),
        Rotation::Left90 => img.rotate270(),
        Rotation::HalfCircle => img.rotate180(),
    }
}

pub fn rotate_if_desired(img: &DynamicImage) -> DynamicImage {
    if ask_to_rotate() {
        let rotation = ask_rotation();
        rotate_image(img, rotation)
    } else {
        img.clone()
    }
}
