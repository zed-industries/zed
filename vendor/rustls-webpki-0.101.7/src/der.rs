// Copyright 2015 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use crate::{calendar, time, Error};
pub(crate) use ring::io::der::{CONSTRUCTED, CONTEXT_SPECIFIC};

// Copied (and extended) from ring's src/der.rs
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum Tag {
    Boolean = 0x01,
    Integer = 0x02,
    BitString = 0x03,
    OctetString = 0x04,
    OID = 0x06,
    Enum = 0x0A,
    Sequence = CONSTRUCTED | 0x10, // 0x30
    UTCTime = 0x17,
    GeneralizedTime = 0x18,

    #[allow(clippy::identity_op)]
    ContextSpecificConstructed0 = CONTEXT_SPECIFIC | CONSTRUCTED | 0,
    ContextSpecificConstructed1 = CONTEXT_SPECIFIC | CONSTRUCTED | 1,
    ContextSpecificConstructed3 = CONTEXT_SPECIFIC | CONSTRUCTED | 3,
}

impl From<Tag> for usize {
    #[allow(clippy::as_conversions)]
    fn from(tag: Tag) -> Self {
        tag as Self
    }
}

impl From<Tag> for u8 {
    #[allow(clippy::as_conversions)]
    fn from(tag: Tag) -> Self {
        tag as Self
    } // XXX: narrowing conversion.
}

#[inline(always)]
pub(crate) fn expect_tag_and_get_value<'a>(
    input: &mut untrusted::Reader<'a>,
    tag: Tag,
) -> Result<untrusted::Input<'a>, Error> {
    let (actual_tag, inner) = read_tag_and_get_value(input)?;
    if usize::from(tag) != usize::from(actual_tag) {
        return Err(Error::BadDer);
    }
    Ok(inner)
}

#[inline(always)]
pub(crate) fn expect_tag_and_get_value_limited<'a>(
    input: &mut untrusted::Reader<'a>,
    tag: Tag,
    size_limit: usize,
) -> Result<untrusted::Input<'a>, Error> {
    let (actual_tag, inner) = read_tag_and_get_value_limited(input, size_limit)?;
    if usize::from(tag) != usize::from(actual_tag) {
        return Err(Error::BadDer);
    }
    Ok(inner)
}

pub(crate) fn nested_limited<'a, R, E: Copy>(
    input: &mut untrusted::Reader<'a>,
    tag: Tag,
    error: E,
    decoder: impl FnOnce(&mut untrusted::Reader<'a>) -> Result<R, E>,
    size_limit: usize,
) -> Result<R, E> {
    expect_tag_and_get_value_limited(input, tag, size_limit)
        .map_err(|_| error)?
        .read_all(error, decoder)
}

// TODO: investigate taking decoder as a reference to reduce generated code
// size.
pub(crate) fn nested<'a, R, E: Copy>(
    input: &mut untrusted::Reader<'a>,
    tag: Tag,
    error: E,
    decoder: impl FnOnce(&mut untrusted::Reader<'a>) -> Result<R, E>,
) -> Result<R, E> {
    nested_limited(input, tag, error, decoder, TWO_BYTE_DER_SIZE)
}

pub(crate) struct Value<'a> {
    value: untrusted::Input<'a>,
}

impl<'a> Value<'a> {
    pub(crate) fn value(&self) -> untrusted::Input<'a> {
        self.value
    }
}

pub(crate) fn expect_tag<'a>(
    input: &mut untrusted::Reader<'a>,
    tag: Tag,
) -> Result<Value<'a>, Error> {
    let (actual_tag, value) = read_tag_and_get_value(input)?;
    if usize::from(tag) != usize::from(actual_tag) {
        return Err(Error::BadDer);
    }

    Ok(Value { value })
}

#[inline(always)]
pub(crate) fn read_tag_and_get_value<'a>(
    input: &mut untrusted::Reader<'a>,
) -> Result<(u8, untrusted::Input<'a>), Error> {
    read_tag_and_get_value_limited(input, TWO_BYTE_DER_SIZE)
}

