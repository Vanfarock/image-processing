pub fn quantize(red: u8, green: u8, blue: u8, level: u8) -> u8 {
    let gray_level_size = 255 / (level as u32);

    let average = (red as f32 + green as f32 + blue as f32) / 3.0;

    let gray_level = average as u32 / gray_level_size;

    let chunk_size = if level <= 1 {
        0.0
    } else {
        255.0 / (level - 1) as f32
    };

    (gray_level as f32 * chunk_size) as u8
}

pub fn quantize_rgb(red: u8, green: u8, blue: u8, level: u8) -> (u8, u8, u8) {
    let level_size = 255 / level;

    let red_level = red / level_size;
    let green_level = green / level_size;
    let blue_level = blue / level_size;

    let chunk_size = if level <= 1 {
        0.0
    } else {
        255.0 / (level - 1) as f32
    };

    let quantized_red = (red_level as f32 * chunk_size) as u8;
    let quantized_green = (green_level as f32 * chunk_size) as u8;
    let quantized_blue = (blue_level as f32 * chunk_size) as u8;

    (quantized_red, quantized_green, quantized_blue)
}
