use bitvec::prelude::*;
pub struct EGARowPlanar;

impl EGARowPlanar {
    //https://moddingwiki.shikadi.net/wiki/Raw_EGA_data#Row-planar_EGA_data
    fn process_row(buffer: &[u8]) -> Vec<u8> {
        let width = buffer.len() * 2;
        let mut nv: Vec<u8> = vec![0; width];

        for (crow_num, color_row) in buffer.chunks(width / 8).enumerate() {
            for (i, b) in color_row.view_bits::<Msb0>().iter().by_vals().enumerate() {
                nv[i] = nv[i] << 1 | b as u8;
            }
        }
        nv
    }

    fn process_input(buffer: &[u8], width: usize) -> Vec<u8> {
        buffer
            .chunks(width / 2)
            .flat_map(|row| Self::process_row(row))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::exp;

    #[test]
    fn test_row() {
        let data: u32 = 0b00011011000110110001101100011011;
        let buffer = exp::EGARowPlanar::process_row(&data.to_be_bytes());
        assert_eq!(buffer.len(), 8);

        assert_eq!(
            buffer,
            vec!(0b0000, 0b0000, 0b0000, 0b1111, 0b1111, 0b0000, 0b1111, 0b1111)
        );
    }

    #[test]
    fn test_input() {
        let data: u128 = 0xFF_FF_FF_FF_FD_7F_F6_9F_F6_9F_FD_7F_FF_FF_FF_FF;
        assert_eq!(
            exp::EGARowPlanar::process_input(&data.to_be_bytes(), 8),
            vec!(
                15, 15, 15, 15, 15, 15, 15, 15, 11, 14, 14, 15, 13, 15, 7, 13, 14, 11, 11, 15, 7,
                15, 13, 7, 15, 15, 15, 15, 15, 15, 15, 15
            )
        );
    }
}
