//! # Functions and types used by generated protobuf code
//!
//! These are not considered to be public API of rust-protobuf,
//! so they can be changed any time (provided compatibility with
//! previously generated code is preserved).

pub(crate) mod map;
mod message;
pub(crate) mod packed;
pub(crate) mod repeated;
pub(crate) mod singular;
pub(crate) mod unknown_or_group;

pub use message::read_singular_message_into_field;
pub use message::write_message_field_with_cached_size;
pub use packed::vec_packed_bool_size;
pub use packed::vec_packed_double_size;
pub use packed::vec_packed_enum_or_unknown_size;
pub use packed::vec_packed_fixed32_size;
pub use packed::vec_packed_fixed64_size;
pub use packed::vec_packed_float_size;
pub use packed::vec_packed_int32_size;
pub use packed::vec_packed_int64_size;
pub use packed::vec_packed_sfixed32_size;
pub use packed::vec_packed_sfixed64_size;
pub use packed::vec_packed_sint32_size;
pub use packed::vec_packed_sint64_size;
pub use packed::vec_packed_uint32_size;
pub use packed::vec_packed_uint64_size;
pub use repeated::read_repeated_packed_enum_or_unknown_into;
pub use singular::bytes_size;
pub use singular::int32_size;
pub use singular::int64_size;
pub use singular::sint32_size;
pub use singular::sint64_size;
pub use singular::string_size;
pub use singular::uint32_size;
pub use singular::uint64_size;
pub use unknown_or_group::read_unknown_or_skip_group;
pub use unknown_or_group::skip_field_for_tag;
pub use unknown_or_group::unknown_fields_size;

pub use crate::cached_size::CachedSize;
pub use crate::lazy::Lazy;
use crate::varint::encode::encoded_varint64_len;
pub use crate::wire_format::WireType;

/// Given `u64` value compute varint encoded length.
pub fn compute_raw_varint64_size(value: u64) -> u64 {
    encoded_varint64_len(value) as u64
}

/// Given `u32` value compute varint encoded length.
pub(crate) fn compute_raw_varint32_size(value: u32) -> u64 {
    compute_raw_varint64_size(value as u64)
}

/// Compute tag size. Size of tag does not depend on wire type.
#[inline]
pub fn tag_size(field_number: u32) -> u64 {
    encoded_varint64_len((field_number as u64) << 3) as u64
}
