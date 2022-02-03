use image::{DynamicImage, GenericImage, GenericImageView};

use crate::blur::kernel::get_gaussian_kernel;
use crate::custom_image::custom_image::{CustomImage, ImageFilter};
use crate::pixel_algorithms::convolution::convolute;

pub fn run(
    base_img: &mut DynamicImage,
    kernel_size: (u32, u32),
    standard_deviation: f32,
) -> Box<dyn ImageFilter> {
    let img = &mut base_img.clone();
    let (width, height) = img.dimensions();
    let kernel = get_gaussian_kernel(kernel_size, standard_deviation);

    for y in 0..height {
        for x in 0..width {
            let mut pixel = img.get_pixel(x, y);

            let channels = &mut pixel.0;
            let (red, green, blue) = convolute(base_img, &kernel, (x, y));

            channels[0] = red;
            channels[1] = green;
            channels[2] = blue;

            img.put_pixel(x, y, pixel);
        }
    }
    Box::new(CustomImage::from(img.clone()))
}
