//handlers.rs

use actix_files::NamedFile;
use actix_web::{web, Error, HttpResponse};
use image::DynamicImage;
use serde_derive::Deserialize;

use crate::image_resizer::ImageResizer;

#[derive(Deserialize)]
pub struct ResizeRequest {
    input_data: String,
    output_path_name: String,
    social_platform_name: String,
}

//decodifica os dados de entrada
fn decode_input_data(input_data: &str) -> Vec<u8> {
    base64::decode(input_data).unwrap()
}

//cria um objeto ImageResizer
fn create_resizer(
    input_data: Vec<u8>,
    output_path_name: &str,
    social_plataform_name: &str,
) -> Option<ImageResizer> {
    let mut output_path = output_path_name.to_string();
    output_path.push_str(".jpg");
    ImageResizer::new(input_data, output_path_name, social_plataform_name)
}

//redimensiona a imagem
fn resize_image_data(input_data: &[u8], resizer: &ImageResizer) -> DynamicImage {
    let img = image::load_from_memory(input_data).unwrap();
    resizer.resize(&img)
}

//handlers
pub async fn resize_image(req: web::Json<ResizeRequest>) -> Result<HttpResponse, Error> {
    let input_data = decode_input_data(&req.input_data);
    let resizer = create_resizer(
        input_data.clone(),
        &req.output_path_name,
        &req.social_platform_name,
    );

    if let Some(resizer) = resizer {
        // Redimensionar a imagem
        let resized_img = resize_image_data(&input_data, &resizer);
        // Salvar a imagem redimensionada
        resizer.save_output_image(&resized_img);

        // Retornar uma resposta de sucesso
        Ok(HttpResponse::Ok().body("Image resized successfully"))
    } else {
        Ok(HttpResponse::BadRequest().body("Invalid parameters"))
    }
}

pub async fn download_image(req: web::Path<String>) -> Result<NamedFile, Error> {
    let filename = req.into_inner();
    let filepath = format!("output/{}", filename);
    Ok(NamedFile::open(filepath)?)
}
