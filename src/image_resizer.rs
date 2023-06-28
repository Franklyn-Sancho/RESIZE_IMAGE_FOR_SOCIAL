use crate::social_plataform::SocialPlatform;
use image::{imageops, DynamicImage};
use std::io;
use std::path::Path;

pub fn read_input_path() -> String {
    let mut input = String::new();
    println!("enter the path of the image you want to resize:");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn read_output_path() -> String {
    let mut output = String::new();
    println!("Enter the name of the output file (it will be saved in the output folder): ");
    io::stdin().read_line(&mut output).unwrap();
    output.trim().to_string()
}

pub struct ImageResizer<'a> {
    input_path: &'a str,
    output_path: String,
    social_plataform: SocialPlatform,
}

impl<'a> ImageResizer<'a> {
    pub fn new(
        input_path: &'a str,
        output_path_name: &str,
        social_plataform_name: &str,
    ) -> Option<ImageResizer<'a>> {
        let output_path = format!("output/{}", output_path_name); //será salvo numa pasta output
        match SocialPlatform::new(social_plataform_name) {
            Some(social_plataform) => Some(ImageResizer {
                input_path,       //nome ou endereço da imagem que você modificar
                output_path,      //local onde a imagem redimensionada é salva
                social_plataform, //a plataforma que o usuário
            }),
            None => None,
        }
    }

    

    pub fn resize(&self) -> DynamicImage {
        let img = self.load_input_image();
        let resized_img = imageops::resize(
            &img,
            self.social_plataform.width,
            self.social_plataform.height,
            imageops::FilterType::CatmullRom,
        );
        DynamicImage::ImageRgba8(resized_img)
    }

    pub fn save_output_image(&self, img: &DynamicImage) {
        if let Err(_) = img.save(&self.output_path) {
            eprintln!("Could not save output image '{}'", self.output_path);
            std::process::exit(1)
        }
    }

    fn load_input_image(&self) -> DynamicImage {
        match image::open(&Path::new(self.input_path)) {
            Ok(img) => img,
            Err(_) => {
                eprintln!("Could not open input image '{}'", self.input_path);
                std::process::exit(1)
            }
        }
    }

    /* pub fn resize(&self) {
        let img = match image::open(&Path::new(self.input_path)) {
            Ok(img) => img,
            Err(_) => {
                eprintln!("Could not open input image '{}'", self.input_path);
                std::process::exit(1)
            }
        };
        let resized_img = imageops::resize(
            &img,
            self.social_plataform.width,
            self.social_plataform.height,
            imageops::FilterType::CatmullRom,
        );
        if let Err(_) = resized_img.save(&self.output_path) {
            eprintln!("Could not sabe output image '{}'", self.output_path);
            std::process::exit(1)
        }
    } */

    pub fn get_input_path(&self) -> &'a str {
        self.input_path
    }

    pub fn get_output_path(&self) -> &str {
        &self.output_path
    }
}
