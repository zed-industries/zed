//! Writes individual bits to a vector of bytes.

use super::sparse_bit_set::BranchFactor;

pub(crate) struct OutputBitStream {
    data: Vec<u8>,
    sub_index: u32,
    branch_factor: BranchFactor,
}

impl OutputBitStream {
    pub(crate) const MAX_HEIGHT: u8 = 31;

    pub(crate) fn new(branch_factor: BranchFactor, height: u8) -> OutputBitStream {
        let mut out = OutputBitStream {
            data: vec![],
            sub_index: 0,
            branch_factor,
        };
        if height > Self::MAX_HEIGHT {
            panic!("Height value exceeds maximum for the branch factor.");
        }
        out.write_header(height);
        out
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.data
    }

    /// Writes a single node worth of bits to the stream.
    ///
    /// `branch_factor` controls the node size.
    pub fn write_node(&mut self, bits: u32) {
        for byte_index in 0..self.branch_factor.bytes_per_node() {
            if self.branch_factor.nodes_per_byte() == 1 || self.sub_index == 0 {
                self.data.push(0);
            }

            let bits = (bits >> (byte_index * 8)) & self.branch_factor.byte_mask();
            let bits = (bits << (self.sub_index * self.branch_factor.value())) as u8;
            *self.data.last_mut().unwrap() |= bits;

            if self.branch_factor.nodes_per_byte() > 1 {
                self.sub_index = (self.sub_index + 1) % self.branch_factor.nodes_per_byte();
            }
        }
    }

    /// Writes the header byte for a sparse bit set.
    ///
    /// See: <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>
    fn write_header(&mut self, height: u8) {
        let byte = (height & 0b00011111) << 2;
        let byte = byte | self.branch_factor.bit_id();
        self.data.push(byte);
    }
}

impl BranchFactor {
    fn nodes_per_byte(&self) -> u32 {
        match self {
            BranchFactor::Two => 4,
            BranchFactor::Four => 2,
            BranchFactor::Eight => 1,
            BranchFactor::ThirtyTwo => 1,
        }
    }

    fn bytes_per_node(&self) -> u32 {
        match self {
            BranchFactor::Two => 1,
            BranchFactor::Four => 1,
            BranchFactor::Eight => 1,
            BranchFactor::ThirtyTwo => 4,
        }
    }

    fn bit_id(&self) -> u8 {
        match self {
            BranchFactor::Two => 0b00,
            BranchFactor::Four => 0b01,
            BranchFactor::Eight => 0b10,
            BranchFactor::ThirtyTwo => 0b11,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unusual_byte_groupings)]
mod test {
    use super::*;

    #[test]
    fn init() {
        let os = OutputBitStream::new(BranchFactor::Two, 13);
        assert_eq!(os.into_bytes(), vec![0b0_01101_00]);

        let os = OutputBitStream::new(BranchFactor::Four, 23);
        assert_eq!(os.into_bytes(), vec![0b0_10111_01]);

        let os = OutputBitStream::new(BranchFactor::Eight, 1);
        assert_eq!(os.into_bytes(), vec![0b0_00001_10]);

        let os = OutputBitStream::new(BranchFactor::ThirtyTwo, 31);
        assert_eq!(os.into_bytes(), vec![0b0_11111_11]);
    }

    #[test]
    fn bf2() {
        let mut os = OutputBitStream::new(BranchFactor::Two, 13);

        os.write_node(0b10);
        os.write_node(0b00);
        os.write_node(0b11);
        os.write_node(0b01);

        os.write_node(0b01);
        os.write_node(0b11);

        assert_eq!(
            os.into_bytes(),
            vec![0b0_01101_00, 0b01_11_00_10, 0b00_00_11_01,]
        );
    }

    #[test]
    fn bf4() {
        let mut os = OutputBitStream::new(BranchFactor::Four, 23);

        os.write_node(0b0010);
        os.write_node(0b0111);

        os.write_node(0b1101);

        assert_eq!(
            os.into_bytes(),
            vec![0b0_10111_01, 0b0111_0010, 0b0000_1101,]
        );
    }

    #[test]
    fn bf8() {
        let mut os = OutputBitStream::new(BranchFactor::Eight, 1);

        os.write_node(0b01110010);
        os.write_node(0b00001101);

        assert_eq!(os.into_bytes(), vec![0b0_00001_10, 0b01110010, 0b00001101,]);
    }

    #[test]
    fn bf32() {
        let mut os = OutputBitStream::new(BranchFactor::ThirtyTwo, 31);

        os.write_node(0b10000000_00000000_00001101_01110010);

        assert_eq!(
            os.into_bytes(),
            vec![0b0_11111_11, 0b01110010, 0b00001101, 0b00000000, 0b10000000]
        );
    }

    #[test]
    fn truncating() {
        let mut os = OutputBitStream::new(BranchFactor::Four, 23);

        os.write_node(0b11110010);

        assert_eq!(os.into_bytes(), vec![0b0_10111_01, 0b0000_0010]);
    }
}
