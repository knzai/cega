use factor::factor::factor;

use crate::image::Image;
use crate::parser::ParserType;

pub struct Raw(Vec<u8>);

impl Raw {
    pub fn new(data: &[u8]) -> Self {
        Self(data.to_owned())
    }
    fn byte_count(&self) -> usize {
        self.0.len()
    }
    fn cga_count(&self) -> usize {
        self.byte_count() * 4
    }
    fn ega_count(&self) -> usize {
        self.byte_count() * 2
    }
    pub fn cga_possible(&self) -> bool {
        self.cga_count() <= 64_000
    }
    pub fn cga_fullscreen(&self) -> bool {
        self.cga_count() == 64_000
    }
    pub fn cga_widths(&self) -> Vec<i64> {
        factor(self.cga_count().try_into().unwrap())
    }
    pub fn ega_widths(&self) -> Vec<i64> {
        factor(self.ega_count().try_into().unwrap())
    }

    pub fn parse(&self, parser: ParserType, width: usize) -> Image {
        Image(parser.process_input(&self.0, width))
    }
}
