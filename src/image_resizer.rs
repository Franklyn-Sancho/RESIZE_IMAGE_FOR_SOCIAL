//image_resizer
use crate::{social_plataform::SocialPlatform, utils::read_input::read_input};
use image::{imageops, DynamicImage};

//Lê o caminho de entrada de uma imagem que o usuário deseja redimensionar (interface cli)
pub fn read_input_path() -> String {
    read_input("Enter the file you want to resize: ")
}

pub fn read_output_path() -> String {
    read_input("Enter the name of the output file (it will be saved in the output folder): ")
}

pub struct ImageResizer<'a> {
    input_data: &'a [u8],
    output_path: String,
    social_plataform: SocialPlatform,
}

//image_resizer.rs
impl <'a> ImageResizer<'a> {
    pub fn new(
        input_data: &'a [u8],
        output_path_name: &str,
        social_plataform_name: &str,
    ) -> Option<ImageResizer<'a>> {
        let output_path = format!("output/{}", output_path_name);
        match SocialPlatform::new(social_plataform_name) {
            Some(social_plataform) => Some(ImageResizer {
                input_data,
                output_path,
                social_plataform,
            }),
            None => None,
        }
    }

    pub fn resize(&self, img: &DynamicImage) -> DynamicImage {
        let resized_img = imageops::resize(
            img,
            self.social_plataform.width, //referencia a largura da estrutura SocialPlataform
            self.social_plataform.height, //referencia a altura da estrutura SocialPlatform
            imageops::FilterType::Lanczos3, //escolhi um filtro que equilibra qualidade e velicocidade
        );
        DynamicImage::ImageRgba8(resized_img)
    }

    pub fn save_output_image(&self, img: &DynamicImage) {
        if let Err(_) = img.save(&self.output_path) {
            eprintln!("Could not save output image '{}'", self.output_path);
            std::process::exit(1)
        }
    }
}
