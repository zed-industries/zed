//! bcrypt Base64 encoding.

use super::{Alphabet, DecodeStep, EncodeStep};

/// bcrypt Base64 encoding.
///
/// ```text
/// ./         [A-Z]      [a-z]     [0-9]
/// 0x2e-0x2f, 0x41-0x5a, 0x61-0x7a, 0x30-0x39
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Base64Bcrypt;

impl Alphabet for Base64Bcrypt {
    const BASE: u8 = b'.';

    const DECODER: &'static [DecodeStep] = &[
        DecodeStep::Range(b'.'..=b'/', -45),
        DecodeStep::Range(b'A'..=b'Z', -62),
        DecodeStep::Range(b'a'..=b'z', -68),
        DecodeStep::Range(b'0'..=b'9', 7),
    ];

    const ENCODER: &'static [EncodeStep] = &[
        EncodeStep::Apply(b'/', 17),
        EncodeStep::Apply(b'Z', 6),
        EncodeStep::Apply(b'z', -75),
    ];

    const PADDED: bool = false;

    type Unpadded = Self;
}
