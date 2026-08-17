//! Base64 encodings

use crate::{
    alphabet::Alphabet,
    errors::{Error, InvalidEncodingError, InvalidLengthError},
};
use core::str;

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

#[cfg(doc)]
use crate::{Base64, Base64Bcrypt, Base64Crypt, Base64Unpadded, Base64Url, Base64UrlUnpadded};

/// Padding character
const PAD: u8 = b'=';

/// Base64 encoding trait.
///
/// This trait must be imported to make use of any Base64 alphabet defined
/// in this crate.
///
/// The following encoding types impl this trait:
///
/// - [`Base64`]: standard Base64 encoding with `=` padding.
/// - [`Base64Bcrypt`]: bcrypt Base64 encoding.
/// - [`Base64Crypt`]: `crypt(3)` Base64 encoding.
/// - [`Base64Unpadded`]: standard Base64 encoding *without* padding.
/// - [`Base64Url`]: URL-safe Base64 encoding with `=` padding.
/// - [`Base64UrlUnpadded`]: URL-safe Base64 encoding *without* padding.
pub trait Encoding: Alphabet {
    /// Decode a Base64 string into the provided destination buffer.
    fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>;

    /// Decode a Base64 string in-place.
    ///
    /// NOTE: this method does not (yet) validate that padding is well-formed,
    /// if the given Base64 encoding is padded.
    fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>;

    /// Decode a Base64 string into a byte vector.
    #[cfg(feature = "alloc")]
    fn decode_vec(input: &str) -> Result<Vec<u8>, Error>;

    /// Encode the input byte slice as Base64.
    ///
    /// Writes the result into the provided destination slice, returning an
    /// ASCII-encoded Base64 string value.
    fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>;

    /// Encode input byte slice into a [`String`] containing Base64.
    ///
    /// # Panics
    /// If `input` length is greater than `usize::MAX/4`.
    #[cfg(feature = "alloc")]
    fn encode_string(input: &[u8]) -> String;

    /// Get the length of Base64 produced by encoding the given bytes.
    ///
    /// WARNING: this function will return `0` for lengths greater than `usize::MAX/4`!
    fn encoded_len(bytes: &[u8]) -> usize;
}

impl<T: Alphabet> Encoding for T {
    fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error> {
        let (src_unpadded, mut err) = if T::PADDED {
            let (unpadded_len, e) = decode_padding(src.as_ref())?;
            (&src.as_ref()[..unpadded_len], e)
        } else {
            (src.as_ref(), 0)
        };

        let dlen = decoded_len(src_unpadded.len());

        if dlen > dst.len() {
            return Err(Error::InvalidLength);
        }

        let dst = &mut dst[..dlen];

        let mut src_chunks = src_unpadded.chunks_exact(4);
        let mut dst_chunks = dst.chunks_exact_mut(3);
        for (s, d) in (&mut src_chunks).zip(&mut dst_chunks) {
            err |= Self::decode_3bytes(s, d);
        }
        let src_rem = src_chunks.remainder();
        let dst_rem = dst_chunks.into_remainder();

        err |= !(src_rem.is_empty() || src_rem.len() >= 2) as i16;
        let mut tmp_out = [0u8; 3];
        let mut tmp_in = [b'A'; 4];
        tmp_in[..src_rem.len()].copy_from_slice(src_rem);
        err |= Self::decode_3bytes(&tmp_in, &mut tmp_out);
        dst_rem.copy_from_slice(&tmp_out[..dst_rem.len()]);

        if err == 0 {
            validate_last_block::<T>(src.as_ref(), dst)?;
            Ok(dst)
        } else {
            Err(Error::InvalidEncoding)
        }
    }

