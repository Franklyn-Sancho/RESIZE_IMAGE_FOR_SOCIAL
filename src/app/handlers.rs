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
    let output_path_name = &req.output_path_name;
    let social_platform_name = &req.social_platform_name;

    println!("Valor recebido em input_data: {:?}", req.input_data);

    // Carregar a imagem a partir dos dados
    let img = image::load_from_memory(&input_data).unwrap();

    // Criar um objeto ImageResizer
    let resizer = ImageResizer::new(input_data, output_path_name, social_platform_name);

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

//lê o arquivo direto do computador
/* pub async fn resize_image(req: HttpRequest) -> impl Responder {
    let query: web::Query<HashMap<String, String>> =
        web::Query::from_query(req.query_string()).unwrap();

    let input_path = query
        .get("input_path")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "".to_string());
    let output_path_name = query
        .get("output_path_name")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "".to_string());
    let social_platform_name = query
        .get("social_platform_name")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "".to_string());
    let resizer = ImageResizer::new(&input_path, &output_path_name, &social_platform_name);
    if let Some(resizer) = resizer {
        let img = resizer.resize();

        resizer.save_output_image(&img);
        // Aqui você pode salvar a imagem redimensionada ou retorná-la na resposta
        HttpResponse::Ok().body("Imagem redimensionada com sucesso")
    } else {
        HttpResponse::BadRequest().body("Parâmetros inválidos")
    }
} */
