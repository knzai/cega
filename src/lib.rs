pub mod cga;

#[cfg(feature = "sdl2")]
pub mod sdl;

mod color;
pub use crate::color::Color;
