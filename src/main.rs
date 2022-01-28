use std::{env, error::Error};

use crate::custom_image::custom_image::{CustomImage, ImageFilter};

mod custom_image;
mod filters;
mod pixel_algorithms;

fn main() -> Result<(), Box<dyn Error>> {
    let mut current_folder = env::current_dir()?;
    current_folder.push("src\\assets\\");

    let path = current_folder.to_str().unwrap();
    let filename = "image1.jpg".to_owned();

    let image = &mut CustomImage::new(path.to_owned(), filename);
    image.quantize(2).save(path.to_owned() + "quantize.png").unwrap();
    image.quantize_rgb(2).save(path.to_owned() + "quantize_rgb.png").unwrap();

    println!("Finished!");

    Ok(())
}
