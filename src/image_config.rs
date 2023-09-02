use std::io;

use image::{imageops, DynamicImage};

use crate::utils::read_input::read_input;

//deform, grayscale, invert, mirror

pub fn set_brightness(img: &DynamicImage) -> DynamicImage {
    let brightness: i32 = read_input("Enter the brightness value: ").parse().unwrap();

    let brightened_img = imageops::brighten(img, brightness);
    DynamicImage::ImageRgba8(brightened_img)
}

//
pub fn set_contrast(img: &DynamicImage) -> DynamicImage {
    let contrast: f32 = read_input("Enter de contrast value: ").parse().unwrap();

    let contrast_img = imageops::contrast(img, contrast);
    DynamicImage::ImageRgba8(contrast_img)
}

pub fn set_grayscale(img: &DynamicImage) -> DynamicImage {

    let grayscale_img = imageops::grayscale(img);
    DynamicImage::ImageLuma8(grayscale_img)
}


pub fn ask_to_adjust_effects() -> bool {
    loop {
        let input = read_input("Would you like to adjust the brightness, contrast, or grayscale of this image? (yes/no): ");
        match input.to_lowercase().as_str() {
            "yes" => return true,
            "no" => return false,
            _ => println!("Invalid input, please enter yes or no"),
        }
    }
}


pub fn menu_adjust_image(img: &DynamicImage) -> DynamicImage {
    let mut adjusted_img = img.clone();
    loop {
        let mut effect = String::new();
        println!("Escolha o efeito a ser ajustado: ");
        println!("1 - Ajustar Brilho");
        println!("2 - Ajustar Contraste");
        println!("3 - Alterar para Escala Cinza");
        println!("4 - Sair");
        io::stdin().read_line(&mut effect).unwrap();
        let format = effect.trim();

        match format {
            "1" => adjusted_img = set_brightness(&adjusted_img),
            "2" => adjusted_img = set_contrast(&adjusted_img),
            "3" => adjusted_img = set_grayscale(&adjusted_img),
            "4" => break,
            _ => eprintln!("Opção de conversão inválida"),
        }
    }
    return adjusted_img
}


