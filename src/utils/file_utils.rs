use std::fs;

use super::read_input::read_input;

pub fn read_input_path() -> String {
    read_input("Enter the file you want to adjust: ")
}

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
    let input_path = loop {
        let input = read_input_path();
        match files.iter().find(|&file| file == &input) {
            Some(file) => break file.to_string(),
            None => {
                eprintln!("Error: The file is not in the list of accepted files. Please try again.")
            }
        }
    };
    Ok(input_path)
}

fn list_file_in_dir(dir_path: &str) -> Result<Vec<String>, String> {
    let entries =
        fs::read_dir(dir_path).map_err(|_| format!("\nCould not read directory '{}'", dir_path))?;

    let files: Vec<String> = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|file_name| image_file_accept(file_name))
        .collect();

    Ok(files)
}
