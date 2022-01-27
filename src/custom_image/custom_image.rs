use image::io::Reader as ImageReader;
use image::DynamicImage;

pub trait ImageFilter {}

pub struct CustomImage {
    pub internal_image: DynamicImage,
    pub path: String,
    pub filename: String,
    pub full_path: String,
}

impl CustomImage {
    pub fn new(path: String, filename: String) -> Self {
        let full_path = format!(
            "{}{}",
            path.trim_end_matches("/"),
            filename.trim_start_matches("/")
        );
        let internal_image = ImageReader::open(&full_path).unwrap().decode().unwrap();

        CustomImage {
            internal_image,
            path,
            filename,
            full_path,
        }
    }
}
