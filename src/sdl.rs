use crate::color::palette::Palette;
use crate::image::Image;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::PixelFormatEnum::RGB888 as ColorFormat;
use sdl2::{event::Event, keyboard::Keycode};

type Sdl2Color = sdl2::pixels::Color;
type MyColor = crate::color::Color;

pub fn color_from_rgb24(rgb24: u32) -> Sdl2Color {
    Sdl2Color::from_u32(&ColorFormat.try_into().unwrap(), rgb24)
}

impl From<&MyColor> for Sdl2Color {
    fn from(c: &MyColor) -> Self {
        color_from_rgb24(c.rgb24())
    }
}

pub fn render_sdl(image: Image) -> Result<(), Box<dyn std::error::Error>> {
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
    canvas.set_draw_color(Sdl2Color::BLACK);
    canvas.clear();

    let sdlpal: Palette<Sdl2Color> = image.palette.iter().map(|m| m.into()).collect();

    for (y, row) in image.output.iter().enumerate() {
        for (x, index) in row.iter().enumerate() {
            dbg!(x, index);
            canvas.pixel(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                sdlpal[*index as usize],
            )?;
        }
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
