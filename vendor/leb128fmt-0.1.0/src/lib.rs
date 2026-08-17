//! Leb128fmt is a library to decode and encode [LEB128][leb128] formatted numbers.
//! LEB128 is a variable length integer compression format.
//!
//! The library does not allocate memory and can be used in `no_std` and
//! `no_std::no_alloc` environments.
//!
//! Various functions are provided which encode and decode signed and unsigned
//! integers with the number of bits in the function name. There are generic
//! functions provided to read and write slices of encoded values as well.
//!
//! There are encoding functions with the word `fixed` in the name which will
//! write out a value using the maximum number of bytes for a given bit size.
//! For instance, using [`encode_fixed_u32`] will always use 5 bytes to
//! write out the value. While always using the maximum number of bytes removes
//! the benefit of compression, in some scenarios, it is beneficial to have a
//! fixed encoding size.
//!
//! Finally, there are macros provided which you can use to build your own
//! encoding and decoding functions for unusual variants like signed 33 bit
//! values.
//!
//! # Examples
//!
//! ## Functions using Arrays
//!
//! ```rust
//! // Encode an unsigned 32 bit number:
//! let (output, written_len) = leb128fmt::encode_u32(43110).unwrap();
//! // The number of bytes written in the output array
//! assert_eq!(written_len, 3);
//! assert_eq!(&output[..written_len], &[0xE6, 0xD0, 0x02]);
//! // The entire output array. Note you should only use &output[..written_len] to copy
//! // into your output buffer
//! assert_eq!(output, [0xE6, 0xD0, 0x02, 0x00, 0x00]);
//!
//! // Decode an unsigned 32 bit number:
//! let input = [0xE6, 0xD0, 0x02, 0x00, 0x00];
//! let (result, read_len) = leb128fmt::decode_u32(input).unwrap();
//! assert_eq!(result, 43110);
//! assert_eq!(read_len, 3);
//! ```
//!
//! ### Helper Functions
//!
//! If you are reading from an input buffer, you can use [`is_last`] and
//! [`max_len`] to determine the bytes to copy into the array.
//!
//! ```rust
//! let buffer = vec![0xFE, 0xFE, 0xE6, 0xD0, 0x02, 0xFE, 0xFE, 0xFE];
//! let pos = 2;
//! let end = buffer.iter().skip(pos).copied().position(leb128fmt::is_last).map(|p| pos + p);
//! if let Some(end) = end {
//!     if end <= pos + leb128fmt::max_len::<32>() {
//!         let mut input = [0u8; leb128fmt::max_len::<32>()];
//!         input[..=end - pos].copy_from_slice(&buffer[pos..=end]);
//!         let (result, read_len) = leb128fmt::decode_u32(input).unwrap();
//!         assert_eq!(result, 43110);
//!         assert_eq!(read_len, 3);
//!     } else {
//!         // invalid LEB128 encoding
//!#        panic!();
//!     }
//! } else {
//!   if buffer.len() - pos < leb128fmt::max_len::<32>() {
//!      // Need more bytes in the buffer
//!#     panic!();
//!   } else {
//!      // invalid LEB128 encoding
//!#     panic!();
//!   }
//! }
//!
//! ```
//!
//! ## Functions Using Slices
//!
//! ```rust
//! let mut buffer = vec![0xFE; 10];
//! let mut pos = 1;
//!
//! // Encode an unsigned 64 bit number with a mutable slice:
//! let result = leb128fmt::encode_uint_slice::<u64, 64>(43110u64, &mut buffer, &mut pos);
//! // The number of bytes written in the output array
//! assert_eq!(result, Some(3));
//! assert_eq!(pos, 4);
//!
//! assert_eq!(buffer, [0xFE, 0xE6, 0xD0, 0x02, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE]);
//!
//! // Decode an unsigned 64 bit number with a slice:
//! pos = 1;
//! let result = leb128fmt::decode_uint_slice::<u64, 64>(&buffer, &mut pos);
//! assert_eq!(result, Ok(43110));
//! assert_eq!(pos, 4);
//! ```
//!
//! ## Functions Using Fixed Sized Encoding
//!
//! There may be several different ways to encode a value. For instance, `0` can
//! be encoded as 32 bits unsigned:
//!
//! ```rust
//! let mut pos = 0;
//! assert_eq!(leb128fmt::decode_uint_slice::<u32, 32>(&[0x00], &mut pos), Ok(0));
//! pos = 0;
//! assert_eq!(leb128fmt::decode_uint_slice::<u32, 32>(&[0x80, 0x00], &mut pos), Ok(0));
//! pos = 0;
//! assert_eq!(leb128fmt::decode_uint_slice::<u32, 32>(&[0x80, 0x80, 0x00], &mut pos), Ok(0));
//! pos = 0;
//! assert_eq!(leb128fmt::decode_uint_slice::<u32, 32>(&[0x80, 0x80, 0x80, 0x00], &mut pos), Ok(0));
//! pos = 0;
//! assert_eq!(leb128fmt::decode_uint_slice::<u32, 32>(&[0x80, 0x80, 0x80, 0x80, 0x00], &mut pos), Ok(0));
//! ```
//!
//! There are functions provided to encode a value using the maximum number of
//! bytes possible for a given bit size. Using the maximum number of bytes
//! removes the benefit of compression, but it may be useful in a few scenarios.
//!
//! For instance, if a binary format needs to store the size or offset of some
//! data before the size of data is known, it can be beneficial to write a fixed
//! sized `0` placeholder value first. Then, once the real value is known, the
//! `0` placeholder can be overwritten without moving other bytes. The real
//! value is also written out using the fixed maximum number of bytes.
//!
//! ```rust
//! // Encode an unsigned 32 bit number with all 5 bytes:
//! let output  = leb128fmt::encode_fixed_u32(43110).unwrap();
//! assert_eq!(output, [0xE6, 0xD0, 0x82, 0x80, 0x00]);
//!
//! // Decode an unsigned 32 bit number:
//! let input = output;
//! let (result, read_len) = leb128fmt::decode_u32(input).unwrap();
//! assert_eq!(result, 43110);
//!
//! // Note that all 5 bytes are read
//! assert_eq!(read_len, 5);
//! ```
//!
//! [leb128]: https://en.wikipedia.org/wiki/LEB128

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

use core::fmt;

/// Returns the maximum byte length that is used to encode a value for a given
/// number of `BITS`.
///
/// A value can possibly be encoded with a fewer number of bytes.
///
/// # Example
///
/// ```rust
/// assert_eq!(5, leb128fmt::max_len::<32>());
/// assert_eq!(10, leb128fmt::max_len::<64>());
///
/// assert_eq!(5, leb128fmt::max_len::<33>());
/// ```
#[inline]
#[must_use]
pub const fn max_len<const BITS: u32>() -> usize {
    let rem = if BITS % 7 == 0 { 0 } else { 1 };
    ((BITS / 7) + rem) as usize
}

