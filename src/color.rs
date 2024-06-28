#[derive(Clone)]
pub enum Color {
    Black(bool), // Black - Dark Gray
    Blue(bool),
    Green(bool),
    Cyan(bool),
    Red(bool),
    Magenta(bool),
    Brown(bool), // Brown - Yellow
    White(bool), // Light Gray - White
}

impl Color {
    pub fn ansi_fg(&self) -> u8 {
        match self {
            Color::Black(false) => 30,
            Color::Red(false) => 31,
            Color::Green(false) => 32,
            Color::Brown(false) => 33,
            Color::Blue(false) => 34,
            Color::Magenta(false) => 35,
            Color::Cyan(false) => 36,
            Color::White(false) => 37,

            Color::Black(true) => 90,
            Color::Red(true) => 91,
            Color::Green(true) => 92,
            Color::Brown(true) => 93,
            Color::Blue(true) => 94,
            Color::Magenta(true) => 95,
            Color::Cyan(true) => 96,
            Color::White(true) => 97,
        }
    }

    pub fn ansi_bg(&self) -> u8 {
        self.ansi_fg() + 10
    }
    pub fn rgb24(&self) -> u32 {
        match self {
            Color::Black(true) => 0x555555,
            Color::Blue(true) => 0x5555FF,
            Color::Green(true) => 0x55FF55,
            Color::Cyan(true) => 0x55FFFF,
            Color::Red(true) => 0xFF5555,
            Color::Magenta(true) => 0xFF55FF,
            Color::Brown(true) => 0xFFFF55,
            Color::White(true) => 0xFFFFFF,
            Color::Black(false) => 0x000000,
            Color::Blue(false) => 0x0000AA,
            Color::Green(false) => 0x00AA00,
            Color::Cyan(false) => 0x00AAAA,
            Color::Red(false) => 0xAA0000,
            Color::Magenta(false) => 0xAA00AA,
            Color::Brown(false) => 0xAA5500,
            Color::White(false) => 0xAAAAAA,
        }
    }
}

pub mod palette {
    use crate::color::Color;

    pub type Palette<T> = Vec<T>;
    pub type ColorPalette = Vec<Color>;
    pub type CGAColorPalette = [Color; 4];
    pub type EGAColorPalette = [Color; 16];

    pub fn palette_from_abbr(name: &str) -> ColorPalette {
        match name {
            "ega" => EGA0.to_vec(),
            "cga0" => CGA0.to_vec(),
            "cga0i" => CGA0I.to_vec(),
            "cga1i" => CGA1I.to_vec(),
            "cga1" | _ => CGA1.to_vec(),
        }
    }

    pub const CGA0: CGAColorPalette = [
        Color::Black(false),
        Color::Cyan(true),
        Color::Magenta(true),
        Color::White(true),
    ];
    pub const CGA0I: CGAColorPalette = [
        Color::Black(false),
        Color::Green(true),
        Color::Red(true),
        Color::Brown(true),
    ];
    pub const CGA1: CGAColorPalette = [
        Color::Black(false),
        Color::Cyan(false),
        Color::Magenta(false),
        Color::White(false),
    ];
    pub const CGA1I: CGAColorPalette = [
        Color::Black(false),
        Color::Cyan(true),
        Color::Magenta(true),
        Color::White(true),
    ];

    pub const EGA0: EGAColorPalette = [
        Color::Black(false),
        Color::Blue(false),
        Color::Green(false),
        Color::Cyan(false),
        Color::Red(false),
        Color::Magenta(false),
        Color::Brown(false),
        Color::White(false),
        Color::Black(true),
        Color::Blue(true),
        Color::Green(true),
        Color::Cyan(true),
        Color::Red(true),
        Color::Magenta(true),
        Color::Brown(true),
        Color::White(true),
    ];
}

#[cfg(test)]
#[test]
fn test_ansi_fg() {
    assert_eq!(Color::Black(false).ansi_fg(), 30);
    assert_eq!(Color::Brown(false).ansi_fg(), 33);
    assert_eq!(Color::Black(true).ansi_fg(), 90);
    assert_eq!(Color::Brown(true).ansi_fg(), 93);
}
