use crate::table::crc32_table;
use crate::util::crc32;
use crate::*;
use crc_catalog::Algorithm;

impl<const L: usize> Crc<u32, Table<L>>
where
    Table<L>: private::Sealed,
{
    pub const fn new(algorithm: &'static Algorithm<u32>) -> Self {
        Self {
            algorithm,
            data: crc32_table(algorithm.width, algorithm.poly, algorithm.refin),
        }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u32 {
        let mut crc = init(self.algorithm, self.algorithm.init);
        crc = self.update(crc, bytes);
        finalize(self.algorithm, crc)
    }

    const fn update(&self, crc: u32, bytes: &[u8]) -> u32 {
        update_table(crc, self.algorithm, &self.data, bytes)
    }

    pub const fn digest(&self) -> Digest<u32, Table<L>> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub const fn digest_with_initial(&self, initial: u32) -> Digest<u32, Table<L>> {
        let value = init(self.algorithm, initial);
        Digest::new(self, value)
    }

    pub const fn table(&self) -> &<Table<L> as Implementation>::Data<u32> {
        &self.data
    }
}

impl<'a, const L: usize> Digest<'a, u32, Table<L>>
where
    Table<L>: private::Sealed,
{
    const fn new(crc: &'a Crc<u32, Table<L>>, value: u32) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u32 {
        finalize(self.crc.algorithm, self.value)
    }
}

const fn init(algorithm: &Algorithm<u32>, initial: u32) -> u32 {
    if algorithm.refin {
        initial.reverse_bits() >> (32u8 - algorithm.width)
    } else {
        initial << (32u8 - algorithm.width)
    }
}

const fn finalize(algorithm: &Algorithm<u32>, mut crc: u32) -> u32 {
    if algorithm.refin ^ algorithm.refout {
        crc = crc.reverse_bits();
    }
    if !algorithm.refout {
        crc >>= 32u8 - algorithm.width;
    }
    crc ^ algorithm.xorout
}

const fn update_table<const L: usize>(
    mut crc: u32,
    algorithm: &Algorithm<u32>,
    table: &[[u32; 256]; L],
    bytes: &[u8],
) -> u32 {
    let len = bytes.len();
    let mut i = 0;
    let reflect = algorithm.refin;

    // Process 16 bytes at a time when L=16
    if L == 16 {
        while i + 16 <= len {
            if reflect {
                // XOR the first 4 bytes with the current CRC value
                let mut current_slice = [0u8; 4];
                current_slice[0] = bytes[i] ^ (crc as u8);
                current_slice[1] = bytes[i + 1] ^ ((crc >> 8) as u8);
                current_slice[2] = bytes[i + 2] ^ ((crc >> 16) as u8);
                current_slice[3] = bytes[i + 3] ^ ((crc >> 24) as u8);

                crc = table[0][bytes[i + 15] as usize]
                    ^ table[1][bytes[i + 14] as usize]
                    ^ table[2][bytes[i + 13] as usize]
                    ^ table[3][bytes[i + 12] as usize]
                    ^ table[4][bytes[i + 11] as usize]
                    ^ table[5][bytes[i + 10] as usize]
                    ^ table[6][bytes[i + 9] as usize]
                    ^ table[7][bytes[i + 8] as usize]
                    ^ table[8][bytes[i + 7] as usize]
                    ^ table[9][bytes[i + 6] as usize]
                    ^ table[10][bytes[i + 5] as usize]
                    ^ table[11][bytes[i + 4] as usize]
                    ^ table[12][current_slice[3] as usize]
                    ^ table[13][current_slice[2] as usize]
                    ^ table[14][current_slice[1] as usize]
                    ^ table[15][current_slice[0] as usize];
            } else {
                // For non-reflected CRC32
                let mut current_slice = [0u8; 4];
                current_slice[0] = bytes[i] ^ ((crc >> 24) as u8);
                current_slice[1] = bytes[i + 1] ^ ((crc >> 16) as u8);
                current_slice[2] = bytes[i + 2] ^ ((crc >> 8) as u8);
                current_slice[3] = bytes[i + 3] ^ (crc as u8);

                crc = table[0][bytes[i + 15] as usize]
                    ^ table[1][bytes[i + 14] as usize]
                    ^ table[2][bytes[i + 13] as usize]
                    ^ table[3][bytes[i + 12] as usize]
                    ^ table[4][bytes[i + 11] as usize]
                    ^ table[5][bytes[i + 10] as usize]
                    ^ table[6][bytes[i + 9] as usize]
                    ^ table[7][bytes[i + 8] as usize]
                    ^ table[8][bytes[i + 7] as usize]
                    ^ table[9][bytes[i + 6] as usize]
                    ^ table[10][bytes[i + 5] as usize]
                    ^ table[11][bytes[i + 4] as usize]
                    ^ table[12][current_slice[3] as usize]
                    ^ table[13][current_slice[2] as usize]
                    ^ table[14][current_slice[1] as usize]
                    ^ table[15][current_slice[0] as usize];
            }
            i += 16;
        }
    }

    // Process remaining bytes one at a time using the table (for L=1 and L=16)
    if L > 0 {
        if reflect {
            while i < len {
                let table_index = ((crc ^ bytes[i] as u32) & 0xFF) as usize;
                crc = table[0][table_index] ^ (crc >> 8);
                i += 1;
            }
        } else {
            while i < len {
                let table_index = (((crc >> 24) ^ bytes[i] as u32) & 0xFF) as usize;
                crc = table[0][table_index] ^ (crc << 8);
                i += 1;
            }
        }
    } else {
        // This section is for NoTable case (L=0)
        let poly = if reflect {
            let poly = algorithm.poly.reverse_bits();
            poly >> (32u8 - algorithm.width)
        } else {
            algorithm.poly << (32u8 - algorithm.width)
        };

        if reflect {
            while i < len {
                let to_crc = (crc ^ bytes[i] as u32) & 0xFF;
                crc = crc32(poly, reflect, to_crc) ^ (crc >> 8);
                i += 1;
            }
        } else {
            while i < len {
                let to_crc = ((crc >> 24) ^ bytes[i] as u32) & 0xFF;
                crc = crc32(poly, reflect, to_crc) ^ (crc << 8);
                i += 1;
            }
        }
    }

    crc
}

#[cfg(test)]
mod test {
    use crate::*;
    use crc_catalog::{Algorithm, CRC_32_ISCSI};

    /// Test this optimized version against the well known implementation to ensure correctness
    #[test]
    fn correctness() {
        let data: &[&str] = &[
            "",
            "1",
            "1234",
            "123456789",
            "0123456789ABCDE",
            "01234567890ABCDEFGHIJK",
            "01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK",
        ];

        pub const CRC_32_ISCSI_NONREFLEX: Algorithm<u32> = Algorithm {
            width: 32,
            poly: 0x1edc6f41,
            init: 0xffffffff,
            // This is the only flag that affects the optimized code path
            refin: false,
            refout: true,
            xorout: 0xffffffff,
            check: 0xe3069283,
            residue: 0xb798b438,
        };

        let algs_to_test = [&CRC_32_ISCSI, &CRC_32_ISCSI_NONREFLEX];

        for alg in algs_to_test {
            for data in data {
                let crc_slice16 = Crc::<u32, Table<16>>::new(alg);
                let crc_nolookup = Crc::<u32, NoTable>::new(alg);
                let expected = Crc::<u32, Table<1>>::new(alg).checksum(data.as_bytes());

                // Check that doing all at once works as expected
                assert_eq!(crc_slice16.checksum(data.as_bytes()), expected);
                assert_eq!(crc_nolookup.checksum(data.as_bytes()), expected);

                let mut digest = crc_slice16.digest();
                digest.update(data.as_bytes());
                assert_eq!(digest.finalize(), expected);

                let mut digest = crc_nolookup.digest();
                digest.update(data.as_bytes());
                assert_eq!(digest.finalize(), expected);

                // Check that we didn't break updating from multiple sources
                if data.len() > 2 {
                    let data = data.as_bytes();
                    let data1 = &data[..data.len() / 2];
                    let data2 = &data[data.len() / 2..];
                    let mut digest = crc_slice16.digest();
                    digest.update(data1);
                    digest.update(data2);
                    assert_eq!(digest.finalize(), expected);
                    let mut digest = crc_nolookup.digest();
                    digest.update(data1);
                    digest.update(data2);
                    assert_eq!(digest.finalize(), expected);
                }
            }
        }
    }
}
