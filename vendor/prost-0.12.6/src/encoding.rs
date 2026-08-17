//! Utility functions and types for encoding and decoding Protobuf types.
//!
//! Meant to be used only from `Message` implementations.

#![allow(clippy::implicit_hasher, clippy::ptr_arg)]

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem;
use core::str;
use core::u32;
use core::usize;

use ::bytes::{Buf, BufMut, Bytes};

use crate::DecodeError;
use crate::Message;

/// Encodes an integer value into LEB128 variable length format, and writes it to the buffer.
/// The buffer must have enough remaining space (maximum 10 bytes).
#[inline]
pub fn encode_varint<B>(mut value: u64, buf: &mut B)
where
    B: BufMut,
{
    // Varints are never more than 10 bytes
    for _ in 0..10 {
        if value < 0x80 {
            buf.put_u8(value as u8);
            break;
        } else {
            buf.put_u8(((value & 0x7F) | 0x80) as u8);
            value >>= 7;
        }
    }
}

/// Decodes a LEB128-encoded variable length integer from the buffer.
#[inline]
pub fn decode_varint<B>(buf: &mut B) -> Result<u64, DecodeError>
where
    B: Buf,
{
    let bytes = buf.chunk();
    let len = bytes.len();
    if len == 0 {
        return Err(DecodeError::new("invalid varint"));
    }

    let byte = bytes[0];
    if byte < 0x80 {
        buf.advance(1);
        Ok(u64::from(byte))
    } else if len > 10 || bytes[len - 1] < 0x80 {
        let (value, advance) = decode_varint_slice(bytes)?;
        buf.advance(advance);
        Ok(value)
    } else {
        decode_varint_slow(buf)
    }
}

/// Decodes a LEB128-encoded variable length integer from the slice, returning the value and the
/// number of bytes read.
///
/// Based loosely on [`ReadVarint64FromArray`][1] with a varint overflow check from
/// [`ConsumeVarint`][2].
///
/// ## Safety
///
/// The caller must ensure that `bytes` is non-empty and either `bytes.len() >= 10` or the last
/// element in bytes is < `0x80`.
///
/// [1]: https://github.com/google/protobuf/blob/3.3.x/src/google/protobuf/io/coded_stream.cc#L365-L406
/// [2]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
#[inline]
fn decode_varint_slice(bytes: &[u8]) -> Result<(u64, usize), DecodeError> {
    // Fully unrolled varint decoding loop. Splitting into 32-bit pieces gives better performance.

    // Use assertions to ensure memory safety, but it should always be optimized after inline.
    assert!(!bytes.is_empty());
    assert!(bytes.len() > 10 || bytes[bytes.len() - 1] < 0x80);

    let mut b: u8 = unsafe { *bytes.get_unchecked(0) };
    let mut part0: u32 = u32::from(b);
    if b < 0x80 {
        return Ok((u64::from(part0), 1));
    };
    part0 -= 0x80;
    b = unsafe { *bytes.get_unchecked(1) };
    part0 += u32::from(b) << 7;
    if b < 0x80 {
        return Ok((u64::from(part0), 2));
    };
    part0 -= 0x80 << 7;
    b = unsafe { *bytes.get_unchecked(2) };
    part0 += u32::from(b) << 14;
    if b < 0x80 {
        return Ok((u64::from(part0), 3));
    };
    part0 -= 0x80 << 14;
    b = unsafe { *bytes.get_unchecked(3) };
    part0 += u32::from(b) << 21;
    if b < 0x80 {
        return Ok((u64::from(part0), 4));
    };
    part0 -= 0x80 << 21;
    let value = u64::from(part0);

    b = unsafe { *bytes.get_unchecked(4) };
    let mut part1: u32 = u32::from(b);
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 5));
    };
    part1 -= 0x80;
    b = unsafe { *bytes.get_unchecked(5) };
    part1 += u32::from(b) << 7;
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 6));
    };
    part1 -= 0x80 << 7;
    b = unsafe { *bytes.get_unchecked(6) };
    part1 += u32::from(b) << 14;
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 7));
    };
    part1 -= 0x80 << 14;
    b = unsafe { *bytes.get_unchecked(7) };
    part1 += u32::from(b) << 21;
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 8));
    };
    part1 -= 0x80 << 21;
    let value = value + ((u64::from(part1)) << 28);

    b = unsafe { *bytes.get_unchecked(8) };
    let mut part2: u32 = u32::from(b);
    if b < 0x80 {
        return Ok((value + (u64::from(part2) << 56), 9));
    };
    part2 -= 0x80;
    b = unsafe { *bytes.get_unchecked(9) };
    part2 += u32::from(b) << 7;
    // Check for u64::MAX overflow. See [`ConsumeVarint`][1] for details.
    // [1]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
    if b < 0x02 {
        return Ok((value + (u64::from(part2) << 56), 10));
    };

    // We have overrun the maximum size of a varint (10 bytes) or the final byte caused an overflow.
    // Assume the data is corrupt.
    Err(DecodeError::new("invalid varint"))
}

/// Decodes a LEB128-encoded variable length integer from the buffer, advancing the buffer as
/// necessary.
///
/// Contains a varint overflow check from [`ConsumeVarint`][1].
///
/// [1]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
#[inline(never)]
#[cold]
fn decode_varint_slow<B>(buf: &mut B) -> Result<u64, DecodeError>
where
    B: Buf,
{
    let mut value = 0;
    for count in 0..min(10, buf.remaining()) {
        let byte = buf.get_u8();
        value |= u64::from(byte & 0x7F) << (count * 7);
        if byte <= 0x7F {
            // Check for u64::MAX overflow. See [`ConsumeVarint`][1] for details.
            // [1]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
            if count == 9 && byte >= 0x02 {
                return Err(DecodeError::new("invalid varint"));
            } else {
                return Ok(value);
            }
        }
    }

    Err(DecodeError::new("invalid varint"))
}

/// Additional information passed to every decode/merge function.
///
/// The context should be passed by value and can be freely cloned. When passing
/// to a function which is decoding a nested object, then use `enter_recursion`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "no-recursion-limit", derive(Default))]
pub struct DecodeContext {
    /// How many times we can recurse in the current decode stack before we hit
    /// the recursion limit.
    ///
    /// The recursion limit is defined by `RECURSION_LIMIT` and cannot be
    /// customized. The recursion limit can be ignored by building the Prost
    /// crate with the `no-recursion-limit` feature.
    #[cfg(not(feature = "no-recursion-limit"))]
    recurse_count: u32,
}

