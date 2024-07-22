use crate::color::palette::palette_from_abbr;
use crate::image::Image;
use crate::parser::ParserType;
use crate::png;
use crate::wasm::FileUpload;

pub struct ImageFile(pub FileUpload);

impl ImageFile {
    pub fn name(&self) -> String {
        if self.0.mime_type.contains("image") {
            self.0.name.to_string()
        } else {
            format!("{}{}", self.0.name, ".png")
        }
    }

    pub fn mime_type(&self) -> String {
        if self.0.mime_type.contains("image") {
            self.0.mime_type.to_string()
        } else {
            "image/png".to_string()
        }
    }

    pub fn data(&self) -> Vec<u8> {
        if self.0.mime_type.contains("image") {
            self.0.data.clone()
        } else {
            let image = Image::new(&self.0.data, 320, ParserType::CGA);
            let palette = palette_from_abbr("cga0");
            let mut bytes: Vec<u8> = Vec::new();
            let _ = png::write_to(&mut bytes, image.data(), palette.clone());
            bytes
        }
    }
}
