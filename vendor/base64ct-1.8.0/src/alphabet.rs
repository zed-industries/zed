//! Base64 alphabets.

// TODO(tarcieri): explicitly checked/wrapped arithmetic
#![allow(clippy::arithmetic_side_effects)]

use core::{fmt::Debug, ops::RangeInclusive};

pub mod bcrypt;
pub mod crypt;
pub mod shacrypt;
pub mod standard;
pub mod url;

/// Core encoder/decoder functions for a particular Base64 alphabet.
pub trait Alphabet: 'static + Copy + Debug + Eq + Send + Sized + Sync {
    /// First character in this Base64 alphabet.
    const BASE: u8;

    /// Decoder passes
    const DECODER: &'static [DecodeStep];

    /// Encoder passes
    const ENCODER: &'static [EncodeStep];

    /// Is this encoding padded?
    const PADDED: bool;

    /// Unpadded equivalent of this alphabet.
    ///
    /// For alphabets that are unpadded to begin with, this should be `Self`.
    type Unpadded: Alphabet;

    /// Decode 3 bytes of a Base64 message.
    #[inline(always)]
    fn decode_3bytes(src: &[u8], dst: &mut [u8]) -> i16 {
        debug_assert_eq!(src.len(), 4);
        debug_assert!(dst.len() >= 3, "dst too short: {}", dst.len());

        let c0 = Self::decode_6bits(src[0]);
        let c1 = Self::decode_6bits(src[1]);
        let c2 = Self::decode_6bits(src[2]);
        let c3 = Self::decode_6bits(src[3]);

        dst[0] = ((c0 << 2) | (c1 >> 4)) as u8;
        dst[1] = ((c1 << 4) | (c2 >> 2)) as u8;
        dst[2] = ((c2 << 6) | c3) as u8;

        ((c0 | c1 | c2 | c3) >> 8) & 1
    }

    /// Decode 6-bits of a Base64 message.
    fn decode_6bits(src: u8) -> i16 {
        let mut ret: i16 = -1;

        for step in Self::DECODER {
            ret += match step {
                DecodeStep::Range(range, offset) => {
                    // Compute exclusive range from inclusive one
                    let start = *range.start() as i16 - 1;
                    let end = *range.end() as i16 + 1;
                    (((start - src as i16) & (src as i16 - end)) >> 8) & (src as i16 + *offset)
                }
                DecodeStep::Eq(value, offset) => {
                    let start = *value as i16 - 1;
                    let end = *value as i16 + 1;
                    (((start - src as i16) & (src as i16 - end)) >> 8) & *offset
                }
            };
        }

        ret
    }

    /// Encode 3-bytes of a Base64 message.
    #[inline(always)]
    fn encode_3bytes(src: &[u8], dst: &mut [u8]) {
        debug_assert_eq!(src.len(), 3);
        debug_assert!(dst.len() >= 4, "dst too short: {}", dst.len());

        let b0 = src[0] as i16;
        let b1 = src[1] as i16;
        let b2 = src[2] as i16;

        dst[0] = Self::encode_6bits(b0 >> 2);
        dst[1] = Self::encode_6bits(((b0 << 4) | (b1 >> 4)) & 63);
        dst[2] = Self::encode_6bits(((b1 << 2) | (b2 >> 6)) & 63);
        dst[3] = Self::encode_6bits(b2 & 63);
    }

    /// Encode 6-bits of a Base64 message.
    #[inline(always)]
    fn encode_6bits(src: i16) -> u8 {
        let mut diff = src + Self::BASE as i16;

        for &step in Self::ENCODER {
            diff += match step {
                EncodeStep::Apply(threshold, offset) => ((threshold as i16 - diff) >> 8) & offset,
                EncodeStep::Diff(threshold, offset) => ((threshold as i16 - src) >> 8) & offset,
            };
        }

        diff as u8
    }
}

/// Constant-time decoder step.
#[derive(Debug)]
pub enum DecodeStep {
    /// Match the given range, offsetting the input on match.
    Range(RangeInclusive<u8>, i16),

    /// Match the given value, returning the associated offset on match.
    Eq(u8, i16),
}

/// Constant-time encoder step.
#[derive(Copy, Clone, Debug)]
pub enum EncodeStep {
    /// Apply the given offset to the cumulative result on match.
    Apply(u8, i16),

    /// Compute a difference using the given offset on match.
    Diff(u8, i16),
}
