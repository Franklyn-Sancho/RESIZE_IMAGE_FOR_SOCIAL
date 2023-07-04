//image_resizer
use crate::social_plataform::SocialPlatform;
use image::{imageops, DynamicImage};
use std::io;
use std::path::Path;

/* //Lê o caminho de entrada de uma imagem que o usuário deseja redimensionar
pub fn read_input_path() -> String {
    let mut input = String::new();
    println!("enter the path of the image you want to resize:");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
} */

//Lê o caminho de saída para salvar a imagem
pub fn read_output_path() -> String {
    let mut output = String::new();
    println!("Enter the name of the output file (it will be saved in the output folder): ");
    io::stdin().read_line(&mut output).unwrap();
    output.trim().to_string()
}


//estrutura do recurso de resize => valor de entrada, valor de saída e a rede social desejada 
pub struct ImageResizer<'a> {
    input_path: &'a str,
    output_path: String,
    social_plataform: SocialPlatform,
}
//image_resizer.rs
impl<'a> ImageResizer<'a> {
    pub fn new(
        input_path: &'a str,
        output_path_name: &str,
        social_plataform_name: &str,
    ) -> Option<ImageResizer<'a>> {
        let output_path = format!("output/{}", output_path_name); //será salvo na pasta output do diretório raiz 
        match SocialPlatform::new(social_plataform_name) {
            Some(social_plataform) => Some(ImageResizer {
                input_path,       
                output_path,      
                social_plataform, 
            }),
            None => None,
        }
    }

    pub fn resize(&self) -> DynamicImage {
        let img = self.load_input_image();
        let resized_img = imageops::resize(
            &img, //guarda a imagem que foi salva na função load_input_image
            self.social_plataform.width, //referencia a largura da estrutura SocialPlataform
            self.social_plataform.height, //referencia a altura da estrutura SocialPlatform
            imageops::FilterType::CatmullRom, //escolhi um filtro que equilibra qualidade e velicocidade
        );
        DynamicImage::ImageRgba8(resized_img)
    }

    //método responsável por salvar a imagem no output (essa função é chamada no main)
    pub fn save_output_image(&self, img: &DynamicImage) {
        if let Err(_) = img.save(&self.output_path) {
            eprintln!("Could not save output image '{}'", self.output_path);
            std::process::exit(1)
        }
    }

    //lê a imagem antes de salvar no output 
    fn load_input_image(&self) -> DynamicImage {
        match image::open(&Path::new(self.input_path)) {
            Ok(img) => img,
            Err(_) => {
                eprintln!("Could not open input image '{}'", self.input_path);
                std::process::exit(1)
            }
        }
    }
}
