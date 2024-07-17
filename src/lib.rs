#![doc = include_str!("../README.md")]

pub mod color;
pub mod image;
pub mod parser;

#[cfg(feature = "gui")]
pub mod sdl;

#[cfg(feature = "terminal")]
pub mod terminal;

#[cfg(feature = "png")]
pub mod png;

pub type RawGrid = Vec<Vec<u8>>;
pub type Grid<T> = Vec<Vec<T>>;

pub type Palette<T> = Vec<T>;
pub type ColorPalette = Vec<crate::color::Color>;
pub type CGAColorPalette = [crate::color::Color; 4];
pub type EGAColorPalette = [crate::color::Color; 16];

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    CGA,
    EGA,
}

impl ImageType {
    pub fn palette_length(&self) -> usize {
        match self {
            Self::CGA => 4,
            Self::EGA => 16,
        }
    }
    pub fn word_size(&self) -> usize {
        match self {
            Self::CGA => 2,
            Self::EGA => 4,
        }
    }
}
