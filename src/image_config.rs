use image::{imageops, DynamicImage};

use crate::utils::read_input::read_input;


pub fn set_brightness(img: &DynamicImage) -> DynamicImage {
    let brightness: i32 = read_input("Enter the brightness value: ").parse().unwrap();

    let brightened_img = imageops::brighten(img, brightness);
    DynamicImage::ImageRgba8(brightened_img)
}

