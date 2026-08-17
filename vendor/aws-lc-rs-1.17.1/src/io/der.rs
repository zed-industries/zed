// Copyright 2015 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! Building blocks for parsing DER-encoded ASN.1 structures.
//!
//! This module contains the foundational parts of an ASN.1 DER parser.

use super::Positive;
use crate::error;

pub const CONSTRUCTED: u8 = 1 << 5;
pub const CONTEXT_SPECIFIC: u8 = 2 << 6;

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Tag {
    Boolean = 0x01,
    Integer = 0x02,
    BitString = 0x03,
    OctetString = 0x04,
    Null = 0x05,
    OID = 0x06,
    Sequence = CONSTRUCTED | 0x10, // 0x30
    UTCTime = 0x17,
    GeneralizedTime = 0x18,

    ContextSpecificConstructed0 = CONTEXT_SPECIFIC | CONSTRUCTED,
    ContextSpecificConstructed1 = CONTEXT_SPECIFIC | CONSTRUCTED | 1,
    ContextSpecificConstructed3 = CONTEXT_SPECIFIC | CONSTRUCTED | 3,
}

impl From<Tag> for usize {
    fn from(tag: Tag) -> Self {
        tag as Self
    }
}

impl From<Tag> for u8 {
    fn from(tag: Tag) -> Self {
        tag as Self
    } // XXX: narrowing conversion.
}

pub fn expect_tag_and_get_value<'a>(
    input: &mut untrusted::Reader<'a>,
    tag: Tag,
) -> Result<untrusted::Input<'a>, error::Unspecified> {
    let (actual_tag, inner) = read_tag_and_get_value(input)?;
    if usize::from(tag) != usize::from(actual_tag) {
        return Err(error::Unspecified);
    }
    Ok(inner)
}

pub fn read_tag_and_get_value<'a>(
    input: &mut untrusted::Reader<'a>,
) -> Result<(u8, untrusted::Input<'a>), error::Unspecified> {
    let tag = input.read_byte()?;
    if (tag & 0x1F) == 0x1F {
        return Err(error::Unspecified); // High tag number form is not allowed.
    }

    // If the high order bit of the first byte is set to zero then the length
    // is encoded in the seven remaining bits of that byte. Otherwise, those
    // seven bits represent the number of bytes used to encode the length.
    let length = match input.read_byte()? {
        n if (n & 0x80) == 0 => usize::from(n),
        0x81 => {
            let second_byte = input.read_byte()?;
            if second_byte < 128 {
                return Err(error::Unspecified); // Not the canonical encoding.
            }
            usize::from(second_byte)
        }
        0x82 => {
            let second_byte = usize::from(input.read_byte()?);
            let third_byte = usize::from(input.read_byte()?);
            let combined = (second_byte << 8) | third_byte;
            if combined < 256 {
                return Err(error::Unspecified); // Not the canonical encoding.
            }
            combined
        }
        _ => {
            return Err(error::Unspecified); // We don't support longer lengths.
        }
    };

    let inner = input.read_bytes(length)?;
    Ok((tag, inner))
}

pub fn bit_string_with_no_unused_bits<'a>(
    input: &mut untrusted::Reader<'a>,
) -> Result<untrusted::Input<'a>, error::Unspecified> {
    nested(input, Tag::BitString, error::Unspecified, |value| {
        let unused_bits_at_end = value.read_byte()?;
        if unused_bits_at_end != 0 {
            return Err(error::Unspecified);
        }
        Ok(value.read_bytes_to_end())
    })
}

