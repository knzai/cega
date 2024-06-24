use crate::color;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;

pub const PALETTE1: [Color; 4] = [
    Color::BLACK,                 //black
    Color::RGB(0x00, 0xAA, 0xAA), //cyan
    Color::RGB(0xAA, 0x00, 0xAA), //magenta
    Color::RGB(0xAA, 0xAA, 0xAA), //gray
];
pub const PALETTE1I: [Color; 4] = [
    Color::BLACK,                 //black
    Color::RGB(0x55, 0xFF, 0xFF), //bright cyan
    Color::RGB(0xFF, 0x55, 0xFF), //bright magenta
    Color::WHITE,                 //white
];

pub fn color_from_rgb24(rgb24: u32) -> Color {
    Color::from_u32(&PixelFormatEnum::RGB888.try_into().unwrap(), rgb24)
}

impl TryFrom<&color::Color> for sdl2::pixels::Color {
    type Error = String;

    fn try_from(c: &color::Color) -> Result<Self, Self::Error> {
        Ok(color_from_rgb24(c.rgb24()))
    }
}
