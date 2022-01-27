use image::io::Reader as ImageReader;
use image::DynamicImage;

use crate::filters;

pub trait ImageFilter {
    fn quantize(img: &mut DynamicImage, level: u8) -> DynamicImage;
    fn quantize_rgb(img: &mut DynamicImage, level: u8) -> DynamicImage;
}

pub struct CustomImage {
    pub internal_image: DynamicImage,
    pub path: String,
    pub filename: String,
    pub full_path: String,
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
            path,
            filename,
            full_path,
        }
    }
}

impl ImageFilter for CustomImage {
    fn quantize(img: &mut DynamicImage, level: u8) -> DynamicImage {
        filters::quantize_n_levels::run(img, level)
    }

    fn quantize_rgb(img: &mut DynamicImage, level: u8) -> DynamicImage {
        filters::quantize_n_levels_rgb::run(img, level)
    }
}
