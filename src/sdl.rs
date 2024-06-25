use crate::color::Palette;
use crate::color::WrapPalette;
use crate::{cga, color};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::{event::Event, keyboard::Keycode};

pub fn color_from_rgb24(rgb24: u32) -> Color {
    Color::from_u32(&PixelFormatEnum::RGB888.try_into().unwrap(), rgb24)
}

impl TryFrom<color::Color> for sdl2::pixels::Color {
    type Error = String;

    fn try_from(c: color::Color) -> Result<Self, Self::Error> {
        Ok(color_from_rgb24(c.rgb24()))
    }
}

impl<const N: usize> TryFrom<color::WrapPalette<color::Color, N>>
    for color::WrapPalette<sdl2::pixels::Color, N>
{
    type Error = String;

    fn try_from(ca: color::WrapPalette<color::Color, N>) -> Result<Self, Self::Error> {
        Ok(color::WrapPalette(
            <[color::Color; N] as Clone>::clone(&ca.0).map(|c| c.try_into().unwrap()),
        ))
    }
}

pub fn render_sdl(
    image: cga::Image,
    palette: Palette<color::Color, 4>,
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

    let wrap: WrapPalette<sdl2::pixels::Color, 4> = WrapPalette(palette).try_into().unwrap();
    let sdlpal = wrap.0;

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
