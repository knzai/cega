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
    chars: palette::CharPalette,
    pub colors: palette::ColorPalette,
    pub terminal: Vec<String>,
}

impl TerminalOptions {
    pub fn new(
        mode: TerminalMode,
        chars: Option<palette::CharPalette>,
        colors: palette::ColorPalette,
    ) -> TerminalOptions {
        let chars_or = match mode {
            TerminalMode::Pixels => [' ', ' ', ' ', ' '].to_vec(),
            TerminalMode::Ascii | TerminalMode::ColoredAscii => {
                chars.unwrap_or(palette::CGACHAR.to_vec())
            }
            TerminalMode::HorizontalHalf => ['▌', '▌', '▌', '▌'].to_vec(),
            TerminalMode::VerticalHalf => ['▀', '▀', '▀', '▀'].to_vec(),
        };
        let term = if let TerminalMode::Ascii = mode {
            chars_or.iter().map(|m| m.to_string()).collect()
        } else {
            let zipped = chars_or.iter().zip(colors.iter());
            match mode {
                TerminalMode::ColoredAscii => zipped
                    .map(|(ch, co)| format!("{}{}m{}{}", ANSIOPEN, co.ansi_fg(), ch, ANSIRESET))
                    .collect::<Vec<_>>(),
                TerminalMode::Pixels => zipped
                    .map(|(ch, co)| format!("{}0;{}m{}{}", ANSIOPEN, co.ansi_bg(), ch, ANSIRESET))
                    .collect::<Vec<_>>(),
                _ => zipped
                    .flat_map(|(ch, co)| {
                        [
                            format!("{}{};", ANSIOPEN, co.ansi_fg()),
                            format!("{}m{}{}", co.ansi_bg(), ch, ANSIRESET),
                        ]
                    })
                    .collect::<Vec<_>>(),
            }
        };

        TerminalOptions {
            mode: mode,
            chars: chars_or,
            colors: colors,
            terminal: term,
        }
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
