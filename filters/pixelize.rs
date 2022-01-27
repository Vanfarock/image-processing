use std::error::Error;

use image::{DynamicImage, GenericImageView, GenericImage};

pub fn run(img: &mut DynamicImage, path: &str, step_size: usize) -> Result<(), Box<dyn Error>> {
    let (width, height) = img.dimensions();

    for y in (0..height).step_by(step_size) {
        for x in (0..width).step_by(step_size) {
            pixelize_chunk(img, (x, y), step_size as u32);
        }
    }

    img.save(path.to_string() + "image1-pixelize-" + &step_size.to_string() + ".png")?;

    Ok(())
}

fn pixelize_chunk(img: &mut DynamicImage, start_pos: (u32, u32), chunk_size: u32) {
    let (start_col, start_line) = start_pos;
    let dimensions = img.dimensions();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    let mut valid_pixels = 0;

    for y in start_line..(start_line + chunk_size) {
        for x in start_col..(start_col + chunk_size) {
            if outside_image_bounds(dimensions, (x, y)) { continue; }
            
            let mut pixel = img.get_pixel(x, y);
            let channels = &mut pixel.0;

            red += channels[0] as u32;
            green += channels[1] as u32;
            blue += channels[2] as u32;
            valid_pixels += 1;
        }
    }

    if valid_pixels == 0 { return; }

    let result_red = (red / valid_pixels) as u8;
    let result_green = (green / valid_pixels) as u8;
    let result_blue = (blue / valid_pixels) as u8;

    for y in start_line..(start_line + chunk_size) {
        for x in start_col..(start_col + chunk_size) {
            if outside_image_bounds(dimensions, (x, y)) { continue; }
            
            let mut pixel = img.get_pixel(x, y);
            let channels = &mut pixel.0;
            
            channels[0] = result_red;
            channels[1] = result_green;
            channels[2] = result_blue;

            img.put_pixel(x, y, pixel);
        }
    }
}

fn outside_image_bounds(dimensions: (u32, u32), current_pos: (u32, u32)) -> bool {
    let (width, height) = dimensions;
    let (x, y) = current_pos;

    x >= width || y >= height
}