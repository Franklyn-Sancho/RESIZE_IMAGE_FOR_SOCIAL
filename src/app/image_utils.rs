use image::DynamicImage;
use uuid::Uuid;

use crate::image_resizer::ImageResizer;


//decodifica os dados de entrada
pub fn decode_input_data(input_data: &str) -> Vec<u8> {
    base64::decode(input_data).unwrap()
}

pub fn encode_input_data(input_data: &[u8]) -> String {
    base64::encode(input_data)
}

//cria um objeto ImageResizer
pub fn create_resizer<'a>(
    input_data: &'a [u8],
    social_plataform_name: &str,
) -> Option<ImageResizer<'a>> {
    let filename = format!("resized_image_{}.jpg", Uuid::new_v4());
    let output_path = format!("/tmp/{}", filename);
    ImageResizer::new(input_data, &output_path, social_plataform_name)
}

//redimensiona a imagem
pub fn resize_image_data(input_data: &[u8], resizer: &ImageResizer) -> DynamicImage {
    let img = image::load_from_memory(input_data).unwrap();
    resizer.resize(&img)
}

pub fn read_image_data(filename: &str) -> Vec<u8> {
    let filepath = format!("/tmp/{}", filename);
    std::fs::read(filepath).unwrap()
}

pub fn save_image(image: &image::DynamicImage, filename: &str) {
    let filepath = format!("/tmp/{}", filename);
    image.save(filepath).unwrap()
}


