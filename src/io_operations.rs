use image::DynamicImage;

pub struct IOOperator {
    output_path: String,
}

impl IOOperator {
    pub fn new(output_path_name: &str) -> IOOperator {
        let output_path = format!("output/{}", output_path_name);
        IOOperator {
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


