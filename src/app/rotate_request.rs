use actix_web::error;
use serde::Deserialize;

use crate::image_rotate::Rotation;

use super::image_utils::decode_input_data;

#[derive(Deserialize)]
pub struct RotateRequest {
    pub input_data: String,
    pub rotation: Option<String>,
}


pub fn validate_and_transform_rotate_request(
    req: &RotateRequest,
) -> Result<(Vec<u8>, Rotation), actix_web::Error> {
    let input_data = decode_input_data(&req.input_data);
    let rotation = match req.rotation.as_deref() {
        Some("None") => Rotation::None,
        Some("Right90") => Rotation::Right90,
        Some("Left90") => Rotation::Left90,
        Some("HalfCircle") => Rotation::HalfCircle,
        _ => return Err(error::ErrorBadRequest("Valor de rotação inválido")),
    };
    Ok((input_data, rotation))
}