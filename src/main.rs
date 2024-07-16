#![cfg(feature = "terminal")]

//std
use std::fs;
use std::path::Path;

use clap::Parser;

//int
use cega::color::palette::palette_from_abbr;
use cega::image::{self, Image};
use cega::parser::ParserType;
use cega::terminal::{self, *, args};
use cega::ImageType;

#[cfg(feature = "png")]
use cega::png;

#[cfg(feature = "sdl2")]
use cega::sdl::render_sdl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::parse();

    let reader = std::fs::read(Path::new(&args.image))?;
    let parser = ParserType::type_str(&args.image_parser);
    let image = Image::new(&reader, args.width, parser);

    let image_data = if args.tile_height.is_some() {
        image::tile(image.data(), args.tile_height.unwrap(), args.max_width)
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
        png::output(output, image_data.clone(), palette.clone())?
    }

    if let Some(ga_file) = args.ga_file {
        let bytes = parser.to_bytes(image_data.clone());
        fs::write(ga_file, bytes).unwrap();
    }

    if args.ascii_mode.is_some() {
        let ascii = if args.custom_ascii.is_some() {
            args.custom_ascii.unwrap().chars().collect()
        } else {
            default_char_palette(parser.image_type())
        };

        let tp = TerminalPalette::new(args.ascii_mode.unwrap(), ascii, palette.clone());
        print!(
            "{}",
            terminal::disable_wrapping(terminal::to_string(tp.apply(image_data.clone())))
        );
    }

    if !args.quiet && !image.is_fullscreen() {
        if args.width == 320 {
            println!("\nImage appears to not be fullscreen 320*200. It may be tiled, try setting a narrower -w width to detect tiles.");
            println!("Possible widths: {:?}", image.width_factors());
        } else if args.tile_height.is_none() && image.is_tall() {
            println!("\nImage height appears to >= 4x its width. If there are tiles, setting -t tile_height will make a more compact view");
            println!("Possible heights: {:?}", image.height_factors());
        }
    }

    #[cfg(feature = "sdl2")]
    if args.sdl {
        render_sdl(image_data, palette)?
    }
    Ok(())
}
