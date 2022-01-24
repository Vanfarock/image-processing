use std::{env, error::Error};

use image::{io::Reader as ImageReader};

mod filters;

fn main() -> Result<(), Box<dyn Error>> {
    let mut current_folder = env::current_dir()?;
    current_folder.push("src\\assets\\");

    let path = current_folder.to_str().unwrap();
    let img = ImageReader::open(path.to_string() + "image1.jpg")?.decode()?;

    filters::quantize_n_levels_rgb::run(&mut img.clone(), path, 2).expect("");
    filters::floyd_steinberg_rgb::run(&mut img.clone(), path, 2).expect("");
    filters::quantize_n_levels::run(&mut img.clone(), path, 2).expect("");
    filters::floyd_steinberg::run(&mut img.clone(), path, 2).expect("");
    
    println!("Finished!");

    Ok(())
}
