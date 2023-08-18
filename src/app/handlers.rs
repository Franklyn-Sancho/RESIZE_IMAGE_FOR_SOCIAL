//handlers.rs

use actix_files::NamedFile;
use actix_web::{error, web, Error, HttpResponse};
use image::DynamicImage;
use serde_derive::Deserialize;
use uuid::Uuid;

use crate::{
    image_resizer::ImageResizer,
    image_rotate::{self, Rotation},
};

#[derive(Deserialize)]
pub struct ResizeRequest {
    input_data: String,
    /* output_path_name: String, */
    social_platform_name: String,
}

#[derive(Deserialize)]
pub struct RotateRequest {
    input_data: String,
    rotation: Option<String>,
}

//decodifica os dados de entrada
fn decode_input_data(input_data: &str) -> Vec<u8> {
    base64::decode(input_data).unwrap()
}

//cria um objeto ImageResizer
fn create_resizer(
    input_data: Vec<u8>,
    social_plataform_name: &str,
) -> Option<ImageResizer> {
    let filename = format!("resized_image_{}.jpg", Uuid::new_v4());
    let output_path = format!("/tmp/{}", filename);
    ImageResizer::new(input_data, &output_path, social_plataform_name)
}

//redimensiona a imagem
fn resize_image_data(input_data: &[u8], resizer: &ImageResizer) -> DynamicImage {
    let img = image::load_from_memory(input_data).unwrap();
    resizer.resize(&img)
}

//handlers

pub async fn download_image(req: web::Path<String>) -> Result<NamedFile, Error> {
    let filename = req.into_inner();
    let filepath = format!("/tmp/{}", filename);
    Ok(NamedFile::open(filepath)?)
}

pub async fn rotate_handler(
    req: web::Json<RotateRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let input_data = decode_input_data(&req.input_data);
    let img = image::load_from_memory(&input_data).unwrap();

    if let Some(rotation) = &req.rotation {
        let rotation = match rotation.as_str() {
            "None" => Rotation::None,
            "Right90" => Rotation::Right90,
            "Left90" => Rotation::Left90,
            "HalfCircle" => Rotation::HalfCircle,
            _ => {
                return Err(error::ErrorBadRequest("Valor de rotação inválido"));
            }
        };

        let rotated_img = image_rotate::rotate_image(&img, rotation);
        let filename = format!("rotated_image_{}.jpg", Uuid::new_v4());
        let filepath = format!("/tmp/{}", filename);
        rotated_img.save(filepath).unwrap();
        // Retornar uma resposta de sucesso com o nome do arquivo da imagem
        Ok(HttpResponse::Ok().json(filename))
    } else {
        Ok(HttpResponse::Ok().finish())
    }
}

pub async fn resize_handler(
    req: web::Json<ResizeRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    // Redimensionar a imagem
    let input_data = decode_input_data(&req.input_data);
    println!("{:?}", input_data);
    let resizer = create_resizer(
        input_data.clone(),
        
        &req.social_platform_name,
    );

    if let Some(resizer) = resizer {
        // Redimensionar a imagem
        let resized_img = resize_image_data(&input_data, &resizer);
        // Salvar a imagem final em um local temporário no servidor
        let filename = format!("resized_image_{}.jpg", Uuid::new_v4());
        let filepath = format!("/tmp/{}", filename);
        resized_img.save(filepath).unwrap();
        // Retornar uma resposta de sucesso com o nome do arquivo da imagem
        Ok(HttpResponse::Ok().json(filename))
    } else {
        return Err(error::ErrorBadRequest("Parâmetros inválidos"));
    }
}

