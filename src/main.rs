use std::path::{Path, PathBuf};

use clap::Parser;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;

use cega::sdl;
use cega::terminal;
use cega::{cga, color};

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Kenzi Connor")]
struct Args {
    #[clap(name = "IMAGE")]
    image: PathBuf,

    #[clap(value_enum, short, long, value_parser = parse_mode_param, default_value="a", help="[possible values: a, c, p]\na = plain ascii\nc = colored ascii\np = full pixels via ansi bg color")]
    terminal_output: terminal::TerminalMode,

    #[clap(value_parser(["0", "0i", "1", "1i"]),num_args(0..=1), short, long)]
    palette: Option<String>,

    #[clap(short, long, value_parser = parse_asci_param, help="4 chars palette like -a \" +%0\"")]
    custom_ascii: Option<String>,

    #[clap(short, long, default_value = "80")]
    width: usize,

    #[clap(short, long, default_value_t = false)]
    sdl: bool,
}

fn parse_mode_param(arg: &str) -> Result<terminal::TerminalMode, String> {
    match arg {
        "a" => Ok(terminal::TerminalMode::Ascii),
        "c" => Ok(terminal::TerminalMode::ColoredAscii),
        "p" => Ok(terminal::TerminalMode::Pixels),
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

    let tp = terminal::TerminalPalette::new(args.terminal_output, custom_ascii, palette);
    let indices = cga::palette_indices(&reader);
    let tiled = cga::tile(&indices, 16, Some(16), Some(*width));

    for (i, index) in tiled.iter().enumerate() {
        if i % width == 0 {
            println!();
        }
        print!("{}", tp.terminal[*index as usize]);
    }

    if args.sdl {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("viewer", 128, 128)
            //.allow_highdpi()
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window
            .into_canvas()
            .build()
            .expect("could not make a canvas");
        canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        canvas.clear();

        for (i, index) in tiled.iter().enumerate() {
            let x = i % width;
            let y = i / width;
            canvas.pixel(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                crate::sdl::PALETTE1[*index as usize],
            )?;
        }
        canvas.present();

        let mut event_pump = sdl_context.event_pump()?;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
