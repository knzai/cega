use crate::terminal::TerminalMode;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Kenzi Connor")]
pub struct Args {
    #[clap(name = "IMAGE")]
    pub image: PathBuf,

    #[clap(value_enum, short, long, default_missing_value="a", num_args(0..=1), value_parser = TerminalMode::from_short, 
        help="images will horizontally crop to terminal\na = plain ascii: default for empty -a \nc = colored ascii\np = full pixels via ansi bg color\nh = horizontal half pixels (UGLY)")]
    pub ascii_preview: Option<TerminalMode>,

    #[clap(value_parser(["cga0", "cga0i", "cga1", "cga1i", "ega"]),num_args(0..=1), short, long, help="ega palette can be used for cga, but not the inverse\n")]
    pub palette: Option<String>,

    #[clap(short, long, value_parser(["ega_row_planar", "erp", "cga", "png"]), default_value="cga")]
    pub image_parser: String,

    #[clap(short, long, value_parser = parse_asci_param, help="4 or 16 chars palette like -a \" +%0\"")]
    pub custom_ascii: Option<String>,

    #[clap(short, long, default_value_t = 320)]
    pub width: usize,

    #[clap(
        short,
        long,
        help = "used for wrapping rows if retiling with tile_height"
    )]
    pub max_width: Option<usize>,

    #[clap(short, long, help = "format based on extension - see image crate")]
    pub output_file: Option<PathBuf>,

    #[clap(short, long)]
    pub ga_file: Option<PathBuf>,

    #[clap(short, long)]
    pub tile_height: Option<usize>,

    #[clap(short, long, default_value_t = false)]
    pub sdl: bool,

    #[clap(short, long, default_value_t = false)]
    pub quiet: bool,
}

fn parse_asci_param(arg: &str) -> Result<String, String> {
    if let 0 | 4 | 16 = arg.len() {
        Ok(arg.to_string())
    } else {
        Err("requires a 4 or 16 character string like: -a \" +%0\"".to_string())
    }
}
