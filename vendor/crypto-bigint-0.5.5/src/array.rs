//! Interop support for `generic-array`

use crate::{Encoding, Integer};
use core::ops::Add;
use generic_array::{typenum::Unsigned, ArrayLength, GenericArray};

/// Alias for a byte array whose size is defined by [`ArrayEncoding::ByteSize`].
pub type ByteArray<T> = GenericArray<u8, <T as ArrayEncoding>::ByteSize>;

/// Support for encoding a big integer as a `GenericArray`.
pub trait ArrayEncoding: Encoding {
    /// Size of a byte array which encodes a big integer.
    type ByteSize: ArrayLength<u8> + Add + Eq + Ord + Unsigned;

    /// Deserialize from a big-endian byte array.
    fn from_be_byte_array(bytes: ByteArray<Self>) -> Self;

    /// Deserialize from a little-endian byte array.
    fn from_le_byte_array(bytes: ByteArray<Self>) -> Self;

    /// Serialize to a big-endian byte array.
    fn to_be_byte_array(&self) -> ByteArray<Self>;

    /// Serialize to a little-endian byte array.
    fn to_le_byte_array(&self) -> ByteArray<Self>;
}

/// Support for decoding a `GenericArray` as a big integer.
pub trait ArrayDecoding {
    /// Big integer which decodes a `GenericArray`.
    type Output: ArrayEncoding + Integer;

    /// Deserialize from a big-endian `GenericArray`.
    fn into_uint_be(self) -> Self::Output;

    /// Deserialize from a little-endian `GenericArray`.
    fn into_uint_le(self) -> Self::Output;
}
