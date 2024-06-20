use std::path::{PathBuf, Path};
use structopt::StructOpt;
//use image::{DynamicImage, RgbImage};
//use std::error::Error;
//use std::io::Write;
//use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod cga;
//mod rascii;

/// Image to ASCII converter
#[derive(StructOpt, Debug)]
#[structopt(name = "rascii")]
struct Opt {
    // /// Enable colored output
    #[structopt(short = "c", long = "color")]
    color: bool,
    //
    // /// Enable braille mode
    // #[structopt(short = "b", long = "braille")]
    // braille: bool,

    #[structopt(short = "w", long = "width", default_value = "80")]
    /// Width in characters of the output
    width: usize,

    // #[structopt(short = "d", long = "depth", default_value = "70")]
    // /// Lumince depth to use. (Number of unique characters)
    // depth: u8,
    //
    // #[structopt(short = "h", long = "height")]
    // /// Height in characters of the output
    // height: Option<u32>,
    //
    // #[structopt(long = "bg")]
    // /// Enable coloring of background chars
    // bg: bool,

    /// Path of image file to convert
    #[structopt(name = "IMAGE", parse(from_os_str))]
    image: PathBuf,
}


fn main() -> Result<(), Box<dyn std::error::Error>> { 
	//rascii::main()
	//let mut stdout = StandardStream::stdout(ColorChoice::Always);
	
	let opt = Opt::from_args();
	
	let reader = std::fs::read(&Path::new(&opt.image))?;
	
	let width = &opt.width;
	
    let indices = cga::palette_indices(&reader);
    let tiled = cga::tile(&indices, 16, Some(16), Some(*width));
	
	if opt.color {
		let chars = cga::to_term(&tiled);
	    for (i, index) in chars.iter().enumerate() {
	        if i % width == 0 {
	            println!();
	        }
	        print!("{}", index);
	    }
	} else {
		let chars = cga::to_char(&tiled);
	    for (i, index) in chars.iter().enumerate() {
	        if i % width == 0 {
	            println!();
	        }
	        print!("{}", index);
	    }
	}
	Ok(())
}