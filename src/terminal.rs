use crate::color::Color;
use crate::palette;

pub const ANSIOPEN: &str = "\x1b[";
pub const ANSIRESET: &str = "\x1b[0m";
pub const DISABLEWRAPPING: &str = "\x1b[?7l";
pub const ENABLEWRAPPING: &str = "\x1b[?7h";
pub const CGACHAR: [char; 4] = [' ', '*', '+', '▒'];

pub fn char_palette_from_string(custom_string: &str) -> [char; 4] {
    custom_string
        .chars()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

#[derive(Clone, Debug)]
pub enum TerminalMode {
    Ascii,
    ColoredAscii,
    Pixels,         //full ansi_bg color pixels
    HorizontalHalf, // half left blocks + bg color for 2x density
    VerticalHalf,   // half top blocks + bg color for 2x density
}

#[allow(dead_code)]
pub struct TerminalPalette<'a> {
    mode: TerminalMode,
    chars: Option<[char; 4]>,
    pub colors: Option<&'a [Color; 4]>,
    pub terminal: [String; 4],
}

impl TerminalPalette<'_> {
    pub fn new(
        mode: TerminalMode,
        chars: Option<[char; 4]>,
        colors: Option<&[Color; 4]>,
    ) -> TerminalPalette {
        let chars_or = match mode {
            TerminalMode::Pixels => [' ', ' ', ' ', ' '],
            TerminalMode::Ascii | TerminalMode::ColoredAscii | _ => chars.unwrap_or(CGACHAR),
        };
        let term = match mode {
            TerminalMode::Ascii => chars_or.map(|m| m.to_string()),
            TerminalMode::ColoredAscii => {
                let colors_or = colors.unwrap_or(&palette::CGA1);
                chars_or
                    .iter()
                    .zip(colors_or.iter())
                    .map(|(ch, co)| format!("{}{}m{}{}", ANSIOPEN, co.ansi_fg(), ch, ANSIRESET))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            }
            _ => {
                let colors_or = colors.unwrap_or(&palette::CGA1);
                chars_or
                    .iter()
                    .zip(colors_or.iter())
                    .map(|(ch, co)| format!("{}0;{}m{}{}", ANSIOPEN, co.ansi_bg(), ch, ANSIRESET))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            }
        };

        TerminalPalette {
            mode: mode,
            chars: chars,
            colors: colors,
            terminal: term,
        }
    }
}