// TODO: investigate taking decoder as a reference to reduce generated code
// size.
pub fn nested<'a, F, R, E: Copy>(
    input: &mut untrusted::Reader<'a>,
    tag: Tag,
    error: E,
    decoder: F,
) -> Result<R, E>
where
    F: FnOnce(&mut untrusted::Reader<'a>) -> Result<R, E>,
{
    let inner = expect_tag_and_get_value(input, tag).map_err(|_| error)?;
    inner.read_all(error, decoder)
}

fn nonnegative_integer<'a>(
    input: &mut untrusted::Reader<'a>,
    min_value: u8,
) -> Result<untrusted::Input<'a>, error::Unspecified> {
    // Verify that |input|, which has had any leading zero stripped off, is the
    // encoding of a value of at least |min_value|.
    fn check_minimum(input: untrusted::Input, min_value: u8) -> Result<(), error::Unspecified> {
        input.read_all(error::Unspecified, |input| {
            let first_byte = input.read_byte()?;
            if input.at_end() && first_byte < min_value {
                return Err(error::Unspecified);
            }
            let _: untrusted::Input = input.read_bytes_to_end();
            Ok(())
        })
    }

    let value = expect_tag_and_get_value(input, Tag::Integer)?;

    value.read_all(error::Unspecified, |input| {
        // Empty encodings are not allowed.
        let first_byte = input.read_byte()?;

        if first_byte == 0 {
            if input.at_end() {
                // |value| is the legal encoding of zero.
                if min_value > 0 {
                    return Err(error::Unspecified);
                }
                return Ok(value);
            }

            let r = input.read_bytes_to_end();
            r.read_all(error::Unspecified, |input| {
                let second_byte = input.read_byte()?;
                if (second_byte & 0x80) == 0 {
                    // A leading zero is only allowed when the value's high bit
                    // is set.
                    return Err(error::Unspecified);
                }
                let _: untrusted::Input = input.read_bytes_to_end();
                Ok(())
            })?;
            check_minimum(r, min_value)?;
            return Ok(r);
        }

        // Negative values are not allowed.
        if (first_byte & 0x80) != 0 {
            return Err(error::Unspecified);
        }

        let _: untrusted::Input = input.read_bytes_to_end();
        check_minimum(value, min_value)?;
        Ok(value)
    })
}

/// Parse as integer with a value in the in the range [0, 255], returning its
/// numeric value. This is typically used for parsing version numbers.
#[inline]
pub fn small_nonnegative_integer(input: &mut untrusted::Reader) -> Result<u8, error::Unspecified> {
    let value = nonnegative_integer(input, 0)?;
    value.read_all(error::Unspecified, |input| {
        let r = input.read_byte()?;
        Ok(r)
    })
}

