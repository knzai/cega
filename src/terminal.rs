use crate::{ColorPalette, Grid, Palette, RawGrid};

pub type CharPalette = Palette<char>;
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
    pub fn adjusted_index(&self, index: usize, i: usize) -> usize {
        match self {
            TerminalMode::HorizontalHalf => (index * 2) + (i % 2),
            _ => index,
        }
    }
}

#[allow(dead_code)]
pub struct TerminalPalette {
    pub mode: TerminalMode,
    pub terminal: Palette<String>,
}

impl TerminalPalette {
    pub fn new(mode: TerminalMode, chars: CharPalette, colors: ColorPalette) -> Self {
        if chars.len() != colors.len() {
            panic!("Incompatible character and color palette lengths");
        }
        let term = match mode {
            TerminalMode::Ascii => chars.iter().map(|m| m.to_string()).collect(),
            TerminalMode::ColoredAscii => chars
                .iter()
                .zip(colors.iter())
                .map(|(ch, co)| ansi_codes(co.ansi_fg(), ch))
                .collect(),
            TerminalMode::Pixels => colors
                .iter()
                .map(|co| ansi_codes(co.ansi_bg(), &' '))
                .collect(),
            TerminalMode::HorizontalHalf => colors
                .iter()
                .flat_map(|co| half_ansi_codes(co.ansi_fg(), co.ansi_bg(), '▌'))
                .collect(),
        };

        Self {
            mode: mode,
            terminal: term,
        }
    }
    pub fn adjusted_get(&self, index: usize, i: usize) -> String {
        self.terminal[self.mode.adjusted_index(index, i)].to_owned()
    }

    pub fn apply(&self, image_data: RawGrid) -> Grid<String> {
        image_data
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .map(|(i, index)| self.adjusted_get(*index as usize, i).to_owned())
                    .collect::<Vec<String>>()
            })
            .collect::<Grid<String>>()
    }
}

pub fn disable_wrapping(string: &str) -> String {
    let mut buffer = DISABLEWRAPPING.to_owned();
    buffer.push_str(string);
    buffer.push_str(ENABLEWRAPPING);
    buffer
}

pub fn to_string(grid: Grid<String>) -> String {
    grid.iter()
        .map(|row| row.join(""))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn ansi_codes(co: u8, ch: &char) -> String {
    format!("{}{}m{}{}", ANSIOPEN, co, ch, ANSIRESET)
}
pub fn half_ansi_codes(fg: u8, bg: u8, ch: char) -> [String; 2] {
    [
        format!("{}{};", ANSIOPEN, fg),
        format!("{}m{}{}", bg, ch, ANSIRESET),
    ]
}
