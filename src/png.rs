use image::{RgbImage, Rgb};
use std::path::PathBuf;
use crate::{RawGrid, ColorPalette};

impl crate::color::Color {
    pub fn to_rgb(&self) -> Rgb<u8> {
		let r = self.rgb24().to_be_bytes();
		Rgb([r[1], r[2], r[3]])
    }
}


pub fn output(path: PathBuf, image_data:RawGrid, palette:ColorPalette) -> Result<(), image::ImageError> {
	let mut img = RgbImage::new(image_data[0].len() as u32, image_data.len() as u32);
	
	for (x, row) in image_data.iter().enumerate(){
		for (y, index) in row.iter().enumerate() {
		    img.put_pixel(y as u32, x as u32, palette[*index as usize].to_rgb());
	    }
	}
	
    img.save(path)
}

