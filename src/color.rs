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

//TODO: generate this with macros from the colors
pub mod palette {
    use crate::color::Color;

    pub const ANSIOPEN: &str = "\x1b[";
    pub const ANSIRESET: &str = "\x1b[0m";

    pub const CGACHAR: [char; 4] = [' ', '*', '+', '▒'];

    pub fn term_pal(name: String) -> [&'static str; 4] {
        match name.as_str() {
            "0" => CGATERM0,
            "0i" => CGATERM0I,
            "1i" => CGATERM1I,
            "1" | _ => CGATERM1,
        }
    }

    pub const CGATERM0: [&str; 4] = [
        "\x1b[0m ",
        "\x1b[0;42m \x1b[0m",
        "\x1b[0;41m \x1b[0m",
        "\x1b[0;43m \x1b[0m",
    ];

    pub const CGATERM0I: [&str; 4] = [
        "\x1b[0m ",
        "\x1b[0;102m \x1b[0m",
        "\x1b[0;101m \x1b[0m",
        "\x1b[0;103m \x1b[0m",
    ];

    pub const CGATERM1: [&str; 4] = [
        "\x1b[0m ",
        "\x1b[0;46m \x1b[0m",
        "\x1b[0;45m \x1b[0m",
        "\x1b[0;47m \x1b[0m",
    ];

    pub const CGATERM1I: [&str; 4] = [
        "\x1b[0m ",
        "\x1b[0;106m \x1b[0m",
        "\x1b[0;105m \x1b[0m",
        "\x1b[0;107m \x1b[0m",
    ];

    pub const CGA0: [Color; 4] = [
        Color::Black(false),
        Color::Green(false),
        Color::Red(false),
        Color::Brown(false),
    ];
    pub const CGA0I: [Color; 4] = [
        Color::Black(true),
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
        Color::Black(true),
        Color::Cyan(true),
        Color::Magenta(true),
        Color::White(true),
    ];
}
