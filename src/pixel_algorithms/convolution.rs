use image::{DynamicImage, GenericImageView};

pub fn convolute(
    img: &mut DynamicImage,
    kernel: &Vec<Vec<f32>>,
    current_pos: (u32, u32),
) -> (u8, u8, u8) {
    let mut radius_y = -((kernel.len() as isize - 1) / 2);
    let mut radius_x = -((kernel.len() as isize - 1) / 2);

    let mut sum_red = 0.0;
    let mut sum_green = 0.0;
    let mut sum_blue = 0.0;
    let mut total_pixels = 0.0;

    let (i_current_x, i_current_y) = (current_pos.0 as isize, current_pos.1 as isize);

    for row in kernel {
        for kernel_value in row {
            let new_pos = (
                (i_current_x + radius_x) as u32,
                (i_current_y + radius_y) as u32,
            );

            if outside_image_bounds(img.dimensions(), new_pos) {
                continue;
            }

            let pixel = img.get_pixel(new_pos.0, new_pos.1);
            let channels = pixel.0;

            sum_red += channels[0] as f32 * kernel_value;
            sum_green += channels[1] as f32 * kernel_value;
            sum_blue += channels[2] as f32 * kernel_value;
            total_pixels += kernel_value;
            radius_x += 1;
        }
        radius_y += 1;
    }

    if total_pixels > 0.0 {
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

fn outside_image_bounds(dimensions: (u32, u32), current_pos: (u32, u32)) -> bool {
    let (width, height) = dimensions;
    let (x, y) = current_pos;

    x >= width || y >= height
}
