//struct das redes sociais

use crate::utils::read_input::read_input;

//Estrutura das redes sociais
pub struct SocialPlatform {
    pub name: String, 
    pub width: u32,   
    pub height: u32,  
}

//lê a rede social que o usuário deseja
pub fn input_social_plataform() -> String {
    read_input("Enter the name of the social media platform: (Facebook, Instagram, Twitter ou Linkedin)")
}

// => preciso adicionar outras redes sociais
impl SocialPlatform {
    pub fn new(name: &str) -> Option<SocialPlatform> {
        match name {
            "Facebook" => Some(SocialPlatform {
                name: name.to_string(),
                width: 1200,
                height: 630,
            }),
            "Instagram" => Some(SocialPlatform {
                name: name.to_string(),
                width: 1080,
                height: 1080,
            }),
            "Twitter" => Some(SocialPlatform {
                name: name.to_string(),
                width: 1024,
                height: 512,
            }),
            "Linkedin" => Some(SocialPlatform {
                name: name.to_string(),
                width: 1200,
                height: 1200,
            }),
            _ => None,
        }
    }
}