    // TODO(tarcieri): explicitly checked/wrapped arithmetic
    #[allow(clippy::arithmetic_side_effects)]
    fn decode_in_place(mut buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError> {
        // TODO: eliminate unsafe code when LLVM12 is stable
        // See: https://github.com/rust-lang/rust/issues/80963
        let mut err = if T::PADDED {
            let (unpadded_len, e) = decode_padding(buf)?;
            buf = &mut buf[..unpadded_len];
            e
        } else {
            0
        };

        let dlen = decoded_len(buf.len());
        let full_chunks = buf.len() / 4;

        for chunk in 0..full_chunks {
            // SAFETY: `p3` and `p4` point inside `buf`, while they may overlap,
            // read and write are clearly separated from each other and done via
            // raw pointers.
            #[allow(unsafe_code)]
            unsafe {
                debug_assert!(3 * chunk + 3 <= buf.len());
                debug_assert!(4 * chunk + 4 <= buf.len());

                let p3 = buf.as_mut_ptr().add(3 * chunk) as *mut [u8; 3];
                let p4 = buf.as_ptr().add(4 * chunk) as *const [u8; 4];

                let mut tmp_out = [0u8; 3];
                err |= Self::decode_3bytes(&*p4, &mut tmp_out);
                *p3 = tmp_out;
            }
        }

        let src_rem_pos = 4 * full_chunks;
        let src_rem_len = buf.len() - src_rem_pos;
        let dst_rem_pos = 3 * full_chunks;
        let dst_rem_len = dlen - dst_rem_pos;

        err |= !(src_rem_len == 0 || src_rem_len >= 2) as i16;
        let mut tmp_in = [b'A'; 4];
        tmp_in[..src_rem_len].copy_from_slice(&buf[src_rem_pos..]);
        let mut tmp_out = [0u8; 3];

        err |= Self::decode_3bytes(&tmp_in, &mut tmp_out);

        if err == 0 {
            // SAFETY: `dst_rem_len` is always smaller than 4, so we don't
            // read outside of `tmp_out`, write and the final slicing never go
            // outside of `buf`.
            #[allow(unsafe_code)]
            unsafe {
                debug_assert!(dst_rem_pos + dst_rem_len <= buf.len());
                debug_assert!(dst_rem_len <= tmp_out.len());
                debug_assert!(dlen <= buf.len());

                core::ptr::copy_nonoverlapping(
                    tmp_out.as_ptr(),
                    buf.as_mut_ptr().add(dst_rem_pos),
                    dst_rem_len,
                );
                Ok(buf.get_unchecked(..dlen))
            }
        } else {
            Err(InvalidEncodingError)
        }
    }

    #[cfg(feature = "alloc")]
    fn decode_vec(input: &str) -> Result<Vec<u8>, Error> {
        let mut output = vec![0u8; decoded_len(input.len())];
        let len = Self::decode(input, &mut output)?.len();

        if len <= output.len() {
            output.truncate(len);
            Ok(output)
        } else {
            Err(Error::InvalidLength)
        }
    }

    fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError> {
        let elen = match encoded_len_inner(src.len(), T::PADDED) {
            Some(v) => v,
            None => return Err(InvalidLengthError),
        };

        if elen > dst.len() {
            return Err(InvalidLengthError);
        }

        let dst = &mut dst[..elen];

        let mut src_chunks = src.chunks_exact(3);
        let mut dst_chunks = dst.chunks_exact_mut(4);

        for (s, d) in (&mut src_chunks).zip(&mut dst_chunks) {
            Self::encode_3bytes(s, d);
        }

        let src_rem = src_chunks.remainder();

        if T::PADDED {
            if let Some(dst_rem) = dst_chunks.next() {
                let mut tmp = [0u8; 3];
                tmp[..src_rem.len()].copy_from_slice(src_rem);
                Self::encode_3bytes(&tmp, dst_rem);

                let flag = src_rem.len() == 1;
                let mask = (flag as u8).wrapping_sub(1);
                dst_rem[2] = (dst_rem[2] & mask) | (PAD & !mask);
                dst_rem[3] = PAD;
            }
        } else {
            let dst_rem = dst_chunks.into_remainder();

            let mut tmp_in = [0u8; 3];
            let mut tmp_out = [0u8; 4];
            tmp_in[..src_rem.len()].copy_from_slice(src_rem);
            Self::encode_3bytes(&tmp_in, &mut tmp_out);
            dst_rem.copy_from_slice(&tmp_out[..dst_rem.len()]);
        }

        debug_assert!(str::from_utf8(dst).is_ok());

        // SAFETY: values written by `encode_3bytes` are valid one-byte UTF-8 chars
        #[allow(unsafe_code)]
        Ok(unsafe { str::from_utf8_unchecked(dst) })
    }

    #[cfg(feature = "alloc")]
    fn encode_string(input: &[u8]) -> String {
        let elen = encoded_len_inner(input.len(), T::PADDED).expect("input is too big");
        let mut dst = vec![0u8; elen];
        let res = Self::encode(input, &mut dst).expect("encoding error");

        debug_assert_eq!(elen, res.len());
        debug_assert!(str::from_utf8(&dst).is_ok());

        // SAFETY: `dst` is fully written and contains only valid one-byte UTF-8 chars
        #[allow(unsafe_code)]
        unsafe {
            String::from_utf8_unchecked(dst)
        }
    }

    fn encoded_len(bytes: &[u8]) -> usize {
        encoded_len_inner(bytes.len(), T::PADDED).unwrap_or(0)
    }
}

/// Validate padding is of the expected length compute unpadded length.
///
/// Note that this method does not explicitly check that the padded data
/// is valid in and of itself: that is performed by `validate_last_block` as a
/// final step.
///
/// Returns length-related errors eagerly as a [`Result`], and data-dependent
/// errors (i.e. malformed padding bytes) as `i16` to be combined with other
/// encoding-related errors prior to branching.
#[inline(always)]
pub(crate) fn decode_padding(input: &[u8]) -> Result<(usize, i16), InvalidEncodingError> {
    if input.len() % 4 != 0 {
        return Err(InvalidEncodingError);
    }

    let unpadded_len = match *input {
        [.., b0, b1] => is_pad_ct(b0)
            .checked_add(is_pad_ct(b1))
            .and_then(|len| len.try_into().ok())
            .and_then(|len| input.len().checked_sub(len))
            .ok_or(InvalidEncodingError)?,
        _ => input.len(),
    };

    let padding_len = input
        .len()
        .checked_sub(unpadded_len)
        .ok_or(InvalidEncodingError)?;

    let err = match *input {
        [.., b0] if padding_len == 1 => is_pad_ct(b0) ^ 1,
        [.., b0, b1] if padding_len == 2 => (is_pad_ct(b0) & is_pad_ct(b1)) ^ 1,
        _ => {
            if padding_len == 0 {
                0
            } else {
                return Err(InvalidEncodingError);
            }
        }
    };

    Ok((unpadded_len, err))
}

/// Validate that the last block of the decoded data round-trips back to the
/// encoded data.
fn validate_last_block<T: Alphabet>(encoded: &[u8], decoded: &[u8]) -> Result<(), Error> {
    if encoded.is_empty() && decoded.is_empty() {
        return Ok(());
    }

    // TODO(tarcieri): explicitly checked/wrapped arithmetic
    #[allow(clippy::arithmetic_side_effects)]
    fn last_block_start(bytes: &[u8], block_size: usize) -> usize {
        (bytes.len().saturating_sub(1) / block_size) * block_size
    }

    let enc_block = encoded
        .get(last_block_start(encoded, 4)..)
        .ok_or(Error::InvalidEncoding)?;

    let dec_block = decoded
        .get(last_block_start(decoded, 3)..)
        .ok_or(Error::InvalidEncoding)?;

    // Round-trip encode the decoded block
    let mut buf = [0u8; 4];
    let block = T::encode(dec_block, &mut buf)?;

    // Non-short-circuiting comparison of padding
    // TODO(tarcieri): better constant-time mechanisms (e.g. `subtle`)?
    if block
        .as_bytes()
        .iter()
        .zip(enc_block.iter())
        .fold(0, |acc, (a, b)| acc | (a ^ b))
        == 0
    {
        Ok(())
    } else {
        Err(Error::InvalidEncoding)
    }
}

/// Get the length of the output from decoding the provided *unpadded*
/// Base64-encoded input.
///
/// Note that this function does not fully validate the Base64 is well-formed
/// and may return incorrect results for malformed Base64.
// TODO(tarcieri): explicitly checked/wrapped arithmetic
#[allow(clippy::arithmetic_side_effects)]
#[inline(always)]
pub(crate) fn decoded_len(input_len: usize) -> usize {
    // overflow-proof computation of `(3*n)/4`
    let k = input_len / 4;
    let l = input_len - 4 * k;
    3 * k + (3 * l) / 4
}

/// Branchless match that a given byte is the `PAD` character
// TODO(tarcieri): explicitly checked/wrapped arithmetic
#[allow(clippy::arithmetic_side_effects)]
#[inline(always)]
fn is_pad_ct(input: u8) -> i16 {
    ((((PAD as i16 - 1) - input as i16) & (input as i16 - (PAD as i16 + 1))) >> 8) & 1
}

// TODO(tarcieri): explicitly checked/wrapped arithmetic
#[allow(clippy::arithmetic_side_effects)]
#[inline(always)]
const fn encoded_len_inner(n: usize, padded: bool) -> Option<usize> {
    match n.checked_mul(4) {
        Some(q) => {
            if padded {
                Some(((q / 3) + 3) & !3)
            } else {
                Some((q / 3) + (q % 3 != 0) as usize)
            }
        }
        None => None,
    }
}
