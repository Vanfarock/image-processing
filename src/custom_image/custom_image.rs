use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageError};

use crate::filters;

pub trait Image {
    fn save(&self, path: String) -> Result<(), ImageError>;
}

pub trait ImageFilter: Image {
    fn quantize(&mut self, level: u8) -> Box<dyn ImageFilter>;
    fn quantize_rgb(&mut self, level: u8) -> Box<dyn ImageFilter>;
    fn brighten(&mut self, brightness: f32) -> Box<dyn ImageFilter>;
    fn floyd_steinberg(&mut self, level: u8) -> Box<dyn ImageFilter>;
    fn floyd_steinberg_rgb(&mut self, level: u8) -> Box<dyn ImageFilter>;
    fn pixelize(&mut self, step_size: usize) -> Box<dyn ImageFilter>;
    // fn average_blur(&mut self, radius: u32) -> Box<dyn ImageFilter>;
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

    pub fn from(img: DynamicImage) -> Self {
        CustomImage {
            internal_image: img,
        }
    }
}

impl Image for CustomImage {
    fn save(&self, path: String) -> Result<(), ImageError> {
        self.internal_image.save(path)
    }
}

impl ImageFilter for CustomImage {
    fn quantize(&mut self, level: u8) -> Box<dyn ImageFilter> {
        filters::quantize_n_levels::run(&mut self.internal_image.clone(), level)
    }

    fn quantize_rgb(&mut self, level: u8) -> Box<dyn ImageFilter> {
        filters::quantize_n_levels_rgb::run(&mut self.internal_image.clone(), level)
    }

    fn brighten(&mut self, brightness: f32) -> Box<dyn ImageFilter> {
        filters::brightness::run(&mut self.internal_image.clone(), brightness)
    }

    fn floyd_steinberg(&mut self, level: u8) -> Box<dyn ImageFilter> {
        filters::floyd_steinberg::run(&mut self.internal_image.clone(), level)
    }

    fn floyd_steinberg_rgb(&mut self, level: u8) -> Box<dyn ImageFilter> {
        filters::floyd_steinberg_rgb::run(&mut self.internal_image.clone(), level)
    }

    fn pixelize(&mut self, step_size: usize) -> Box<dyn ImageFilter> {
        filters::pixelize::run(&mut self.internal_image.clone(), step_size)
    }
}