/// Returns true if this is the last byte in an encoded LEB128 value.
///
/// # Example
///
/// ```rust
/// let bytes = &[0x42, 0x8F, 0xFF, 0x7F, 0xFF];
/// let pos = 1;
/// let end = bytes.iter().skip(pos).copied().position(leb128fmt::is_last);
/// let end = end.unwrap();
/// assert_eq!(pos + end, 3);
/// let value = &bytes[pos..=pos + end];
/// ```
#[inline]
#[must_use]
pub const fn is_last(byte: u8) -> bool {
    byte & 0x80 == 0
}

/// Builds custom unsigned integer encode functions.
///
/// The macro's 3 parameters are:
///
/// 1. The name of the function.
/// 2. The type to return.
/// 3. The number of encoded BITS to decode.
///
/// ```rust
/// leb128fmt::encode_uint_arr!(encode_u33, u64, 33);
///
/// let result = encode_u33(0);
/// assert_eq!(Some(([0x00, 0x00, 0x00, 0x00, 0x00], 1)), result);
///
/// let result = encode_u33(8589934591);
/// assert_eq!(Some(([0xFF, 0xFF, 0xFF, 0xFF, 0x1F], 5)), result);
/// ```
#[macro_export]
macro_rules! encode_uint_arr {
    ($func:ident, $num_ty:ty, $bits:literal) => {
        /// Encodes a value as an unsigned LEB128 number.
        ///
        /// If the value can be encoded in the given number of bits, then return
        /// the encoded output and the index after the last byte written.
        ///
        /// If the value cannot be encoded with the given number of bits, then return None.
        #[must_use]
        pub const fn $func(
            mut value: $num_ty,
        ) -> Option<(
            [u8; (($bits / 7) + if $bits % 7 == 0 { 0 } else { 1 }) as usize],
            usize,
        )> {
            const BITS: u32 = $bits;
            if <$num_ty>::BITS > BITS && 1 < value >> BITS - 1 {
                return None;
            }

            let mut output = [0; (($bits / 7) + if $bits % 7 == 0 { 0 } else { 1 }) as usize];
            let mut index = 0;
            loop {
                let mut b = (value & 0x7f) as u8;

                value >>= 7;
                let done = value == 0;

                if !done {
                    b |= 0x80;
                }

                output[index] = b;
                index += 1;

                if done {
                    return Some((output, index));
                }
            }
        }
    };
}

encode_uint_arr!(encode_u32, u32, 32);
encode_uint_arr!(encode_u64, u64, 64);

/// Builds custom unsigned integer encode functions with the max byte length of
/// byte arrays used.
///
/// The macro's 3 parameters are:
///
/// 1. The name of the function.
/// 2. The type to return.
/// 3. The number of encoded BITS to decode.
///
/// ```rust
/// leb128fmt::encode_fixed_uint_arr!(encode_fixed_u33, u64, 33);
///
/// let output = encode_fixed_u33(0);
/// assert_eq!(Some([0x80, 0x80, 0x80, 0x80, 0x00]), output);
///
/// let output = encode_fixed_u33(8589934591);
/// assert_eq!(Some([0xFF, 0xFF, 0xFF, 0xFF, 0x1F]), output);
/// ```
#[macro_export]
macro_rules! encode_fixed_uint_arr {
    ($func:ident, $num_ty:ty, $bits:literal) => {
        /// Encodes an unsigned LEB128 number with using the maximum number of
        /// bytes for the given bits length.
        ///
        /// If the value can be encoded in the given number of bits, then return
        /// the encoded value.
        ///
        /// If the value cannot be encoded with the given number of bits, then return None.
        #[must_use]
        pub const fn $func(
            value: $num_ty,
        ) -> Option<[u8; (($bits / 7) + if $bits % 7 == 0 { 0 } else { 1 }) as usize]> {
            const BITS: u32 = $bits;
            if <$num_ty>::BITS > BITS && 1 < value >> BITS - 1 {
                return None;
            }

            let mut output = [0; (($bits / 7) + if $bits % 7 == 0 { 0 } else { 1 }) as usize];

            let mut index = 0;
            let mut shift: u32 = 0;
            loop {
                let v = value >> shift;

                let mut b = (v & 0x7f) as u8;

                let done = shift == BITS - (BITS % 7);

                if !done {
                    b |= 0x80;
                }

                output[index] = b;
                index += 1;
                shift += 7;

                if done {
                    return Some(output);
                }
            }
        }
    };
}

encode_fixed_uint_arr!(encode_fixed_u32, u32, 32);
encode_fixed_uint_arr!(encode_fixed_u64, u64, 64);

/// Builds custom unsigned integer decode functions.
///
/// The macro's 3 parameters are:
///
/// 1. The name of the function.
/// 2. The type to return.
/// 3. The number of encoded BITS to decode.
///
/// ```rust
/// leb128fmt::decode_uint_arr!(decode_u33, u64, 33);
///
/// let input = [0xFF, 0xFF, 0xFF, 0xFF, 0x1F];
/// let result = decode_u33(input);
/// assert_eq!(Some((8589934591, 5)), result);
/// ```
#[macro_export]
macro_rules! decode_uint_arr {
    ($func:ident, $num_ty:ty, $bits:literal) => {
        /// Decodes an unsigned LEB128 number.
        ///
        /// If there is a valid encoded value, returns the decoded value and the
        /// index after the last byte read.
        ///
        /// If the encoding is incorrect, returns `None`.
        ///
        /// If the size in bits of the returned type is less than the size of the value in bits, returns `None`.
        /// For instance, if 33 bits are being decoded, then the returned type must be at least a `u64`.
        #[must_use]
        pub const fn $func(
            input: [u8; (($bits / 7) + if $bits % 7 == 0 { 0 } else { 1 }) as usize],
        ) -> Option<($num_ty, usize)> {
            const BITS: u32 = $bits;
            if <$num_ty>::BITS < BITS {
                return None;
            }

            let n = input[0];
            if n & 0x80 == 0 {
                return Some((n as $num_ty, 1));
            }

            let mut result = (n & 0x7f) as $num_ty;
            let mut shift = 7;
            let mut pos = 1;
            loop {
                let n = input[pos];

                // If unnecessary bits are set (the bits would be dropped when
                // the value is shifted), then return an error.
                //
                // This error may be too strict.
                //
                // There should be at least a simple check to quickly
                // determine that the decoding has failed instead of
                // misinterpreting further data.
                //
                // For a less strict check, the && condition could be:
                //
                // (n & 0x80) != 0
                //
                // Another stricter condition is if the last byte has a 0 value.
                // The encoding is correct but not the minimal number of bytes
                // was used to express the final value.
                if shift == BITS - (BITS % 7) && 1 << (BITS % 7) <= n {
                    return None;
                }

                if n & 0x80 == 0 {
                    result |= (n as $num_ty) << shift;
                    return Some((result, pos + 1));
                }

                result |= ((n & 0x7f) as $num_ty) << shift;
                shift += 7;
                pos += 1;
            }
        }
    };
}

