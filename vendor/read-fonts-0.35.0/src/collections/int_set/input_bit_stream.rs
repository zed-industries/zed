//! Reads individual bits from a array of bytes.

use super::sparse_bit_set::BranchFactor;

pub(crate) struct InputBitStream<'a, const BF: u8> {
    data: &'a [u8],
    byte_index: usize,
    sub_index: u32,
}

impl<const BF: u8> Iterator for InputBitStream<'_, BF> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        match BF {
            2 | 4 => {
                let mask = (1 << BF) - 1;
                let byte = self.data.get(self.byte_index)?;
                let val = (*byte as u32 & (mask << self.sub_index)) >> self.sub_index;
                self.sub_index = (self.sub_index + BF as u32) % 8;
                if self.sub_index == 0 {
                    self.byte_index += 1;
                }
                Some(val)
            }
            8 => {
                let r = self.data.get(self.byte_index).map(|v| *v as u32)?;
                self.byte_index += 1;
                Some(r)
            }

            32 => {
                let b1 = self.data.get(self.byte_index).map(|v| *v as u32)?;
                let b2 = self.data.get(self.byte_index + 1).map(|v| *v as u32)?;
                let b3 = self.data.get(self.byte_index + 2).map(|v| *v as u32)?;
                let b4 = self.data.get(self.byte_index + 3).map(|v| *v as u32)?;
                self.byte_index += 4;
                Some(b1 | (b2 << 8) | (b3 << 16) | (b4 << 24))
            }
            _ => panic!("Unsupported branch factor."),
        }
    }
}

impl<'a, const BF: u8> InputBitStream<'a, BF> {
    /// Decodes and returns the branch factor and height encoded in the header byte.
    ///
    /// See: <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>
    /// Returns None if the stream does not have enough remaining bits.
    #[allow(clippy::unusual_byte_groupings)] // Used to separate bit values into units used in the set encoding.
    pub(crate) fn decode_header(data: &'a [u8]) -> Option<(BranchFactor, u8)> {
        let first_byte = data.first()?;
        let bf_bits = 0b0_00000_11 & first_byte;
        let depth_bits = (0b0_11111_00 & first_byte) >> 2;

        let branch_factor = match bf_bits {
            0b00 => BranchFactor::Two,
            0b01 => BranchFactor::Four,
            0b10 => BranchFactor::Eight,
            0b11 => BranchFactor::ThirtyTwo,
            _ => panic!("Invalid branch factor encoding."),
        };

        Some((branch_factor, depth_bits))
    }

    pub(crate) fn from(data: &'a [u8]) -> Self {
        Self {
            data,
            byte_index: 1,
            sub_index: 0,
        }
    }

    /// Skips the given number of nodes, returns true if this did not overrun the data buffer.
    pub(crate) fn skip_nodes(&mut self, n: u32) -> bool {
        match BF {
            2 | 4 => {
                let bit_index = self.sub_index + n * (BF as u32);
                self.byte_index += (bit_index / 8) as usize;
                self.sub_index = bit_index % 8;
            }
            8 => {
                self.byte_index += n as usize;
            }

            32 => {
                self.byte_index += 4 * (n as usize);
            }
            _ => panic!("Unsupported branch factor."),
        };

        self.bytes_consumed() <= self.data.len()
    }

    /// Returns the number of bytes consumed so far (including partially consumed).
    pub(crate) fn bytes_consumed(&self) -> usize {
        self.byte_index + if self.sub_index > 0 { 1 } else { 0 }
    }
}

#[cfg(test)]
#[allow(clippy::unusual_byte_groupings)]
mod test {
    use super::*;

    #[test]
    fn read_header() {
        assert_eq!(
            Some((BranchFactor::Two, 25u8)),
            InputBitStream::<2>::decode_header(&[0b1_11001_00u8])
        );

        assert_eq!(
            Some((BranchFactor::Four, 0u8)),
            InputBitStream::<2>::decode_header(&[0b1_00000_01u8])
        );

        assert_eq!(
            Some((BranchFactor::Eight, 31u8)),
            InputBitStream::<2>::decode_header(&[0b1_11111_10u8])
        );

        assert_eq!(
            Some((BranchFactor::ThirtyTwo, 9u8)),
            InputBitStream::<2>::decode_header(&[0b1_01001_11u8])
        );
    }

