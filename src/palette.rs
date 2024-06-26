use crate::color::Color;

pub struct WrapPalette<T, const N: usize>(pub Palette<T, N>);
pub type Palette<T, const N: usize> = [T; N];
pub type ColorPalette<const N: usize> = [Color; N];
pub type CGAPalette<T> = [T; 4];
pub type CGAColorPalette = [Color; 4];

pub const CGA0: CGAPalette<Color> = [
    Color::Black(false),
    Color::Cyan(true),
    Color::Magenta(true),
    Color::White(true),
];
pub const CGA0I: CGAPalette<Color> = [
    Color::Black(false),
    Color::Green(true),
    Color::Red(true),
    Color::Brown(true),
];
pub const CGA1: CGAPalette<Color> = [
    Color::Black(false),
    Color::Cyan(false),
    Color::Magenta(false),
    Color::White(false),
];
pub const CGA1I: CGAPalette<Color> = [
    Color::Black(false),
    Color::Cyan(true),
    Color::Magenta(true),
    Color::White(true),
];

pub const CGACHAR: CGAPalette<char> = [' ', '*', '+', '▒'];

pub fn cga_palette_from_abbr(name: &str) -> CGAPalette<Color> {
    match name {
        "0" => CGA0,
        "0i" => CGA0I,
        "1i" => CGA1I,
        "1" | _ => CGA1,
    }
}

pub fn custom_cga_chars_from_str(custom_string: &str) -> CGAPalette<char> {
    custom_string
        .chars()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
