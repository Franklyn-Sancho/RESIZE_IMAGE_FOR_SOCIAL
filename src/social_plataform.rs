//struct das redes sociais

pub struct SocialPlatform {
    pub name: String,
    pub width: u32,
    pub height: u32,
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
