//struct das redes sociais

use std::io;

pub struct SocialPlatform {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

pub fn input_social_plataform() -> String {
    let mut platform = String::new();
    println!("Enter the name of the social media platform: ");
    io::stdin().read_line(&mut platform).unwrap();
    platform.trim().to_string()
}

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
            _ => None
        }
    }
}
