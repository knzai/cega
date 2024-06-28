use crate::color::Color;

pub type Palette<T> = Vec<T>;
pub type ColorPalette = Vec<Color>;
pub type CharPalette = Vec<char>;

pub type CGAColorPalette = [Color; 4];
pub type CGACharPalette = [char; 4];
pub type EGAColorPalette = [Color; 16];
pub type EGACharPalette = [char; 16];

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

pub const CGACHAR: CGACharPalette = [' ', '*', '+', '▒'];
pub const EGACHAR: EGACharPalette = [
    ' ', '.', ':', '-', '=', '+', '*', '▒', '▓', '•', '#', '‖', '%', '@', '⁌', '█',
];

pub fn palette_from_abbr(name: &str) -> ColorPalette {
    match name {
        "e" => EGA0.to_vec(),
        "0" => CGA0.to_vec(),
        "0i" => CGA0I.to_vec(),
        "1i" => CGA1I.to_vec(),
        "1" | _ => CGA1.to_vec(),
    }
}

pub fn cga_char_palette() -> Palette<char> {
    CGACHAR.to_vec()
}
pub fn ega_char_palette() -> Palette<char> {
    EGACHAR.to_vec()
}
