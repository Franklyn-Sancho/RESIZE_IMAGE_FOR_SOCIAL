//handlers.rs

use actix_files::NamedFile;
use actix_web::{web, Error, HttpResponse, Responder};
use image::DynamicImage;
use serde_derive::Deserialize;

use crate::{image_resizer::ImageResizer, image_rotate::{Rotation, self}};

#[derive(Deserialize)]
pub struct ResizeRotateRequest {
    input_data: String,
    output_path_name: String,
    social_platform_name: String,
    rotation: Option<String>,
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

pub async fn download_image(req: web::Path<String>) -> Result<NamedFile, Error> {
    let filename = req.into_inner();
    let filepath = format!("output/{}", filename);
    Ok(NamedFile::open(filepath)?)
}

async fn resize_image(req: &ResizeRotateRequest) -> Result<DynamicImage, HttpResponse> {
    let input_data = decode_input_data(&req.input_data);
    let resizer = create_resizer(
        input_data.clone(),
        &req.output_path_name,
        &req.social_platform_name,
    );

    if let Some(resizer) = resizer {
        // Redimensionar a imagem
        let resized_img = resize_image_data(&input_data, &resizer);
        Ok(resized_img)
    } else {
        Err(HttpResponse::BadRequest().body("Parâmetros inválidos"))
    }
}

async fn rotate_image(
    req: &ResizeRotateRequest,
    img: DynamicImage,
) -> Result<DynamicImage, HttpResponse> {
    // Verifique se a rotação é necessária
    if let Some(rotation) = &req.rotation {
        // Se a rotação não for None, converta-a em uma constante Rotation
        let rotation = match rotation.as_str() {
            "None" => Rotation::None,
            "Right90" => Rotation::Right90,
            "Left90" => Rotation::Left90,
            "HalfCircle" => Rotation::HalfCircle,
            _ => return Err(HttpResponse::BadRequest().body("Valor de rotação inválido")),
        };

        // Rotacionar a imagem
        let rotated_img = image_rotate::rotate_image(&img, rotation);
        Ok(rotated_img)
    } else {
        // O campo `rotation` está vazio, então apenas retorne a imagem original
        Ok(img)
    }
}



pub async fn resize_rotate_handler(req: web::Json<ResizeRotateRequest>) -> impl Responder {
    // Redimensionar a imagem
    let resized_img = match resize_image(&req).await {
        Ok(img) => img,
        Err(response) => return response,
    };

    // Rotacionar a imagem (se necessário)
    let final_img = match rotate_image(&req, resized_img).await {
        Ok(img) => img,
        Err(response) => return response,
    };

    // Salvar a imagem final
    let resizer = create_resizer(
        Vec::new(),
        &req.output_path_name,
        &req.social_platform_name,
    )
    .unwrap();
    resizer.save_output_image(&final_img);

    // Retornar uma resposta de sucesso
    HttpResponse::Ok().body("Imagem processada com sucesso")
}