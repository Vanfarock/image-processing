use std::error::Error;

use image::{DynamicImage, GenericImageView, GenericImage};

pub fn run(img: &mut DynamicImage, path: &str, radius: u8) -> Result<(), Box<dyn Error>> {
    let base_img = &mut img.clone();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let mut pixel = img.get_pixel(x, y);

            let channels = &mut pixel.0;
            let (red, green, blue) = calculate_average(base_img, radius, (x, y));
            
            channels[0] = red;
            channels[1] = green;
            channels[2] = blue;
            
            img.put_pixel(x, y, pixel);
        }
    }

    img.save(path.to_string() + "image1-average-blur.png")?;

    Ok(())
}

fn calculate_average(base_img: &mut DynamicImage, radius: u8, current_pos: (u32, u32)) -> (u8, u8, u8) {
    let dimensions = base_img.dimensions();
    let range_radius = (radius as i64) / 2;
    let s_x = current_pos.0 as i64;
    let s_y= current_pos.1 as i64;

    let mut sum_red: u32 = 0;
    let mut sum_green: u32 = 0;
    let mut sum_blue: u32 = 0;
    let mut total_pixels: u32 = 0;
    
    for x_kernel in -range_radius..=range_radius {
        for y_kernel in -range_radius..=range_radius {
            let (new_pos_x, new_pos_y) = (s_x.checked_add(x_kernel), s_y.checked_add(y_kernel));

            if new_pos_x.is_none() || new_pos_y.is_none() { continue; }
            if new_pos_x.unwrap() < 0 || new_pos_y.unwrap() < 0 { continue; }
            if outside_image_bounds(dimensions, (new_pos_x.unwrap() as u32, new_pos_y.unwrap() as u32)) { continue; }
            
            let pixel = base_img.get_pixel(new_pos_x.unwrap() as u32, new_pos_y.unwrap() as u32);
            let channels = pixel.0;
            
            sum_red += channels[0] as u32;
            sum_green += channels[1] as u32;
            sum_blue += channels[2] as u32;
            total_pixels += 1;
        }
    }

    if total_pixels > 0 {
        (
            div_and_clamp(sum_red, total_pixels),
            div_and_clamp(sum_green, total_pixels),
            div_and_clamp(sum_blue, total_pixels),
        )
    } else {
        let pixel = base_img.get_pixel(current_pos.0, current_pos.1);
        let channels = pixel.0;
        (channels[0], channels[1], channels[2])
    }

}

fn div_and_clamp(value: u32, other_value: u32) -> u8 {
    let max = 255;
    let min = 0;

    match value.checked_div(other_value) {
        Some(result) =>  {
            if value / other_value > max as u32 { max }
            else { result as u8 }
        },
        None => min,
    }
}

fn outside_image_bounds(dimensions: (u32, u32), current_pos: (u32, u32)) -> bool {   
    let (width, height) = dimensions;
    let (x, y) = current_pos;

    x >= width || y >= height
}