#[cfg(not(feature = "no-recursion-limit"))]
impl Default for DecodeContext {
    #[inline]
    fn default() -> DecodeContext {
        DecodeContext {
            recurse_count: crate::RECURSION_LIMIT,
        }
    }
}

impl DecodeContext {
    /// Call this function before recursively decoding.
    ///
    /// There is no `exit` function since this function creates a new `DecodeContext`
    /// to be used at the next level of recursion. Continue to use the old context
    // at the previous level of recursion.
    #[cfg(not(feature = "no-recursion-limit"))]
    #[inline]
    pub(crate) fn enter_recursion(&self) -> DecodeContext {
        DecodeContext {
            recurse_count: self.recurse_count - 1,
        }
    }

    #[cfg(feature = "no-recursion-limit")]
    #[inline]
    pub(crate) fn enter_recursion(&self) -> DecodeContext {
        DecodeContext {}
    }

    /// Checks whether the recursion limit has been reached in the stack of
    /// decodes described by the `DecodeContext` at `self.ctx`.
    ///
    /// Returns `Ok<()>` if it is ok to continue recursing.
    /// Returns `Err<DecodeError>` if the recursion limit has been reached.
    #[cfg(not(feature = "no-recursion-limit"))]
    #[inline]
    pub(crate) fn limit_reached(&self) -> Result<(), DecodeError> {
        if self.recurse_count == 0 {
            Err(DecodeError::new("recursion limit reached"))
        } else {
            Ok(())
        }
    }

    #[cfg(feature = "no-recursion-limit")]
    #[inline]
    #[allow(clippy::unnecessary_wraps)] // needed in other features
    pub(crate) fn limit_reached(&self) -> Result<(), DecodeError> {
        Ok(())
    }
}

/// Returns the encoded length of the value in LEB128 variable length format.
/// The returned value will be between 1 and 10, inclusive.
#[inline]
pub fn encoded_len_varint(value: u64) -> usize {
    // Based on [VarintSize64][1].
    // [1]: https://github.com/google/protobuf/blob/3.3.x/src/google/protobuf/io/coded_stream.h#L1301-L1309
    ((((value | 1).leading_zeros() ^ 63) * 9 + 73) / 64) as usize
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WireType {
    Varint = 0,
    SixtyFourBit = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    ThirtyTwoBit = 5,
}

pub const MIN_TAG: u32 = 1;
pub const MAX_TAG: u32 = (1 << 29) - 1;

impl TryFrom<u64> for WireType {
    type Error = DecodeError;

    #[inline]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WireType::Varint),
            1 => Ok(WireType::SixtyFourBit),
            2 => Ok(WireType::LengthDelimited),
            3 => Ok(WireType::StartGroup),
            4 => Ok(WireType::EndGroup),
            5 => Ok(WireType::ThirtyTwoBit),
            _ => Err(DecodeError::new(format!(
                "invalid wire type value: {}",
                value
            ))),
        }
    }
}

/// Encodes a Protobuf field key, which consists of a wire type designator and
/// the field tag.
#[inline]
pub fn encode_key<B>(tag: u32, wire_type: WireType, buf: &mut B)
where
    B: BufMut,
{
    debug_assert!((MIN_TAG..=MAX_TAG).contains(&tag));
    let key = (tag << 3) | wire_type as u32;
    encode_varint(u64::from(key), buf);
}

/// Decodes a Protobuf field key, which consists of a wire type designator and
/// the field tag.
#[inline(always)]
pub fn decode_key<B>(buf: &mut B) -> Result<(u32, WireType), DecodeError>
where
    B: Buf,
{
    let key = decode_varint(buf)?;
    if key > u64::from(u32::MAX) {
        return Err(DecodeError::new(format!("invalid key value: {}", key)));
    }
    let wire_type = WireType::try_from(key & 0x07)?;
    let tag = key as u32 >> 3;

    if tag < MIN_TAG {
        return Err(DecodeError::new("invalid tag value: 0"));
    }

    Ok((tag, wire_type))
}

/// Returns the width of an encoded Protobuf field key with the given tag.
/// The returned width will be between 1 and 5 bytes (inclusive).
#[inline]
pub fn key_len(tag: u32) -> usize {
    encoded_len_varint(u64::from(tag << 3))
}

/// Checks that the expected wire type matches the actual wire type,
/// or returns an error result.
#[inline]
pub fn check_wire_type(expected: WireType, actual: WireType) -> Result<(), DecodeError> {
    if expected != actual {
        return Err(DecodeError::new(format!(
            "invalid wire type: {:?} (expected {:?})",
            actual, expected
        )));
    }
    Ok(())
}

/// Helper function which abstracts reading a length delimiter prefix followed
/// by decoding values until the length of bytes is exhausted.
pub fn merge_loop<T, M, B>(
    value: &mut T,
    buf: &mut B,
    ctx: DecodeContext,
    mut merge: M,
) -> Result<(), DecodeError>
where
    M: FnMut(&mut T, &mut B, DecodeContext) -> Result<(), DecodeError>,
    B: Buf,
{
    let len = decode_varint(buf)?;
    let remaining = buf.remaining();
    if len > remaining as u64 {
        return Err(DecodeError::new("buffer underflow"));
    }

    let limit = remaining - len as usize;
    while buf.remaining() > limit {
        merge(value, buf, ctx.clone())?;
    }

    if buf.remaining() != limit {
        return Err(DecodeError::new("delimited length exceeded"));
    }
    Ok(())
}

pub fn skip_field<B>(
    wire_type: WireType,
    tag: u32,
    buf: &mut B,
    ctx: DecodeContext,
) -> Result<(), DecodeError>
where
    B: Buf,
{
    ctx.limit_reached()?;
    let len = match wire_type {
        WireType::Varint => decode_varint(buf).map(|_| 0)?,
        WireType::ThirtyTwoBit => 4,
        WireType::SixtyFourBit => 8,
        WireType::LengthDelimited => decode_varint(buf)?,
        WireType::StartGroup => loop {
            let (inner_tag, inner_wire_type) = decode_key(buf)?;
            match inner_wire_type {
                WireType::EndGroup => {
                    if inner_tag != tag {
                        return Err(DecodeError::new("unexpected end group tag"));
                    }
                    break 0;
                }
                _ => skip_field(inner_wire_type, inner_tag, buf, ctx.enter_recursion())?,
            }
        },
        WireType::EndGroup => return Err(DecodeError::new("unexpected end group tag")),
    };

    if len > buf.remaining() as u64 {
        return Err(DecodeError::new("buffer underflow"));
    }

    buf.advance(len as usize);
    Ok(())
}

