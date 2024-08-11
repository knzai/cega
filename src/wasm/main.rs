#![cfg(feature = "wasm")]

use std::collections::HashMap;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use gloo_utils::format::JsValueSerdeExt;
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
pub fn previews(data: &[u8]) -> JsValue {
    let file_data = Raw::new(data);
    let mut hm = HashMap::new();
    hm.insert("CGA".to_string(), preview(&file_data, ParserType::CGA));
    hm.insert(
        "EGARowPlanar".to_string(),
        preview(&file_data, ParserType::EGARowPlanar),
    );
    JsValue::from_serde(&hm).unwrap()
}

pub fn preview(data: &Raw, parser: ParserType) -> Vec<String> {
    let palette = parser.image_type().default_color_palette();
    data.previews(parser)
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
