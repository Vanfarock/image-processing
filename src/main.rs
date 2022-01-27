use std::{env, error::Error};

use image::io::Reader as ImageReader;

use crate::custom_image::custom_image::CustomImage;

mod custom_image;
mod filters;

fn main() -> Result<(), Box<dyn Error>> {
    let mut current_folder = env::current_dir()?;
    current_folder.push("src\\assets\\");

    let path = current_folder.to_str().unwrap().to_string();
    let filename = "image1.jpg".to_string();

    let custom_image = CustomImage::new(path, filename);

    println!("Finished!");

    Ok(())
}
