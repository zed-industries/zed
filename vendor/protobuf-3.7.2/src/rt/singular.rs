use crate::rt::compute_raw_varint64_size;
use crate::rt::tag_size;
use crate::varint::generic::ProtobufVarint;
use crate::zigzag::ProtobufVarintZigzag;

/// Integer value size when encoded.
#[inline]
fn varint_size<T: ProtobufVarint>(field_number: u32, value: T) -> u64 {
    tag_size(field_number) + value.len_varint()
}

/// Encoded `int32` size.
#[inline]
pub fn int32_size(field_number: u32, value: i32) -> u64 {
    varint_size(field_number, value)
}

/// Encoded `int64` size.
#[inline]
pub fn int64_size(field_number: u32, value: i64) -> u64 {
    varint_size(field_number, value)
}

/// Encoded `uint32` size.
#[inline]
pub fn uint32_size(field_number: u32, value: u32) -> u64 {
    varint_size(field_number, value)
}

/// Encoded `uint64` size.
#[inline]
pub fn uint64_size(field_number: u32, value: u64) -> u64 {
    varint_size(field_number, value)
}

/// Integer value size when encoded as specified wire type.
pub(crate) fn value_varint_zigzag_size_no_tag<T: ProtobufVarintZigzag>(value: T) -> u64 {
    value.len_varint_zigzag()
}

/// Length of value when encoding with zigzag encoding with tag
#[inline]
fn value_varint_zigzag_size<T: ProtobufVarintZigzag>(field_number: u32, value: T) -> u64 {
    tag_size(field_number) + value_varint_zigzag_size_no_tag(value)
}

/// Size of serialized `sint32` field.
#[inline]
pub fn sint32_size(field_number: u32, value: i32) -> u64 {
    value_varint_zigzag_size(field_number, value)
}

/// Size of serialized `sint64` field.
#[inline]
pub fn sint64_size(field_number: u32, value: i64) -> u64 {
    value_varint_zigzag_size(field_number, value)
}

/// Size of encoded bytes field.
pub(crate) fn bytes_size_no_tag(bytes: &[u8]) -> u64 {
    compute_raw_varint64_size(bytes.len() as u64) + bytes.len() as u64
}

/// Size of encoded bytes field.
#[inline]
pub fn bytes_size(field_number: u32, bytes: &[u8]) -> u64 {
    tag_size(field_number) + bytes_size_no_tag(bytes)
}

/// Size of encoded string field.
pub(crate) fn string_size_no_tag(s: &str) -> u64 {
    bytes_size_no_tag(s.as_bytes())
}

/// Size of encoded string field.
#[inline]
pub fn string_size(field_number: u32, s: &str) -> u64 {
    tag_size(field_number) + string_size_no_tag(s)
}
