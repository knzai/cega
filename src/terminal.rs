use crate::image::Image;

use crate::palette;

pub const ANSIOPEN: &str = "\x1b[";
pub const ANSIRESET: &str = "\x1b[0m";
pub const DISABLEWRAPPING: &str = "\x1b[?7l";
pub const ENABLEWRAPPING: &str = "\x1b[?7h";

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
pub struct TerminalOptions {
    pub mode: TerminalMode,
    pub terminal: Vec<String>,
}

impl TerminalOptions {
    pub fn new(
        mode: TerminalMode,
        chars: Option<palette::CharPalette>,
        colors: palette::ColorPalette,
    ) -> TerminalOptions {
        let chars = chars.unwrap_or(palette::cga_char_palette()).into_iter();
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
                .flat_map(|co| {
                    [
                        format!("{}{};", ANSIOPEN, co.ansi_fg()),
                        format!("{}m{}{}", co.ansi_bg(), '▌', ANSIRESET),
                    ]
                })
                .collect::<Vec<_>>(),
            TerminalMode::VerticalHalf => colors
                .flat_map(|co| {
                    [
                        format!("{}{};", ANSIOPEN, co.ansi_fg()),
                        format!("{}m{}{}", co.ansi_bg(), '▀', ANSIRESET),
                    ]
                })
                .collect(),
        };

        TerminalOptions {
            mode: mode,
            terminal: term,
        }
    }
    pub fn ansi_codes(co: u8, ch: char) -> String {
        format!("{}{}m{}{}", ANSIOPEN, co, ch, ANSIRESET)
    }

    pub fn output_image_string(&self, image: &Image) -> String {
        let mut buffer: String = DISABLEWRAPPING.to_owned();
        for (i, index) in image.output.iter().enumerate() {
            if i % image.width == 0 {
                buffer.push_str("\n");
            }
            let ind = match self.mode {
                TerminalMode::HorizontalHalf => (index * 2) as usize + i % 2,
                _ => *index as usize,
            };
            buffer.push_str(&self.terminal[ind]);
        }
        //     }
        // }
        buffer.push_str(ENABLEWRAPPING);

        buffer
    }
}
