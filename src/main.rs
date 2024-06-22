use clap::Parser;
use std::path::{Path, PathBuf};

use cega::cga;
use cega::color::palette;

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Kenzi Connor")]
struct Args {
    #[clap(name = "IMAGE")]
    image: PathBuf,

    #[clap(value_parser(["0", "0i", "1", "1i"]), short, long, default_value = "1")]
    palette: String,

    #[clap(short, long, help = "default ASCI palette")]
    asci: bool,

    //TODO document explict " " passing somewhere
    #[clap(short, long, num_args(4), help = "4 characters for custom ASCI art")]
    custom_asci: Vec<char>,

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

    if args.asci {
        let chars = cga::to_char(&tiled, &palette::CGACHAR);
        for (i, index) in chars.iter().enumerate() {
            if i % width == 0 {
                println!();
            }
            print!("{}", index);
        }
    } else if args.custom_asci.len() == 4 {
        let chars = cga::to_char(&tiled, &args.custom_asci[..]);
        for (i, index) in chars.iter().enumerate() {
            if i % width == 0 {
                println!();
            }
            print!("{}", index);
        }
    } else {
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
            _ => {}
        }
    }

    Ok(())
}
