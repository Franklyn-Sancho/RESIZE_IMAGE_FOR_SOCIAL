use std::fs;

use super::read_input::read_input;

pub fn read_input_path(prompt: &str) -> String {
    read_input(prompt)
}

fn image_file_accept(file_name: &str) -> bool {
    let valid_extensions = ["jpg", "jpeg", "png", "gif", "bmp"];
    valid_extensions.iter().any(|ext| file_name.ends_with(ext))
}

//método para listar os arquivos aceitos e controlar a entrada do usuário
pub fn select_file_from_dir(dir_path: &str) -> Result<String, String> {
    let files = list_file_in_dir(dir_path)?;

    println!("Accepted files:");
    for file in &files {
        println!("{}", file);
    }

    loop {
        let input = read_input_path("Enter the image you want to adjust: ");
        if files.contains(&input) {
            return Ok(input);
        } else {
            eprintln!("Error: The file is not in the list of accepted files. Please try again.");
        }
    }
}

fn list_file_in_dir(dir_path: &str) -> Result<Vec<String>, String> {
    let entries =
        fs::read_dir(dir_path).map_err(|_| format!("\nCould not read directory '{}'", dir_path))?;

    let files: Vec<String> = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|entry| entry.file_name().to_string_lossy().into_owned().into())
        .filter(|file_name| image_file_accept(file_name))
        .collect();

    Ok(files)
}
