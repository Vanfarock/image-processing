use image::io::Reader as ImageReader;
use image::DynamicImage;

use crate::filters;

pub trait ImageFilter {
    fn quantize(&mut self, level: u8) -> DynamicImage;
    fn quantize_rgb(&mut self, level: u8) -> DynamicImage;
}

pub struct CustomImage {
    pub internal_image: DynamicImage,
}

impl CustomImage {
    pub fn new(path: String, filename: String) -> Self {
        let full_path = format!(
            "{}{}",
            path.trim_end_matches("/"),
            filename.trim_start_matches("/")
        );
        let internal_image = ImageReader::open(&full_path).unwrap().decode().unwrap();

        CustomImage {
            internal_image,
        }
    }
}

impl ImageFilter for CustomImage {
    fn quantize(&mut self, level: u8) -> DynamicImage {
        filters::quantize_n_levels::run(&mut self.internal_image, level)
    }

    fn quantize_rgb(&mut self, level: u8) -> DynamicImage {
        filters::quantize_n_levels_rgb::run(&mut self.internal_image, level)
    }
}
