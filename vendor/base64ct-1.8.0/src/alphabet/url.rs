//! URL-safe Base64 encoding.

use super::{Alphabet, DecodeStep, EncodeStep};

/// URL-safe Base64 encoding with `=` padding.
///
/// ```text
/// [A-Z]      [a-z]      [0-9]      -     _
/// 0x41-0x5a, 0x61-0x7a, 0x30-0x39, 0x2d, 0x5f
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Base64Url;

impl Alphabet for Base64Url {
    const BASE: u8 = b'A';
    const DECODER: &'static [DecodeStep] = DECODER;
    const ENCODER: &'static [EncodeStep] = ENCODER;
    const PADDED: bool = true;
    type Unpadded = Base64UrlUnpadded;
}

/// URL-safe Base64 encoding *without* padding.
///
/// ```text
/// [A-Z]      [a-z]      [0-9]      -     _
/// 0x41-0x5a, 0x61-0x7a, 0x30-0x39, 0x2d, 0x5f
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Base64UrlUnpadded;

impl Alphabet for Base64UrlUnpadded {
    const BASE: u8 = b'A';
    const DECODER: &'static [DecodeStep] = DECODER;
    const ENCODER: &'static [EncodeStep] = ENCODER;
    const PADDED: bool = false;
    type Unpadded = Self;
}

/// URL-safe Base64 decoder
const DECODER: &[DecodeStep] = &[
    DecodeStep::Range(b'A'..=b'Z', -64),
    DecodeStep::Range(b'a'..=b'z', -70),
    DecodeStep::Range(b'0'..=b'9', 5),
    DecodeStep::Eq(b'-', 63),
    DecodeStep::Eq(b'_', 64),
];

/// URL-safe Base64 encoder
const ENCODER: &[EncodeStep] = &[
    EncodeStep::Diff(25, 6),
    EncodeStep::Diff(51, -75),
    EncodeStep::Diff(61, -(b'-' as i16 - 0x20)),
    EncodeStep::Diff(62, b'_' as i16 - b'-' as i16 - 1),
];
