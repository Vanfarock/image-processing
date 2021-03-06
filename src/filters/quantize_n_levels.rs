use image::{DynamicImage, GenericImage, GenericImageView};

use crate::custom_image::custom_image::{ImageFilter, CustomImage};
use crate::pixel_algorithms::quantization::quantize;

pub fn run(base_img: &mut DynamicImage, level: u8) -> Box<dyn ImageFilter> {
    let img = &mut base_img.clone();
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut pixel = img.get_pixel(x, y);

            let channels = &mut pixel.0;

            let quantize_value = quantize(channels[0], channels[1], channels[2], level);

            channels[0] = quantize_value;
            channels[1] = quantize_value;
            channels[2] = quantize_value;

            img.put_pixel(x, y, pixel);
        }
    }
    Box::new(CustomImage::from(img.clone()))
}
