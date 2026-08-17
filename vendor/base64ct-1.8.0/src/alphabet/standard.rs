//! Standard Base64 encoding.

use super::{Alphabet, DecodeStep, EncodeStep};

/// Standard Base64 encoding with `=` padding.
///
/// ```text
/// [A-Z]      [a-z]      [0-9]      +     /
/// 0x41-0x5a, 0x61-0x7a, 0x30-0x39, 0x2b, 0x2f
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Base64;

impl Alphabet for Base64 {
    const BASE: u8 = b'A';
    const DECODER: &'static [DecodeStep] = DECODER;
    const ENCODER: &'static [EncodeStep] = ENCODER;
    const PADDED: bool = true;
    type Unpadded = Base64Unpadded;
}

/// Standard Base64 encoding *without* padding.
///
/// ```text
/// [A-Z]      [a-z]      [0-9]      +     /
/// 0x41-0x5a, 0x61-0x7a, 0x30-0x39, 0x2b, 0x2f
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Base64Unpadded;

impl Alphabet for Base64Unpadded {
    const BASE: u8 = b'A';
    const DECODER: &'static [DecodeStep] = DECODER;
    const ENCODER: &'static [EncodeStep] = ENCODER;
    const PADDED: bool = false;
    type Unpadded = Self;
}

/// Standard Base64 decoder
const DECODER: &[DecodeStep] = &[
    DecodeStep::Range(b'A'..=b'Z', -64),
    DecodeStep::Range(b'a'..=b'z', -70),
    DecodeStep::Range(b'0'..=b'9', 5),
    DecodeStep::Eq(b'+', 63),
    DecodeStep::Eq(b'/', 64),
];

/// Standard Base64 encoder
const ENCODER: &[EncodeStep] = &[
    EncodeStep::Diff(25, 6),
    EncodeStep::Diff(51, -75),
    EncodeStep::Diff(61, -(b'+' as i16 - 0x1c)),
    EncodeStep::Diff(62, b'/' as i16 - b'+' as i16 - 1),
];