/// Helper macro which emits an `encode_repeated` function for the type.
macro_rules! encode_repeated {
    ($ty:ty) => {
        pub fn encode_repeated<B>(tag: u32, values: &[$ty], buf: &mut B)
        where
            B: BufMut,
        {
            for value in values {
                encode(tag, value, buf);
            }
        }
    };
}

/// Helper macro which emits a `merge_repeated` function for the numeric type.
macro_rules! merge_repeated_numeric {
    ($ty:ty,
     $wire_type:expr,
     $merge:ident,
     $merge_repeated:ident) => {
        pub fn $merge_repeated<B>(
            wire_type: WireType,
            values: &mut Vec<$ty>,
            buf: &mut B,
            ctx: DecodeContext,
        ) -> Result<(), DecodeError>
        where
            B: Buf,
        {
            if wire_type == WireType::LengthDelimited {
                // Packed.
                merge_loop(values, buf, ctx, |values, buf, ctx| {
                    let mut value = Default::default();
                    $merge($wire_type, &mut value, buf, ctx)?;
                    values.push(value);
                    Ok(())
                })
            } else {
                // Unpacked.
                check_wire_type($wire_type, wire_type)?;
                let mut value = Default::default();
                $merge(wire_type, &mut value, buf, ctx)?;
                values.push(value);
                Ok(())
            }
        }
    };
}

/// Macro which emits a module containing a set of encoding functions for a
/// variable width numeric type.
macro_rules! varint {
    ($ty:ty,
     $proto_ty:ident) => (
        varint!($ty,
                $proto_ty,
                to_uint64(value) { *value as u64 },
                from_uint64(value) { value as $ty });
    );

    ($ty:ty,
     $proto_ty:ident,
     to_uint64($to_uint64_value:ident) $to_uint64:expr,
     from_uint64($from_uint64_value:ident) $from_uint64:expr) => (

         pub mod $proto_ty {
            use crate::encoding::*;

            pub fn encode<B>(tag: u32, $to_uint64_value: &$ty, buf: &mut B) where B: BufMut {
                encode_key(tag, WireType::Varint, buf);
                encode_varint($to_uint64, buf);
            }

            pub fn merge<B>(wire_type: WireType, value: &mut $ty, buf: &mut B, _ctx: DecodeContext) -> Result<(), DecodeError> where B: Buf {
                check_wire_type(WireType::Varint, wire_type)?;
                let $from_uint64_value = decode_varint(buf)?;
                *value = $from_uint64;
                Ok(())
            }

            encode_repeated!($ty);

            pub fn encode_packed<B>(tag: u32, values: &[$ty], buf: &mut B) where B: BufMut {
                if values.is_empty() { return; }

                encode_key(tag, WireType::LengthDelimited, buf);
                let len: usize = values.iter().map(|$to_uint64_value| {
                    encoded_len_varint($to_uint64)
                }).sum();
                encode_varint(len as u64, buf);

                for $to_uint64_value in values {
                    encode_varint($to_uint64, buf);
                }
            }

            merge_repeated_numeric!($ty, WireType::Varint, merge, merge_repeated);

            #[inline]
            pub fn encoded_len(tag: u32, $to_uint64_value: &$ty) -> usize {
                key_len(tag) + encoded_len_varint($to_uint64)
            }

            #[inline]
            pub fn encoded_len_repeated(tag: u32, values: &[$ty]) -> usize {
                key_len(tag) * values.len() + values.iter().map(|$to_uint64_value| {
                    encoded_len_varint($to_uint64)
                }).sum::<usize>()
            }

            #[inline]
            pub fn encoded_len_packed(tag: u32, values: &[$ty]) -> usize {
                if values.is_empty() {
                    0
                } else {
                    let len = values.iter()
                                    .map(|$to_uint64_value| encoded_len_varint($to_uint64))
                                    .sum::<usize>();
                    key_len(tag) + encoded_len_varint(len as u64) + len
                }
            }

            #[cfg(test)]
            mod test {
                use proptest::prelude::*;

                use crate::encoding::$proto_ty::*;
                use crate::encoding::test::{
                    check_collection_type,
                    check_type,
                };

                proptest! {
                    #[test]
                    fn check(value: $ty, tag in MIN_TAG..=MAX_TAG) {
                        check_type(value, tag, WireType::Varint,
                                   encode, merge, encoded_len)?;
                    }
                    #[test]
                    fn check_repeated(value: Vec<$ty>, tag in MIN_TAG..=MAX_TAG) {
                        check_collection_type(value, tag, WireType::Varint,
                                              encode_repeated, merge_repeated,
                                              encoded_len_repeated)?;
                    }
                    #[test]
                    fn check_packed(value: Vec<$ty>, tag in MIN_TAG..=MAX_TAG) {
                        check_type(value, tag, WireType::LengthDelimited,
                                   encode_packed, merge_repeated,
                                   encoded_len_packed)?;
                    }
                }
            }
         }

    );
}
varint!(bool, bool,
        to_uint64(value) u64::from(*value),
        from_uint64(value) value != 0);
varint!(i32, int32);
varint!(i64, int64);
varint!(u32, uint32);
varint!(u64, uint64);
varint!(i32, sint32,
to_uint64(value) {
    ((value << 1) ^ (value >> 31)) as u32 as u64
},
from_uint64(value) {
    let value = value as u32;
    ((value >> 1) as i32) ^ (-((value & 1) as i32))
});
varint!(i64, sint64,
to_uint64(value) {
    ((value << 1) ^ (value >> 63)) as u64
},
from_uint64(value) {
    ((value >> 1) as i64) ^ (-((value & 1) as i64))
});

