use std::error::Error;

use image::{DynamicImage, GenericImageView, GenericImage};

pub fn run(img: &mut DynamicImage, path: &str, level: u8) -> Result<(), Box<dyn Error>> {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut pixel = img.get_pixel(x, y);

            let channels = &mut pixel.0;
            
            let (q_red, q_green, q_blue) = quantize(channels[0], channels[1], channels[2], level);

            channels[0] = q_red;
            channels[1] = q_green;
            channels[2] = q_blue;

            img.put_pixel(x, y, pixel);
        }
    }

    img.save(path.to_string() + "image1-quantize-" + &level.to_string() + "-levels-rgb.png")?;

    Ok(())
}

fn quantize(red: u8, green: u8, blue: u8, level: u8) -> (u8, u8, u8) {
    let level_size = 255 / level;
    
    let red_level = red / level_size;
    let green_level = green / level_size;
    let blue_level = blue / level_size;

    let chunk_size = if level <= 1 { 0.0 } else { 255.0 / (level - 1) as f32 };
    
    let quantized_red = (red_level as f32 * chunk_size) as u8;
    let quantized_green = (green_level as f32 * chunk_size) as u8;
    let quantized_blue = (blue_level as f32 * chunk_size) as u8;

    (quantized_red, quantized_green, quantized_blue)
}