#[inline(always)]
pub(crate) fn read_tag_and_get_value_limited<'a>(
    input: &mut untrusted::Reader<'a>,
    size_limit: usize,
) -> Result<(u8, untrusted::Input<'a>), Error> {
    let tag = input.read_byte()?;
    if (tag & HIGH_TAG_RANGE_START) == HIGH_TAG_RANGE_START {
        return Err(Error::BadDer); // High tag number form is not allowed.
    }

    // If the high order bit of the first byte is set to zero then the length
    // is encoded in the seven remaining bits of that byte. Otherwise, those
    // seven bits represent the number of bytes used to encode the length.
    let length = match input.read_byte()? {
        n if (n & SHORT_FORM_LEN_MAX) == 0 => usize::from(n),
        LONG_FORM_LEN_ONE_BYTE => {
            let length_byte = input.read_byte()?;
            if length_byte < SHORT_FORM_LEN_MAX {
                return Err(Error::BadDer); // Not the canonical encoding.
            }
            usize::from(length_byte)
        }
        LONG_FORM_LEN_TWO_BYTES => {
            let length_byte_one = usize::from(input.read_byte()?);
            let length_byte_two = usize::from(input.read_byte()?);
            let combined = (length_byte_one << 8) | length_byte_two;
            if combined <= LONG_FORM_LEN_ONE_BYTE_MAX {
                return Err(Error::BadDer); // Not the canonical encoding.
            }
            combined
        }
        LONG_FORM_LEN_THREE_BYTES => {
            let length_byte_one = usize::from(input.read_byte()?);
            let length_byte_two = usize::from(input.read_byte()?);
            let length_byte_three = usize::from(input.read_byte()?);
            let combined = (length_byte_one << 16) | (length_byte_two << 8) | length_byte_three;
            if combined <= LONG_FORM_LEN_TWO_BYTES_MAX {
                return Err(Error::BadDer); // Not the canonical encoding.
            }
            combined
        }
        LONG_FORM_LEN_FOUR_BYTES => {
            let length_byte_one = usize::from(input.read_byte()?);
            let length_byte_two = usize::from(input.read_byte()?);
            let length_byte_three = usize::from(input.read_byte()?);
            let length_byte_four = usize::from(input.read_byte()?);
            let combined = (length_byte_one << 24)
                | (length_byte_two << 16)
                | (length_byte_three << 8)
                | length_byte_four;
            if combined <= LONG_FORM_LEN_THREE_BYTES_MAX {
                return Err(Error::BadDer); // Not the canonical encoding.
            }
            combined
        }
        _ => {
            return Err(Error::BadDer); // We don't support longer lengths.
        }
    };

    if length >= size_limit {
        return Err(Error::BadDer); // The length is larger than the caller accepts.
    }

    let inner = input.read_bytes(length)?;
    Ok((tag, inner))
}

// Long-form DER encoded lengths of two bytes can express lengths up to the following limit.
//
// The upstream ring::io::der::read_tag_and_get_value() function limits itself to up to two byte
// long-form DER lengths, and so this limit represents the maximum length that was possible to
// read before the introduction of the read_tag_and_get_value_limited function.
pub(crate) const TWO_BYTE_DER_SIZE: usize = LONG_FORM_LEN_TWO_BYTES_MAX;

// The maximum size of a DER value that Webpki can support reading.
//
// Webpki limits itself to four byte long-form DER lengths, and so this limit represents
// the maximum size tagged DER value that can be read for any purpose.
pub(crate) const MAX_DER_SIZE: usize = LONG_FORM_LEN_FOUR_BYTES_MAX;

// DER Tag identifiers have two forms:
// * Low tag number form (for tags values in the range [0..30]
// * High tag number form (for tag values in the range [31..]
// We only support low tag number form.
const HIGH_TAG_RANGE_START: u8 = 31;

// DER length octets have two forms:
// * Short form: 1 octet supporting lengths between 0 and 127.
// * Long definite form: 2 to 127 octets, number of octets encoded into first octet.
const SHORT_FORM_LEN_MAX: u8 = 128;

// Leading octet for long definite form DER length expressed in second byte.
const LONG_FORM_LEN_ONE_BYTE: u8 = 0x81;

// Maximum size that can be expressed in a one byte long form len.
const LONG_FORM_LEN_ONE_BYTE_MAX: usize = 0xff;

// Leading octet for long definite form DER length expressed in subsequent two bytes.
const LONG_FORM_LEN_TWO_BYTES: u8 = 0x82;