/// Macro which emits a module containing a set of encoding functions for a
/// fixed width numeric type.
macro_rules! fixed_width {
    ($ty:ty,
     $width:expr,
     $wire_type:expr,
     $proto_ty:ident,
     $put:ident,
     $get:ident) => {
        pub mod $proto_ty {
            use crate::encoding::*;

            pub fn encode<B>(tag: u32, value: &$ty, buf: &mut B)
            where
                B: BufMut,
            {
                encode_key(tag, $wire_type, buf);
                buf.$put(*value);
            }

            pub fn merge<B>(
                wire_type: WireType,
                value: &mut $ty,
                buf: &mut B,
                _ctx: DecodeContext,
            ) -> Result<(), DecodeError>
            where
                B: Buf,
            {
                check_wire_type($wire_type, wire_type)?;
                if buf.remaining() < $width {
                    return Err(DecodeError::new("buffer underflow"));
                }
                *value = buf.$get();
                Ok(())
            }

            encode_repeated!($ty);

            pub fn encode_packed<B>(tag: u32, values: &[$ty], buf: &mut B)
            where
                B: BufMut,
            {
                if values.is_empty() {
                    return;
                }

                encode_key(tag, WireType::LengthDelimited, buf);
                let len = values.len() as u64 * $width;
                encode_varint(len as u64, buf);

                for value in values {
                    buf.$put(*value);
                }
            }

            merge_repeated_numeric!($ty, $wire_type, merge, merge_repeated);

            #[inline]
            pub fn encoded_len(tag: u32, _: &$ty) -> usize {
                key_len(tag) + $width
            }

            #[inline]
            pub fn encoded_len_repeated(tag: u32, values: &[$ty]) -> usize {
                (key_len(tag) + $width) * values.len()
            }

            #[inline]
            pub fn encoded_len_packed(tag: u32, values: &[$ty]) -> usize {
                if values.is_empty() {
                    0
                } else {
                    let len = $width * values.len();
                    key_len(tag) + encoded_len_varint(len as u64) + len
                }
            }

            #[cfg(test)]
            mod test {
                use proptest::prelude::*;

                use super::super::test::{check_collection_type, check_type};
                use super::*;

                proptest! {
                    #[test]
                    fn check(value: $ty, tag in MIN_TAG..=MAX_TAG) {
                        check_type(value, tag, $wire_type,
                                   encode, merge, encoded_len)?;
                    }
                    #[test]
                    fn check_repeated(value: Vec<$ty>, tag in MIN_TAG..=MAX_TAG) {
                        check_collection_type(value, tag, $wire_type,
                                              encode_repeated, merge_repeated,
                                              encoded_len_repeated)?;
                    }
                    #[test]
                    fn check_packed(value: Vec<$ty>, tag in MIN_TAG..=MAX_TAG) {
                        check_type(value, tag, WireType::LengthDelimited,
                                   encode_packed, merge_repeated,
                                   encoded_len_packed)?;
                    }
                }
            }
        }
    };
}
fixed_width!(
    f32,
    4,
    WireType::ThirtyTwoBit,
    float,
    put_f32_le,
    get_f32_le
);
fixed_width!(
    f64,
    8,
    WireType::SixtyFourBit,
    double,
    put_f64_le,
    get_f64_le
);
fixed_width!(
    u32,
    4,
    WireType::ThirtyTwoBit,
    fixed32,
    put_u32_le,
    get_u32_le
);
fixed_width!(
    u64,
    8,
    WireType::SixtyFourBit,
    fixed64,
    put_u64_le,
    get_u64_le
);
fixed_width!(
    i32,
    4,
    WireType::ThirtyTwoBit,
    sfixed32,
    put_i32_le,
    get_i32_le
);
fixed_width!(
    i64,
    8,
    WireType::SixtyFourBit,
    sfixed64,
    put_i64_le,
    get_i64_le
);

/// Macro which emits encoding functions for a length-delimited type.
macro_rules! length_delimited {
    ($ty:ty) => {
        encode_repeated!($ty);

        pub fn merge_repeated<B>(
            wire_type: WireType,
            values: &mut Vec<$ty>,
            buf: &mut B,
            ctx: DecodeContext,
        ) -> Result<(), DecodeError>
        where
            B: Buf,
        {
            check_wire_type(WireType::LengthDelimited, wire_type)?;
            let mut value = Default::default();
            merge(wire_type, &mut value, buf, ctx)?;
            values.push(value);
            Ok(())
        }

        #[inline]
        pub fn encoded_len(tag: u32, value: &$ty) -> usize {
            key_len(tag) + encoded_len_varint(value.len() as u64) + value.len()
        }

        #[inline]
        pub fn encoded_len_repeated(tag: u32, values: &[$ty]) -> usize {
            key_len(tag) * values.len()
                + values
                    .iter()
                    .map(|value| encoded_len_varint(value.len() as u64) + value.len())
                    .sum::<usize>()
        }
    };
}

pub mod string {
    use super::*;

    pub fn encode<B>(tag: u32, value: &String, buf: &mut B)
    where
        B: BufMut,
    {
        encode_key(tag, WireType::LengthDelimited, buf);
        encode_varint(value.len() as u64, buf);
        buf.put_slice(value.as_bytes());
    }
    pub fn merge<B>(
        wire_type: WireType,
        value: &mut String,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        // ## Unsafety
        //
        // `string::merge` reuses `bytes::merge`, with an additional check of utf-8
        // well-formedness. If the utf-8 is not well-formed, or if any other error occurs, then the
        // string is cleared, so as to avoid leaking a string field with invalid data.
        //
        // This implementation uses the unsafe `String::as_mut_vec` method instead of the safe
        // alternative of temporarily swapping an empty `String` into the field, because it results
        // in up to 10% better performance on the protobuf message decoding benchmarks.
        //
        // It's required when using `String::as_mut_vec` that invalid utf-8 data not be leaked into
        // the backing `String`. To enforce this, even in the event of a panic in `bytes::merge` or
        // in the buf implementation, a drop guard is used.
        unsafe {
            struct DropGuard<'a>(&'a mut Vec<u8>);
            impl<'a> Drop for DropGuard<'a> {
                #[inline]
                fn drop(&mut self) {
                    self.0.clear();
                }
            }

            let drop_guard = DropGuard(value.as_mut_vec());
            bytes::merge_one_copy(wire_type, drop_guard.0, buf, ctx)?;
            match str::from_utf8(drop_guard.0) {
                Ok(_) => {
                    // Success; do not clear the bytes.
                    mem::forget(drop_guard);
                    Ok(())
                }
                Err(_) => Err(DecodeError::new(
                    "invalid string value: data is not UTF-8 encoded",
                )),
            }
        }
    }

    length_delimited!(String);

    #[cfg(test)]
    mod test {
        use proptest::prelude::*;

        use super::super::test::{check_collection_type, check_type};
        use super::*;

        proptest! {
            #[test]
            fn check(value: String, tag in MIN_TAG..=MAX_TAG) {
                super::test::check_type(value, tag, WireType::LengthDelimited,
                                        encode, merge, encoded_len)?;
            }
            #[test]
            fn check_repeated(value: Vec<String>, tag in MIN_TAG..=MAX_TAG) {
                super::test::check_collection_type(value, tag, WireType::LengthDelimited,
                                                   encode_repeated, merge_repeated,
                                                   encoded_len_repeated)?;
            }
        }
    }
}

