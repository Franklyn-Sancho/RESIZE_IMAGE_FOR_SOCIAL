use image::DynamicImage;

pub struct IOOperator<'a> {
    input_data: &'a [u8],
    output_path: String,
}

impl<'a> IOOperator<'a> {
    pub fn new(input_data: &'a [u8], output_path_name: &str) -> IOOperator<'a> {
        let output_path = format!("output/{}", output_path_name);
        IOOperator {
            input_data,
            output_path,
        }
    }

    pub fn save_output_image(&self, img: &DynamicImage) {
        if let Err(_) = img.save(&self.output_path) {
            eprintln!("Could not save output image '{}'", self.output_path);
            std::process::exit(1)
        }
    }
}