// Maximum size that can be expressed in a two byte long form len.
const LONG_FORM_LEN_TWO_BYTES_MAX: usize = 0xff_ff;

// Leading octet for long definite form DER length expressed in subsequent three bytes.
const LONG_FORM_LEN_THREE_BYTES: u8 = 0x83;

// Maximum size that can be expressed in a three byte long form len.
const LONG_FORM_LEN_THREE_BYTES_MAX: usize = 0xff_ff_ff;

// Leading octet for long definite form DER length expressed in subsequent four bytes.
const LONG_FORM_LEN_FOUR_BYTES: u8 = 0x84;

// Maximum size that can be expressed in a four byte long form der len.
const LONG_FORM_LEN_FOUR_BYTES_MAX: usize = 0xff_ff_ff_ff;

// TODO: investigate taking decoder as a reference to reduce generated code
// size.
pub(crate) fn nested_of_mut<'a, E>(
    input: &mut untrusted::Reader<'a>,
    outer_tag: Tag,
    inner_tag: Tag,
    error: E,
    mut decoder: impl FnMut(&mut untrusted::Reader<'a>) -> Result<(), E>,
) -> Result<(), E>
where
    E: Copy,
{
    nested(input, outer_tag, error, |outer| {
        loop {
            nested(outer, inner_tag, error, |inner| decoder(inner))?;
            if outer.at_end() {
                break;
            }
        }
        Ok(())
    })
}

pub(crate) fn bit_string_with_no_unused_bits<'a>(
    input: &mut untrusted::Reader<'a>,
) -> Result<untrusted::Input<'a>, Error> {
    nested(input, Tag::BitString, Error::BadDer, |value| {
        let unused_bits_at_end = value.read_byte().map_err(|_| Error::BadDer)?;
        if unused_bits_at_end != 0 {
            return Err(Error::BadDer);
        }
        Ok(value.read_bytes_to_end())
    })
}

pub(crate) struct BitStringFlags<'a> {
    raw_bits: &'a [u8],
}

impl<'a> BitStringFlags<'a> {
    pub(crate) fn bit_set(&self, bit: usize) -> bool {
        let byte_index = bit / 8;
        let bit_shift = 7 - (bit % 8);

        if self.raw_bits.len() < (byte_index + 1) {
            false
        } else {
            ((self.raw_bits[byte_index] >> bit_shift) & 1) != 0
        }
    }
}

// ASN.1 BIT STRING fields for sets of flags are encoded in DER with some peculiar details related
// to padding. Notably this means we expect a Tag::BitString, a length, an indicator of the number
// of bits of padding, and then the actual bit values. See this Stack Overflow discussion[0], and
// ITU X690-0207[1] Section 8.6 and Section 11.2 for more information.
//
// [0]: https://security.stackexchange.com/a/10396
// [1]: https://www.itu.int/ITU-T/studygroups/com17/languages/X.690-0207.pdf
pub(crate) fn bit_string_flags<'a>(
    input: &mut untrusted::Reader<'a>,
) -> Result<BitStringFlags<'a>, Error> {
    expect_tag_and_get_value(input, Tag::BitString)?.read_all(Error::BadDer, |bit_string| {
        // ITU X690-0207 11.2:
        //   "The initial octet shall encode, as an unsigned binary integer with bit 1 as the least
        //   significant bit, the number of unused bits in the final subsequent octet.
        //   The number shall be in the range zero to seven"
        let padding_bits = bit_string.read_byte().map_err(|_| Error::BadDer)?;
        let raw_bits = bit_string.read_bytes_to_end().as_slice_less_safe();

        // It's illegal to have more than 7 bits of padding. Similarly, if the raw bitflags
        // are empty there should be no padding.
        if padding_bits > 7 || (raw_bits.is_empty() && padding_bits != 0) {
            return Err(Error::BadDer);
        }

        // If there are padding bits then the last bit of the last raw byte must be 0 or the
        // distinguished encoding rules are not being followed.
        let last_byte = raw_bits[raw_bits.len() - 1];
        let padding_mask = (1 << padding_bits) - 1;

        match padding_bits > 0 && (last_byte & padding_mask) != 0 {
            true => Err(Error::BadDer),
            false => Ok(BitStringFlags { raw_bits }),
        }
    })
}

