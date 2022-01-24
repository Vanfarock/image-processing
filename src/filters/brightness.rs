use std::{error::Error};

use image::{DynamicImage, GenericImageView, GenericImage};

pub fn run(img: &mut DynamicImage, path: &str, mut brightness: f32) -> Result<(), Box<dyn Error>> {
    if brightness > 100.0 { brightness = 100.0; }

    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut pixel = img.get_pixel(x, y);

            let channels = &mut pixel.0;
            
            channels[0] = (channels[0] as f32 * (brightness as f32 / 100.0)) as u8;
            channels[1] = (channels[1] as f32 * (brightness as f32 / 100.0)) as u8;
            channels[2] = (channels[2] as f32 * (brightness as f32 / 100.0)) as u8;

            img.put_pixel(x, y, pixel);
        }
    }

    img.save(path.to_string() + "image1-brightness-" + &brightness.to_string() + ".png")?;

    Ok(())
}