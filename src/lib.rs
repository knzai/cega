pub mod color;
pub mod image;
pub mod parser;

//#[cfg(feature = "sdl2")]
pub mod sdl;

//#[cfg(feature = "terminal")]
pub mod terminal;

pub type RawGrid = Vec<Vec<u8>>;
pub type Grid<T> = Vec<Vec<T>>;
