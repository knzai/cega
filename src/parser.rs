#[cfg(feature = "png")]
use crate::png;
use crate::ImageType;
use crate::RawGrid;
use bitvec::prelude::*;

//https://moddingwiki.shikadi.net/wiki/Raw_EGA_data#Row-planar_EGA_data
#[derive(Debug, Clone, Copy)]
pub struct EGARowPlanar;
#[derive(Debug, Clone, Copy)]
pub struct CGA;

#[derive(Debug, Clone, Copy)]
pub enum ParserType {
    CGA,
    EGARowPlanar,
    #[cfg(feature = "png")]
    Png,
}

impl ParserType {
    pub fn image_type(&self) -> ImageType {
        match self {
            Self::EGARowPlanar => ImageType::EGA,
            _ => ImageType::CGA,
        }
    }
    pub fn process_input(&self, buffer: &[u8], width: usize) -> RawGrid {
        match self {
            Self::CGA => CGA.process_input(buffer, width),
            Self::EGARowPlanar => EGARowPlanar.process_input(buffer, width),
            #[cfg(feature = "png")]
            Self::Png => png::process_input(buffer),
        }
    }

    pub fn type_str(str: &str) -> ParserType {
        match str {
            "ega_row_parser" | "erp" => ParserType::EGARowPlanar,
            #[cfg(feature = "png")]
            "png" => ParserType::Png,
            _ => ParserType::CGA,
        }
    }

    pub fn to_bytes(&self, image_data: RawGrid) -> Vec<u8> {
        CGA.to_bytes(image_data)
    }
}

pub trait ProcessBinary {
    fn image_type(&self) -> ImageType;
    fn word_size(&self) -> usize {
        self.image_type().word_size()
    }

    fn pixels_per_byte(&self) -> usize {
        8 / self.word_size()
    }

    fn process_input(&self, buffer: &[u8], width: usize) -> RawGrid;
}

impl ProcessBinary for CGA {
    fn image_type(&self) -> ImageType {
        ImageType::CGA
    }

    fn process_input(&self, buffer: &[u8], width: usize) -> RawGrid {
        self.words_to_bytes(buffer)
            .chunks(width)
            .map(|v| v.into())
            .collect()
    }
}

impl CGA {
    fn words_to_bytes(&self, buffer: &[u8]) -> Vec<u8> {
        buffer
            .view_bits::<Msb0>()
            .chunks(self.word_size())
            .map(|m| m.load::<u8>())
            .collect()
    }

    fn to_bytes(self, image_data: RawGrid) -> Vec<u8> {
        let bytes_per_new_byte = 8 / self.word_size();
        image_data
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .chunks(bytes_per_new_byte)
            .map(Self::compress_bytes_to_words)
            .collect()
    }
    fn compress_bytes_to_words(bytes: &[u8]) -> u8 {
        let mut raw = 0u8;
        let bits = raw.view_bits_mut::<Msb0>();
        for (i, byte) in bytes.iter().enumerate() {
            bits[i * 2..=i * 2 + 1].store_be::<u8>(*byte);
        }
        raw
    }
}

impl ProcessBinary for EGARowPlanar {
    fn image_type(&self) -> ImageType {
        ImageType::EGA
    }

    fn process_input(&self, buffer: &[u8], width: usize) -> RawGrid {
        if width < 8 {
            //TODO don't know if the spec supports this due to row planar. Maybe smarter handling of row chunking
            panic!("This parser cannot handle width less than 8")
        }
        buffer
            .chunks(width / self.pixels_per_byte())
            .map(|row| self.words_to_bytes_row(row))
            .collect()
    }
}

impl EGARowPlanar {
    fn words_to_bytes_row(&self, buffer: &[u8]) -> Vec<u8> {
        let width = buffer.len() * 2;
        let mut nv: Vec<u8> = vec![0; width];

        for color_row in buffer.chunks(width / 8) {
            for (i, b) in color_row.view_bits::<Msb0>().iter().by_vals().enumerate() {
                nv[i] = nv[i] << 1 | b as u8;
            }
        }
        nv
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::*;

    #[test]
    fn test_ega_row_planar_row() {
        let data: u32 = 0b00011011000110110001101100011011;
        let buffer = EGARowPlanar.words_to_bytes_row(&data.to_be_bytes());
        assert_eq!(buffer.len(), 8);

        assert_eq!(
            buffer,
            vec!(0b0000, 0b0000, 0b0000, 0b1111, 0b1111, 0b0000, 0b1111, 0b1111)
        );
    }

    #[test]
    fn test_ega_row_planar_process_input() {
        let data: u128 = 0xFF_FF_FF_FF_FD_7F_F6_9F_F6_9F_FD_7F_FF_FF_FF_FF;
        assert_eq!(
            EGARowPlanar.process_input(&data.to_be_bytes(), 8),
            vec!(
                vec!(15, 15, 15, 15, 15, 15, 15, 15),
                vec!(11, 14, 14, 15, 13, 15, 7, 13),
                vec!(14, 11, 11, 15, 7, 15, 13, 7),
                vec!(15, 15, 15, 15, 15, 15, 15, 15),
            )
        );
    }
    #[test]
    fn test_cga_process_input() {
        let data: u128 = 0xFF_FF_FF_FF_FD_7F_F6_9F_F6_9F_FD_7F_FF_FF_FF_FF;
        assert_eq!(
            CGA.process_input(&data.to_be_bytes(), 8),
            vec!(
                vec!(3, 3, 3, 3, 3, 3, 3, 3),
                vec!(3, 3, 3, 3, 3, 3, 3, 3),
                vec!(3, 3, 3, 1, 1, 3, 3, 3),
                vec!(3, 3, 1, 2, 2, 1, 3, 3),
                vec!(3, 3, 1, 2, 2, 1, 3, 3),
                vec!(3, 3, 3, 1, 1, 3, 3, 3),
                vec!(3, 3, 3, 3, 3, 3, 3, 3),
                vec!(3, 3, 3, 3, 3, 3, 3, 3),
            )
        );
    }

    #[test]
    fn test_cga_to_bytes() {
        let data = vec![
            vec![3, 3, 3, 3, 3, 3, 3, 3],
            vec![3, 3, 3, 3, 3, 3, 3, 3],
            vec![3, 3, 3, 1, 1, 3, 3, 3],
            vec![3, 3, 1, 2, 2, 1, 3, 3],
            vec![3, 3, 1, 2, 2, 1, 3, 3],
            vec![3, 3, 3, 1, 1, 3, 3, 3],
            vec![3, 3, 3, 3, 3, 3, 3, 3],
            vec![3, 3, 3, 3, 3, 3, 3, 3],
        ];
        assert_eq!(
            CGA.to_bytes(data),
            0xFF_FF_FF_FF_FD_7F_F6_9F_F6_9F_FD_7F_FF_FF_FF_FFu128.to_be_bytes()
        );
    }
    #[test]
    fn test_compress_bytes_to_words() {
        assert_eq!(
            CGA::compress_bytes_to_words(&[0b00, 0b01, 0b10, 0b11]),
            0b00011011
        );
    }
}
