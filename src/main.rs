use std::{env, error::Error};

use crate::custom_image::custom_image::{CustomImage, ImageFilter};

mod blur;
mod custom_image;
mod filters;
mod pixel_algorithms;

fn main() -> Result<(), Box<dyn Error>> {
    let mut current_folder = env::current_dir()?;
    current_folder.push("src\\assets\\");

    let path = current_folder.to_str().unwrap();
    let filename = "image1.jpg".to_owned();

    // let bla = blur::kernel::get_gaussian_kernel((3, 3), 1.0);
    // println!("{:#?}", bla);
    let image = &mut CustomImage::new(path.to_owned(), filename);

    image
        .gaussian_blur((10, 10), 5.0)
        .save(path.to_owned() + "bla.png")
        .unwrap();

    println!("Finished!");

    Ok(())
}
