use actix_web::error;
use serde::Deserialize;

use crate::image_rotate::Rotation;

use super::image_utils::decode_input_data;


#[derive(Deserialize)]
pub struct ResizeRequest {
    pub input_data: String,
    pub social_platform_name: String,
}

#[derive(Deserialize)]
pub struct RotateRequest {
    pub input_data: String,
    pub rotation: Option<String>,
}

#[derive(Deserialize)]
pub struct ConvertRequest {
    pub input_data: String,
    pub format: Option<String>,
}



pub fn validate_and_transform_resize_request(
    req: &ResizeRequest,
) -> Result<(Vec<u8>, &str), actix_web::Error> {
    let input_data = decode_input_data(&req.input_data)
        .map_err(|_| error::ErrorBadRequest("Falha ao decodificar os dados de entrada"))?;
    let social_platform_name = &req.social_platform_name;
    Ok((input_data, social_platform_name))
}


pub fn validate_and_transform_rotate_request(
    req: &RotateRequest,
) -> Result<(Vec<u8>, Rotation), actix_web::Error> {
    println!("Recebendo input_data: {}", req.input_data);

    let input_data = decode_input_data(&req.input_data)
        .map_err(|_| {
            eprintln!("Erro ao decodificar input_data");
            error::ErrorBadRequest("Falha ao decodificar os dados de entrada")
        })?;

    let rotation = match req.rotation.as_deref() {
        Some("None") => Rotation::None,
        Some("Right90") => Rotation::Right90,
        Some("Left90") => Rotation::Left90,
        Some("HalfCircle") => Rotation::HalfCircle,
        _ => {
            eprintln!("Valor de rotação inválido: {:?}", req.rotation);
            return Err(error::ErrorBadRequest("Valor de rotação inválido"));
        }
    };

    Ok((input_data, rotation))
}



pub fn validate_and_transform_convert_request(
    req: &ConvertRequest,
) -> Result<(Vec<u8>, image::ImageFormat, &'static str), actix_web::Error> {
    // Decodifica os dados de entrada.
    let input_data = decode_input_data(&req.input_data)
        .map_err(|_| error::ErrorBadRequest("Falha ao decodificar os dados de entrada"))?;

    // Determina o formato de saída e a extensão do arquivo.
    let (format, extension) = match req.format.as_deref() {
        Some("jpeg") => (image::ImageFormat::Jpeg, "jpeg"),
        Some("png") => (image::ImageFormat::Png, "png"),
        _ => return Err(error::ErrorBadRequest("Formato de conversão inválido")),
    };

    Ok((input_data, format, extension))
}




