pub fn brighten(red: u8, green: u8, blue: u8, brightness: f32) -> (u8, u8, u8) {
    let percent = brightness / 100.0;
    (multiply_and_clamp(red, percent), multiply_and_clamp(green, percent), multiply_and_clamp(blue, percent))
}

fn multiply_and_clamp(color: u8, brightness: f32) -> u8 {
    let float_color = color as f32;
    
    if float_color * brightness > 255.0 { 255 }
    else if float_color * brightness < 0.0 { 0 }
    else { (float_color * brightness) as u8 }
}