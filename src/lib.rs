pub mod color;
pub mod image;
pub mod parser;

//#[cfg(feature = "sdl2")]
pub mod sdl;

//#[cfg(feature = "terminal")]
pub mod terminal;

pub type RawGrid = Vec<Vec<u8>>;
pub type Grid<T> = Vec<Vec<T>>;

pub type Palette<T> = Vec<T>;
pub type ColorPalette = Vec<crate::color::Color>;
pub type CGAColorPalette = [crate::color::Color; 4];
pub type EGAColorPalette = [crate::color::Color; 16];