// Like mozilla::pkix, we accept the nonconformant explicit encoding of
// the default value (false) for compatibility with real-world certificates.
pub(crate) fn optional_boolean(input: &mut untrusted::Reader) -> Result<bool, Error> {
    if !input.peek(Tag::Boolean.into()) {
        return Ok(false);
    }
    nested(input, Tag::Boolean, Error::BadDer, |input| {
        match input.read_byte() {
            Ok(0xff) => Ok(true),
            Ok(0x00) => Ok(false),
            _ => Err(Error::BadDer),
        }
    })
}

pub(crate) fn small_nonnegative_integer(input: &mut untrusted::Reader) -> Result<u8, Error> {
    ring::io::der::small_nonnegative_integer(input).map_err(|_| Error::BadDer)
}

pub(crate) fn time_choice(input: &mut untrusted::Reader) -> Result<time::Time, Error> {
    let is_utc_time = input.peek(Tag::UTCTime.into());
    let expected_tag = if is_utc_time {
        Tag::UTCTime
    } else {
        Tag::GeneralizedTime
    };

    fn read_digit(inner: &mut untrusted::Reader) -> Result<u64, Error> {
        const DIGIT: core::ops::RangeInclusive<u8> = b'0'..=b'9';
        let b = inner.read_byte().map_err(|_| Error::BadDerTime)?;
        if DIGIT.contains(&b) {
            return Ok(u64::from(b - DIGIT.start()));
        }
        Err(Error::BadDerTime)
    }

    fn read_two_digits(inner: &mut untrusted::Reader, min: u64, max: u64) -> Result<u64, Error> {
        let hi = read_digit(inner)?;
        let lo = read_digit(inner)?;
        let value = (hi * 10) + lo;
        if value < min || value > max {
            return Err(Error::BadDerTime);
        }
        Ok(value)
    }

    nested(input, expected_tag, Error::BadDer, |value| {
        let (year_hi, year_lo) = if is_utc_time {
            let lo = read_two_digits(value, 0, 99)?;
            let hi = if lo >= 50 { 19 } else { 20 };
            (hi, lo)
        } else {
            let hi = read_two_digits(value, 0, 99)?;
            let lo = read_two_digits(value, 0, 99)?;
            (hi, lo)
        };

        let year = (year_hi * 100) + year_lo;
        let month = read_two_digits(value, 1, 12)?;
        let days_in_month = calendar::days_in_month(year, month);
        let day_of_month = read_two_digits(value, 1, days_in_month)?;
        let hours = read_two_digits(value, 0, 23)?;
        let minutes = read_two_digits(value, 0, 59)?;
        let seconds = read_two_digits(value, 0, 59)?;

        let time_zone = value.read_byte().map_err(|_| Error::BadDerTime)?;
        if time_zone != b'Z' {
            return Err(Error::BadDerTime);
        }

        calendar::time_from_ymdhms_utc(year, month, day_of_month, hours, minutes, seconds)
    })
}

