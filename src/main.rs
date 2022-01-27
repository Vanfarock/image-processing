use std::{env, error::Error};

use image::io::Reader as ImageReader;

use crate::{custom_image::custom_image::CustomImage, filters::floyd_steinberg};

mod custom_image;
mod filters;
mod pixel_algorithms;

fn main() -> Result<(), Box<dyn Error>> {
    let mut current_folder = env::current_dir()?;
    current_folder.push("src\\assets\\");

    let path = current_folder.to_str().unwrap();
    let filename = "image1.jpg".to_owned();

    let image = CustomImage::new(path.to_string(), filename);

    println!("Finished!");

    Ok(())
}
