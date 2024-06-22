use sdl2::pixels::Color;

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
