use bitvec::prelude::*;
use factor::factor::factor;

pub struct Image {
    pub width: Option<usize>,
    pub data: Vec<u8>,
    pub output: Vec<u8>,
}

impl Image {
    pub fn new(buffer: &[u8], width: Option<usize>) -> Self {
        let data = Image::palette_indices(buffer);
        Self {
            data: data.clone(),
            width: width,
            output: data.clone(),
        }
    }

    pub fn from_file(path: &str, width: Option<usize>) -> Self {
        Self::new(&std::fs::read(path).unwrap(), width)
    }

    pub fn is_fullscreen(&self) -> bool {
        self.pixel_count() == 64_000
    }

    pub fn pixel_count(&self) -> usize {
        self.data.len()
    }

    pub fn is_tall(&self) -> bool {
        if self.width.is_none() {
            false
        } else {
            let w = self.width.unwrap();
            let h = self.pixel_count() / w;
            if h >= w * 4 {
                true
            } else {
                false
            }
        }
    }

    pub fn factors(&self) -> Vec<i64> {
        factor(self.data.len().try_into().unwrap())
    }

    pub fn tile(
        &mut self,
        tile_width: Option<usize>,
        tile_height: Option<usize>,
        max_width: Option<usize>,
    ) -> &Self {
        if tile_width.is_none() {
            return self;
        }
        let tile_width = tile_width.unwrap();
        let pixel_count = self.pixel_count();
        let tile_height = tile_height.unwrap_or(pixel_count / tile_width);

        let max_width = max_width.unwrap_or(tile_width);

        let tiles_per_row = max_width / tile_width;
        let pixel_per_tile = tile_width * tile_height;
        let num_tiles = pixel_count / pixel_per_tile;
        let tile_rows = num_tiles.div_ceil(tiles_per_row);

        let mut output: Vec<u8> = vec![0; max_width * tile_rows * tile_height];

        for (i, index) in self.data.iter().enumerate() {
            output[new_index(
                i,
                pixel_per_tile,
                tile_width,
                tile_height,
                max_width,
                tiles_per_row,
            )] = *index;
        }
        self.output = output;
        self
    }

    fn palette_indices(buffer: &[u8]) -> Vec<u8> {
        buffer
            .view_bits::<Msb0>()
            .chunks(2)
            .map(|m| m.load::<u8>())
            .collect()
    }
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
