use image::DynamicImage;
use serde_derive::Deserialize;
use std::io;

#[derive(Deserialize, Clone)]
pub enum Rotation {
    None,
    Right90,
    Left90,
    HalfCircle,
}


//método principal do recurso de rotacionar as imagens
pub fn rotate_image(img: &DynamicImage, rotation: Rotation) -> DynamicImage {
    apply_rotation(img, rotation)

}

//após redimencionar a imagem, essa função é retornado para o usuário, perguntando se ele quer rotacionar
pub fn ask_to_rotate() -> bool {
    let mut to_rotate = String::new();
    println!("Do you want to rotate image (yes/no): ");
    io::stdin().read_line(&mut to_rotate).unwrap();
    let to_rotate = to_rotate.trim();
    to_rotate == "yes" //caso sim, ask_rotation é chamado
}

//ao digitar "yes", este método será invocado
pub fn ask_rotation() -> Rotation {
    let mut rotation = String::new();
    //melhorar as opções para algo mais intuitivo
    println!("Choose the rotation option: ");
    println!("1 - Rotate 90 degrees to the right");
    println!("2 - Rotate 90 degrees to the left");
    println!("3 - Rotate HalfCircle");
    io::stdin().read_line(&mut rotation).unwrap();
    let rotation = rotation.trim();

    //o enum Rotation é chamado no operador match
    match rotation {
        "1" => Rotation::Right90,
        "2" => Rotation::Left90,
        "3" => Rotation::HalfCircle,
        _ => {
            eprintln!("Invalid rotation option");
            std::process::exit(1)
        }
    }
}

//o método que aplica a rotação
fn apply_rotation(img: &DynamicImage, rotation: Rotation) -> DynamicImage {
    match rotation {
        Rotation::None => img.clone(),
        Rotation::Right90 => img.rotate90(),
        Rotation::Left90 => img.rotate270(),
        Rotation::HalfCircle => img.rotate180(),
    }
}
