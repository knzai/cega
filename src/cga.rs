use bitvec::prelude::*;
use factor::factor::factor;

pub struct Image {
    pub width: Option<usize>,
    pub data: Vec<u8>,
}

impl Image {
    pub fn new(buffer: &[u8], width: Option<usize>) -> Self {
        Self {
            data: Image::palette_indices(buffer),
            width: width,
        }
    }

    pub fn from_file(path: &str, width: Option<usize>) -> Self {
        Self::new(&std::fs::read(path).unwrap(), width)
    }

    pub fn is_fullscreen(&self) -> bool {
        self.data.len() == 64_000
    }

    pub fn factors(&self) -> Vec<i64> {
        factor(self.data.len().try_into().unwrap())
    }

    // pub fn help() -> &str {
    //     if !is_fullscreen && width.is_none() {
    //         "Image appears to not be fullscreen 320*200"
    //     }
    // }

    fn palette_indices(buffer: &[u8]) -> Vec<u8> {
        buffer
            .view_bits::<Msb0>()
            .chunks(2)
            .map(|m| m.load::<u8>())
            .collect()
    }
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
    use crate::cga::Image;

    #[test]
    fn is_fullscreen() {
        let data: u32 = 0b00011011000110110001101100011011;
        let image = cga::Image::new(&data.to_be_bytes(), None);

        assert!(!image.is_fullscreen());
        //todo!("Test with actual fullscreen data");
    }

    #[test]
    fn indices() {
        let data: u128 = 0xFF_FF_FF_FF_FD_7F_F6_9F_F6_9F_FD_7F_FF_FF_FF_FF;
        let buffer = data.to_be_bytes();
        assert_eq!(
            cga::Image::new(&buffer, None).data,
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
            cga::tile(&cga::Image::new(&buffer, None).data, 2, Some(2), Some(4)),
            [0, 1, 0, 1, 2, 3, 2, 3, 0, 1, 0, 1, 2, 3, 2, 3]
        );

        let data: u64 = 0b0001101100011011000110110001101100011011000110110001101100011011;
        let buffer = data.to_be_bytes();
        assert_eq!(
            cga::tile(&cga::Image::new(&buffer, None).data, 2, Some(2), Some(6)),
            [
                0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1,
                0, 0, 2, 3, 2, 3, 0, 0
            ]
        );
    }
}
