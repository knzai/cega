use clap::Parser;
use std::path::{Path, PathBuf};

use cega::cga;
use cega::color::palette;
use cega::color::TermPalette;

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Kenzi Connor")]
struct Args {
    #[clap(name = "IMAGE")]
    image: PathBuf,

    #[clap(value_parser(["0", "0i", "1", "1i"]),num_args(0..=1), short, long)]
    palette: Option<String>,

    #[clap(short, long, num_args(0..=1), default_value="", value_parser = parse_asci_param, help="blank or \"\" for default or 4 chars -a \" +%0\"")]
    ascii: Option<String>,

    #[clap(short, long, default_value = "80")]
    width: usize,
}

fn parse_asci_param(arg: &str) -> Result<String, String> {
	if let 1 | 2 | 3 = arg.len() {
		Err(format!("requires a 4 character string like: -a \" +%0\""))
	} else {
		Ok(arg.to_string())
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //rascii::main()
    //let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let args = Args::parse();

    let reader = std::fs::read(&Path::new(&args.image))?;

    let width = &args.width;

    let indices = cga::palette_indices(&reader);
    let tiled = cga::tile(&indices, 16, Some(16), Some(*width));

	// println!("-------------------------");
	// //println!("{:?}", args.ascii);
	//
	println!("--------");
	
	// if args.ascii.is_none() {
	// 	println!("None")
	// } else {
	// 	print!("{}", args.ascii.unwrap());
	// }
	if args.ascii.is_none() {
		println!("None")
	} else {
		print!("{}", args.ascii.unwrap());
	}
	//print!("{}", args.ascii.unwrap_or("".to_string()));
	//print!("{}", args.palette.unwrap_or("".to_string()));

	// let tp = color::TermPalette.new(chars: chars, colors: colors).term;
	// for (i, index) in tiled.iter().enumerate() {
	//         if i % width == 0 {
	//             println!();
	//         }
	//         print!("{}", index);
	// }

    // if args.asci {
    //     let chars = cga::to_char(&tiled, &palette::CGACHAR);
    //     for (i, index) in chars.iter().enumerate() {
    //         if i % width == 0 {
    //             println!();
    //         }
    //         print!("{}", index);
    //     }
    // } else if args.custom_asci.len() == 4 {
    //     let chars = cga::to_char(&tiled, &args.custom_asci[..]);
    //     for (i, index) in chars.iter().enumerate() {
    //         if i % width == 0 {
    //             println!();
    //         }
    //         print!("{}", index);
    //     }
    // } else {
    //     match args.palette.as_str() {
    //         "0" | "0i" | "1" | "1i" => {
    //             let chars = cga::to_term(&tiled, args.palette);
    //             for (i, index) in chars.iter().enumerate() {
    //                 if i % width == 0 {
    //                     println!();
    //                 }
    //                 print!("{}", index);
    //             }
    //         }
    //         _ => {}
    //     }
    // }


    Ok(())
}
