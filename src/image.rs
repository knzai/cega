use factor::factor::factor;

use crate::{parser, RawGrid};

pub struct Image(RawGrid);

impl Image {
    const MAX_WIDTH: usize = 320;

    pub fn new(buffer: &[u8], width: usize, parser: parser::ParserType) -> Self {
        Self(parser.process_input(buffer, width))
    }

    pub fn data(&self) -> RawGrid {
        self.0.clone()
    }

    pub fn pixel_count(&self) -> usize {
        self.height() * self.width()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }
    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn is_fullscreen(&self) -> bool {
        self.pixel_count() == 64_000
    }

    pub fn is_tall(&self) -> bool {
        (self.height() / self.width()) > 4
    }

    pub fn width_factors(&self) -> Vec<i64> {
        factor(self.pixel_count().try_into().unwrap())
    }

    pub fn height_factors(&self) -> Vec<i64> {
        factor(
            <usize as TryInto<i64>>::try_into(self.pixel_count()).unwrap()
                / <usize as TryInto<i64>>::try_into(self.width()).unwrap(),
        )
    }

    pub fn suggestions(&self) -> String {
        if !self.is_fullscreen() && self.width() == 320 {
            format!(
                "Image appears to not be fullscreen 320*200.\
                It may be tiled, try setting a narrower -w width to detect tiles.\n\
                Possible widths: {:?}",
                self.width_factors()
            )
        } else if self.is_tall() {
            format!("Image height appears to >= 4x its width.\
                If there are tiles, setting a smaller -t tile_height will make a more compact view\n\
                Possible heights: {:?}", self.height_factors())
        } else {
            "".to_string()
        }
    }
}

pub fn tile<T: std::clone::Clone>(data: Vec<Vec<T>>, tile_height: usize) -> Vec<Vec<T>> {
    let width = data[0].len();
    let tiles_per_row = Image::MAX_WIDTH / width;

    data.chunks(tiles_per_row * tile_height)
        .flat_map(|tile_row| concat_tiles(tile_row.to_vec(), tile_height))
        .collect()
}

fn concat_tiles<T: std::clone::Clone>(tiles: Vec<Vec<T>>, num_rows: usize) -> Vec<Vec<T>> {
    //TODO; make this into a fold?
    let mut rows: Vec<Vec<T>> = vec![vec![]; num_rows];
    for tile in tiles.chunks(num_rows) {
        for (i, row) in tile.iter().enumerate() {
            rows[i].extend(row.clone());
        }
    }
    rows
}

#[cfg(test)]
mod tests {
    use crate::image::{self, Image};
    use crate::parser::ParserType;

    #[test]
    fn basic_properties() {
        let data: u32 = 0b00011011000110110001101100011011;
        let mut image = Image::new(&data.to_be_bytes(), 4, ParserType::type_str("cga"));

        assert_eq!(image.pixel_count(), 16);
        assert_eq!(image.width(), 4);
        assert_eq!(image.height(), 4);
        assert!(!image.is_fullscreen()); //todo!("Test with actual fullscreen data");
        assert_eq!(image.width_factors(), [2, 4, 8]);
        assert_eq!(image.height_factors(), [2]);
        assert!(!image.is_tall());
        image = Image::new(
            &0b0001101100011011000110110001101100011011000110110001101100011011_u64.to_be_bytes(),
            2,
            ParserType::type_str("cga"),
        );
        assert!(image.is_tall());
    }

    #[test]
    fn concat_vecs() {
        let tiles = vec![
            vec![0, 1],
            vec![2, 3],
            vec![4, 5],
            vec![6, 7],
            vec![8, 9],
            vec![10, 11],
        ];
        let new_vecs = image::concat_tiles(tiles, 2);
        assert_eq!(
            vec![vec![0, 1, 4, 5, 8, 9], vec![2, 3, 6, 7, 10, 11]],
            new_vecs
        );
    }

    #[test]
    //rework these tests to actually be wider than max_width, or do something clever to overwrite it
    fn tiling() {
        let data: u32 = 0b00011011000110110001101100011011;
        let tiled = image::tile(
            Image::new(&data.to_be_bytes(), 2, ParserType::type_str("cga")).data(),
            2,
        );
        assert_eq!(
            tiled,
            [vec![0, 1, 0, 1, 0, 1, 0, 1], vec![2, 3, 2, 3, 2, 3, 2, 3],]
        );

        let data: u64 = 0b0001101100011011000110110001101100011011000110110001101100011011;
        let tiled = image::tile(
            Image::new(&data.to_be_bytes(), 2, ParserType::type_str("cga")).data(),
            2,
        );
        assert_eq!(
            tiled,
            vec![
                vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
                vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
            ]
        );
    }
}
