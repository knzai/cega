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
    fn ansi_fg(&self) -> u8 {
        match self {
            //this is really just 1-7 with either a 30 or 90 offset
            //maybe revisit and make a macro with that math
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
