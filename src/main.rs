use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};

use cega::cga;
use cega::color;
use cega::color::palette;
use cega::color::TerminalPalette;
//use cega::color::TerminalMode;

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Kenzi Connor")]
struct Args {
    #[clap(name = "IMAGE")]
    image: PathBuf,

    #[clap(value_enum, short, long, value_parser = parse_mode_param, default_value="a", help="[possible values: a, c, p]\na = plain ascii\nc = colored ascii\np = full pixels via ansi bg color")]
    terminal_output: color::TerminalMode,

    #[clap(value_parser(["0", "0i", "1", "1i"]),num_args(0..=1), short, long)]
    palette: Option<String>,

    #[clap(short, long, value_parser = parse_asci_param, help="4 chars palette like -a \" +%0\"")]
    custom_ascii: Option<String>,

    #[clap(short, long, default_value = "80")]
    width: usize,
}

fn parse_mode_param(arg: &str) -> Result<color::TerminalMode, String> {
    match arg {
        "a" => Ok(color::TerminalMode::Ascii),
        "c" => Ok(color::TerminalMode::ColoredAscii),
        "p" => Ok(color::TerminalMode::Pixels),
        _ => Err(format!("possible values: a, c, p")),
    }
}

fn parse_asci_param(arg: &str) -> Result<String, String> {
    if let 0 | 4 = arg.len() {
        Ok(arg.to_string())
    } else {
        Err(format!("requires a 4 character string like: -a \" +%0\""))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let reader = std::fs::read(&Path::new(&args.image))?;

    let width = &args.width;

    let palette = if args.palette.is_some() {
        Some(color::palette::palette_from_abbr(
            &args.palette.unwrap()[..],
        ))
    } else {
        None
    };

    let custom_ascii = if args.custom_ascii.is_some() {
        Some(
            args.custom_ascii
                .unwrap()
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    } else {
        None
    };

    let tp = color::TerminalPalette::new(args.terminal_output, custom_ascii, palette);
    let indices = cga::palette_indices(&reader);
    let tiled = cga::tile(&indices, 16, Some(16), Some(*width));

    for (i, index) in tiled.iter().enumerate() {
        if i % width == 0 {
            println!();
        }
        print!("{}", tp.terminal[*index as usize]);
    }

    Ok(())
}
