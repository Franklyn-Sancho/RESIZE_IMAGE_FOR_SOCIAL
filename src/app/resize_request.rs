use serde::Deserialize;

use super::image_utils::decode_input_data;

#[derive(Deserialize)]
pub struct ResizeRequest {
    pub input_data: String,
    pub social_platform_name: String,
}

pub fn validate_and_transform_resize_request(
    req: &ResizeRequest,
) -> Result<(Vec<u8>, &str), actix_web::Error> {
    let input_data = decode_input_data(&req.input_data);
    let social_platform_name = &req.social_platform_name;
    Ok((input_data, social_platform_name))
}