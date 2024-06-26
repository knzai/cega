use crate::palette;
use bitvec::prelude::*;

use factor::factor::factor;

pub trait ImageInputFormat {
    fn word_size(&self) -> usize;
    fn palette_size(&self) -> usize;

    fn palette_indices(&self, buffer: &[u8]) -> Vec<u8> {
        buffer
            .view_bits::<Msb0>()
            .chunks(self.word_size())
            .map(|m| m.load::<u8>())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub enum ImageType {
    CGA(CGA),
    EGA(EGA),
}

impl std::fmt::Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            ImageType::CGA(_) => "cga",
            ImageType::EGA(_) => "ega",
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug, Clone)]
pub struct CGA;
#[derive(Debug, Clone)]
pub struct EGA;

pub fn type_from_str(str: &str) -> ImageType {
    match str {
        "cga" => ImageType::CGA(CGA),
        "ega" => ImageType::EGA(EGA),
        _ => panic!("invalid ImageType"),
    }
}

impl ImageInputFormat for ImageType {
    fn word_size(&self) -> usize {
        match self {
            ImageType::CGA(_) => 2,
            ImageType::EGA(_) => 4,
        }
    }
    fn palette_size(&self) -> usize {
        match self {
            ImageType::CGA(_) => 4,
            ImageType::EGA(_) => 16,
        }
    }
}

pub struct Image<IT> {
    pub width: usize,
    pub data: Vec<u8>,
    pub output: Vec<u8>,
    pub palette: palette::CGAColorPalette,
    pub image_type: IT,
}

impl<IT: ImageInputFormat> Image<IT> {
    pub fn new(
        buffer: &[u8],
        width: Option<usize>,
        palette: palette::CGAColorPalette,
        image_type: IT,
        img_string: &str,
    ) -> Self {
        let image_type_from_string = type_from_str(img_string);
        let data = image_type_from_string.palette_indices(buffer);

        Self {
            data: data.clone(),
            width: width.unwrap_or(320),
            output: data.clone(),
            palette: palette,
            image_type: image_type,
        }
    }

    pub fn is_fullscreen(&self) -> bool {
        self.pixel_count() == 64_000
    }

    pub fn pixel_count(&self) -> usize {
        self.data.len()
    }

    pub fn is_tall(&self) -> bool {
        let h = self.pixel_count() / self.width;
        if h >= self.width * 4 {
            true
        } else {
            false
        }
    }

    pub fn width_factors(&self) -> Vec<i64> {
        factor(self.pixel_count().try_into().unwrap())
    }

    pub fn height_factors(&self) -> Vec<i64> {
        factor(
            <usize as TryInto<i64>>::try_into(self.pixel_count()).unwrap()
                / <usize as TryInto<i64>>::try_into(self.width).unwrap(),
        )
    }

    pub fn retile(&mut self, width: usize, tile_height: Option<usize>, max_width: usize) -> &Self {
        self.width = width;
        if tile_height.is_none() {
            self.output = self.data.clone();
            return self;
        }

        let pc = self.pixel_count();
        let tile_height = tile_height.unwrap();
        //let max_width = max_width.unwrap_or(pc / tile_height);

        let tiles_per_row = max_width / width;
        self.width = tiles_per_row * width;
        let pixel_per_tile = width * tile_height;
        let num_tiles = pc / pixel_per_tile;
        let tile_rows = num_tiles.div_ceil(tiles_per_row);

        let mut output: Vec<u8> = vec![0; max_width * tile_rows * tile_height];

        for (i, index) in self.data.iter().enumerate() {
            output[new_index(
                i,
                pixel_per_tile,
                width,
                tile_height,
                max_width,
                tiles_per_row,
            )] = *index;
        }
        self.output = output;
        self
    }
}

fn new_index(
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
    use crate::image::Image;

    #[test]
    fn is_fullscreen() {
        let data: u32 = 0b00011011000110110001101100011011;
        let image = Image::new(&data.to_be_bytes(), None);

        assert!(!image.is_fullscreen());
        //todo!("Test with actual fullscreen data");
    }

    #[test]
    fn indices() {
        let data: u128 = 0xFF_FF_FF_FF_FD_7F_F6_9F_F6_9F_FD_7F_FF_FF_FF_FF;
        assert_eq!(
            Image::new(&data.to_be_bytes(), None).output,
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
        let mut image = Image::new(&data.to_be_bytes(), None);
        image.retile(2, Some(2), 4);
        assert_eq!(
            image.output,
            [0, 1, 0, 1, 2, 3, 2, 3, 0, 1, 0, 1, 2, 3, 2, 3]
        );

        let data: u64 = 0b0001101100011011000110110001101100011011000110110001101100011011;
        let mut image = Image::new(&data.to_be_bytes(), None);
        image.retile(2, Some(2), 6);
        assert_eq!(
            image.output,
            [
                0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1,
                0, 0, 2, 3, 2, 3, 0, 0
            ]
        );
    }
}
