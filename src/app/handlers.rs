//handlers.rs

use actix_files::NamedFile;
use actix_web::{error, web, Error, HttpResponse};
use serde_derive::Deserialize;
use uuid::Uuid;

use crate::{
    app::image_utils::{create_resizer, resize_image_data},
    image_rotate::{self, Rotation}, image_config::{set_brightness, set_contrast, set_grayscale},
};

use super::image_utils::{decode_input_data, encode_input_data, read_image_data, save_image};

#[derive(Deserialize)]
pub struct ResizeRequest {
    input_data: String,
    social_platform_name: String,
}

pub struct ConvertRequest {
    input_data: String,
    format: Option<String>
}

#[derive(Deserialize)]
pub struct RotateRequest {
    input_data: String,
    rotation: Option<String>,
}

#[derive(Deserialize)]
pub struct RotateAndResizeRequest {
    input_data: String,
    rotation: Option<String>,
    social_platform_name: String,
    format: Option<String>,
    brightness: Option<i32>,
    contrast: Option<f32>,
    greyscale:  Option<bool>,
}

#[derive(Deserialize)]
pub struct AdjustRequest {
    pub input_data: String,
    pub brightness: Option<i32>,
    pub contrast: Option<f32>,
    pub greyscale: Option<bool>,
}


pub async fn rotate_handler(
    req: web::Json<RotateRequest>,
    filename: &str,
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
        /* let filename = format!("rotated_image_{}.jpg", Uuid::new_v4()); */
        save_image(&rotated_img, filename);
        Ok(HttpResponse::Ok().json(filename))
        // Retornar uma resposta de sucesso com o nome do arquivo da imagem
    } else {
        Ok(HttpResponse::Ok().finish())
    }
}

pub async fn resize_handler(
    req: &ResizeRequest,
    filename: &str,
) -> Result<HttpResponse, actix_web::Error> {
    // Redimensionar a imagem
    let input_data = decode_input_data(&req.input_data);
    let resizer = create_resizer(&input_data, &req.social_platform_name);

    if let Some(resizer) = resizer {
        let resized_img = resize_image_data(&input_data, &resizer);
        save_image(&resized_img, filename);
        Ok(HttpResponse::Ok().json(filename))
    } else {
        Err(error::ErrorBadRequest("Parâmetros inválidos"))
    }
}

pub async fn convert_handler(
    req: web::Json<ConvertRequest>,
    filename: &str,
) -> Result<HttpResponse, actix_web::Error> {
    let input_data = decode_input_data(&req.input_data);

    if let Some(format) = req.format.as_deref() {
        let extension = match format {
            "jpeg" => ".jpeg",
            "png" => ".png",
            _ => return Err(error::ErrorBadRequest("Invalid conversion format")),
        };

        let img = image::load_from_memory(&input_data).unwrap();
        let output_filename = format!("{}{}", filename, extension);
        img.save(&output_filename).unwrap();
        Ok(HttpResponse::Ok().json(filename))
    } else {
        Ok(HttpResponse::Ok().finish())
    }
}

pub async fn adjust_handler(
    req: web::Json<AdjustRequest>,
    filename: &str,
) -> Result<HttpResponse, actix_web::Error> {
    // Imprima os valores de brilho e contraste
    println!("Brightness: {:?}", req.brightness);
    println!("Contrast: {:?}", req.contrast);

    let input_data = decode_input_data(&req.input_data);
    let img = image::load_from_memory(&input_data).unwrap();
    let mut adjusted_img = img.clone();

    if let Some(brightness) = req.brightness {
        adjusted_img = set_brightness(&adjusted_img, brightness);
    }

    if let Some(contrast) = req.contrast {
        adjusted_img = set_contrast(&adjusted_img, contrast);
    }

    if req.greyscale.unwrap_or(false) {
        adjusted_img = set_grayscale(&adjusted_img);
    }

    save_image(&adjusted_img, filename);
    Ok(HttpResponse::Ok().json(filename))
}




pub async fn process_image_handler(
    req: web::Json<RotateAndResizeRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    
    let filename = format!("new_image_{}.jpg", Uuid::new_v4());

    let rotate_req = web::Json(RotateRequest {
        input_data: req.input_data.clone(),
        rotation: req.rotation.clone(),
    });
    let _ = rotate_handler(rotate_req, &filename).await;

    let resize_req = web::Json(ResizeRequest {
        input_data: encode_input_data(&read_image_data(&filename)),
        social_platform_name: req.social_platform_name.clone(),
    });
    let _ = resize_handler(&resize_req, &filename).await;

    let convert_req = web::Json(ConvertRequest {
        input_data: encode_input_data(&read_image_data(&filename)),
        format: req.format.clone(),
    });
    let _ = convert_handler(convert_req, &filename).await;

    let adjust_req = web::Json(AdjustRequest {
        input_data: encode_input_data(&read_image_data(&filename)),
        brightness: req.brightness.clone(),
        contrast: req.contrast.clone(),
        greyscale: req.greyscale.clone(),
    });
    let _ = adjust_handler(adjust_req, &filename).await;

    let image_data = read_image_data(&filename);

    Ok(HttpResponse::Ok().body(image_data))
}


pub async fn download_image(req: web::Path<String>) -> Result<NamedFile, Error> {
    let filename = req.into_inner();
    let filepath = format!("/tmp/{}", filename);
    Ok(NamedFile::open(filepath)?)
}
