use std::{env, error::Error};

use image::{io::Reader as ImageReader};

mod filters;

fn main() -> Result<(), Box<dyn Error>> {
    let mut current_folder = env::current_dir()?;
    current_folder.push("src\\assets\\");

    let path = current_folder.to_str().unwrap();
    let img = ImageReader::open(path.to_string() + "image1.jpg")?.decode()?;

    filters::average_blur::run(&mut img.clone(), path, 15).expect("");
    
    println!("Finished!");

    Ok(())
}
