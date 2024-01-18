//handlers.rs

use actix_files::NamedFile;
use actix_web::{error, web, Error, HttpResponse};
use serde_derive::Deserialize;
use uuid::Uuid;

use crate::{
    app::image_utils::{create_resizer, resize_image_data},
    image_adjust::{adjust_image, Effect},
    image_rotate::{self},
};

use super::{
    handlers_request::{
        validate_and_transform_convert_request, validate_and_transform_resize_request,
        validate_and_transform_rotate_request, ConvertRequest, ResizeRequest, RotateRequest,
    },
    image_utils::{decode_input_data, encode_input_data, read_image_data, save_image},
};

#[derive(Deserialize)]
pub struct RotateAndResizeRequest {
    input_data: String,
    rotation: Option<String>,
    social_platform_name: String,
    format: Option<String>,
    brightness: Option<i32>,
    contrast: Option<f32>,
    greyscale: Option<bool>,
}

#[derive(Deserialize)]
pub struct AdjustRequest {
    pub input_data: String,
    pub brightness: Option<i32>,
    pub contrast: Option<f32>,
    pub greyscale: Option<bool>,
}

pub fn resize_handler(
    req: &ResizeRequest,
    filename: &str,
) -> Result<HttpResponse, actix_web::Error> {
    let (input_data, social_platform_name) = validate_and_transform_resize_request(&req)?;
    match create_resizer(social_platform_name) {
        Some(resizer) => {
            let resized_img = resize_image_data(&input_data, &resizer);
            save_image(&resized_img, filename);
            Ok(HttpResponse::Ok().json(filename))
        }
        None => Err(error::ErrorBadRequest("Parâmetros inválidos")),
    }
}

pub fn rotate_handler(
    req: web::Json<RotateRequest>,
    filename: &str,
) -> Result<HttpResponse, actix_web::Error> {
    let (input_data, rotation) = validate_and_transform_rotate_request(&req)?;
    let img = image::load_from_memory(&input_data)
        .map_err(|_| error::ErrorBadRequest("Failed to load image from memory"))?;
    let rotated_img = image_rotate::rotate_image(&img, rotation);
    save_image(&rotated_img, filename);
    Ok(HttpResponse::Ok().json(filename))
}

pub fn convert_handler(
    req: web::Json<ConvertRequest>,
    filename: &str,
) -> Result<HttpResponse, actix_web::Error> {
    let (input_data, format) = validate_and_transform_convert_request(&req)?;
    let img = image::load_from_memory(&input_data)
        .map_err(|_| error::ErrorBadRequest("Failed to load image from memory"))?;
    img.save_with_format(filename, format)
        .map_err(|_| error::ErrorBadRequest("Failed to save image with format"))?;
    Ok(HttpResponse::Ok().json(filename))
}

pub async fn adjust_handler(
    req: web::Json<AdjustRequest>,
    filename: &str,
) -> Result<HttpResponse, actix_web::Error> {
    let input_data = decode_input_data(&req.input_data);
    let img = image::load_from_memory(&input_data).unwrap();

    let adjusted_img = match (req.brightness, req.contrast, req.greyscale) {
        (Some(brightness), _, _) => adjust_image(&img, Effect::Brightness(brightness)),
        (_, Some(contrast), _) => adjust_image(&img, Effect::Contrast(contrast)),
        (_, _, Some(true)) => adjust_image(&img, Effect::Grayscale),
        _ => img.clone(),
    };

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
    let _ = rotate_handler(rotate_req, &filename);

    let resize_req = web::Json(ResizeRequest {
        input_data: encode_input_data(&read_image_data(&filename)),
        social_platform_name: req.social_platform_name.clone(),
    });
    let _ = resize_handler(&resize_req, &filename);

    let convert_req = web::Json(ConvertRequest {
        input_data: encode_input_data(&read_image_data(&filename)),
        format: req.format.clone(),
    });
    let _ = convert_handler(convert_req, &filename);

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
