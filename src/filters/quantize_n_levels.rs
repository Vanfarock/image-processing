use std::error::Error;

use image::{DynamicImage, GenericImageView, GenericImage};

pub fn run(img: &mut DynamicImage, path: &str, level: u8) -> Result<(), Box<dyn Error>> {
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

    img.save(path.to_string() + "image1-quantize-" + &level.to_string() + "-levels.png")?;

    Ok(())
}

fn quantize(red: u8, green: u8, blue: u8, level: u8) -> u8 {
    let gray_level_size = 255 / (level as u32);
    
    let average = (red as f32 + green as f32 + blue as f32) / 3.0;

    let gray_level = average as u32 / gray_level_size;

    let chunk_size = if level <= 1 { 0.0 } else { 255.0 / (level - 1) as f32 };
    
    (gray_level as f32 * chunk_size) as u8
}