pub trait BytesAdapter: sealed::BytesAdapter {}

mod sealed {
    use super::{Buf, BufMut};

    pub trait BytesAdapter: Default + Sized + 'static {
        fn len(&self) -> usize;

        /// Replace contents of this buffer with the contents of another buffer.
        fn replace_with<B>(&mut self, buf: B)
        where
            B: Buf;

        /// Appends this buffer to the (contents of) other buffer.
        fn append_to<B>(&self, buf: &mut B)
        where
            B: BufMut;

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }
}

impl BytesAdapter for Bytes {}

impl sealed::BytesAdapter for Bytes {
    fn len(&self) -> usize {
        Buf::remaining(self)
    }

    fn replace_with<B>(&mut self, mut buf: B)
    where
        B: Buf,
    {
        *self = buf.copy_to_bytes(buf.remaining());
    }

    fn append_to<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        buf.put(self.clone())
    }
}

impl BytesAdapter for Vec<u8> {}

impl sealed::BytesAdapter for Vec<u8> {
    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn replace_with<B>(&mut self, buf: B)
    where
        B: Buf,
    {
        self.clear();
        self.reserve(buf.remaining());
        self.put(buf);
    }

    fn append_to<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        buf.put(self.as_slice())
    }
}

pub mod bytes {
    use super::*;

    pub fn encode<A, B>(tag: u32, value: &A, buf: &mut B)
    where
        A: BytesAdapter,
        B: BufMut,
    {
        encode_key(tag, WireType::LengthDelimited, buf);
        encode_varint(value.len() as u64, buf);
        value.append_to(buf);
    }

