use bitvec::prelude::*;

//#[cfg(feature = "sdl2")]
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

// pub struct Tiling {
//     tile_width: usize,
//     tile_height: Option<usize>,
//     max_width: Option<usize>,
//     total_pixels: Option<usize>,
//     pixel_per_tile: Option<usize>,
// }
//
// impl Tiling {
//     fn new(
//         tile_width: usize,
//         width: usize,
//         tile_height: Option<usize>,
//         total_pixels: Option<usize>,
//     ) -> Tiling {
//         let tiles_per_row: Option<usize> = None;
//         let pixel_per_tile: Option<usize> = None;
//         let num_tiles: Option<usize> = None;
//         let rows_of_tiles: Option<usize> = None;
//
//         if let tile_height(th) = th {
//             let ppt = tile_width * th;
//
//             if let max_width(mw) = mw {
//                 //integer division to figure out how wide a perfect fit of tiles is
//                 let width = (tile_width / mw) * tile_width;
//             } else if let total_pixel(tp) = tp {
//                 let width = tp / th;
//             } else {
//                 //Switching to a 2d array would remove this restriction. May do later
//                 panic!("Setting a tile height currently requires either max_width or total_pixes to calculate row length for the offsets");
//             }
//         }
//     }
//
//     pub fn new_index(&self, i: usize) -> usize {
//         //let pixel_num = i % self.pixel_per_tile;
//         //let tile_num = i / self.pixel_per_tile;
//
//         //col = i % width;
//
//         //() * width + * width
//
//         //width * ((pixel_num / tile_width) + tile_height * (tile_num / tiles_per_row) *
//
//         let col = i % width;
//         let inner_row = (i / tw) % tw;
//         let tile_row =

//let row = ((i / tw) % tw) + i /
//let col = i % tile_width;
//let row = (pixel_num / tile_width) * width;
//let tile_col = (tile_num % tiles_per_row) * tile_width;
//let tile_row = (tile_num / tiles_per_row) * tile_height * max_width;
//col + row + tile_col + tile_row
//     }
// }

// if max_width.none() && total_pixel.some() {
//
// } else if max_width.some() && total_pixel.none() {
//
// } else if max_width.none() && total_pixel.none() {
//     panic!("Setting a tile height requires either max_width or total_pixes to calculate row length for the offsets");
// }
//}

// Tiling {
//     tile_width: tile_width,
//     tile_height: tile_height,
//     max_width: max_width,
//     total_pixels: total_pixels,
//     tiles_per_row: ,
//     pixel_per_tile: ,
//     num_tiles: ,
//     rows_of_tiles: ,
// }

// let pixel_count = buffer.len();
// let tile_height = tile_height.unwrap_or(pixel_count / tile_width);
// let max_width = max_width.unwrap_or(320);
// let tiles_per_row = max_width / tile_width;
// let pixel_per_tile = tile_width * tile_height;
// let num_tiles = pixel_count / pixel_per_tile;
// let tile_rows = num_tiles.div_ceil(tiles_per_row);

//#[cfg(feature = "sdl2")]
pub fn out_cgatiles(
    path: &str,
    canvas: &mut sdl2::render::WindowCanvas,
) -> Result<(), Box<dyn std::error::Error>> {
    canvas.set_draw_color(sdl2::pixels::Color::BLACK);
    canvas.clear();

    let reader = std::fs::read(path)?;
    let indices = palette_indices(&reader);
    let tiled = tile(&indices, 16, Some(16), Some(80));

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

#[cfg(test)]
mod tests {
    use crate::cga;

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
