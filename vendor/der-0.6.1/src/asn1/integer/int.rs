//! Support for encoding negative integers

use super::is_highest_bit_set;
use crate::{ErrorKind, Length, Result, Writer};

/// Decode an unsigned integer of the specified size.
///
/// Returns a byte array of the requested size containing a big endian integer.
pub(super) fn decode_to_array<const N: usize>(bytes: &[u8]) -> Result<[u8; N]> {
    match N.checked_sub(bytes.len()) {
        Some(offset) => {
            let mut output = [0xFFu8; N];
            output[offset..].copy_from_slice(bytes);
            Ok(output)
        }
        None => {
            let expected_len = Length::try_from(N)?;
            let actual_len = Length::try_from(bytes.len())?;

            Err(ErrorKind::Incomplete {
                expected_len,
                actual_len,
            }
            .into())
        }
    }
}

/// Encode the given big endian bytes representing an integer as ASN.1 DER.
pub(super) fn encode_bytes<W>(writer: &mut W, bytes: &[u8]) -> Result<()>
where
    W: Writer + ?Sized,
{
    writer.write(strip_leading_ones(bytes))
}

/// Get the encoded length for the given unsigned integer serialized as bytes.
#[inline]
pub(super) fn encoded_len(bytes: &[u8]) -> Result<Length> {
    Length::try_from(strip_leading_ones(bytes).len())
}

/// Strip the leading all-ones bytes from the given byte slice.
fn strip_leading_ones(mut bytes: &[u8]) -> &[u8] {
    while let Some((byte, rest)) = bytes.split_first() {
        if *byte == 0xFF && is_highest_bit_set(rest) {
            bytes = rest;
            continue;
        }

        break;
    }

    bytes
}
