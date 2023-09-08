use actix_web::error;
use serde::Deserialize;

use super::image_utils::decode_input_data;

#[derive(Deserialize)]
pub struct ConvertRequest {
    pub input_data: String,
    pub format: Option<String>,
}

pub fn validate_and_transform_convert_request(
    req: &ConvertRequest,
) -> Result<(Vec<u8>, image::ImageFormat), actix_web::Error> {
    let input_data = decode_input_data(&req.input_data);
    let format = match req.format.as_deref() {
        Some("jpeg") => image::ImageFormat::Jpeg,
        Some("png") => image::ImageFormat::Png,
        _ => return Err(error::ErrorBadRequest("Invalid conversion format")),
    };
    Ok((input_data, format))
}