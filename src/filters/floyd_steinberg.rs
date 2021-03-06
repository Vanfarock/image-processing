use image::{DynamicImage, GenericImageView, GenericImage};

use crate::custom_image::custom_image::{ImageFilter, CustomImage};
use crate::pixel_algorithms::quantization::quantize;

pub fn run(base_img: &mut DynamicImage, level: u8) -> Box<dyn ImageFilter> {
    let img = base_img;
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let mut pixel = img.get_pixel(x, y);

            let channels = &mut pixel.0;
            
            let pixel_average = ((channels[0] as f32 + channels[1] as f32 + channels[2] as f32) / 3.0) as u8;
            let dithered_value = quantize(channels[0], channels[1], channels[2], level);
            
            channels[0] = dithered_value;
            channels[1] = dithered_value;
            channels[2] = dithered_value;
            
            img.put_pixel(x, y, pixel);

            let error = (pixel_average as i16) - (dithered_value as i16);
            distribute_error_diffusion(img, error, x, y);
        }
    }
    Box::new(CustomImage::from(img.clone()))
}

fn distribute_error_diffusion(img: &mut DynamicImage, error: i16, x: u32, y: u32) {
    if error == 0 { () }

    let dimensions = img.dimensions();

    let neighbor_pixels = [
        (Some(x + 1)     , y),
        (x.checked_sub(1), y + 1),
        (Some(x)         , y + 1),
        (Some(x + 1)     , y + 1),
    ];
    let diffusion_values = [
        0.4375, // 7/16
        0.1875, // 3/16
        0.3125, // 5/16
        0.0625, // 1/16
    ];

    for i in 0..4 {
        if outside_image_bounds(dimensions, neighbor_pixels[i]) { continue; }

        let (x, y) = neighbor_pixels[i];
        let mut pixel = img.get_pixel(x.unwrap(), y);
        let channels = &mut pixel.0;
        let spread = diffusion_values[i];

        channels[0] = add_and_clamp(channels[0] as i16, (error as f32 * spread) as i16, 0, 255);
        channels[1] = add_and_clamp(channels[1] as i16, (error as f32 * spread) as i16, 0, 255);
        channels[2] = add_and_clamp(channels[2] as i16, (error as f32 * spread) as i16, 0, 255);
        
        img.put_pixel(x.unwrap(), y, pixel);
    }
}

fn add_and_clamp(value: i16, other_value: i16, min: u8, max: u8) -> u8 {
    match value.checked_add(other_value) {
        Some(result) =>  {
            if value + other_value < 0 { min }
            else if value + other_value > max as i16 { max }
            else { result as u8 }
        },
        None => min,
    }
}

fn outside_image_bounds(dimensions: (u32, u32), current_pos: (Option<u32>, u32)) -> bool {
    if current_pos.0.is_none() { return true; }
    
    let (width, height) = dimensions;
    let (x, y) = current_pos;

    x.unwrap() >= width || y >= height
}
