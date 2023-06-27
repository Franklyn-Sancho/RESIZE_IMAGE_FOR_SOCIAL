use crate::social_plataform::SocialPlatform;
use image::imageops;
use std::path::Path;

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
                input_path, //nome ou endereço da imagem que você modificar
                output_path, //local onde a imagem redimensionada é salva
                social_plataform,
            }),
            None => None,
        }
    }

    pub fn resize(&self) -> Result<(), String> {
        let img = match image::open(&Path::new(self.input_path)) {
            Ok(img) => img,
            Err(_) => {
                return Err(format!(
                    "Could not open input image '{}'",
                    self.input_path
                ))
            }
        };
        let resized_img = imageops::resize(
            &img,
            self.social_plataform.width,
            self.social_plataform.height,
            imageops::FilterType::CatmullRom
        );
        match resized_img.save(&self.output_path) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!(
                "Could not save output image '{}'",
                self.output_path
            )),
        }
    }
}