/// Parses a positive DER integer, returning the big-endian-encoded value,
/// sans any leading zero byte.
pub fn positive_integer<'a>(
    input: &mut untrusted::Reader<'a>,
) -> Result<Positive<'a>, error::Unspecified> {
    Ok(Positive::new_non_empty_without_leading_zeros(
        nonnegative_integer(input, 1)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use untrusted::Input;

    fn with_good_i<F, R>(value: &[u8], f: F)
    where
        F: FnOnce(&mut untrusted::Reader) -> Result<R, error::Unspecified>,
    {
        let r = Input::from(value).read_all(error::Unspecified, f);
        assert!(r.is_ok());
    }

    fn with_bad_i<F, R>(value: &[u8], f: F)
    where
        F: FnOnce(&mut untrusted::Reader) -> Result<R, error::Unspecified>,
    {
        let r = Input::from(value).read_all(error::Unspecified, f);
        assert!(r.is_err());
    }

    static ZERO_INTEGER: &[u8] = &[0x02, 0x01, 0x00];

    static GOOD_POSITIVE_INTEGERS: &[(&[u8], u8)] = &[
        (&[0x02, 0x01, 0x01], 0x01),
        (&[0x02, 0x01, 0x02], 0x02),
        (&[0x02, 0x01, 0x7e], 0x7e),
        (&[0x02, 0x01, 0x7f], 0x7f),
        // Values that need to have an 0x00 prefix to disambiguate them from
        // them from negative values.
        (&[0x02, 0x02, 0x00, 0x80], 0x80),
        (&[0x02, 0x02, 0x00, 0x81], 0x81),
        (&[0x02, 0x02, 0x00, 0xfe], 0xfe),
        (&[0x02, 0x02, 0x00, 0xff], 0xff),
    ];

    #[allow(clippy::type_complexity)]
    static GOOD_BIG_POSITIVE_INTEGERS: &[((&[u8], &[u8]), (&[u8], &[u8]))] = &[
        ((&[0x02, 0x81, 129u8, 1], &[0; 128]), (&[1], &[0; 128])),
        ((&[0x02, 0x82, 0x01, 0x00, 1], &[0; 255]), (&[1], &[0; 255])),
    ];

    static BAD_NONNEGATIVE_INTEGERS: &[&[u8]] = &[
        &[],           // At end of input
        &[0x02],       // Tag only
        &[0x02, 0x00], // Empty value
        // Length mismatch
        &[0x02, 0x00, 0x01],
        &[0x02, 0x01],
        &[0x02, 0x01, 0x00, 0x01],
        &[0x02, 0x01, 0x01, 0x00], // Would be valid if last byte is ignored.
        &[0x02, 0x02, 0x01],
        // Negative values
        &[0x02, 0x01, 0x80],
        &[0x02, 0x01, 0xfe],
        &[0x02, 0x01, 0xff],
        // Values that have an unnecessary leading 0x00
        &[0x02, 0x02, 0x00, 0x00],
        &[0x02, 0x02, 0x00, 0x01],
        &[0x02, 0x02, 0x00, 0x02],
        &[0x02, 0x02, 0x00, 0x7e],
        &[0x02, 0x02, 0x00, 0x7f],
    ];

    #[test]
    fn test_small_nonnegative_integer() {
        with_good_i(ZERO_INTEGER, |input| {
            assert_eq!(small_nonnegative_integer(input)?, 0x00);
            Ok(())
        });
        for &(test_in, test_out) in GOOD_POSITIVE_INTEGERS {
            with_good_i(test_in, |input| {
                assert_eq!(small_nonnegative_integer(input)?, test_out);
                Ok(())
            });
        }
        for &test_in in BAD_NONNEGATIVE_INTEGERS {
            with_bad_i(test_in, |input| {
                let _: u8 = small_nonnegative_integer(input)?;
                Ok(())
            });
        }
    }

    #[test]
    fn test_positive_integer() {
        with_bad_i(ZERO_INTEGER, |input| {
            let _: Positive<'_> = positive_integer(input)?;
            Ok(())
        });
        for &(test_in, test_out) in GOOD_POSITIVE_INTEGERS {
            with_good_i(test_in, |input| {
                let test_out = [test_out];
                assert_eq!(
                    positive_integer(input)?
                        .big_endian_without_leading_zero_as_input()
                        .as_slice_less_safe(),
                    Input::from(&test_out[..]).as_slice_less_safe()
                );
                Ok(())
            });
        }
        for &test_in in BAD_NONNEGATIVE_INTEGERS {
            with_bad_i(test_in, |input| {
                let _: Positive<'_> = positive_integer(input)?;
                Ok(())
            });
        }
    }

    #[test]
    fn test_tag() {
        let tgt = usize::from(Tag::GeneralizedTime);
        assert_eq!(0x18usize, tgt);

        let tgt = u8::from(Tag::GeneralizedTime);
        assert_eq!(0x18u8, tgt);

        let tgt = Tag::GeneralizedTime;
        assert_eq!(tgt, Tag::GeneralizedTime);
    }

    #[test]
    fn test_big() {
        for &((bytes_in_a, bytes_in_b), (bytes_out_a, bytes_out_b)) in GOOD_BIG_POSITIVE_INTEGERS {
            let mut bytes_in = Vec::new();
            bytes_in.extend(bytes_in_a);
            bytes_in.extend(bytes_in_b);
            let mut bytes_out: Vec<u8> = Vec::new();
            bytes_out.extend(bytes_out_a);
            bytes_out.extend(bytes_out_b);

            with_good_i(&bytes_in, |input| {
                let positive = positive_integer(input)?;
                let expected_bytes = positive.big_endian_without_leading_zero();
                assert_eq!(expected_bytes, &bytes_out);
                Ok(())
            });
        }
    }

    #[test]
    fn test_bit_string_with_no_unused_bits() {
        // Not a BitString
        let mut reader_bad = untrusted::Reader::new(Input::from(&[0x02, 0x01]));
        assert!(bit_string_with_no_unused_bits(&mut reader_bad).is_err());
        // Unused bits at end
        let mut reader_bad2 = untrusted::Reader::new(Input::from(&[0x03, 0x01, 0x01]));
        assert!(bit_string_with_no_unused_bits(&mut reader_bad2).is_err());

        let mut reader_good = untrusted::Reader::new(Input::from(&[0x03, 0x01, 0x00]));
        let input = bit_string_with_no_unused_bits(&mut reader_good).unwrap();
        let expected_result: &[u8] = &[];
        assert_eq!(expected_result, input.as_slice_less_safe());
    }
}
