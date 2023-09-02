use std::fs;

use crate::image_resizer::read_input_path;

/**
 * método para filtrar os tipos que são aceitos
 * a biblioteca image aceita outros tipos, 
 * mas por enquanto esta aplicaçaõ só funciona com estas
 */
fn image_file_accept(file_name: &str) -> bool {
    file_name.ends_with(".png")
        || file_name.ends_with(".jpg")
        || file_name.ends_with(".jpeg")
        || file_name.ends_with(".gif")
}

//método para listar os arquivos aceitos e controlar a entrada do usuário
pub fn select_file_from_dir(dir_path: &str) -> Result<String, String> {
    let files = list_file_in_dir(dir_path)?;
    println!("Accepted files:");
    for file in &files {
        println!("{}", file);
    }
    loop {
        let input_path = read_input_path();
        //se o valor do input não etstiver contido em files, o arquivo retorna um erro
        if !files.contains(&input_path) {
            eprintln!("Error: The file is not in the list of accepted files. Please try again.")
        } else {
            return Ok(input_path);
        }
    }
}

//método para listar os arquivos do diretório atual (arquivos aceitos pela aplicação)
pub fn list_file_in_dir(dir_path: &str) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    let entries = fs::read_dir(dir_path).map_err(|_| {
        format!(
            "
        Could not read directory '{}'",
            dir_path,
        )
    })?;
    //iterando as entradas sobre o diretório
    for entry in entries {
        //verifica se a entrada é válida, caso positivo, será atribuído ao entry
        if let Ok(entry) = entry {
            //verifica se o tipo de arquivo pode ser obtivo, caso positivo, é atribuido ao file_type
            if let Ok(file_type) = entry.file_type() {
                //verifica se o valor de entrada é um arquivo
                if file_type.is_file() {
                    //converte em uma string
                    let file_name = entry.file_name().to_string_lossy().into_owned();
                    //verifica se o arquivo é aceito pela aplicação (por enquanto aplicaçaõ só aceita essas)
                    if image_file_accept(&file_name) {
                        //o nome do arquivo é adicionado ao vetor
                        files.push(file_name);
                    }
                }
            }
        }
    }
    Ok(files)
}
