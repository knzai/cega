use crate::color::Color;
use crate::palette;

#[derive(Clone, Debug)]
pub enum TerminalMode {
    Ascii,
    ColoredAscii,
    Pixels,         //full ansi_bg color pixels
    HorizontalHalf, // half left blocks + bg color for 2x density
    VerticalHalf,   // half top blocks + bg color for 2x density
}

#[allow(dead_code)]
pub struct TerminalPalette {
    mode: TerminalMode,
    chars: Option<[char; 4]>,
    colors: Option<[Color; 4]>,
    pub terminal: [String; 4],
}

impl TerminalPalette {
    pub fn new(
        mode: TerminalMode,
        chars: Option<[char; 4]>,
        colors: Option<[Color; 4]>,
    ) -> TerminalPalette {
        let chars_or = match mode {
            TerminalMode::Pixels => [' ', ' ', ' ', ' '],
            TerminalMode::Ascii | TerminalMode::ColoredAscii | _ => {
                chars.unwrap_or(palette::CGACHAR)
            }
        };
        let term = match mode {
            TerminalMode::Ascii => chars_or.map(|m| m.to_string()),
            TerminalMode::ColoredAscii => {
                let colors_or = colors.as_ref().unwrap_or(&palette::CGA1);
                chars_or
                    .iter()
                    .zip(colors_or.iter())
                    .map(|(ch, co)| {
                        format!(
                            "{}{}m{}{}",
                            palette::ANSIOPEN,
                            co.ansi_fg(),
                            ch,
                            palette::ANSIRESET
                        )
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            }
            _ => {
                let colors_or = colors.as_ref().unwrap_or(&palette::CGA1);
                chars_or
                    .iter()
                    .zip(colors_or.iter())
                    .map(|(ch, co)| {
                        format!(
                            "{}0;{}m{}{}",
                            palette::ANSIOPEN,
                            co.ansi_bg(),
                            ch,
                            palette::ANSIRESET
                        )
                    })
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
