//handlers.rs

use actix_files::NamedFile;
use actix_web::{web, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use serde_derive::Deserialize;

use crate::image_resizer::ImageResizer;

#[derive(Deserialize)]
pub struct ResizeRequest {
    input_data: String,
    output_path_name: String,
    social_platform_name: String,
}

pub async fn resize_image(req: web::Json<ResizeRequest>) -> Result<HttpResponse, Error> {
    let input_data = base64::decode(&req.input_data).unwrap();
    let mut output_path_name = req.output_path_name.clone();
    let social_platform_name = &req.social_platform_name;

    output_path_name.push_str(".jpg");

    // Carregar a imagem a partir dos dados
    let img = image::load_from_memory(&input_data).unwrap();

    // Criar um objeto ImageResizer
    let resizer = ImageResizer::new(input_data, &output_path_name, social_platform_name);

    if let Some(resizer) = resizer {
        // Redimensionar a imagem
        let resized_img = resizer.resize(&img);

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

