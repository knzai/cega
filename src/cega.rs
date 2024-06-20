pub mod cga {
	use bitvec::prelude::*;
	
	use sdl2::gfx::primitives::DrawRenderer;
	use sdl2::render::WindowCanvas;
	use sdl2::pixels::Color;
	
    const SDLPALETTE1: [Color; 4] = [
        Color::BLACK,                 //black
        Color::RGB(0x00, 0xAA, 0xAA), //cyan
        Color::RGB(0xAA, 0x00, 0xAA), //magenta
        Color::RGB(0xAA, 0xAA, 0xAA), //gray
    ];
    const SDLPALETTE1I: [Color; 4] = [
        Color::BLACK,                 //black
        Color::RGB(0x55, 0xFF, 0xFF), //bright cyan
        Color::RGB(0xFF, 0x55, 0xFF), //bright magenta
        Color::WHITE,                 //white
    ];

    const PALETTECHAR: [char; 4] = ['.', '+', 'X', '0'];
    const PALETTE1: [u32; 4] = [0x000000FF, 0x00AAAAFF, 0xAA00AAFF, 0xAAAAAAFF];
    const PALETTE1I: [u32; 4] = [0x000000FF, 0x55FFFFFF, 0xFF55FFFF, 0xFFFFFFFF];

    pub fn out_cgatiles(
        path: &str,
        canvas: &mut WindowCanvas,
    ) -> Result<(), Box<dyn std::error::Error>> {
        canvas.set_draw_color(Color::BLACK);
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
        for (i, index) in tile(&indices, 16, Some(16), Some(width))
            .iter()
            .enumerate()
        {
            let x = i % width;
            let y = i / width;
            canvas.pixel(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                SDLPALETTE1[*index as usize],
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

    pub fn to_char(buffer: &[u8]) -> Vec<char> {
        buffer
            .iter()
            .map(|i| PALETTECHAR[*i as usize])
            .collect::<Vec<char>>()
    }

    pub fn to_rgba(buffer: &[u8]) -> Vec<u32> {
        palette_indices(buffer)
            .iter()
            .map(|index| PALETTE1I[*index as usize])
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
    use crate::cega::cga;

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
