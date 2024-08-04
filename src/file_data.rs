use crate::image::Image;
use crate::parser::ParserType;
use crate::ImageType;

pub struct Raw(Vec<u8>);

impl Raw {
    pub fn new(data: &[u8]) -> Self {
        Self(data.to_owned())
    }
    fn byte_count(&self) -> usize {
        self.0.len()
    }

    pub fn pixel_count(&self, itype: ImageType) -> usize {
        itype.pixel_count(self.byte_count())
    }

    pub fn fullscreen(&self, itype: ImageType) -> bool {
        itype.fullscreen(self.byte_count())
    }

    pub fn widths(&self, itype: ImageType) -> Vec<i64> {
        itype.widths(self.byte_count())
    }

    pub fn parse(&self, parser: ParserType, width: usize) -> Image {
        Image(parser.process_input(&self.0, width))
    }

    pub fn previews(&self) -> Vec<Image> {
        self.widths(ImageType::CGA)
            .iter()
            .map(|w| Image(ParserType::CGA.process_input(&self.0, *w as usize)))
            .collect()
    }
}
