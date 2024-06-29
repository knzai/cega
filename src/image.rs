use crate::parser;

#[allow(unused_imports)]
use factor::factor::factor;

type Grid = Vec<Vec<u8>>;

pub struct Image {
    pub data: Grid,
}

impl Image {
    pub fn new(buffer: &[u8], width: usize, image_parser: &str) -> Self {
        let parser = parser::ParserType::type_str(image_parser);
        // let parser_pal_len = parser.image_type().palette_length();
        // if parser_pal_len > palette.len() {
        //     panic!(
        //         "{:?} needs palette_length of at least {}",
        //         parser, parser_pal_len
        //     )
        // }

        let data = parser.process_input(buffer, width);

        Self { data }
    }

    pub fn pixel_count(&self) -> usize {
        self.height() * self.width()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }
    pub fn width(&self) -> usize {
        self.data[0].len()
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

    fn concat_tiles(tiles: Vec<Vec<u8>>, num_rows: usize) -> Vec<Vec<u8>> {
        //TODO; make this into a fold?
        let mut rows: Vec<Vec<u8>> = vec![vec![]; num_rows];
        for tile in tiles.chunks(num_rows) {
            for (i, row) in tile.into_iter().enumerate() {
                rows[i].extend(row);
            }
        }
        rows
    }

    pub fn retile(&mut self, tile_height: usize, max_width: Option<usize>) -> Vec<Vec<u8>> {
        let tiles_per_row = if max_width.is_some() {
            max_width.unwrap() / self.width()
        } else {
            self.pixel_count() / tile_height
        };
        Self::tile(self.data.clone(), tile_height, tiles_per_row)
    }

    pub fn tile(data: Vec<Vec<u8>>, tile_height: usize, tiles_per_row: usize) -> Vec<Vec<u8>> {
        data.chunks(tiles_per_row * tile_height)
            .map(|tile_row| Self::concat_tiles(tile_row.to_vec(), tile_height))
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::image::Image;

    #[test]
    fn basic_properties() {
        let data: u32 = 0b00011011000110110001101100011011;
        let mut image = Image::new(&data.to_be_bytes(), 4, "cga");

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
            "cga",
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
        let new_vecs = Image::concat_tiles(tiles, 2);
        assert_eq!(
            vec![vec![0, 1, 4, 5, 8, 9], vec![2, 3, 6, 7, 10, 11]],
            new_vecs
        );
    }

    #[test]
    fn tiling() {
        let data: u32 = 0b00011011000110110001101100011011;
        let mut image = Image::new(&data.to_be_bytes(), 2, "cga");
        image.retile(2, Some(4));
        assert_eq!(
            image.data,
            [
                vec![0, 1],
                vec![2, 3],
                vec![0, 1],
                vec![2, 3],
                vec![0, 1],
                vec![2, 3],
                vec![0, 1],
                vec![2, 3]
            ]
        );

        let data: u64 = 0b0001101100011011000110110001101100011011000110110001101100011011;
        let mut image = Image::new(&data.to_be_bytes(), 2, "cga");
        image.retile(2, Some(6));
        assert_eq!(
            image.data,
            vec![
                vec![0, 1],
                vec![2, 3],
                vec![0, 1],
                vec![2, 3],
                vec![0, 1],
                vec![2, 3],
                vec![0, 1],
                vec![2, 3],
                vec![0, 1],
                vec![2, 3],
                vec![0, 1],
                vec![2, 3],
                vec![0, 1],
                vec![2, 3],
                vec![0, 1],
                vec![2, 3]
            ]
        );
    }
}
