use image::{DynamicImage, GenericImageView};

pub fn convolute(
    img: &mut DynamicImage,
    kernel: Vec<Vec<f32>>,
    current_pos: (u32, u32),
) -> (u8, u8, u8) {
    let mut sum_red: f32 = 0.0;
    let mut sum_green: f32 = 0.0;
    let mut sum_blue: f32 = 0.0;
    let mut total_pixels: u32 = 0;

    for (x, row) in kernel.iter().enumerate() {
        for (y, kernel_value) in row.iter().enumerate() {
            let x: u32 = x.try_into().unwrap();
            let y: u32 = y.try_into().unwrap();

            let pixel = img.get_pixel(current_pos.0 + x, current_pos.1 + y);
            let channels = pixel.0;

            sum_red += channels[0] as f32 * kernel_value;
            sum_green += channels[1] as f32 * kernel_value;
            sum_blue += channels[2] as f32 * kernel_value;
            total_pixels += 1;
        }
    }

    if total_pixels > 0 {
        let total_pixels = total_pixels as f32;
        (
            div_and_clamp(sum_red, total_pixels),
            div_and_clamp(sum_green, total_pixels),
            div_and_clamp(sum_blue, total_pixels),
        )
    } else {
        let pixel = img.get_pixel(current_pos.0, current_pos.1);
        let channels = pixel.0;
        (channels[0], channels[1], channels[2])
    }
}

fn div_and_clamp(value: f32, other_value: f32) -> u8 {
    if value / other_value > 255.0 {
        255
    } else if value / other_value < 0.0 {
        0
    } else {
        (value / other_value) as u8
    }
}
