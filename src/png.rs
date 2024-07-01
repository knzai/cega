use crate::{ColorPalette, RawGrid};
use image::io::Reader as ImageReader;
use image::{Rgb, RgbImage};
use std::io::Cursor;
use std::path::PathBuf;

impl crate::color::Color {
    pub fn to_rgb(&self) -> Rgb<u8> {
        let r = self.rgb24().to_be_bytes();
        Rgb([r[1], r[2], r[3]])
    }
}

pub fn process_input(buffer: &[u8]) -> RawGrid {
    let mut reader = ImageReader::new(Cursor::new(buffer));
    reader.set_format(image::ImageFormat::Png);
    let img = reader.decode().unwrap().to_rgb8();
    img.rows()
        .map(|row| {
            row.map(|pixel| match pixel.0 {
                [0, 0, 0] => 0,
                [85, 255, 255] => 1,
                [255, 85, 255] => 2,
                [255, 255, 255] => 3,
                _ => 4,
            })
            .collect()
        })
        .collect()
}

pub fn output(
    path: PathBuf,
    image_data: RawGrid,
    palette: ColorPalette,
) -> Result<(), image::ImageError> {
    let mut img = RgbImage::new(image_data[0].len() as u32, image_data.len() as u32);

    for (x, row) in image_data.iter().enumerate() {
        for (y, index) in row.iter().enumerate() {
            img.put_pixel(y as u32, x as u32, palette[*index as usize].to_rgb());
        }
    }

    img.save(path)
}
