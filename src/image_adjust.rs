use image::{imageops, DynamicImage};

use crate::utils::read_input::read_input;

pub enum Effect {
    Brightness(i32),
    Contrast(f32),
    Grayscale,
    Exit,
}

pub fn adjust_image(img: &DynamicImage, effect: &Effect) -> DynamicImage {
    match effect {
        Effect::Brightness(brightness) => {
            let brightened_img = imageops::brighten(img, *brightness);
            DynamicImage::ImageRgba8(brightened_img)
        }
        Effect::Contrast(contrast) => {
            let contrast_img = imageops::contrast(img, *contrast);
            DynamicImage::ImageRgba8(contrast_img)
        }
        Effect::Grayscale => {
            let grayscale_img = imageops::grayscale(img);
            DynamicImage::ImageLuma8(grayscale_img)
        }
        Effect::Exit => img.clone(),
    }
}

pub fn ask_to_adjust_effects() -> Option<Effect> {
    loop {
        let input = read_input(
            "Would you like to adjust the brightness, contrast, or grayscale of this image? (yes/no): ",
        );
        match input.to_lowercase().as_str() {
            "yes" => return Some(choose_effect()),
            "no" => return None,
            _ => eprintln!("Invalid input, please enter 'yes' or 'no'."),
        }
    }
}

pub fn choose_effect() -> Effect {
    loop {
        println!("Escolha o efeito a ser ajustado: ");
        println!("1 - Ajustar Brilho");
        println!("2 - Ajustar Contraste");
        println!("3 - Alterar para Escala Cinza");
        println!("4 - Sair");

        let effect = read_input("");
        match effect.trim() {
            "1" => {
                let brightness: i32 = read_input("Enter the brightness value: ").parse().unwrap();
                return Effect::Brightness(brightness);
            }
            "2" => {
                let contrast: f32 = read_input("Enter the contrast value: ").parse().unwrap();
                return Effect::Contrast(contrast);
            }
            "3" => return Effect::Grayscale,
            "4" => return Effect::Exit,
            _ => eprintln!("Opção de conversão inválida. Tente novamente."),
        }
    }
}

pub fn adjust_image_effects(img: &DynamicImage) -> DynamicImage {
    let mut adjusted_img = img.clone();

    loop {
        if let Some(effect) = ask_to_adjust_effects() {
            adjusted_img = adjust_image(&adjusted_img, &effect);
            if let Effect::Exit = effect {
                break;
            }
        } else {
            break;
        }
    }

    adjusted_img
}
