pub type Palette<T, const N: usize> = [T; N];
pub struct WrapPalette<T, const N: usize>(pub Palette<T, N>);

pub const CGA0: Palette<Color, 4> = [
    Color::Black(false),
    Color::Cyan(true),
    Color::Magenta(true),
    Color::White(true),
];
pub const CGA0I: Palette<Color, 4> = [
    Color::Black(false),
    Color::Green(true),
    Color::Red(true),
    Color::Brown(true),
];
pub const CGA1: Palette<Color, 4> = [
    Color::Black(false),
    Color::Cyan(false),
    Color::Magenta(false),
    Color::White(false),
];
pub const CGA1I: Palette<Color, 4> = [
    Color::Black(false),
    Color::Cyan(true),
    Color::Magenta(true),
    Color::White(true),
];

pub const CGACHAR: Palette<char, 4> = [' ', '*', '+', '▒'];
pub const CGACHAR2: Palette<char, 5> = [' ', '*', '+', '▒', '▒'];
// enum Palette {
//     CGAColor([Color; 4]),
//     CGAChar([char; 4]),
// }
// const Palettes:
// pub mod palette {
//     pub enum CGA {
//         CGA0,
//         CGA0I,
//         CGA1,
//         CGA01I,
//     }
//
//     CGA
//     pub const CGAArray = [
//     ]
//
// }

// struct CGAPalette(Color, Color, Color, Color)
// struct CGACharPalette(char, char, char)
// struct TerminalPalette(String, String, String)

pub fn palette_from_abbr(name: &str) -> [Color; 4] {
    match name {
        "0" => CGA0,
        "0i" => CGA0I,
        "1i" => CGA1I,
        "1" | _ => CGA1,
    }
}

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
            //this is really just 0-7 with either a 30 or 90 offset
            //maybe revisit and make a macro with that math
            // Color::Black(i) => {
            //     if *i {
            //         90
            //     } else {
            //         30
            //     }
            // }
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

#[cfg(test)]
#[test]
fn test_ansi_fg() {
    assert_eq!(Color::Black(false).ansi_fg(), 30);
    assert_eq!(Color::Brown(false).ansi_fg(), 33);
    assert_eq!(Color::Black(true).ansi_fg(), 90);
    assert_eq!(Color::Brown(true).ansi_fg(), 93);
}