macro_rules! oid {
    ( $first:expr, $second:expr, $( $tail:expr ),* ) =>
    (
        [(40 * $first) + $second, $( $tail ),*]
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_optional_boolean() {
        use super::{optional_boolean, Error};

        // Empty input results in false
        assert!(!optional_boolean(&mut bytes_reader(&[])).unwrap());

        // Optional, so another data type results in false
        assert!(!optional_boolean(&mut bytes_reader(&[0x05, 0x00])).unwrap());

        // Only 0x00 and 0xff are accepted values
        assert_eq!(
            optional_boolean(&mut bytes_reader(&[0x01, 0x01, 0x42])).unwrap_err(),
            Error::BadDer,
        );

        // True
        assert!(optional_boolean(&mut bytes_reader(&[0x01, 0x01, 0xff])).unwrap());

        // False
        assert!(!optional_boolean(&mut bytes_reader(&[0x01, 0x01, 0x00])).unwrap());
    }

    #[test]
    fn test_bit_string_with_no_unused_bits() {
        use super::{bit_string_with_no_unused_bits, Error};

        // Unexpected type
        assert_eq!(
            bit_string_with_no_unused_bits(&mut bytes_reader(&[0x01, 0x01, 0xff])).unwrap_err(),
            Error::BadDer,
        );

        // Unexpected nonexistent type
        assert_eq!(
            bit_string_with_no_unused_bits(&mut bytes_reader(&[0x42, 0xff, 0xff])).unwrap_err(),
            Error::BadDer,
        );

        // Unexpected empty input
        assert_eq!(
            bit_string_with_no_unused_bits(&mut bytes_reader(&[])).unwrap_err(),
            Error::BadDer,
        );

        // Valid input with non-zero unused bits
        assert_eq!(
            bit_string_with_no_unused_bits(&mut bytes_reader(&[0x03, 0x03, 0x04, 0x12, 0x34]))
                .unwrap_err(),
            Error::BadDer,
        );

        // Valid input
        assert_eq!(
            bit_string_with_no_unused_bits(&mut bytes_reader(&[0x03, 0x03, 0x00, 0x12, 0x34]))
                .unwrap()
                .as_slice_less_safe(),
            &[0x12, 0x34],
        );
    }

    fn bytes_reader(bytes: &[u8]) -> untrusted::Reader {
        return untrusted::Reader::new(untrusted::Input::from(bytes));
    }

    #[test]
    fn read_tag_and_get_value_default_limit() {
        use super::{read_tag_and_get_value, Error};

        let inputs = &[
            // DER with short-form length encoded as three bytes.
            &[EXAMPLE_TAG, 0x83, 0xFF, 0xFF, 0xFF].as_slice(),
            // DER with short-form length encoded as four bytes.
            &[EXAMPLE_TAG, 0x84, 0xFF, 0xFF, 0xFF, 0xFF].as_slice(),
        ];

        for input in inputs {
            let mut bytes = untrusted::Reader::new(untrusted::Input::from(input));
            // read_tag_and_get_value should reject DER with encoded lengths larger than two
            // bytes as BadDer.
            assert!(matches!(
                read_tag_and_get_value(&mut bytes),
                Err(Error::BadDer)
            ));
        }
    }

    #[test]
    fn read_tag_and_get_value_limited_high_form() {
        use super::{read_tag_and_get_value_limited, Error, LONG_FORM_LEN_TWO_BYTES_MAX};

        let mut bytes = untrusted::Reader::new(untrusted::Input::from(&[0xFF]));
        // read_tag_and_get_value_limited_high_form should reject DER with "high tag number form" tags.
        assert!(matches!(
            read_tag_and_get_value_limited(&mut bytes, LONG_FORM_LEN_TWO_BYTES_MAX),
            Err(Error::BadDer)
        ));
    }

    #[test]
    fn read_tag_and_get_value_limited_non_canonical() {
        use super::{read_tag_and_get_value_limited, Error, LONG_FORM_LEN_TWO_BYTES_MAX};

        let inputs = &[
            // Two byte length, with expressed length < 128.
            &[EXAMPLE_TAG, 0x81, 0x01].as_slice(),
            // Three byte length, with expressed length < 256.
            &[EXAMPLE_TAG, 0x82, 0x00, 0x01].as_slice(),
            // Four byte length, with expressed length, < 65536.
            &[EXAMPLE_TAG, 0x83, 0x00, 0x00, 0x01].as_slice(),
            // Five byte length, with expressed length < 16777216.
            &[EXAMPLE_TAG, 0x84, 0x00, 0x00, 0x00, 0x01].as_slice(),
        ];

        for input in inputs {
            let mut bytes = untrusted::Reader::new(untrusted::Input::from(input));
            // read_tag_and_get_value_limited should reject DER with non-canonical lengths.
            assert!(matches!(
                read_tag_and_get_value_limited(&mut bytes, LONG_FORM_LEN_TWO_BYTES_MAX),
                Err(Error::BadDer)
            ));
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn read_tag_and_get_value_limited_limits() {
        use super::{read_tag_and_get_value_limited, Error};

        let short_input = &[0xFF];
        let short_input_encoded = &[
            &[EXAMPLE_TAG],
            der_encode_length(short_input.len()).as_slice(),
            short_input,
        ]
        .concat();

        let long_input = &[1_u8; 65537];
        let long_input_encoded = &[
            &[EXAMPLE_TAG],
            der_encode_length(long_input.len()).as_slice(),
            long_input,
        ]
        .concat();

        struct Testcase<'a> {
            input: &'a [u8],
            limit: usize,
            err: Option<Error>,
        }

        let testcases = &[
            Testcase {
                input: short_input_encoded,
                limit: 1,
                err: Some(Error::BadDer),
            },
            Testcase {
                input: short_input_encoded,
                limit: short_input_encoded.len() + 1,
                err: None,
            },
            Testcase {
                input: long_input_encoded,
                limit: long_input.len(),
                err: Some(Error::BadDer),
            },
            Testcase {
                input: long_input_encoded,
                limit: long_input.len() + 1,
                err: None,
            },
        ];

        for tc in testcases {
            let mut bytes = untrusted::Reader::new(untrusted::Input::from(tc.input));

            let res = read_tag_and_get_value_limited(&mut bytes, tc.limit);
            match tc.err {
                None => assert!(res.is_ok()),
                Some(e) => {
                    let actual = res.unwrap_err();
                    assert_eq!(actual, e)
                }
            }
        }
    }

    #[allow(clippy::as_conversions)] // infallible.
    const EXAMPLE_TAG: u8 = super::Tag::Sequence as u8;

    #[cfg(feature = "alloc")]
    #[allow(clippy::as_conversions)] // test code.
    fn der_encode_length(length: usize) -> Vec<u8> {
        if length < 128 {
            vec![length as u8]
        } else {
            let mut encoded: Vec<u8> = Vec::new();
            let mut remaining_length = length;

            while remaining_length > 0 {
                let byte = (remaining_length & 0xFF) as u8;
                encoded.insert(0, byte);
                remaining_length >>= 8;
            }

            let length_octet = encoded.len() as u8 | 0x80;
            encoded.insert(0, length_octet);

            encoded
        }
    }

    #[allow(clippy::as_conversions)] // infallible.
    const BITSTRING_TAG: u8 = super::Tag::BitString as u8;

    #[test]
    fn misencoded_bit_string_flags() {
        use super::{bit_string_flags, Error};

        let mut bad_padding_example = untrusted::Reader::new(untrusted::Input::from(&[
            BITSTRING_TAG, // BitString
            0x2,           // 2 bytes of content.
            0x08,          // 8 bit of padding (illegal!).
            0x06,          // 1 byte of bit flags asserting bits 5 and 6.
        ]));
        assert!(matches!(
            bit_string_flags(&mut bad_padding_example),
            Err(Error::BadDer)
        ));

        let mut bad_padding_example = untrusted::Reader::new(untrusted::Input::from(&[
            BITSTRING_TAG, // BitString
            0x2,           // 2 bytes of content.
            0x01,          // 1 bit of padding.
                           // No flags value (illegal with padding!).
        ]));
        assert!(matches!(
            bit_string_flags(&mut bad_padding_example),
            Err(Error::BadDer)
        ));

        let mut trailing_zeroes = untrusted::Reader::new(untrusted::Input::from(&[
            BITSTRING_TAG, // BitString
            0x2,           // 2 bytes of content.
            0x01,          // 1 bit of padding.
            0xFF,          // Flag data with
            0x00,          // trailing zeros.
        ]));
        assert!(matches!(
            bit_string_flags(&mut trailing_zeroes),
            Err(Error::BadDer)
        ))
    }

    #[test]
    fn valid_bit_string_flags() {
        use super::bit_string_flags;

        let mut example_key_usage = untrusted::Reader::new(untrusted::Input::from(&[
            BITSTRING_TAG, // BitString
            0x2,           // 2 bytes of content.
            0x01,          // 1 bit of padding.
            0x06,          // 1 byte of bit flags asserting bits 5 and 6.
        ]));
        let res = bit_string_flags(&mut example_key_usage).unwrap();

        assert!(!res.bit_set(0));
        assert!(!res.bit_set(1));
        assert!(!res.bit_set(2));
        assert!(!res.bit_set(3));
        assert!(!res.bit_set(4));
        // NB: Bits 5 and 6 should be set.
        assert!(res.bit_set(5));
        assert!(res.bit_set(6));
        assert!(!res.bit_set(7));
        assert!(!res.bit_set(8));
        // Bits outside the range of values shouldn't be considered set.
        assert!(!res.bit_set(256));
    }
}
