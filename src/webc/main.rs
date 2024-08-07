#![cfg(feature = "webc")]

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use wasm_bindgen::prelude::*;

use cega::color::palette::palette_from_abbr;
use cega::file_data::Raw;
use cega::parser::ParserType;
use cega::png;

#[wasm_bindgen]
pub fn png(data: &[u8]) -> String {
    let file_data = Raw::new(data);
    let parser = ParserType::CGA;
    let image = file_data.parse(parser, 320);
    let palette = palette_from_abbr("cga0");
    let result = png::write2(image.data(), palette.clone());
    format!("data:application/png;base64,{}", STANDARD.encode(result))
}

#[wasm_bindgen]
pub fn previews(data: &[u8]) -> Vec<String> {
    let file_data = Raw::new(data);
    let palette = palette_from_abbr("cga0");
    file_data
        .previews()
        .iter()
        .map(|p| {
            format!(
                "data:application/png;base64,{}",
                STANDARD.encode(png::write2(p.data(), palette.clone()))
            )
        })
        .collect()
}

fn main() {}
