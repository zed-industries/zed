//! `crypt(3)` Base64 encoding for sha* family.

use super::{Alphabet, DecodeStep, EncodeStep};

/// `crypt(3)` Base64 encoding for the following schemes.
///  * sha1_crypt,
///  * sha256_crypt,
///  * sha512_crypt,
///  * md5_crypt
///
/// ```text
/// [.-9]      [A-Z]      [a-z]
/// 0x2e-0x39, 0x41-0x5a, 0x61-0x7a
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Base64ShaCrypt;

impl Alphabet for Base64ShaCrypt {
    const BASE: u8 = b'.';

    const DECODER: &'static [DecodeStep] = &[
        DecodeStep::Range(b'.'..=b'9', -45),
        DecodeStep::Range(b'A'..=b'Z', -52),
        DecodeStep::Range(b'a'..=b'z', -58),
    ];

    const ENCODER: &'static [EncodeStep] =
        &[EncodeStep::Apply(b'9', 7), EncodeStep::Apply(b'Z', 6)];

    const PADDED: bool = false;

    type Unpadded = Self;

    #[inline(always)]
    fn decode_3bytes(src: &[u8], dst: &mut [u8]) -> i16 {
        debug_assert_eq!(src.len(), 4);
        debug_assert!(dst.len() >= 3, "dst too short: {}", dst.len());

        let c0 = Self::decode_6bits(src[0]);
        let c1 = Self::decode_6bits(src[1]);
        let c2 = Self::decode_6bits(src[2]);
        let c3 = Self::decode_6bits(src[3]);

        dst[0] = (c0 | ((c1 & 0x3) << 6)) as u8;
        dst[1] = ((c1 >> 2) | ((c2 & 0xF) << 4)) as u8;
        dst[2] = ((c2 >> 4) | (c3 << 2)) as u8;

        ((c0 | c1 | c2 | c3) >> 8) & 1
    }

    #[inline(always)]
    fn encode_3bytes(src: &[u8], dst: &mut [u8]) {
        debug_assert_eq!(src.len(), 3);
        debug_assert!(dst.len() >= 4, "dst too short: {}", dst.len());

        let b0 = src[0] as i16;
        let b1 = src[1] as i16;
        let b2 = src[2] as i16;

        dst[0] = Self::encode_6bits(b0 & 63);
        dst[1] = Self::encode_6bits(((b1 << 2) | (b0 >> 6)) & 63);
        dst[2] = Self::encode_6bits(((b2 << 4) | (b1 >> 4)) & 63);
        dst[3] = Self::encode_6bits(b2 >> 2);
    }
}
