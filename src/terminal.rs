use crate::color::palette;

pub type CharPalette = Vec<char>;
pub type CGACharPalette = [char; 4];
pub type EGACharPalette = [char; 16];

pub const ANSIOPEN: &str = "\x1b[";
pub const ANSIRESET: &str = "\x1b[0m";
pub const DISABLEWRAPPING: &str = "\x1b[?7l";
pub const ENABLEWRAPPING: &str = "\x1b[?7h";

pub const CGACHAR: CGACharPalette = [' ', '*', '+', '▒'];
pub const EGACHAR: EGACharPalette = [
    ' ', '.', ':', '-', '=', '+', '*', '▒', '▓', '•', '#', '‖', '%', '@', '⁌', '█',
];

pub fn cga_char_palette() -> CharPalette {
    CGACHAR.to_vec()
}
pub fn ega_char_palette() -> CharPalette {
    EGACHAR.to_vec()
}

#[derive(Clone, Debug)]
pub enum TerminalMode {
    Ascii,
    ColoredAscii,
    Pixels,         //full ansi_bg color pixels
    HorizontalHalf, // half left blocks + bg color for 2x density
    VerticalHalf,   // half top blocks + bg color for 2x density
}
impl TerminalMode {
    pub fn from_short(short: &str) -> Result<TerminalMode, String> {
        match short {
            "a" => Ok(TerminalMode::Ascii),
            "c" => Ok(TerminalMode::ColoredAscii),
            "p" => Ok(TerminalMode::Pixels),
            "h" => Ok(TerminalMode::HorizontalHalf),
            _ => Err(format!("possible values: a, c, p, h")),
        }
    }
}

#[allow(dead_code)]
pub struct TerminalPalette {
    pub mode: TerminalMode,
    pub terminal: Vec<String>,
}

impl TerminalPalette {
    pub fn new(mode: TerminalMode, chars: Option<&str>, colors: palette::ColorPalette) -> Self {
        let chars = if chars.is_some() {
            chars.unwrap().chars().collect()
        } else {
            ega_char_palette()
        }
        .into_iter();
        let colors = colors.iter();
        let term = match mode {
            TerminalMode::Ascii => chars.map(|m| m.to_string()).collect(),
            TerminalMode::ColoredAscii => chars
                .zip(colors)
                .map(|(ch, co)| Self::ansi_codes(co.ansi_fg(), ch))
                .collect(),
            TerminalMode::Pixels => colors
                .map(|co| Self::ansi_codes(co.ansi_bg(), ' '))
                .collect(),
            TerminalMode::HorizontalHalf => colors
                .flat_map(|co| Self::half_ansi_codes(co.ansi_fg(), co.ansi_bg(), '▌'))
                .collect(),
            TerminalMode::VerticalHalf => colors
                .flat_map(|co| Self::half_ansi_codes(co.ansi_fg(), co.ansi_bg(), '▀'))
                .collect(),
        };

        Self {
            mode: mode,
            terminal: term,
        }
    }
    pub fn half_ansi_codes(fg: u8, bg: u8, ch: char) -> [String; 2] {
        [
            format!("{}{};", ANSIOPEN, fg),
            format!("{}m{}{}", bg, ch, ANSIRESET),
        ]
    }
    pub fn ansi_codes(co: u8, ch: char) -> String {
        format!("{}{}m{}{}", ANSIOPEN, co, ch, ANSIRESET)
    }

    pub fn output_image_string(&self, image_data: Vec<Vec<u8>>) -> String {
        let mut buffer: String = DISABLEWRAPPING.to_owned();

        let out = image_data
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .map(|(i, index)| {
                        let mut ind = *index as usize;
                        if let TerminalMode::HorizontalHalf = self.mode {
                            ind = Self::hhalf_adjusted_index(ind, i);
                        };
                        self.terminal[ind].to_owned()
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        buffer.push_str(&out);
        buffer.push_str(ENABLEWRAPPING);

        buffer
    }

    pub fn hhalf_adjusted_index(index: usize, i: usize) -> usize {
        (index * 2) + (i % 2)
    }
}
