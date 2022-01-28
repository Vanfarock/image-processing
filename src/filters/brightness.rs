use image::{DynamicImage, GenericImageView, GenericImage};

use crate::custom_image::custom_image::{ImageFilter, CustomImage};
use crate::pixel_algorithms::brightness::brighten;

pub fn run(base_img: &mut DynamicImage, brightness: f32) -> Box<dyn ImageFilter> {
    let img = &mut base_img.clone();
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut pixel = img.get_pixel(x, y);

            let channels = &mut pixel.0;
            
            let (b_red, b_green, b_blue) = brighten(channels[0],channels[1], channels[2], brightness);

            channels[0] = b_red;
            channels[1] = b_green;
            channels[2] = b_blue;

            img.put_pixel(x, y, pixel);
        }
    }
    Box::new(CustomImage::from(img.clone()))
}