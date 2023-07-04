//handlers.rs
use std::{collections::HashMap};

use crate::image_resizer::ImageResizer;
use actix_web::{web, HttpRequest, HttpResponse, Responder};



// => rodando em ambiente local
pub async fn resize_image(req: HttpRequest) -> impl Responder {
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
}

/* async fn resize_image(mut payload: Multipart) -> Result<HttpResponse, dyn Error> {
     //Lê o arquivo enviado pelo cliente
} */
