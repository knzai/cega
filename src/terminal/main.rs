#![cfg(feature = "terminal")]

use std::fs;
use std::path::Path;

use clap::Parser;

use cega::color::palette::palette_from_abbr;
//use cega::file_data::Raw;
use cega::image::{self, Image};
use cega::parser::ParserType;
#[cfg(feature = "png")]
use cega::png;
#[cfg(feature = "gui")]
use cega::sdl::render_sdl;
use cega::terminal::{self, args, *};
use cega::ImageType;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::parse();

    let file_data = &std::fs::read(Path::new(&args.image))?;
    let parser = ParserType::type_str(&args.image_parser);
    let image = Image(parser.process_input(&file_data, args.width));

    let image_data = if args.tile_height.is_some() {
        image::tile(image.data(), args.tile_height.unwrap())
    } else {
        image.data()
    };

    //dbg!(image_data.clone());

    let palette_string = if let ImageType::EGA = parser.image_type() {
        "ega".to_owned()
    } else {
        args.palette.unwrap_or("cga1".to_owned())
    };

    let palette = palette_from_abbr(&palette_string);

    #[cfg(feature = "png")]
    if let Some(output) = args.output_file {
        png::save(output, image_data.clone(), palette.clone())?
    }

    if let Some(ga_file) = args.ga_file {
        let bytes = parser.to_bytes(image_data.clone());
        fs::write(ga_file, bytes).unwrap();
    }

    if let Some(ascii_mode) = args.ascii_preview {
        let ascii = if args.custom_ascii.is_some() {
            args.custom_ascii.unwrap().chars().collect()
        } else {
            default_char_palette(parser.image_type())
        };

        let tp = TerminalPalette::new(ascii_mode, ascii, palette.clone());
        print!(
            "{}",
            terminal::disable_wrapping(terminal::to_string(tp.apply(image_data.clone())))
        );
    }

    //downside of the current approach is the image doesn't know it's been retiled, so need to know to not ask it if it's tall
    if !args.quiet && args.tile_height.is_none() {
        println!("\n{}", image.suggestions());
    }

    #[cfg(feature = "sdl2")]
    if args.sdl {
        render_sdl(image_data, palette)?
    }
    Ok(())
}
