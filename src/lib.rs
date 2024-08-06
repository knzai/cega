#![doc = include_str!("../README.md")]

use factor::factor::factor;

pub mod color;
pub mod file_data;
pub mod image;
pub mod parser;

#[cfg(feature = "gui")]
pub mod sdl;

#[cfg(feature = "terminal")]
pub mod terminal;

#[cfg(feature = "png")]
pub mod png;

#[cfg(feature = "wasm")]
pub mod wasm;

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

    pub fn words_per_byte(&self) -> usize {
        8 / self.word_size()
    }

    pub fn pixel_count(&self, byte_count: usize) -> usize {
        byte_count * self.words_per_byte()
    }

    pub fn fullscreen(&self, byte_count: usize) -> bool {
        //this does not yet handle all the different EGA cases, or cga monochrome etc
        self.pixel_count(byte_count) == 64_000
    }

    pub fn widths(&self, byte_count: usize) -> Vec<i64> {
        Self::factors(self.pixel_count(byte_count), 80)
    }

    pub fn heights(&self, byte_count: usize, width: usize) -> Vec<i64> {
        Self::factors(self.pixel_count(byte_count) / width, 50)
    }

    pub fn factors(num: usize, upper: usize) -> Vec<i64> {
        factor(num.try_into().unwrap())
            .into_iter()
            .filter(|&x| x > 4 && x <= upper.try_into().unwrap())
            .collect()
    }
}
