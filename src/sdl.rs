use crate::palette::WrapPalette;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::PixelFormatEnum::RGB888 as ColorFormat;
use sdl2::{event::Event, keyboard::Keycode};

type Sdl2Color = sdl2::pixels::Color;
type MyColor = crate::color::Color;

pub fn color_from_rgb24(rgb24: u32) -> Sdl2Color {
    Sdl2Color::from_u32(&ColorFormat.try_into().unwrap(), rgb24)
}

impl TryFrom<MyColor> for Sdl2Color {
    type Error = String;

    fn try_from(c: MyColor) -> Result<Self, Self::Error> {
        Ok(color_from_rgb24(c.rgb24()))
    }
}

impl<const N: usize> TryFrom<WrapPalette<MyColor, N>> for WrapPalette<Sdl2Color, N> {
    type Error = String;

    fn try_from(ca: WrapPalette<MyColor, N>) -> Result<Self, Self::Error> {
        Ok(WrapPalette(
            <[MyColor; N] as Clone>::clone(&ca.0).map(|c| c.try_into().unwrap()),
        ))
    }
}

pub fn render_sdl(
    image: crate::cga::Image,
    palette: crate::palette::Palette<MyColor, 4>,
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
    canvas.set_draw_color(Sdl2Color::BLACK);
    canvas.clear();

    let wrap: WrapPalette<Sdl2Color, 4> = WrapPalette(palette).try_into().unwrap();
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
