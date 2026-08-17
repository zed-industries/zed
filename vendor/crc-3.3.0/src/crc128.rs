use crate::table::crc128_table;
use crate::util::crc128;
use crate::*;
use crc_catalog::Algorithm;

impl<const L: usize> Crc<u128, Table<L>>
where
    Table<L>: private::Sealed,
{
    pub const fn new(algorithm: &'static Algorithm<u128>) -> Self {
        Self {
            algorithm,
            data: crc128_table(algorithm.width, algorithm.poly, algorithm.refin),
        }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u128 {
        let mut crc = init(self.algorithm, self.algorithm.init);
        crc = self.update(crc, bytes);
        finalize(self.algorithm, crc)
    }

    const fn update(&self, crc: u128, bytes: &[u8]) -> u128 {
        update_table(crc, self.algorithm, &self.data, bytes)
    }

    pub const fn digest(&self) -> Digest<u128, Table<L>> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub const fn digest_with_initial(&self, initial: u128) -> Digest<u128, Table<L>> {
        let value = init(self.algorithm, initial);
        Digest::new(self, value)
    }

    pub const fn table(&self) -> &<Table<L> as Implementation>::Data<u128> {
        &self.data
    }
}

impl<'a, const L: usize> Digest<'a, u128, Table<L>>
where
    Table<L>: private::Sealed,
{
    const fn new(crc: &'a Crc<u128, Table<L>>, value: u128) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u128 {
        finalize(self.crc.algorithm, self.value)
    }
}

const fn init(algorithm: &Algorithm<u128>, initial: u128) -> u128 {
    if algorithm.refin {
        initial.reverse_bits() >> (128u8 - algorithm.width)
    } else {
        initial << (128u8 - algorithm.width)
    }
}

const fn finalize(algorithm: &Algorithm<u128>, mut crc: u128) -> u128 {
    if algorithm.refin ^ algorithm.refout {
        crc = crc.reverse_bits();
    }
    if !algorithm.refout {
        crc >>= 128u8 - algorithm.width;
    }
    crc ^ algorithm.xorout
}

const fn update_table<const L: usize>(
    mut crc: u128,
    algorithm: &Algorithm<u128>,
    table: &[[u128; 256]; L],
    bytes: &[u8],
) -> u128 {
    let len = bytes.len();
    let mut i = 0;
    let reflect = algorithm.refin;

    // Process 16 bytes at a time when L=16
    if L == 16 {
        while i + 16 <= len {
            if reflect {
                // XOR the first 16 bytes with the current CRC value
                let current0 = bytes[i] ^ (crc as u8);
                let current1 = bytes[i + 1] ^ ((crc >> 8) as u8);
                let current2 = bytes[i + 2] ^ ((crc >> 16) as u8);
                let current3 = bytes[i + 3] ^ ((crc >> 24) as u8);
                let current4 = bytes[i + 4] ^ ((crc >> 32) as u8);
                let current5 = bytes[i + 5] ^ ((crc >> 40) as u8);
                let current6 = bytes[i + 6] ^ ((crc >> 48) as u8);
                let current7 = bytes[i + 7] ^ ((crc >> 56) as u8);
                let current8 = bytes[i + 8] ^ ((crc >> 64) as u8);
                let current9 = bytes[i + 9] ^ ((crc >> 72) as u8);
                let current10 = bytes[i + 10] ^ ((crc >> 80) as u8);
                let current11 = bytes[i + 11] ^ ((crc >> 88) as u8);
                let current12 = bytes[i + 12] ^ ((crc >> 96) as u8);
                let current13 = bytes[i + 13] ^ ((crc >> 104) as u8);
                let current14 = bytes[i + 14] ^ ((crc >> 112) as u8);
                let current15 = bytes[i + 15] ^ ((crc >> 120) as u8);

                crc = table[0][current15 as usize]
                    ^ table[1][current14 as usize]
                    ^ table[2][current13 as usize]
                    ^ table[3][current12 as usize]
                    ^ table[4][current11 as usize]
                    ^ table[5][current10 as usize]
                    ^ table[6][current9 as usize]
                    ^ table[7][current8 as usize]
                    ^ table[8][current7 as usize]
                    ^ table[9][current6 as usize]
                    ^ table[10][current5 as usize]
                    ^ table[11][current4 as usize]
                    ^ table[12][current3 as usize]
                    ^ table[13][current2 as usize]
                    ^ table[14][current1 as usize]
                    ^ table[15][current0 as usize];
            } else {
                // For non-reflected CRC128
                let current0 = bytes[i] ^ ((crc >> 120) as u8);
                let current1 = bytes[i + 1] ^ ((crc >> 112) as u8);
                let current2 = bytes[i + 2] ^ ((crc >> 104) as u8);
                let current3 = bytes[i + 3] ^ ((crc >> 96) as u8);
                let current4 = bytes[i + 4] ^ ((crc >> 88) as u8);
                let current5 = bytes[i + 5] ^ ((crc >> 80) as u8);
                let current6 = bytes[i + 6] ^ ((crc >> 72) as u8);
                let current7 = bytes[i + 7] ^ ((crc >> 64) as u8);
                let current8 = bytes[i + 8] ^ ((crc >> 56) as u8);
                let current9 = bytes[i + 9] ^ ((crc >> 48) as u8);
                let current10 = bytes[i + 10] ^ ((crc >> 40) as u8);
                let current11 = bytes[i + 11] ^ ((crc >> 32) as u8);
                let current12 = bytes[i + 12] ^ ((crc >> 24) as u8);
                let current13 = bytes[i + 13] ^ ((crc >> 16) as u8);
                let current14 = bytes[i + 14] ^ ((crc >> 8) as u8);
                let current15 = bytes[i + 15] ^ (crc as u8);

                crc = table[0][current15 as usize]
                    ^ table[1][current14 as usize]
                    ^ table[2][current13 as usize]
                    ^ table[3][current12 as usize]
                    ^ table[4][current11 as usize]
                    ^ table[5][current10 as usize]
                    ^ table[6][current9 as usize]
                    ^ table[7][current8 as usize]
                    ^ table[8][current7 as usize]
                    ^ table[9][current6 as usize]
                    ^ table[10][current5 as usize]
                    ^ table[11][current4 as usize]
                    ^ table[12][current3 as usize]
                    ^ table[13][current2 as usize]
                    ^ table[14][current1 as usize]
                    ^ table[15][current0 as usize];
            }
            i += 16;
        }
    }

    // Process remaining bytes one at a time using the table (for L=1 and L=16)
    if L > 0 {
        if reflect {
            while i < len {
                let table_index = ((crc ^ bytes[i] as u128) & 0xFF) as usize;
                crc = table[0][table_index] ^ (crc >> 8);
                i += 1;
            }
        } else {
            while i < len {
                let table_index = (((crc >> 120) ^ bytes[i] as u128) & 0xFF) as usize;
                crc = table[0][table_index] ^ (crc << 8);
                i += 1;
            }
        }
    } else {
        // This section is for NoTable case (L=0)
        let poly = if reflect {
            let poly = algorithm.poly.reverse_bits();
            poly >> (128u8 - algorithm.width)
        } else {
            algorithm.poly << (128u8 - algorithm.width)
        };

        if reflect {
            while i < len {
                let to_crc = (crc ^ bytes[i] as u128) & 0xFF;
                crc = crc128(poly, reflect, to_crc) ^ (crc >> 8);
                i += 1;
            }
        } else {
            while i < len {
                let to_crc = ((crc >> 120) ^ bytes[i] as u128) & 0xFF;
                crc = crc128(poly, reflect, to_crc) ^ (crc << 8);
                i += 1;
            }
        }
    }

    crc
}

#[cfg(test)]
mod test {
    use crate::*;
    use crc_catalog::{Algorithm, CRC_82_DARC};

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

        pub const CRC_82_DARC_NONREFLEX: Algorithm<u128> = Algorithm {
            width: 82,
            poly: 0x0308c0111011401440411,
            init: 0x000000000000000000000,
            refin: false,
            refout: true,
            xorout: 0x000000000000000000000,
            check: 0x09ea83f625023801fd612,
            residue: 0x000000000000000000000,
        };

        let algs_to_test = [&CRC_82_DARC, &CRC_82_DARC_NONREFLEX];

        for alg in algs_to_test {
            for data in data {
                let crc_slice16 = Crc::<u128, Table<16>>::new(alg);
                let crc_nolookup = Crc::<u128, NoTable>::new(alg);
                let expected = Crc::<u128, Table<1>>::new(alg).checksum(data.as_bytes());

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
