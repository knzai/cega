use bitvec::prelude::*;

const PALETTECHAR: [&str; 4] = [" ", "*", "+", "▒"];
const PALETTERTERM: [&str; 4] = [
    "\x1b[0m ",
    "\x1b[0;46m \x1b[0m",
    "\x1b[0;45m \x1b[0m",
    "\x1b[0;47m \x1b[0m",
];

//const PALETTE1: [u32; 4] = [Black, Cyan, Magenta, White];
//const PALETTE1I: [u32; 4] = [Black, LightCyan, LightMagenta, White];

#[cfg(feature = "sdl2")]
use sdl2::gfx::primitives::DrawRenderer;

// pub struct Image {
//     // tile_width: u16,
//     // tile_height: u16,
//     data: Vec<u8>,
// }
// // impl<'buffer> Default for Image<'buffer> {
// //     fn default() -> Cga<'buffer> {
// //         Cga {
// //             color_indices: &[],
// //             tile_width: 0,
// //             tile_height: 0,
// //         }
// //     }
// // }
//
// impl Image {
//     fn new(buffer: &[u8]) -> Self {
//         Self {
//             data: palette_indices(buffer),
//         }
//     }
//
//     fn from_file(path: &str) -> Self {
//         Self::new(&std::fs::read(path).unwrap())
//     }
// }

#[cfg(feature = "sdl2")]
pub fn out_cgatiles(
    path: &str,
    canvas: &mut sdl2::render::WindowCanvas,
) -> Result<(), Box<dyn std::error::Error>> {
    canvas.set_draw_color(sdl2::pixels::Color::BLACK);
    canvas.clear();

    let reader = std::fs::read(path)?;
    let indices = palette_indices(&reader);
    let tiled = tile(&indices, 16, Some(16), Some(80));
    let chars = to_char(&tiled);

    for (i, index) in chars.iter().enumerate() {
        if i % 80 == 0 {
            println!();
        }
        print!("{}", index);
    }

    let width = 128;
    for (i, index) in tile(&indices, 16, Some(16), Some(width)).iter().enumerate() {
        let x = i % width;
        let y = i / width;
        canvas.pixel(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
            crate::sdl::PALETTE1[*index as usize],
        )?;
    }
    canvas.present();
    Ok(())
}

pub fn palette_indices(buffer: &[u8]) -> Vec<u8> {
    buffer
        .view_bits::<Msb0>()
        .chunks(2)
        .map(|m| m.load::<u8>())
        .collect()
}

pub fn to_char(buffer: &[u8]) -> Vec<&str> {
    buffer
        .iter()
        .map(|i| PALETTECHAR[*i as usize])
        .collect::<Vec<&str>>()
}

pub fn to_term(buffer: &[u8]) -> Vec<&str> {
    buffer
        .iter()
        .map(|i| PALETTERTERM[*i as usize])
        .collect::<Vec<&str>>()
}

// pub fn to_rgba(buffer: &[u8]) -> Vec<u32> {
//     palette_indices(buffer)
//         .iter()
//         .map(|index| PALETTE1I[*index as usize])
//         .collect()
// }

pub fn tile(
    buffer: &[u8],
    tile_width: usize,
    tile_height: Option<usize>,
    max_width: Option<usize>,
) -> Vec<u8> {
    let pixel_count = buffer.len();
    let tile_height = tile_height.unwrap_or(pixel_count / tile_width);
    let max_width = max_width.unwrap_or(320);
    let tiles_per_row = max_width / tile_width;
    let pixel_per_tile = tile_width * tile_height;
    let num_tiles = pixel_count / pixel_per_tile;
    let tile_rows = num_tiles.div_ceil(tiles_per_row);

    // dbg!(
    //     pixel_count,
    //     tile_height,
    //     max_width,
    //     tiles_per_row,
    //     pixel_per_tile,
    //     num_tiles,
    //     tile_rows
    // );

    let mut output: Vec<u8> = vec![0; max_width * tile_rows * tile_height];

    for (i, index) in buffer.iter().enumerate() {
        output[new_index(
            i,
            pixel_per_tile,
            tile_width,
            tile_height,
            max_width,
            tiles_per_row,
        )] = *index;
    }
    output
}
pub fn new_index(
    i: usize,
    pixel_per_tile: usize,
    tile_width: usize,
    tile_height: usize,
    max_width: usize,
    tiles_per_row: usize,
) -> usize {
    let pixel_num = i % pixel_per_tile;
    let tile_num = i / pixel_per_tile;

    let col = i % tile_width;
    let row = (pixel_num / tile_width) * max_width;
    let tile_col = (tile_num % tiles_per_row) * tile_width;
    let tile_row = (tile_num / tiles_per_row) * tile_height * max_width;
    col + row + tile_col + tile_row
}

// #[test]
// fn test_bitvec() {
//     let byte8: u8 = 0b00011011;
//     let mut chunks = byte8.view_bits::<Msb0>().chunks(2);
//     assert_eq!(chunks.next().unwrap().load::<u8>(), 0);
//     assert_eq!(chunks.next().unwrap().load::<u8>(), 1);
//     assert_eq!(chunks.next().unwrap().load::<u8>(), 2);
//     assert_eq!(chunks.next().unwrap().load::<u8>(), 3);
// }

#[cfg(test)]
mod tests {
    use crate::cga;

    #[test]
    fn to_rgba() {
        let data: u128 = 0xFF_FF_FF_FF_FD_7F_F6_9F_F6_9F_FD_7F_FF_FF_FF_FF;
        let buffer = data.to_be_bytes();
        let rgba: Vec<u32> = cga::to_rgba(&buffer);
        assert_eq!(rgba[18], 0xFFFFFFFF);
        assert_eq!(rgba[19], 0x55FFFFFF);
        assert_eq!(rgba[27], 0xFF55FFFF);
    }

    #[test]
    fn indices() {
        let data: u128 = 0xFF_FF_FF_FF_FD_7F_F6_9F_F6_9F_FD_7F_FF_FF_FF_FF;
        let buffer = data.to_be_bytes();
        assert_eq!(
            cga::palette_indices(&buffer),
            [
                3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 3, 3, 3, 3, 3, 1, 2,
                2, 1, 3, 3, 3, 3, 1, 2, 2, 1, 3, 3, 3, 3, 3, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
                3, 3, 3, 3, 3, 3, 3, 3
            ]
        );
    }

    #[test]
    fn tiling() {
        let data: u32 = 0b00011011000110110001101100011011;
        let buffer = data.to_be_bytes();
        assert_eq!(
            cga::tile(&cga::palette_indices(&buffer), 2, Some(2), Some(4)),
            [0, 1, 0, 1, 2, 3, 2, 3, 0, 1, 0, 1, 2, 3, 2, 3]
        );

        let data: u64 = 0b0001101100011011000110110001101100011011000110110001101100011011;
        let buffer = data.to_be_bytes();
        assert_eq!(
            cga::tile(&cga::palette_indices(&buffer), 2, Some(2), Some(6)),
            [
                0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1,
                0, 0, 2, 3, 2, 3, 0, 0
            ]
        );
    }
}
