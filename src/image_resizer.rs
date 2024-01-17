//image_resizer
use crate::{social_plataform::SocialPlatform, utils::read_input::read_input};
use image::{imageops, DynamicImage};

//Lê o caminho de entrada de uma imagem que o usuário deseja redimensionar (interface cli)
/* pub fn read_input_path() -> String {
    read_input("Enter the file you want to resize: ")
}

pub fn read_output_path() -> String {
    read_input("Enter the name of the output file (it will be saved in the output folder): ")
} */

pub struct ImageResizer {
    social_platform: SocialPlatform,
}

impl ImageResizer {
    pub fn new(social_platform_name: &str) -> Option<ImageResizer> {
        match SocialPlatform::new(social_platform_name) {
            Some(social_platform) => Some(ImageResizer {
                social_platform,
            }),
            None => None,
        }
    }

    pub fn resize(&self, img: &DynamicImage) -> DynamicImage {
        let resized_img = imageops::resize(
            img,
            self.social_platform.width, 
            self.social_platform.height, 
            imageops::FilterType::Lanczos3,
        );
        DynamicImage::ImageRgba8(resized_img)
    }
}