    #[test]
    fn read_2() {
        let mut stream = InputBitStream::<2>::from(&[0b00000000, 0b11_10_01_00, 0b00_01_10_11]);
        assert_eq!(stream.bytes_consumed(), 1); // Initially one byte consumed for the header.

        assert_eq!(stream.next(), Some(0b00));
        assert_eq!(stream.bytes_consumed(), 2);
        assert_eq!(stream.next(), Some(0b01));
        assert_eq!(stream.next(), Some(0b10));
        assert_eq!(stream.next(), Some(0b11));
        assert_eq!(stream.bytes_consumed(), 2);

        assert_eq!(stream.next(), Some(0b11));
        assert_eq!(stream.bytes_consumed(), 3);
        assert_eq!(stream.next(), Some(0b10));
        assert_eq!(stream.next(), Some(0b01));
        assert_eq!(stream.next(), Some(0b00));
        assert_eq!(stream.bytes_consumed(), 3);

        assert_eq!(stream.next(), None);

        let mut stream = InputBitStream::<2>::from(&[]);
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn skip_2() {
        let mut stream = InputBitStream::<2>::from(&[0b00000000, 0b11_10_01_00, 0b00_01_10_11]);
        assert_eq!(stream.bytes_consumed(), 1); // Initially one byte consumed for the header.

        assert!(stream.skip_nodes(1));
        assert_eq!(stream.bytes_consumed(), 2);
        assert!(stream.skip_nodes(2));
        assert_eq!(stream.bytes_consumed(), 2);
        assert!(stream.skip_nodes(1));
        assert_eq!(stream.bytes_consumed(), 2);

        assert!(stream.skip_nodes(1));
        assert_eq!(stream.bytes_consumed(), 3);
        assert!(stream.skip_nodes(3));
        assert_eq!(stream.bytes_consumed(), 3);
        assert!(!stream.skip_nodes(1));
    }

    #[test]
    fn skip_2_unaligned() {
        let mut stream = InputBitStream::<2>::from(&[0b00000000, 0b11_10_01_00, 0b00_01_10_11]);
        assert_eq!(stream.bytes_consumed(), 1); // Initially one byte consumed for the header.

        assert!(stream.skip_nodes(3));
        assert_eq!(stream.bytes_consumed(), 2);

        assert!(stream.skip_nodes(3));
        assert_eq!(stream.bytes_consumed(), 3);

        assert!(!stream.skip_nodes(3));
    }

    #[test]
    fn read_4() {
        let mut stream = InputBitStream::<4>::from(&[0b00000000, 0b1110_0100, 0b0001_1011]);
        assert_eq!(stream.bytes_consumed(), 1);

        assert_eq!(stream.next(), Some(0b0100));
        assert_eq!(stream.bytes_consumed(), 2);
        assert_eq!(stream.next(), Some(0b1110));
        assert_eq!(stream.bytes_consumed(), 2);

        assert_eq!(stream.next(), Some(0b1011));
        assert_eq!(stream.bytes_consumed(), 3);
        assert_eq!(stream.next(), Some(0b0001));
        assert_eq!(stream.bytes_consumed(), 3);

        assert_eq!(stream.next(), None);

        let mut stream = InputBitStream::<4>::from(&[]);
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn skip_4() {
        let mut stream = InputBitStream::<4>::from(&[0b00000000, 0b1110_0100, 0b0001_1011]);
        assert_eq!(stream.bytes_consumed(), 1); // Initially one byte consumed for the header.

        assert!(stream.skip_nodes(1));
        assert_eq!(stream.bytes_consumed(), 2);
        assert!(stream.skip_nodes(1));
        assert_eq!(stream.bytes_consumed(), 2);

        assert!(stream.skip_nodes(2));
        assert_eq!(stream.bytes_consumed(), 3);

        assert!(!stream.skip_nodes(1));
    }

    #[test]
    fn read_8() {
        let mut stream = InputBitStream::<8>::from(&[0b00000000, 0b11100100, 0b00011011]);
        assert_eq!(stream.bytes_consumed(), 1);
        assert_eq!(stream.next(), Some(0b11100100));
        assert_eq!(stream.bytes_consumed(), 2);
        assert_eq!(stream.next(), Some(0b00011011));
        assert_eq!(stream.bytes_consumed(), 3);
        assert_eq!(stream.next(), None);

        let mut stream = InputBitStream::<8>::from(&[]);
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn skip_8() {
        let mut stream = InputBitStream::<8>::from(&[0b00000000, 0b11100100, 0b00011011]);
        assert_eq!(stream.bytes_consumed(), 1);

        assert!(stream.skip_nodes(1));
        assert_eq!(stream.bytes_consumed(), 2);
        assert!(stream.skip_nodes(1));
        assert_eq!(stream.bytes_consumed(), 3);
        assert!(!stream.skip_nodes(1));

        let mut stream = InputBitStream::<8>::from(&[0b00000000, 0b11100100, 0b00011011]);
        assert_eq!(stream.bytes_consumed(), 1);

        assert!(stream.skip_nodes(2));
        assert_eq!(stream.bytes_consumed(), 3);
        assert!(!stream.skip_nodes(1));
    }

    #[test]
    fn read_32() {
        let mut stream = InputBitStream::<32>::from(&[
            0b00000000, 0b00000000, 0b11111111, 0b11100100, 0b00011011,
        ]);

        assert_eq!(stream.bytes_consumed(), 1);
        assert_eq!(stream.next(), Some(0b00011011_11100100_11111111_00000000));
        assert_eq!(stream.bytes_consumed(), 5);
        assert_eq!(stream.next(), None);

        let mut stream = InputBitStream::<32>::from(&[
            0b00000000, 0b00000000, 0b11111111, 0b11100100, 0b00011011, 0b00000001,
        ]);
        assert_eq!(stream.next(), Some(0b00011011_11100100_11111111_00000000));
        assert_eq!(stream.next(), None);

        let mut stream = InputBitStream::<32>::from(&[]);
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn skip_32() {
        let mut stream = InputBitStream::<32>::from(&[
            0b00000000, 0b00000000, 0b11111111, 0b11100100, 0b00011011,
        ]);

        assert_eq!(stream.bytes_consumed(), 1);

        assert!(stream.skip_nodes(1));
        assert_eq!(stream.bytes_consumed(), 5);
        assert!(!stream.skip_nodes(1));
    }
}
