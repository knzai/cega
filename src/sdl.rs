use crate::{cga, color};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::{event::Event, keyboard::Keycode};

pub const PALETTE1: [Color; 4] = [
    Color::BLACK,                 //black
    Color::RGB(0x00, 0xAA, 0xAA), //cyan
    Color::RGB(0xAA, 0x00, 0xAA), //magenta
    Color::RGB(0xAA, 0xAA, 0xAA), //gray
];
pub const PALETTE1I: [Color; 4] = [
    Color::BLACK,                 //black
    Color::RGB(0x55, 0xFF, 0xFF), //bright cyan
    Color::RGB(0xFF, 0x55, 0xFF), //bright magenta
    Color::WHITE,                 //white
];

pub fn color_from_rgb24(rgb24: u32) -> Color {
    Color::from_u32(&PixelFormatEnum::RGB888.try_into().unwrap(), rgb24)
}

impl TryFrom<&color::Color> for sdl2::pixels::Color {
    type Error = String;

    fn try_from(c: &color::Color) -> Result<Self, Self::Error> {
        Ok(color_from_rgb24(c.rgb24()))
    }
}

pub fn render_sdl(
    image: cga::Image,
    palette: [color::Color; 4],
) -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("viewer", 320, 200)
        //.allow_highdpi()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    canvas.set_draw_color(sdl2::pixels::Color::BLACK);
    canvas.clear();

    let sdlpal: Vec<sdl2::pixels::Color> = palette.iter().map(|c| c.try_into().unwrap()).collect();

    for (i, index) in image.output.iter().enumerate() {
        let x = i % image.width;
        let y = i / image.width;
        canvas.pixel(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
            sdlpal[*index as usize],
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
    Ok(())
}
