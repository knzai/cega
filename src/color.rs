pub enum Color {
    Black, // Black - Dark Gray
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown, // Brown - Yellow
    White, // Light Gray - White
}
impl Color {
    fn ansi_base(&self) -> u8 {
        match self {
            Color::Black => 0,
            Color::Red => 1,
            Color::Green => 2,
            Color::Brown => 3,
            Color::Blue => 4,
            Color::Magenta => 5,
            Color::Cyan => 6,
            Color::White => 7,
        }
    }
    pub fn ansi_fg(&self, bright: bool) -> u8 {
        self.ansi_base() + if bright { 90 } else { 30 }
    }
    pub fn ansi_bg(&self, bright: bool) -> u8 {
        self.ansi_fg(bright) + 10
    }
    pub fn rgb24(&self, bright: bool) -> u32 {
        if bright {
            match self {
                Color::Black => 0x555555,
                Color::Blue => 0x5555FF,
                Color::Green => 0x55FF55,
                Color::Cyan => 0x55FFFF,
                Color::Red => 0xFF5555,
                Color::Magenta => 0xFF55FF,
                Color::Brown => 0xFFFF55,
                Color::White => 0xFFFFFF,
            }
        } else {
            match self {
                Color::Black => 0x000000,
                Color::Blue => 0x0000AA,
                Color::Green => 0x00AA00,
                Color::Cyan => 0x00AAAA,
                Color::Red => 0xAA0000,
                Color::Magenta => 0xAA00AA,
                Color::Brown => 0xAA5500,
                Color::White => 0xAAAAAA,
            }
        }
    }
}
