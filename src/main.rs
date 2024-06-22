use clap::Parser;
use std::path::{Path, PathBuf};
//use image::{DynamicImage, RgbImage};
//use std::error::Error;
//use std::io::Write;
//use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use cega::cga;
//use cega::color::Palette;
//mod rascii;

// #[derive(ValueEnum, Clone, Debug)]
// enum Palette {
//     Char,
//     P0,
//     Cga0i,
//     Cga1,
//     Cga1i,
// }

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Kenzi Connor")]
struct Args {
    #[clap(name = "IMAGE", parse(from_os_str))]
    image: PathBuf,

    // /// Enable colored output
    #[clap(possible_values = ["c", "0", "0i", "1", "1i"], short, long, default_value="c")]
    palette: String,

    #[clap(short, long, default_value = "80")]
    width: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //rascii::main()
    //let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let args = Args::parse();

    let reader = std::fs::read(&Path::new(&args.image))?;

    let width = &args.width;

    let indices = cga::palette_indices(&reader);
    let tiled = cga::tile(&indices, 16, Some(16), Some(*width));

    match args.palette.as_str() {
        "0" | "0i" | "1" | "1i" => {
            let chars = cga::to_term(&tiled, args.palette);
            for (i, index) in chars.iter().enumerate() {
                if i % width == 0 {
                    println!();
                }
                print!("{}", index);
            }
        }
        "c" | _ => {
            let chars = cga::to_char(&tiled);
            for (i, index) in chars.iter().enumerate() {
                if i % width == 0 {
                    println!();
                }
                print!("{}", index);
            }
        }
    }

    Ok(())
}
