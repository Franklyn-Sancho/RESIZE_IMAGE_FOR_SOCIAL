//struct das redes sociais

use std::collections::HashMap;

use crate::utils::read_input::read_input;

//Estrutura das redes sociais
#[derive(Debug)]
pub struct SocialPlatform {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

// => preciso adicionar outras redes sociais
impl SocialPlatform {
    pub fn new(name: &str) -> Option<Self> {
        let platform = Self::platform_data();

        platform.get(name).map(|&(width, height)| SocialPlatform {
            name: name.to_string(),
            width,
            height,
        })
    }

    fn platform_data() -> HashMap<&'static str, (u32, u32)> {
        HashMap::from([
            ("Facebook", (1200, 630)),
            ("Instagram", (1080, 1080)),
            ("Twitter", (1024, 512)),
            ("Linkedin", (1200, 1200)),
        ])
    }
}

/// Lê a rede social desejada pelo usuário.
pub fn input_social_platform() -> String {
    println!("Enter the social platform name: (Facebook, Instagram, Twitter, Linkedin)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string() // Retorna apenas o nome como `String`
}

