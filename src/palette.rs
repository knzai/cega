use crate::color::Color;

pub const CGACHAR: [char; 4] = [' ', '*', '+', '▒'];

pub fn palette_from_abbr(name: &str) -> [Color; 4] {
    match name {
        "0" => CGA0,
        "0i" => CGA0I,
        "1i" => CGA1I,
        "1" | _ => CGA1,
    }
}

pub fn char_palette_from_string(custom_string: &str) -> [char; 4] {
    custom_string
        .chars()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub const CGA0: [Color; 4] = [
    Color::Black(false),
    Color::Green(false),
    Color::Red(false),
    Color::Brown(false),
];
pub const CGA0I: [Color; 4] = [
    Color::Black(false),
    Color::Green(true),
    Color::Red(true),
    Color::Brown(true),
];
pub const CGA1: [Color; 4] = [
    Color::Black(false),
    Color::Cyan(false),
    Color::Magenta(false),
    Color::White(false),
];
pub const CGA1I: [Color; 4] = [
    Color::Black(false),
    Color::Cyan(true),
    Color::Magenta(true),
    Color::White(true),
];
