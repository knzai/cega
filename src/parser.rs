use bitvec::prelude::*;

//https://moddingwiki.shikadi.net/wiki/Raw_EGA_data#Row-planar_EGA_data
#[derive(Debug, Clone, Copy)]
pub struct EGARowPlanar;
#[derive(Debug, Clone, Copy)]
pub struct CGA;

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    CGA,
    EGA,
}

#[derive(Debug, Clone, Copy)]
pub enum ParserType {
    CGA,
    EGARowPlanar,
}

impl ParserType {
    pub fn image_type(&self) -> ImageType {
        match self {
            Self::CGA => ImageType::CGA,
            Self::EGARowPlanar => ImageType::EGA,
        }
    }
    pub fn process_input(&self, buffer: &[u8], width: usize) -> Vec<Vec<u8>> {
        match self {
            Self::CGA => CGA.process_input(buffer, width),
            Self::EGARowPlanar => EGARowPlanar.process_input(buffer, width),
        }
    }

    pub fn type_str(str: &str) -> ParserType {
        match str {
            "ega_row_parser" | "erp" => ParserType::EGARowPlanar,
            _ => ParserType::CGA,
        }
    }
}

impl ImageType {
    pub fn palette_length(&self) -> usize {
        match self {
            Self::CGA => 4,
            Self::EGA => 16,
        }
    }
    pub fn word_size(&self) -> usize {
        match self {
            Self::CGA => 2,
            Self::EGA => 4,
        }
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

    fn process_input(&self, buffer: &[u8], width: usize) -> Vec<Vec<u8>>;
}

impl ProcessBinary for CGA {
    fn image_type(&self) -> ImageType {
        ImageType::CGA
    }

    fn process_input(&self, buffer: &[u8], width: usize) -> Vec<Vec<u8>> {
        self.process_bytes(buffer)
            .chunks(width)
            .map(|v| v.into())
            .collect()
    }
}

impl CGA {
    fn process_bytes(&self, buffer: &[u8]) -> Vec<u8> {
        buffer
            .view_bits::<Msb0>()
            .chunks(self.word_size())
            .map(|m| m.load::<u8>())
            .collect()
    }
}

impl ProcessBinary for EGARowPlanar {
    fn image_type(&self) -> ImageType {
        ImageType::EGA
    }

    fn process_input(&self, buffer: &[u8], width: usize) -> Vec<Vec<u8>> {
        if width < 8 {
            //TODO don't know if the spec supports this due to row planar. Maybe smarter handling of row chunking
            panic!("This parser cannot handle width less than 8")
        }
        buffer
            .chunks(width / self.pixels_per_byte())
            .map(|row| self.process_row(row))
            .collect()
    }
}

impl EGARowPlanar {
    fn process_row(&self, buffer: &[u8]) -> Vec<u8> {
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
        let buffer = EGARowPlanar.process_row(&data.to_be_bytes());
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
}
