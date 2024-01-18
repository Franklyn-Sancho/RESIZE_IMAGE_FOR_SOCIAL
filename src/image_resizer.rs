//image_resizer
use crate::social_plataform::SocialPlatform;
use image::{imageops, DynamicImage};


pub struct ImageResizer {
    social_platform: SocialPlatform,
}

impl ImageResizer {
    pub fn new(social_platform_name: &str) -> Option<ImageResizer> {
        match SocialPlatform::new(social_platform_name) {
            Some(social_platform) => Some(ImageResizer {
                social_platform,
            }),
            None => None,
        }
    }

    pub fn resize(&self, img: &DynamicImage) -> DynamicImage {
        let resized_img = imageops::resize(
            img,
            self.social_platform.width, 
            self.social_platform.height, 
            imageops::FilterType::Lanczos3,
        );
        DynamicImage::ImageRgba8(resized_img)
    }
}


