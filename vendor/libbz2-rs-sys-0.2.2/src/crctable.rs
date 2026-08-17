/// The polynomial used for the crc32 lookup table.
///
/// See also https://en.wikipedia.org/wiki/Cyclic_redundancy_check#Polynomial_representations
const POLYNOMIAL: u32 = 0x04C11DB7;

/// Most implementations (ethernet, zlib) use the reflected version of this polynomial.
const _: () = assert!(POLYNOMIAL.reverse_bits() == 0xEDB88320);

/// Lookup table to speed up crc32 checksum calculation.
///
/// The original C implementation notes:
///
/// > I think this is an implementation of the AUTODIN-II,
/// > Ethernet & FDDI 32-bit CRC standard.  Vaguely derived
/// > from code by Rob Warnock, in Section 51 of the
/// > comp.compression FAQ.
pub(crate) static BZ2_CRC32TABLE: [u32; 256] = generate_crc32_table(POLYNOMIAL);

/// Generate the crc32 lookup table.
///
/// Note that contrary to most material you'll find on the internet, we're using the non-reflected
/// polynomial, which impacts some of the logic (e.g. we bitwise and with 0x80000000 instead of 0x1).
///
/// This [article] has some excellent additional detail on how crc works, and how to make it fast.
///
/// [article]: https://create.stephan-brumme.com/crc32/
const fn generate_crc32_table(polynomial: u32) -> [u32; 256] {
    let mut table = [0u32; 256];

    let mut i = 0;
    while i < 256 {
        let mut crc = (i as u32) << 24;

        let mut j = 0;
        while j < 8 {
            if (crc & 0x80000000) != 0 {
                crc = (crc << 1) ^ polynomial;
            } else {
                crc <<= 1;
            }

            j += 1;
        }

        table[i] = crc;

        i += 1;
    }

    table
}
