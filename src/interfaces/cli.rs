use crate::{
    file_utils::select_file_from_dir,
    image_resizer::{read_output_path, ImageResizer},
    image_rotate::{self, rotate_image},
    social_plataform::input_social_plataform,
};

pub fn run_cli() {
    let dir_path = "."; //pasta atual -> diretório raiz da aplicação
    match select_file_from_dir(dir_path) {
        //retorna e controla apenas os arquivos válidos
        Ok(input_data) => {
            println!("Selected file: {}", input_data);
            let mut output_path = read_output_path();
            let social_plataform = input_social_plataform();
            let input_data = std::fs::read(input_data).unwrap();
            output_path.push_str(".jpg");
            let img = image::load_from_memory(&input_data).unwrap();
            let resizer = ImageResizer::new(input_data, &output_path, &social_plataform).unwrap();

            let resized_img = resizer.resize(&img);
            if image_rotate::ask_to_rotate() {
                // perguntar ao usuário se ele quer rotacionar a imagem
                let rotation = image_rotate::ask_rotation();
                let rotated_img = rotate_image(&resized_img, rotation);
                resizer.save_output_image(&rotated_img)
            } else {
                resizer.save_output_image(&resized_img)
            }
        }
        Err(e) => {
            eprintln!("{}", e)
        }
    };
}
