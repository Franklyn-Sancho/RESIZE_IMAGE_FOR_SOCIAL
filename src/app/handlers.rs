//handlers.rs

use actix_files::NamedFile;
use actix_web::{ error, web, Error, HttpResponse };
use serde_derive::Deserialize;
use uuid::Uuid;
use serde_json::json;

use crate::{
    app::image_utils::{ create_resizer, resize_image_data },
    image_adjust::{ adjust_image, Effect },
    image_rotate::{ self },
};

use super::{
    handlers_request::{
        validate_and_transform_convert_request,
        validate_and_transform_resize_request,
        validate_and_transform_rotate_request,
        ConvertRequest,
        ResizeRequest,
        RotateRequest,
    },
    image_utils::{ decode_input_data, save_image },
};

/* #[derive(Deserialize)]
pub struct RotateAndResizeRequest {
    input_data: String,
    rotation: Option<String>,
    social_platform_name: String,
    format: Option<String>,
    brightness: Option<i32>,
    contrast: Option<f32>,
    greyscale: Option<bool>,
} */

#[derive(Deserialize)]
pub struct AdjustRequest {
    pub input_data: String,
    pub brightness: Option<i32>,
    pub contrast: Option<f32>,
    pub greyscale: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct RotateResponse {
    filename: String,
}

#[derive(serde::Serialize)]
pub struct ResizeResponse {
    filename: String,
}

#[derive(serde::Serialize)]
pub struct ConvertResponse {
    filename: String,
}

#[derive(serde::Serialize)]
struct AdjustResponse {
    filename: String,
    adjustment: String, 
    value: Option<f64>, 
}

pub async fn resize_handler(
    req: web::Json<ResizeRequest> 
) -> Result<HttpResponse, actix_web::Error> {
    let filename = "resize_image.jpg"; 

    let (input_data, social_platform_name) = validate_and_transform_resize_request(&req)?;
    match create_resizer(social_platform_name) {
        Some(resizer) => {
            let resized_img = resize_image_data(&input_data, &resizer);
            save_image(&resized_img, filename);
            Ok(
                HttpResponse::Ok().json(ResizeResponse {
                    filename: filename.to_string(),
                })
            )
        }
        None => Err(actix_web::error::ErrorBadRequest("Parâmetros inválidos")),
    }
}

pub async fn rotate_handler(
    req: web::Json<RotateRequest>
) -> Result<HttpResponse, actix_web::Error> {
    let filename = "rotated_image.jpg"; 

    
    let (input_data, rotation) = validate_and_transform_rotate_request(&req)?;

    
    let img = image
        ::load_from_memory(&input_data)
        .map_err(|_| actix_web::error::ErrorBadRequest("Falha ao carregar a imagem"))?;

   
    let rotated_img = image_rotate::rotate_image(&img, rotation);

    
    save_image(&rotated_img, filename);

    
    Ok(
        HttpResponse::Ok().json(RotateResponse {
            filename: filename.to_string(),
        })
    )
}

pub async fn convert_handler(
    req: web::Json<ConvertRequest> 
) -> Result<HttpResponse, actix_web::Error> {
    
    let (input_data, format, extension) = validate_and_transform_convert_request(&req)?;

    
    let filename = format!("converted_image.{}", extension);
    let filepath = format!("/tmp/{}", filename);

    
    let img = image
        ::load_from_memory(&input_data)
        .map_err(|_| error::ErrorBadRequest("Falha ao carregar a imagem"))?;

    
    img
        .save_with_format(&filepath, format)
        .map_err(|_| error::ErrorBadRequest("Falha ao salvar a imagem no formato solicitado"))?;

    println!("Arquivo convertido salvo em: {}", filepath);

    
    Ok(
        HttpResponse::Ok().json(ConvertResponse {
            filename,
        })
    )
}

pub async fn adjust_handler(
    req: web::Json<AdjustRequest>
) -> Result<HttpResponse, actix_web::Error> {
    let filename = "adjusted_image.jpg";

    let input_data = match decode_input_data(&req.input_data) {
        Ok(data) => data,
        Err(_) => {
            return Ok(
                HttpResponse::BadRequest().json(
                    json!({
                "error": "Failed to decode input data"
            })
                )
            );
        }
    };

    let img = match image::load_from_memory(&input_data) {
        Ok(img) => img,
        Err(_) => {
            return Ok(
                HttpResponse::BadRequest().json(
                    json!({
                "error": "Failed to load image from memory"
            })
                )
            );
        }
    };

    let mut adjustment = "none".to_string();
    let mut value = None;

    let adjusted_img = if let Some(brightness) = req.brightness {
        adjustment = "brightness".to_string();
        value = Some(brightness as f64);
        adjust_image(&img, &Effect::Brightness(brightness))
    } else if let Some(contrast) = req.contrast {
        adjustment = "contrast".to_string();
        value = Some(contrast as f64);
        adjust_image(&img, &Effect::Contrast(contrast))
    } else if req.greyscale.unwrap_or(false) {
        adjustment = "greyscale".to_string();
        value = None;
        adjust_image(&img, &Effect::Grayscale)
    } else {
        img.clone()
    };

    let filepath = format!("/tmp/{}", filename);
    if let Err(_) = adjusted_img.save(&filepath) {
        return Ok(
            HttpResponse::InternalServerError().json(
                json!({
            "error": "Failed to save adjusted image"
        })
            )
        );
    }

    Ok(
        HttpResponse::Ok().json(AdjustResponse {
            filename: filename.to_string(),
            adjustment,
            value,
        })
    )
}

pub async fn download_image(path: web::Path<String>) -> Result<HttpResponse, actix_web::Error> {
    let filename = path.into_inner();
    let filepath = format!("/tmp/{}", filename);

    if std::path::Path::new(&filepath).exists() {
        let data = std::fs
            ::read(&filepath)
            .map_err(|_| {
                error::ErrorInternalServerError("Erro ao ler o arquivo do sistema de arquivos")
            })?;
        Ok(HttpResponse::Ok().content_type("application/octet-stream").body(data))
    } else {
        Err(error::ErrorNotFound("Arquivo não encontrado"))
    }
}

/* pub async fn process_image_handler(
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
} */