    pub fn merge<A, B>(
        wire_type: WireType,
        value: &mut A,
        buf: &mut B,
        _ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        A: BytesAdapter,
        B: Buf,
    {
        check_wire_type(WireType::LengthDelimited, wire_type)?;
        let len = decode_varint(buf)?;
        if len > buf.remaining() as u64 {
            return Err(DecodeError::new("buffer underflow"));
        }
        let len = len as usize;

        // Clear the existing value. This follows from the following rule in the encoding guide[1]:
        //
        // > Normally, an encoded message would never have more than one instance of a non-repeated
        // > field. However, parsers are expected to handle the case in which they do. For numeric
        // > types and strings, if the same field appears multiple times, the parser accepts the
        // > last value it sees.
        //
        // [1]: https://developers.google.com/protocol-buffers/docs/encoding#optional
        //
        // This is intended for A and B both being Bytes so it is zero-copy.
        // Some combinations of A and B types may cause a double-copy,
        // in which case merge_one_copy() should be used instead.
        value.replace_with(buf.copy_to_bytes(len));
        Ok(())
    }

    pub(super) fn merge_one_copy<A, B>(
        wire_type: WireType,
        value: &mut A,
        buf: &mut B,
        _ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        A: BytesAdapter,
        B: Buf,
    {
        check_wire_type(WireType::LengthDelimited, wire_type)?;
        let len = decode_varint(buf)?;
        if len > buf.remaining() as u64 {
            return Err(DecodeError::new("buffer underflow"));
        }
        let len = len as usize;

        // If we must copy, make sure to copy only once.
        value.replace_with(buf.take(len));
        Ok(())
    }

    length_delimited!(impl BytesAdapter);

    #[cfg(test)]
    mod test {
        use proptest::prelude::*;

        use super::super::test::{check_collection_type, check_type};
        use super::*;

        proptest! {
            #[test]
            fn check_vec(value: Vec<u8>, tag in MIN_TAG..=MAX_TAG) {
                super::test::check_type::<Vec<u8>, Vec<u8>>(value, tag, WireType::LengthDelimited,
                                                            encode, merge, encoded_len)?;
            }

            #[test]
            fn check_bytes(value: Vec<u8>, tag in MIN_TAG..=MAX_TAG) {
                let value = Bytes::from(value);
                super::test::check_type::<Bytes, Bytes>(value, tag, WireType::LengthDelimited,
                                                        encode, merge, encoded_len)?;
            }

            #[test]
            fn check_repeated_vec(value: Vec<Vec<u8>>, tag in MIN_TAG..=MAX_TAG) {
                super::test::check_collection_type(value, tag, WireType::LengthDelimited,
                                                   encode_repeated, merge_repeated,
                                                   encoded_len_repeated)?;
            }

            #[test]
            fn check_repeated_bytes(value: Vec<Vec<u8>>, tag in MIN_TAG..=MAX_TAG) {
                let value = value.into_iter().map(Bytes::from).collect();
                super::test::check_collection_type(value, tag, WireType::LengthDelimited,
                                                   encode_repeated, merge_repeated,
                                                   encoded_len_repeated)?;
            }
        }
    }
}

pub mod message {
    use super::*;

    pub fn encode<M, B>(tag: u32, msg: &M, buf: &mut B)
    where
        M: Message,
        B: BufMut,
    {
        encode_key(tag, WireType::LengthDelimited, buf);
        encode_varint(msg.encoded_len() as u64, buf);
        msg.encode_raw(buf);
    }

    pub fn merge<M, B>(
        wire_type: WireType,
        msg: &mut M,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        M: Message,
        B: Buf,
    {
        check_wire_type(WireType::LengthDelimited, wire_type)?;
        ctx.limit_reached()?;
        merge_loop(
            msg,
            buf,
            ctx.enter_recursion(),
            |msg: &mut M, buf: &mut B, ctx| {
                let (tag, wire_type) = decode_key(buf)?;
                msg.merge_field(tag, wire_type, buf, ctx)
            },
        )
    }

    pub fn encode_repeated<M, B>(tag: u32, messages: &[M], buf: &mut B)
    where
        M: Message,
        B: BufMut,
    {
        for msg in messages {
            encode(tag, msg, buf);
        }
    }

    pub fn merge_repeated<M, B>(
        wire_type: WireType,
        messages: &mut Vec<M>,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        M: Message + Default,
        B: Buf,
    {
        check_wire_type(WireType::LengthDelimited, wire_type)?;
        let mut msg = M::default();
        merge(WireType::LengthDelimited, &mut msg, buf, ctx)?;
        messages.push(msg);
        Ok(())
    }

    #[inline]
    pub fn encoded_len<M>(tag: u32, msg: &M) -> usize
    where
        M: Message,
    {
        let len = msg.encoded_len();
        key_len(tag) + encoded_len_varint(len as u64) + len
    }

    #[inline]
    pub fn encoded_len_repeated<M>(tag: u32, messages: &[M]) -> usize
    where
        M: Message,
    {
        key_len(tag) * messages.len()
            + messages
                .iter()
                .map(Message::encoded_len)
                .map(|len| len + encoded_len_varint(len as u64))
                .sum::<usize>()
    }
}

pub mod group {
    use super::*;

    pub fn encode<M, B>(tag: u32, msg: &M, buf: &mut B)
    where
        M: Message,
        B: BufMut,
    {
        encode_key(tag, WireType::StartGroup, buf);
        msg.encode_raw(buf);
        encode_key(tag, WireType::EndGroup, buf);
    }

    pub fn merge<M, B>(
        tag: u32,
        wire_type: WireType,
        msg: &mut M,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        M: Message,
        B: Buf,
    {
        check_wire_type(WireType::StartGroup, wire_type)?;

        ctx.limit_reached()?;
        loop {
            let (field_tag, field_wire_type) = decode_key(buf)?;
            if field_wire_type == WireType::EndGroup {
                if field_tag != tag {
                    return Err(DecodeError::new("unexpected end group tag"));
                }
                return Ok(());
            }

            M::merge_field(msg, field_tag, field_wire_type, buf, ctx.enter_recursion())?;
        }
    }

    pub fn encode_repeated<M, B>(tag: u32, messages: &[M], buf: &mut B)
    where
        M: Message,
        B: BufMut,
    {
        for msg in messages {
            encode(tag, msg, buf);
        }
    }

    pub fn merge_repeated<M, B>(
        tag: u32,
        wire_type: WireType,
        messages: &mut Vec<M>,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        M: Message + Default,
        B: Buf,
    {
        check_wire_type(WireType::StartGroup, wire_type)?;
        let mut msg = M::default();
        merge(tag, WireType::StartGroup, &mut msg, buf, ctx)?;
        messages.push(msg);
        Ok(())
    }

    #[inline]
    pub fn encoded_len<M>(tag: u32, msg: &M) -> usize
    where
        M: Message,
    {
        2 * key_len(tag) + msg.encoded_len()
    }

    #[inline]
    pub fn encoded_len_repeated<M>(tag: u32, messages: &[M]) -> usize
    where
        M: Message,
    {
        2 * key_len(tag) * messages.len() + messages.iter().map(Message::encoded_len).sum::<usize>()
    }
}

/// Rust doesn't have a `Map` trait, so macros are currently the best way to be
/// generic over `HashMap` and `BTreeMap`.
macro_rules! map {
    ($map_ty:ident) => {
        use crate::encoding::*;
        use core::hash::Hash;

        /// Generic protobuf map encode function.
        pub fn encode<K, V, B, KE, KL, VE, VL>(
            key_encode: KE,
            key_encoded_len: KL,
            val_encode: VE,
            val_encoded_len: VL,
            tag: u32,
            values: &$map_ty<K, V>,
            buf: &mut B,
        ) where
            K: Default + Eq + Hash + Ord,
            V: Default + PartialEq,
            B: BufMut,
            KE: Fn(u32, &K, &mut B),
            KL: Fn(u32, &K) -> usize,
            VE: Fn(u32, &V, &mut B),
            VL: Fn(u32, &V) -> usize,
        {
            encode_with_default(
                key_encode,
                key_encoded_len,
                val_encode,
                val_encoded_len,
                &V::default(),
                tag,
                values,
                buf,
            )
        }

        /// Generic protobuf map merge function.
        pub fn merge<K, V, B, KM, VM>(
            key_merge: KM,
            val_merge: VM,
            values: &mut $map_ty<K, V>,
            buf: &mut B,
            ctx: DecodeContext,
        ) -> Result<(), DecodeError>
        where
            K: Default + Eq + Hash + Ord,
            V: Default,
            B: Buf,
            KM: Fn(WireType, &mut K, &mut B, DecodeContext) -> Result<(), DecodeError>,
            VM: Fn(WireType, &mut V, &mut B, DecodeContext) -> Result<(), DecodeError>,
        {
            merge_with_default(key_merge, val_merge, V::default(), values, buf, ctx)
        }

        /// Generic protobuf map encode function.
        pub fn encoded_len<K, V, KL, VL>(
            key_encoded_len: KL,
            val_encoded_len: VL,
            tag: u32,
            values: &$map_ty<K, V>,
        ) -> usize
        where
            K: Default + Eq + Hash + Ord,
            V: Default + PartialEq,
            KL: Fn(u32, &K) -> usize,
            VL: Fn(u32, &V) -> usize,
        {
            encoded_len_with_default(key_encoded_len, val_encoded_len, &V::default(), tag, values)
        }

        /// Generic protobuf map encode function with an overridden value default.
        ///
        /// This is necessary because enumeration values can have a default value other
        /// than 0 in proto2.
        pub fn encode_with_default<K, V, B, KE, KL, VE, VL>(
            key_encode: KE,
            key_encoded_len: KL,
            val_encode: VE,
            val_encoded_len: VL,
            val_default: &V,
            tag: u32,
            values: &$map_ty<K, V>,
            buf: &mut B,
        ) where
            K: Default + Eq + Hash + Ord,
            V: PartialEq,
            B: BufMut,
            KE: Fn(u32, &K, &mut B),
            KL: Fn(u32, &K) -> usize,
            VE: Fn(u32, &V, &mut B),
            VL: Fn(u32, &V) -> usize,
        {
            for (key, val) in values.iter() {
                let skip_key = key == &K::default();
                let skip_val = val == val_default;

                let len = (if skip_key { 0 } else { key_encoded_len(1, key) })
                    + (if skip_val { 0 } else { val_encoded_len(2, val) });

                encode_key(tag, WireType::LengthDelimited, buf);
                encode_varint(len as u64, buf);
                if !skip_key {
                    key_encode(1, key, buf);
                }
                if !skip_val {
                    val_encode(2, val, buf);
                }
            }
        }

        /// Generic protobuf map merge function with an overridden value default.
        ///
        /// This is necessary because enumeration values can have a default value other
        /// than 0 in proto2.
        pub fn merge_with_default<K, V, B, KM, VM>(
            key_merge: KM,
            val_merge: VM,
            val_default: V,
            values: &mut $map_ty<K, V>,
            buf: &mut B,
            ctx: DecodeContext,
        ) -> Result<(), DecodeError>
        where
            K: Default + Eq + Hash + Ord,
            B: Buf,
            KM: Fn(WireType, &mut K, &mut B, DecodeContext) -> Result<(), DecodeError>,
            VM: Fn(WireType, &mut V, &mut B, DecodeContext) -> Result<(), DecodeError>,
        {
            let mut key = Default::default();
            let mut val = val_default;
            ctx.limit_reached()?;
            merge_loop(
                &mut (&mut key, &mut val),
                buf,
                ctx.enter_recursion(),
                |&mut (ref mut key, ref mut val), buf, ctx| {
                    let (tag, wire_type) = decode_key(buf)?;
                    match tag {
                        1 => key_merge(wire_type, key, buf, ctx),
                        2 => val_merge(wire_type, val, buf, ctx),
                        _ => skip_field(wire_type, tag, buf, ctx),
                    }
                },
            )?;
            values.insert(key, val);

            Ok(())
        }

        /// Generic protobuf map encode function with an overridden value default.
        ///
        /// This is necessary because enumeration values can have a default value other
        /// than 0 in proto2.
        pub fn encoded_len_with_default<K, V, KL, VL>(
            key_encoded_len: KL,
            val_encoded_len: VL,
            val_default: &V,
            tag: u32,
            values: &$map_ty<K, V>,
        ) -> usize
        where
            K: Default + Eq + Hash + Ord,
            V: PartialEq,
            KL: Fn(u32, &K) -> usize,
            VL: Fn(u32, &V) -> usize,
        {
            key_len(tag) * values.len()
                + values
                    .iter()
                    .map(|(key, val)| {
                        let len = (if key == &K::default() {
                            0
                        } else {
                            key_encoded_len(1, key)
                        }) + (if val == val_default {
                            0
                        } else {
                            val_encoded_len(2, val)
                        });
                        encoded_len_varint(len as u64) + len
                    })
                    .sum::<usize>()
        }
    };
}

#[cfg(feature = "std")]
pub mod hash_map {
    use std::collections::HashMap;
    map!(HashMap);
}

pub mod btree_map {
    map!(BTreeMap);
}

#[cfg(test)]
mod test {
    #[cfg(not(feature = "std"))]
    use alloc::string::ToString;
    use core::borrow::Borrow;
    use core::fmt::Debug;
    use core::u64;

    use ::bytes::BytesMut;
    use proptest::{prelude::*, test_runner::TestCaseResult};

    use super::*;

    pub fn check_type<T, B>(
        value: T,
        tag: u32,
        wire_type: WireType,
        encode: fn(u32, &B, &mut BytesMut),
        merge: fn(WireType, &mut T, &mut Bytes, DecodeContext) -> Result<(), DecodeError>,
        encoded_len: fn(u32, &B) -> usize,
    ) -> TestCaseResult
    where
        T: Debug + Default + PartialEq + Borrow<B>,
        B: ?Sized,
    {
        prop_assume!((MIN_TAG..=MAX_TAG).contains(&tag));

        let expected_len = encoded_len(tag, value.borrow());

        let mut buf = BytesMut::with_capacity(expected_len);
        encode(tag, value.borrow(), &mut buf);

        let mut buf = buf.freeze();

        prop_assert_eq!(
            buf.remaining(),
            expected_len,
            "encoded_len wrong; expected: {}, actual: {}",
            expected_len,
            buf.remaining()
        );

        if !buf.has_remaining() {
            // Short circuit for empty packed values.
            return Ok(());
        }

        let (decoded_tag, decoded_wire_type) =
            decode_key(&mut buf).map_err(|error| TestCaseError::fail(error.to_string()))?;
        prop_assert_eq!(
            tag,
            decoded_tag,
            "decoded tag does not match; expected: {}, actual: {}",
            tag,
            decoded_tag
        );

        prop_assert_eq!(
            wire_type,
            decoded_wire_type,
            "decoded wire type does not match; expected: {:?}, actual: {:?}",
            wire_type,
            decoded_wire_type,
        );

        match wire_type {
            WireType::SixtyFourBit if buf.remaining() != 8 => Err(TestCaseError::fail(format!(
                "64bit wire type illegal remaining: {}, tag: {}",
                buf.remaining(),
                tag
            ))),
            WireType::ThirtyTwoBit if buf.remaining() != 4 => Err(TestCaseError::fail(format!(
                "32bit wire type illegal remaining: {}, tag: {}",
                buf.remaining(),
                tag
            ))),
            _ => Ok(()),
        }?;

        let mut roundtrip_value = T::default();
        merge(
            wire_type,
            &mut roundtrip_value,
            &mut buf,
            DecodeContext::default(),
        )
        .map_err(|error| TestCaseError::fail(error.to_string()))?;

        prop_assert!(
            !buf.has_remaining(),
            "expected buffer to be empty, remaining: {}",
            buf.remaining()
        );

        prop_assert_eq!(value, roundtrip_value);

        Ok(())
    }

    pub fn check_collection_type<T, B, E, M, L>(
        value: T,
        tag: u32,
        wire_type: WireType,
        encode: E,
        mut merge: M,
        encoded_len: L,
    ) -> TestCaseResult
    where
        T: Debug + Default + PartialEq + Borrow<B>,
        B: ?Sized,
        E: FnOnce(u32, &B, &mut BytesMut),
        M: FnMut(WireType, &mut T, &mut Bytes, DecodeContext) -> Result<(), DecodeError>,
        L: FnOnce(u32, &B) -> usize,
    {
        prop_assume!((MIN_TAG..=MAX_TAG).contains(&tag));

        let expected_len = encoded_len(tag, value.borrow());

        let mut buf = BytesMut::with_capacity(expected_len);
        encode(tag, value.borrow(), &mut buf);

        let mut buf = buf.freeze();

        prop_assert_eq!(
            buf.remaining(),
            expected_len,
            "encoded_len wrong; expected: {}, actual: {}",
            expected_len,
            buf.remaining()
        );

        let mut roundtrip_value = Default::default();
        while buf.has_remaining() {
            let (decoded_tag, decoded_wire_type) =
                decode_key(&mut buf).map_err(|error| TestCaseError::fail(error.to_string()))?;

            prop_assert_eq!(
                tag,
                decoded_tag,
                "decoded tag does not match; expected: {}, actual: {}",
                tag,
                decoded_tag
            );

            prop_assert_eq!(
                wire_type,
                decoded_wire_type,
                "decoded wire type does not match; expected: {:?}, actual: {:?}",
                wire_type,
                decoded_wire_type
            );

            merge(
                wire_type,
                &mut roundtrip_value,
                &mut buf,
                DecodeContext::default(),
            )
            .map_err(|error| TestCaseError::fail(error.to_string()))?;
        }

        prop_assert_eq!(value, roundtrip_value);

        Ok(())
    }

    #[test]
    fn string_merge_invalid_utf8() {
        let mut s = String::new();
        let buf = b"\x02\x80\x80";

        let r = string::merge(
            WireType::LengthDelimited,
            &mut s,
            &mut &buf[..],
            DecodeContext::default(),
        );
        r.expect_err("must be an error");
        assert!(s.is_empty());
    }

    #[test]
    fn varint() {
        fn check(value: u64, encoded: &[u8]) {
            // Small buffer.
            let mut buf = Vec::with_capacity(1);
            encode_varint(value, &mut buf);
            assert_eq!(buf, encoded);

            // Large buffer.
            let mut buf = Vec::with_capacity(100);
            encode_varint(value, &mut buf);
            assert_eq!(buf, encoded);

            assert_eq!(encoded_len_varint(value), encoded.len());

            // See: https://github.com/tokio-rs/prost/pull/1008 for copying reasoning.
            let mut encoded_copy = encoded;
            let roundtrip_value = decode_varint(&mut encoded_copy).expect("decoding failed");
            assert_eq!(value, roundtrip_value);

            let mut encoded_copy = encoded;
            let roundtrip_value =
                decode_varint_slow(&mut encoded_copy).expect("slow decoding failed");
            assert_eq!(value, roundtrip_value);
        }

        check(2u64.pow(0) - 1, &[0x00]);
        check(2u64.pow(0), &[0x01]);

        check(2u64.pow(7) - 1, &[0x7F]);
        check(2u64.pow(7), &[0x80, 0x01]);
        check(300, &[0xAC, 0x02]);

        check(2u64.pow(14) - 1, &[0xFF, 0x7F]);
        check(2u64.pow(14), &[0x80, 0x80, 0x01]);

        check(2u64.pow(21) - 1, &[0xFF, 0xFF, 0x7F]);
        check(2u64.pow(21), &[0x80, 0x80, 0x80, 0x01]);

        check(2u64.pow(28) - 1, &[0xFF, 0xFF, 0xFF, 0x7F]);
        check(2u64.pow(28), &[0x80, 0x80, 0x80, 0x80, 0x01]);

        check(2u64.pow(35) - 1, &[0xFF, 0xFF, 0xFF, 0xFF, 0x7F]);
        check(2u64.pow(35), &[0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);

        check(2u64.pow(42) - 1, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F]);
        check(2u64.pow(42), &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);

        check(
            2u64.pow(49) - 1,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
        );
        check(
            2u64.pow(49),
            &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
        );

        check(
            2u64.pow(56) - 1,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
        );
        check(
            2u64.pow(56),
            &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
        );

        check(
            2u64.pow(63) - 1,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
        );
        check(
            2u64.pow(63),
            &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
        );

        check(
            u64::MAX,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
        );
    }

    const U64_MAX_PLUS_ONE: &[u8] = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x02];

    #[test]
    fn varint_overflow() {
        let mut copy = U64_MAX_PLUS_ONE;
        decode_varint(&mut copy).expect_err("decoding u64::MAX + 1 succeeded");
    }

    #[test]
    fn variant_slow_overflow() {
        let mut copy = U64_MAX_PLUS_ONE;
        decode_varint_slow(&mut copy).expect_err("slow decoding u64::MAX + 1 succeeded");
    }

    /// This big bowl o' macro soup generates an encoding property test for each combination of map
    /// type, scalar map key, and value type.
    /// TODO: these tests take a long time to compile, can this be improved?
    #[cfg(feature = "std")]
    macro_rules! map_tests {
        (keys: $keys:tt,
         vals: $vals:tt) => {
            mod hash_map {
                map_tests!(@private HashMap, hash_map, $keys, $vals);
            }
            mod btree_map {
                map_tests!(@private BTreeMap, btree_map, $keys, $vals);
            }
        };

        (@private $map_type:ident,
                  $mod_name:ident,
                  [$(($key_ty:ty, $key_proto:ident)),*],
                  $vals:tt) => {
            $(
                mod $key_proto {
                    use std::collections::$map_type;

                    use proptest::prelude::*;

                    use crate::encoding::*;
                    use crate::encoding::test::check_collection_type;

                    map_tests!(@private $map_type, $mod_name, ($key_ty, $key_proto), $vals);
                }
            )*
        };

        (@private $map_type:ident,
                  $mod_name:ident,
                  ($key_ty:ty, $key_proto:ident),
                  [$(($val_ty:ty, $val_proto:ident)),*]) => {
            $(
                proptest! {
                    #[test]
                    fn $val_proto(values: $map_type<$key_ty, $val_ty>, tag in MIN_TAG..=MAX_TAG) {
                        check_collection_type(values, tag, WireType::LengthDelimited,
                                              |tag, values, buf| {
                                                  $mod_name::encode($key_proto::encode,
                                                                    $key_proto::encoded_len,
                                                                    $val_proto::encode,
                                                                    $val_proto::encoded_len,
                                                                    tag,
                                                                    values,
                                                                    buf)
                                              },
                                              |wire_type, values, buf, ctx| {
                                                  check_wire_type(WireType::LengthDelimited, wire_type)?;
                                                  $mod_name::merge($key_proto::merge,
                                                                   $val_proto::merge,
                                                                   values,
                                                                   buf,
                                                                   ctx)
                                              },
                                              |tag, values| {
                                                  $mod_name::encoded_len($key_proto::encoded_len,
                                                                         $val_proto::encoded_len,
                                                                         tag,
                                                                         values)
                                              })?;
                    }
                }
             )*
        };
    }

    #[cfg(feature = "std")]
    map_tests!(keys: [
        (i32, int32),
        (i64, int64),
        (u32, uint32),
        (u64, uint64),
        (i32, sint32),
        (i64, sint64),
        (u32, fixed32),
        (u64, fixed64),
        (i32, sfixed32),
        (i64, sfixed64),
        (bool, bool),
        (String, string)
    ],
    vals: [
        (f32, float),
        (f64, double),
        (i32, int32),
        (i64, int64),
        (u32, uint32),
        (u64, uint64),
        (i32, sint32),
        (i64, sint64),
        (u32, fixed32),
        (u64, fixed64),
        (i32, sfixed32),
        (i64, sfixed64),
        (bool, bool),
        (String, string),
        (Vec<u8>, bytes)
    ]);
}