decode_uint_arr!(decode_u32, u32, 32);
decode_uint_arr!(decode_u64, u64, 64);

mod private {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for u128 {}

    impl Sealed for i8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for i128 {}
}

/// Sealed trait for supported unsigned integer types.
pub trait UInt: private::Sealed {
    /// Size of the type in bits.
    const BITS: u32;
}

impl UInt for u8 {
    const BITS: u32 = u8::BITS;
}

impl UInt for u16 {
    const BITS: u32 = u16::BITS;
}

impl UInt for u32 {
    const BITS: u32 = u32::BITS;
}

impl UInt for u64 {
    const BITS: u32 = u64::BITS;
}

impl UInt for u128 {
    const BITS: u32 = u128::BITS;
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InnerError {
    NeedMoreBytes,
    InvalidEncoding,
}

/// Error when decoding a LEB128 value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(InnerError);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            InnerError::NeedMoreBytes => f.write_str("need more bytes"),
            InnerError::InvalidEncoding => f.write_str("invalid encoding"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl Error {
    /// If more bytes are needed in the slice to decode the value
    #[inline]
    #[must_use]
    pub const fn is_more_bytes_needed(&self) -> bool {
        matches!(self.0, InnerError::NeedMoreBytes)
    }

    /// If the value has an invalid encoding
    #[inline]
    #[must_use]
    pub const fn is_invalid_encoding(&self) -> bool {
        matches!(self.0, InnerError::InvalidEncoding)
    }
}

/// Encodes a given value into an output slice using the fixed set of bytes.
///
/// # Examples
///
/// ```rust
/// let mut buffer = vec![254; 10];
/// let mut pos = 0;
/// let result = leb128fmt::encode_uint_slice::<_, 32>(0u32, &mut buffer, &mut pos);
/// assert_eq!(Some(1), result);
/// assert_eq!(1, pos);
/// assert_eq!(&[0x00], &buffer[..pos]);
///
/// assert_eq!(&[0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE], buffer.as_slice());
///
/// let result = leb128fmt::encode_uint_slice::<_, 32>(u32::MAX, &mut buffer, &mut pos);
/// assert_eq!(Some(5), result);
/// assert_eq!(6, pos);
/// assert_eq!(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F], &buffer[1..pos]);
///
/// assert_eq!(&[0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x0F, 0xFE, 0xFE, 0xFE, 0xFE], buffer.as_slice());
///
/// // Will try to encode even if the output slice is not as big as the maximum
/// // number of bytes required to output every value for the given BITS
/// let mut buffer = vec![254; 4];
/// let mut pos = 0;
/// let result = leb128fmt::encode_uint_slice::<_, 32>(1028u32, &mut buffer, &mut pos);
/// assert_eq!(Some(2), result);
/// assert_eq!(&[0x84, 0x08, 0xFE, 0xFE], buffer.as_slice());
///
/// // Will return `None` if the output buffer is not long enough but will have partially written
/// // the value
/// let mut buffer = vec![254; 4];
/// let mut pos = 0;
/// let result = leb128fmt::encode_uint_slice::<_, 32>(u32::MAX, &mut buffer, &mut pos);
/// assert_eq!(None, result);
/// assert_eq!(&[0xFF, 0xFF, 0xFF, 0xFF], buffer.as_slice());
///
/// // Will return `None` if the given value cannot be encoded with the given number of bits.
/// let mut buffer = vec![254; 10];
/// let mut pos = 0;
/// let result = leb128fmt::encode_uint_slice::<_, 32>(u64::MAX, &mut buffer, &mut pos);
/// assert_eq!(None, result);
/// assert_eq!(&[0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE], buffer.as_slice());
/// ```
#[allow(clippy::manual_let_else)]
pub fn encode_uint_slice<T, const BITS: u32>(
    mut value: T,
    output: &mut [u8],
    pos: &mut usize,
) -> Option<usize>
where
    T: Copy
        + PartialEq
        + core::ops::BitAnd
        + core::ops::Shr<u32>
        + core::ops::ShrAssign<u32>
        + From<u8>
        + UInt,
    <T as core::ops::Shr<u32>>::Output: PartialEq<T>,
    u8: TryFrom<<T as core::ops::BitAnd<T>>::Output>,
{
    if BITS < T::BITS && value >> BITS != T::from(0) {
        return None;
    }

    let mut index = *pos;
    loop {
        if output.len() <= index {
            return None;
        }

        let mut b = match u8::try_from(value & T::from(0x7f)) {
            Ok(b) => b,
            Err(_) => unreachable!(),
        };

        value >>= 7;

        let done = value == T::from(0);

        if !done {
            b |= 0x80;
        }

        output[index] = b;
        index += 1;

        if done {
            let len = index - *pos;
            *pos = index;
            return Some(len);
        }
    }
}

/// Encodes a given value into an output slice using a fixed set of bytes.
///
/// # Examples
///
/// ```rust
/// let mut buffer = vec![254; 10];
/// let mut pos = 0;
/// let result = leb128fmt::encode_fixed_uint_slice::<_, 32>(0u32, &mut buffer, &mut pos);
/// assert_eq!(Some(5), result);
/// assert_eq!(5, pos);
/// assert_eq!(&[0x80, 0x80, 0x80, 0x80, 0x00], &buffer[..pos]);
///
/// assert_eq!(&[0x80, 0x80, 0x80, 0x80, 0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE], buffer.as_slice());
///
/// let result = leb128fmt::encode_fixed_uint_slice::<_, 32>(u32::MAX, &mut buffer, &mut pos);
/// assert_eq!(Some(5), result);
/// assert_eq!(10, pos);
/// assert_eq!(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F], &buffer[5..pos]);
///
/// assert_eq!(&[0x80, 0x80, 0x80, 0x80, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x0F], buffer.as_slice());
///
/// // Will return `None` if the output buffer is not long enough.
/// let mut buffer = vec![254; 4];
/// let mut pos = 0;
/// let result = leb128fmt::encode_fixed_uint_slice::<_, 32>(u32::MAX, &mut buffer, &mut pos);
/// assert_eq!(None, result);
/// assert_eq!(&[0xFE, 0xFE, 0xFE, 0xFE], buffer.as_slice());
///
/// // Will return `None` if the given value cannot be encoded with the given number of bits.
/// let mut buffer = vec![254; 10];
/// let mut pos = 0;
/// let result = leb128fmt::encode_fixed_uint_slice::<_, 32>(u64::MAX, &mut buffer, &mut pos);
/// assert_eq!(None, result);
/// assert_eq!(&[0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE], buffer.as_slice());
/// ```
#[allow(clippy::manual_let_else)]
pub fn encode_fixed_uint_slice<T, const BITS: u32>(
    mut value: T,
    output: &mut [u8],
    pos: &mut usize,
) -> Option<usize>
where
    T: Copy + core::ops::BitAnd + core::ops::Shr<u32> + core::ops::ShrAssign<u32> + From<u8> + UInt,
    <T as core::ops::Shr<u32>>::Output: PartialEq<T>,
    u8: TryFrom<<T as core::ops::BitAnd>::Output>,
{
    if BITS < T::BITS && value >> BITS != T::from(0) {
        return None;
    }

    if output[*pos..].len() < max_len::<BITS>() {
        return None;
    }

    let mut index = *pos;
    for _ in 0..(max_len::<BITS>() - 1) {
        let mut b = match u8::try_from(value & T::from(0x7f)) {
            Ok(b) => b,
            Err(_) => unreachable!(),
        };

        b |= 0x80;

        value >>= 7;

        output[index] = b;
        index += 1;
    }

    let b = match u8::try_from(value & T::from(0x7f)) {
        Ok(b) => b,
        Err(_) => unreachable!(),
    };
    output[index] = b;
    index += 1;

    let len = index - *pos;
    *pos = index;
    Some(len)
}

/// Decodes an unsigned integer from a slice of bytes and starting at a given position.
///
/// # Errors
///
/// Returns an error if the value is not properly encoded or if more bytes are
/// needed to decode the value.
///
/// # Panics
///
/// Panics if the size in bits of the returned type is less than the size of the value in bits.
/// For instance, if 33 bits are being decoded, then the returned type must be at least a `u64`.
///
/// ```rust
/// let input = [0x42, 0x8F, 0xFF, 0x7F, 0xFF];
/// let mut pos = 1;
/// let result = leb128fmt::decode_uint_slice::<u32, 32>(&input, &mut pos);
/// assert_eq!(result, Ok(2097039));
/// assert_eq!(pos, 4);
/// ```
pub fn decode_uint_slice<T, const BITS: u32>(input: &[u8], pos: &mut usize) -> Result<T, Error>
where
    T: core::ops::Shl<u32, Output = T> + core::ops::BitOrAssign + From<u8> + UInt,
{
    assert!(BITS <= T::BITS);
    if input.len() <= *pos {
        return Err(Error(InnerError::NeedMoreBytes));
    }

    let n = input[*pos];
    if is_last(n) {
        *pos += 1;
        return Ok(T::from(n));
    }

    let mut result = T::from(n & 0x7f);
    let mut shift: u32 = 7;

    let mut idx = *pos + 1;
    loop {
        if input.len() <= idx {
            return Err(Error(InnerError::NeedMoreBytes));
        }

        let n = input[idx];

        // If unnecessary bits are set (the bits would be dropped when
        // the value is shifted), then return an error.
        //
        // This error may be too strict.
        //
        // There should be at least a simple check to quickly
        // determine that the decoding has failed instead of
        // misinterpreting further data.
        //
        // For a less strict check, the && condition could be:
        //
        // (n & 0x80) != 0
        //
        // Another stricter condition is if the last byte has a 0 value.
        // The encoding is correct but not the minimal number of bytes
        // was used to express the final value.
        if shift == BITS - (BITS % 7) && 1 << (BITS % 7) <= n {
            return Err(Error(InnerError::InvalidEncoding));
        }

        if is_last(n) {
            result |= T::from(n) << shift;
            *pos = idx + 1;
            return Ok(result);
        }

        result |= T::from(n & 0x7f) << shift;
        shift += 7;
        idx += 1;
    }
}

/// Builds custom signed integer encode functions.
///
/// The macro's 3 parameters are:
///
/// 1. The name of the function.
/// 2. The type to return.
/// 3. The number of encoded BITS to decode.
///
/// ```rust
/// leb128fmt::encode_sint_arr!(encode_s33, i64, 33);
///
/// let result = encode_s33(0);
/// assert_eq!(Some(([0x00, 0x00, 0x00, 0x00, 0x00], 1)), result);
///
/// let result = encode_s33(4_294_967_295);
/// assert_eq!(Some(([0xFF, 0xFF, 0xFF, 0xFF, 0x0F], 5)), result);
///
/// let result = encode_s33(-4_294_967_296);
/// assert_eq!(Some(([0x80, 0x80, 0x80, 0x80, 0x70], 5)), result);
///
/// let result = encode_s33(-1);
/// assert_eq!(Some(([0x7F, 0x00, 0x00, 0x00, 0x00], 1)), result);
/// ```
#[macro_export]
macro_rules! encode_sint_arr {
    ($func:ident, $num_ty:ty, $bits:literal) => {
        /// Encodes a value as a signed LEB128 number.
        #[must_use]
        pub fn $func(
            mut value: $num_ty,
        ) -> Option<(
            [u8; (($bits / 7) + if $bits % 7 == 0 { 0 } else { 1 }) as usize],
            usize,
        )> {
            const BITS: u32 = $bits;
            if BITS < <$num_ty>::BITS {
                let v: $num_ty = value >> BITS - 1;
                if v != 0 && v != -1 {
                    return None;
                }
            }

            let mut output = [0; (($bits / 7) + if $bits % 7 == 0 { 0 } else { 1 }) as usize];
            let mut index = 0;
            loop {
                let b = (value & 0x7f) as u8;

                value >>= 7;

                if (value == 0 && b & 0x40 == 0) || (value == -1 && (b & 0x40) != 0) {
                    output[index] = b;
                    return Some((output, index + 1));
                }

                output[index] = b | 0x80;
                index += 1;
            }
        }
    };
}

encode_sint_arr!(encode_s32, i32, 32);
encode_sint_arr!(encode_s64, i64, 64);

/// Builds custom signed integer encode functions with the max byte length of
/// byte arrays used.
///
/// The macro's 3 parameters are:
///
/// 1. The name of the function.
/// 2. The type to return.
/// 3. The number of encoded BITS to decode.
///
/// ```rust
/// leb128fmt::encode_fixed_sint_arr!(encode_fixed_s33, i64, 33);
///
/// let result = encode_fixed_s33(0);
/// assert_eq!(Some([0x80, 0x80, 0x80, 0x80, 0x00]), result);
///
/// let result = encode_fixed_s33(4_294_967_295);
/// assert_eq!(Some([0xFF, 0xFF, 0xFF, 0xFF, 0x0F]), result);
///
/// let result = encode_fixed_s33(-4_294_967_296);
/// assert_eq!(Some([0x80, 0x80, 0x80, 0x80, 0x70]), result);
///
/// let result = encode_fixed_s33(-1);
/// assert_eq!(Some([0xFF, 0xFF, 0xFF, 0xFF, 0x7F]), result);
/// ```
#[macro_export]
macro_rules! encode_fixed_sint_arr {
    ($func:ident, $num_ty:ty, $bits:literal) => {
        /// Encodes a value as a signed LEB128 number.
        #[must_use]
        pub const fn $func(
            mut value: $num_ty,
        ) -> Option<[u8; (($bits / 7) + if $bits % 7 == 0 { 0 } else { 1 }) as usize]> {
            const BITS: u32 = $bits;
            if BITS < <$num_ty>::BITS {
                let v = value >> BITS - 1;
                if v != 0 && v != -1 {
                    return None;
                }
            }

            let mut output = [0; (($bits / 7) + if $bits % 7 == 0 { 0 } else { 1 }) as usize];
            let mut index = 0;
            let mut extend_negative = false;
            loop {
                let b = (value & 0x7f) as u8;

                value >>= 7;

                output[index] = b | 0x80;
                index += 1;

                if value == 0 && b & 0x40 == 0 {
                    break;
                }
                if value == -1 && (b & 0x40) != 0 {
                    extend_negative = true;
                    break;
                }
            }

            loop {
                if index == output.len() {
                    output[index - 1] &= 0x7F;
                    return Some(output);
                }

                if extend_negative {
                    output[index] = 0xFF;
                } else {
                    output[index] = 0x80;
                }

                index += 1;
            }
        }
    };
}

encode_fixed_sint_arr!(encode_fixed_s32, i32, 32);
encode_fixed_sint_arr!(encode_fixed_s64, i64, 64);

/// Builds custom signed integer decode functions.
///
/// The macro's 3 parameters are:
///
/// 1. The name of the function.
/// 2. The type to return.
/// 3. The number of encoded BITS to decode.
///
/// ```rust
/// leb128fmt::decode_sint_arr!(decode_s33, i64, 33);
///
/// let input = [0xFF, 0xFF, 0xFF, 0xFF, 0x0F];
/// let result = decode_s33(input);
/// assert_eq!(Some((4_294_967_295, 5)), result);
///
/// let input = [0x7F, 0x00, 0x00, 0x00, 0x00];
/// let result = decode_s33(input);
/// assert_eq!(Some((-1, 1)), result);
///
/// let input = [0xFF, 0xFF, 0xFF, 0xFF, 0x7F];
/// let result = decode_s33(input);
/// assert_eq!(Some((-1, 5)), result);
///
/// let input = [0xFF, 0xFF, 0xFF, 0xFF, 0x1F];
/// let result = decode_s33(input);
/// assert_eq!(None, result);
/// ```
#[macro_export]
macro_rules! decode_sint_arr {
    ($func:ident, $num_ty:ty, $bits:literal) => {
        /// Decodes an unsigned LEB128 number.
        ///
        /// If there is a valid encoded value, returns the decoded value and the
        /// index after the last byte read.
        ///
        /// If the encoding is incorrect, returns `None`.
        ///
        /// If the size in bits of the returned type is less than the size of the value in bits, returns `None`.
        /// For instance, if 33 bits are being decoded, then the returned type must be at least a `u64`.
        #[must_use]
        pub const fn $func(
            input: [u8; (($bits / 7) + if $bits % 7 == 0 { 0 } else { 1 }) as usize],
        ) -> Option<($num_ty, usize)> {
            const BITS: u32 = $bits;
            if <$num_ty>::BITS < BITS {
                return None;
            }

            let mut result = 0;
            let mut shift = 0;
            let mut n;
            let mut pos = 0;

            loop {
                n = input[pos];
                let more = n & 0x80 != 0;

                // For the last valid shift, perform some checks to ensure the
                // encoding is valid.
                //
                // Notably, the one bit that MUST NOT be set is the high order bit
                // indicating there are more bytes to decode.
                //
                // For a signed integer, depending on if the value is positive or negative,
                // some bits SHOULD or SHOULD NOT be set.
                //
                // The expectation is that if this is a negative number, then
                // there should have been a sign extension so that all the bits
                // greater than the highest order bit is a 1.
                //
                // 32-bit
                // ------
                //
                // The maximum shift value is 28 meaning a 32-bit number is
                // encoded in a maximum of 5 bytes. If the shift value is 35 or
                // greater, then, the byte's value will be shifted out beyond the
                // 32-bit value.
                //
                // With 28 being the highest valid shift value, the highest
                // order relevant bit in the final byte should be 0x08 or:
                //
                // 0000 1000
                //
                // Any higher bit is "lost" during the bitshift.
                //
                // Due to the encoding rules and two's complement, if the
                // highest order relevant bit is set, then the number is
                // negative and the `1` is extended to the higher bits like:
                //
                // 0111 1000
                //
                // Note that the highest order bit (the first bit from left to right)
                // MUST BE a 0. It is the bit which indicates more bytes should
                // be processed. For the maximum final byte (byte #5 for a
                // 32-bit number)), it MUST be 0. There are no additional bytes
                // to decode.
                //
                // If the highest order relevant bit is not set, then the
                // integer is positive. Any of the lower bits can be set.
                //
                // 0000 0111
                //
                // So the conditions to check are:
                //
                // 1. The highest order bit is not set (so there are no more
                //    bytes to decode). If it is set, the encoding is invalid.
                //    This is the "more" check.
                //
                // 2. Determine if any sign extended negative bit is set.
                //    So is any bit in:
                //
                //    0111 1000
                //
                //    set. If none of the bits are set, then the number is
                //    positive, and the encoding is valid.
                //    This is the "(n & mask != 0)" check.
                // 3. If any sign extended negative bits are set, the number is
                //    negative, and ALL of the bits MUST be set for a valid negative number.
                //    This is the "(n < mask)"" check.
                //    An equivalent check would be that "(n < mask) || (n >= 0x80)"
                //    But the earlier check for "more" removes the need for the additional check.
                //
                //    The check could also be "(n & mask) != mask".
                //
                // Another stricter condition is if the last byte has a 0 value.
                // The encoding is correct but not the minimal number of bytes
                // was used to express the final value.
                if shift == BITS - (BITS % 7) {
                    #[allow(clippy::cast_sign_loss)]
                    let mask = ((-1i8 << ((BITS % 7).saturating_sub(1))) & 0x7f) as u8;
                    if more || (n & mask != 0 && n < mask) {
                        return None;
                    }
                }

                result |= ((n & 0x7f) as $num_ty) << shift;
                shift += 7;
                pos += 1;

                if !more {
                    break;
                }
            }

            if shift < <$num_ty>::BITS && n & 0x40 != 0 {
                result |= -1 << shift;
            }

            Some((result, pos))
        }
    };
}

decode_sint_arr!(decode_s32, i32, 32);
decode_sint_arr!(decode_s64, i64, 64);

/// Sealed trait for supported signed integer types.
pub trait SInt: private::Sealed {
    /// Size of the type in bits.
    const BITS: u32;
}

impl SInt for i8 {
    const BITS: u32 = i8::BITS;
}

impl SInt for i16 {
    const BITS: u32 = i16::BITS;
}

impl SInt for i32 {
    const BITS: u32 = i32::BITS;
}

impl SInt for i64 {
    const BITS: u32 = i64::BITS;
}

impl SInt for i128 {
    const BITS: u32 = i128::BITS;
}

/// Encodes a given value into an output slice using the fixed set of bytes.
///
/// # Examples
///
/// ```rust
/// let mut buffer = vec![254; 10];
/// let mut pos = 0;
/// let result = leb128fmt::encode_sint_slice::<_, 32>(0i32, &mut buffer, &mut pos);
/// assert_eq!(Some(1), result);
/// assert_eq!(1, pos);
/// assert_eq!(&[0x00], &buffer[..pos]);
///
/// assert_eq!(&[0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE], buffer.as_slice());
///
/// let result = leb128fmt::encode_sint_slice::<_, 32>(i32::MAX, &mut buffer, &mut pos);
/// assert_eq!(Some(5), result);
/// assert_eq!(6, pos);
/// assert_eq!(&[0xFF, 0xFF, 0xFF, 0xFF, 0x07], &buffer[1..pos]);
///
/// assert_eq!(&[0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x07, 0xFE, 0xFE, 0xFE, 0xFE], buffer.as_slice());
///
/// // Will try to encode even if the output slice is not as big as the maximum
/// // number of bytes required to output every value for the given BITS
/// let mut buffer = vec![254; 4];
/// let mut pos = 0;
/// let result = leb128fmt::encode_sint_slice::<_, 32>(1028i32, &mut buffer, &mut pos);
/// assert_eq!(Some(2), result);
/// assert_eq!(&[0x84, 0x08, 0xFE, 0xFE], buffer.as_slice());
///
/// // Will return `None` if the output buffer is not long enough but will have partially written
/// // the value
/// let mut buffer = vec![254; 4];
/// let mut pos = 0;
/// let result = leb128fmt::encode_sint_slice::<_, 32>(i32::MAX, &mut buffer, &mut pos);
/// assert_eq!(None, result);
/// assert_eq!(&[0xFF, 0xFF, 0xFF, 0xFF], buffer.as_slice());
///
/// // Will return `None` if the given value cannot be encoded with the given number of bits.
/// let mut buffer = vec![254; 10];
/// let mut pos = 0;
/// let result = leb128fmt::encode_sint_slice::<_, 32>(i64::MAX, &mut buffer, &mut pos);
/// assert_eq!(None, result);
/// assert_eq!(&[0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE], buffer.as_slice());
/// ```
#[allow(clippy::manual_let_else)]
pub fn encode_sint_slice<T, const BITS: u32>(
    mut value: T,
    output: &mut [u8],
    pos: &mut usize,
) -> Option<usize>
where
    T: Copy
        + PartialEq
        + core::ops::BitAnd
        + core::ops::Shr<u32>
        + core::ops::ShrAssign<u32>
        + From<i8>
        + SInt,
    <T as core::ops::Shr<u32>>::Output: PartialEq<T>,
    u8: TryFrom<<T as core::ops::BitAnd<T>>::Output>,
{
    if BITS < T::BITS {
        let v = value >> BITS;
        if v != T::from(0) && v != T::from(-1) {
            return None;
        }
    }

    let mut index = *pos;
    loop {
        if output.len() <= index {
            return None;
        }

        let b = match u8::try_from(value & T::from(0x7f)) {
            Ok(b) => b,
            Err(_) => unreachable!(),
        };

        value >>= 7;

        if (value == T::from(0) && b & 0x40 == 0) || (value == T::from(-1) && (b & 0x40) != 0) {
            output[index] = b;
            index += 1;
            let len = index - *pos;
            *pos = index;
            return Some(len);
        }

        output[index] = b | 0x80;
        index += 1;
    }
}

/// Encodes a given value into an output slice using a fixed set of bytes.
///
/// # Examples
///
/// ```rust
/// let mut buffer = vec![254; 10];
/// let mut pos = 0;
/// let result = leb128fmt::encode_fixed_sint_slice::<_, 32>(0i32, &mut buffer, &mut pos);
/// assert_eq!(Some(5), result);
/// assert_eq!(5, pos);
/// assert_eq!(&[0x80, 0x80, 0x80, 0x80, 0x00], &buffer[..pos]);
///
/// assert_eq!(&[0x80, 0x80, 0x80, 0x80, 0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE], buffer.as_slice());
///
/// let result = leb128fmt::encode_fixed_sint_slice::<_, 32>(i32::MAX, &mut buffer, &mut pos);
/// assert_eq!(Some(5), result);
/// assert_eq!(10, pos);
/// assert_eq!(&[0xFF, 0xFF, 0xFF, 0xFF, 0x07], &buffer[5..pos]);
///
/// assert_eq!(&[0x80, 0x80, 0x80, 0x80, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x07], buffer.as_slice());
///
/// // Will return `None` if the output buffer is not long enough.
/// let mut buffer = vec![254; 4];
/// let mut pos = 0;
/// let result = leb128fmt::encode_fixed_sint_slice::<_, 32>(i32::MAX, &mut buffer, &mut pos);
/// assert_eq!(None, result);
/// assert_eq!(&[0xFE, 0xFE, 0xFE, 0xFE], buffer.as_slice());
///
/// // Will return `None` if the given value cannot be encoded with the given number of bits.
/// let mut buffer = vec![254; 10];
/// let mut pos = 0;
/// let result = leb128fmt::encode_fixed_sint_slice::<_, 32>(i64::MAX, &mut buffer, &mut pos);
/// assert_eq!(None, result);
/// assert_eq!(&[0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE], buffer.as_slice());
/// ```
#[allow(clippy::manual_let_else)]
pub fn encode_fixed_sint_slice<T, const BITS: u32>(
    mut value: T,
    output: &mut [u8],
    pos: &mut usize,
) -> Option<usize>
where
    T: Copy
        + PartialEq
        + core::ops::BitAnd
        + core::ops::Shr<u32>
        + core::ops::ShrAssign<u32>
        + From<i8>
        + SInt,
    <T as core::ops::Shr<u32>>::Output: PartialEq<T>,
    u8: TryFrom<<T as core::ops::BitAnd>::Output>,
{
    if BITS < T::BITS {
        let v = value >> BITS;
        if v != T::from(0) && v != T::from(-1) {
            return None;
        }
    }

    if output[*pos..].len() < max_len::<BITS>() {
        return None;
    }

    let mut index = *pos;
    let mut extend_negative = false;
    loop {
        let b = match u8::try_from(value & T::from(0x7f)) {
            Ok(b) => b,
            Err(_) => unreachable!(),
        };

        value >>= 7;

        output[index] = b | 0x80;
        index += 1;

        if value == T::from(0) && b & 0x40 == 0 {
            break;
        }
        if value == T::from(-1) && (b & 0x40) != 0 {
            extend_negative = true;
            break;
        }
    }

    loop {
        if index == *pos + max_len::<BITS>() {
            output[index - 1] &= 0x7F;
            let len = index - *pos;
            *pos = index;
            return Some(len);
        }

        if extend_negative {
            output[index] = 0xFF;
        } else {
            output[index] = 0x80;
        }

        index += 1;
    }
}

/// Decodes an unsigned integer from a slice of bytes and starting at a given position.
///
/// # Errors
///
/// Returns an error if the value is not properly encoded or if more bytes are
/// needed to decode the value.
///
/// # Panics
///
/// Panics if the size in bits of the returned type is less than the size of the value in bits.
/// For instance, if 33 bits are being decoded, then the returned type must be at least a `u64`.
///
/// ```rust
/// let input = [0x42, 0x8F, 0xFF, 0x7F, 0xFF];
/// let mut pos = 1;
/// let result = leb128fmt::decode_sint_slice::<i32, 32>(&input, &mut pos);
/// assert_eq!(result, Ok(-113));
/// assert_eq!(pos, 4);
/// ```
pub fn decode_sint_slice<T, const BITS: u32>(input: &[u8], pos: &mut usize) -> Result<T, Error>
where
    T: core::ops::Shl<u32, Output = T> + core::ops::BitOrAssign + From<i8> + From<u8> + SInt,
{
    assert!(BITS <= T::BITS);

    let mut result = T::from(0i8);
    let mut shift = 0;
    let mut n;

    let mut idx = *pos;
    loop {
        if input.len() <= idx {
            return Err(Error(InnerError::NeedMoreBytes));
        }

        n = input[idx];
        let more = n & 0x80 != 0;

        // For the last valid shift, perform some checks to ensure the
        // encoding is valid.
        //
        // Notably, the one bit that MUST NOT be set is the high order bit
        // indicating there are more bytes to decode.
        //
        // For a signed integer, depending on if the value is positive or negative,
        // some bits SHOULD or SHOULD NOT be set.
        //
        // The expectation is that if this is a negative number, then
        // there should have been a sign extension so that all the bits
        // greater than the highest order bit is a 1.
        //
        // 32-bit
        // ------
        //
        // The maximum shift value is 28 meaning a 32-bit number is
        // encoded in a maximum of 5 bytes. If the shift value is 35 or
        // greater, then, the byte's value will be shifted out beyond the
        // 32-bit value.
        //
        // With 28 being the highest valid shift value, the highest
        // order relevant bit in the final byte should be 0x08 or:
        //
        // 0000 1000
        //
        // Any higher bit is "lost" during the bitshift.
        //
        // Due to the encoding rules and two's complement, if the
        // highest order relevant bit is set, then the number is
        // negative and the `1` is extended to the higher bits like:
        //
        // 0111 1000
        //
        // Note that the highest order bit (the first bit from left to right)
        // MUST BE a 0. It is the bit which indicates more bytes should
        // be processed. For the maximum final byte (byte #5 for a
        // 32-bit number)), it MUST be 0. There are no additional bytes
        // to decode.
        //
        // If the highest order relevant bit is not set, then the
        // integer is positive. Any of the lower bits can be set.
        //
        // 0000 0111
        //
        // So the conditions to check are:
        //
        // 1. The highest order bit is not set (so there are no more
        //    bytes to decode). If it is set, the encoding is invalid.
        //    This is the "more" check.
        //
        // 2. Determine if any sign extended negative bit is set.
        //    So is any bit in:
        //
        //    0111 1000
        //
        //    set. If none of the bits are set, then the number is
        //    positive, and the encoding is valid.
        //    This is the "(n & mask != 0)" check.
        // 3. If any sign extended negative bits are set, the number is
        //    negative, and ALL of the bits MUST be set for a valid negative number.
        //    This is the "(n < mask)"" check.
        //    An equivalent check would be that "(n < mask) || (n >= 0x80)"
        //    But the earlier check for "more" removes the need for the additional check.
        //
        //    The check could also be "(n & mask) != mask".
        //
        // Another stricter condition is if the last byte has a 0 value.
        // The encoding is correct but not the minimal number of bytes
        // was used to express the final value.
        if shift == BITS - (BITS % 7) {
            #[allow(clippy::cast_sign_loss)]
            let mask = ((-1i8 << ((BITS % 7).saturating_sub(1))) & 0x7f) as u8;
            if more || (n & mask != 0 && n < mask) {
                return Err(Error(InnerError::InvalidEncoding));
            }
        }

        result |= T::from(n & 0x7f) << shift;
        shift += 7;
        idx += 1;

        if !more {
            break;
        }
    }

    if shift < T::BITS && n & 0x40 != 0 {
        result |= T::from(-1i8) << shift;
    }

    *pos = idx;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_u8() {
        let mut buffer = [0; 4];
        let mut pos = 1;
        let written = encode_fixed_uint_slice::<_, 8>(u8::MAX, &mut buffer, &mut pos);
        assert_eq!(3, pos);
        assert_eq!([0x00, 0xFF, 0x01, 0x00], buffer);
        assert_eq!(Some(2), written);
    }

    #[test]
    fn test_encode_u32() {
        let mut buffer = [0; 6];
        let mut pos = 1;
        let written = encode_fixed_uint_slice::<_, 32>(u32::MAX, &mut buffer, &mut pos);
        assert_eq!(6, pos);
        assert_eq!([0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x0F], buffer);
        assert_eq!(Some(5), written);
    }

    #[test]
    fn test_encode_u64_as_33_bits_2() {
        let mut buffer = [0; 6];
        let mut pos = 1;
        let written = encode_fixed_uint_slice::<_, 33>(2u64.pow(33) - 1, &mut buffer, &mut pos);
        let mut pos = 1;
        let value = decode_uint_slice::<u64, 33>(&buffer, &mut pos).unwrap();
        assert_eq!(8_589_934_592 - 1, value);
        assert_eq!(6, pos);
        assert_eq!([0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F], buffer);
        assert_eq!(Some(5), written);
    }

    #[test]
    fn test_encode_u64_as_33_bits_with_too_large_value() {
        let mut buffer = [0; 6];
        let mut pos = 1;
        let written = encode_fixed_uint_slice::<_, 33>(2u64.pow(34) - 1, &mut buffer, &mut pos);
        assert_eq!(1, pos);
        assert_eq!([0x00, 0x00, 0x00, 0x00, 0x00, 0x00], buffer);
        assert_eq!(None, written);
    }

    #[test]
    fn test_encode_u64() {
        let mut buffer = [0; 20];
        let mut pos = 1;
        let written = encode_fixed_uint_slice::<_, 64>(u64::MAX, &mut buffer, &mut pos);
        assert_eq!(11, pos);
        assert_eq!(Some(10), written);
    }

    #[test]
    fn test_decode_u32() {
        let input = [0xff, 0xff, 0xff, 0xff, 0x0f];
        let result = decode_u32(input);
        assert_eq!(result, Some((u32::MAX, 5)));

        let input = [0x00, 0x00, 0x00, 0x00, 0x00];
        let result = decode_u32(input);
        assert_eq!(result, Some((u32::MIN, 1)));

        // Valid but in-efficient way to encode 0.
        let input = [0x80, 0x80, 0x80, 0x80, 0x00];
        let result = decode_u32(input);
        assert_eq!(result, Some((u32::MIN, 5)));
    }

    #[test]
    fn test_decode_u32_errors() {
        // Maximum of 5 bytes encoding, the 0x80 bit must not be set.
        let input = [0xff, 0xff, 0xff, 0xff, 0x8f];
        let result = decode_u32(input);
        assert_eq!(result, None);

        // Parts of 0x1f (0x10) will be shifted out of the final value and lost.
        // This may too strict of a check since it could be ok.
        let input = [0xff, 0xff, 0xff, 0xff, 0x1f];
        let result = decode_u32(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_decode_u64() {
        let input = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01];
        let result = decode_u64(input);
        assert_eq!(result, Some((u64::MAX, 10)));

        let input = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = decode_u64(input);
        assert_eq!(result, Some((u64::MIN, 1)));

        // Valid but in-efficient way to encode 0.
        let input = [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00];
        let result = decode_u64(input);
        assert_eq!(result, Some((u64::MIN, 10)));
    }

    #[test]
    fn test_decode_u64_errors() {
        // Maximum of 10 bytes encoding, the 0x80 bit must not be set in the final byte.
        let input = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x81];
        let result = decode_u64(input);
        assert_eq!(result, None);

        // 0x02 will be shifted out of the final value and lost.
        // This may too strict of a check since it could be ok.
        let input = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02];
        let result = decode_u64(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_decode_s32() {
        let input = [0xff, 0xff, 0xff, 0xff, 0x07];
        let result = decode_s32(input);
        assert_eq!(result, Some((i32::MAX, 5)));

        let input = [0x80, 0x80, 0x80, 0x80, 0x78];
        let result = decode_s32(input);
        assert_eq!(result, Some((i32::MIN, 5)));

        let input = [0x00, 0x00, 0x00, 0x00, 0x00];
        let result = decode_s32(input);
        assert_eq!(result, Some((0, 1)));

        // Valid but in-efficient way to encode 0.
        let input = [0x80, 0x80, 0x80, 0x80, 0x00];
        let result = decode_s32(input);
        assert_eq!(result, Some((0, 5)));

        let input = [0x40, 0x00, 0x00, 0x00, 0x00];
        let result = decode_s32(input);
        assert_eq!(result, Some((-64, 1)));

        // Valid but in-efficient way to encode -64.
        let input = [0xc0, 0x7f, 0x00, 0x00, 0x00];
        let result = decode_s32(input);
        assert_eq!(result, Some((-64, 2)));
    }

    #[test]
    fn test_decode_s32_errors() {
        // Maximum of 5 bytes encoding, the 0x80 bit must not be set in the final byte.
        let input = [0x80, 0x80, 0x80, 0x80, 0x80];
        let result = decode_s32(input);
        assert_eq!(result, None);

        // If the highest valid bit is set, it should be sign extended. (final byte should be 0x78)
        let input = [0x80, 0x80, 0x80, 0x80, 0x08];
        let result = decode_s32(input);
        assert_eq!(result, None);

        // If the highest valid bit is set, it should be sign extended. (final byte should be 0x78)
        let input = [0x80, 0x80, 0x80, 0x80, 0x38];
        let result = decode_s32(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_decode_s33() {
        decode_sint_arr!(decode_s33, i64, 33);

        let input = [0xff, 0xff, 0xff, 0xff, 0x0f];
        let result = decode_s33(input);
        assert_eq!(result, Some((i64::from(u32::MAX), 5)));

        let input = [0x80, 0x80, 0x80, 0x80, 0x70];
        let result = decode_s33(input);
        assert_eq!(result, Some((i64::from(i32::MIN) * 2, 5)));

        let input = [0x00, 0x00, 0x00, 0x00, 0x00];
        let result = decode_s33(input);
        assert_eq!(result, Some((0, 1)));

        // Valid but in-efficient way to encode 0.
        let input = [0x80, 0x80, 0x80, 0x80, 0x00];
        let result = decode_s33(input);
        assert_eq!(result, Some((0, 5)));

        let input = [0x40, 0x00, 0x00, 0x00, 0x00];
        let result = decode_s33(input);
        assert_eq!(result, Some((-64, 1)));

        // Valid but in-efficient way to encode -64.
        let input = [0xc0, 0x7f, 0x00, 0x00, 0x00];
        let result = decode_s33(input);
        assert_eq!(result, Some((-64, 2)));
    }

    #[test]
    fn test_decode_s33_errors() {
        decode_sint_arr!(decode_s33, i64, 33);

        // Maximum of 5 bytes encoding, the 0x80 bit must not be set in the final byte.
        let input = [0x80, 0x80, 0x80, 0x80, 0x80];
        let result = decode_s33(input);
        assert_eq!(result, None);

        // If the highest valid bit is set, it should be sign extended. (final byte should be 0x70)
        let input = [0x80, 0x80, 0x80, 0x80, 0x10];
        let result = decode_s33(input);
        assert_eq!(result, None);

        // If the highest valid bit is set, it should be sign extended. (final byte should be 0x70)
        let input = [0x80, 0x80, 0x80, 0x80, 0x30];
        let result = decode_s33(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_decode_s64() {
        let input = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00];
        let result = decode_s64(input);
        assert_eq!(result, Some((i64::MAX, 10)));

        let input = [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7f];
        let result = decode_s64(input);
        assert_eq!(result, Some((i64::MIN, 10)));

        let input = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = decode_s64(input);
        assert_eq!(result, Some((0, 1)));

        // Valid but in-efficient way to encode 0.
        let input = [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00];
        let result = decode_s64(input);
        assert_eq!(result, Some((0, 10)));
    }

    #[test]
    fn test_decode_s64_errors() {
        // Maximum of 10 bytes encoding, the 0x80 bit must not be set in the final byte.
        let input = [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80];
        let result = decode_s64(input);
        assert_eq!(result, None);

        // If the highest valid bit is set, it should be sign extended. (final byte should be 0x78)
        let input = [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x08];
        let result = decode_s64(input);
        assert_eq!(result, None);

        // If the highest valid bit is set, it should be sign extended. (final byte should be 0x78)
        let input = [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x28];
        let result = decode_s64(input);
        assert_eq!(result, None);
    }
}
