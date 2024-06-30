use image::{RgbImage, Rgb};
use std::path::PathBuf;

pub fn output(path: PathBuf) -> Result<(), image::ImageError> {
	let mut img = RgbImage::new(32, 32);
	for x in 15..=17 {
	    for y in 8..24 {
	        img.put_pixel(x, y, Rgb([255, 0, 0]));
	        img.put_pixel(y, x, Rgb([255, 0, 0]));
	    }
	}
	
    img.save(path)
}

