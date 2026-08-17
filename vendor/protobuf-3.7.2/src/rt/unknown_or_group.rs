use crate::rt::compute_raw_varint64_size;
use crate::rt::singular::bytes_size_no_tag;
use crate::rt::tag_size;
use crate::wire_format::Tag;
use crate::wire_format::WireType;
use crate::CodedInputStream;
use crate::UnknownFields;
use crate::UnknownValueRef;

fn skip_group(is: &mut CodedInputStream) -> crate::Result<()> {
    loop {
        let (_, wire_type) = is.read_tag_unpack()?;
        if wire_type == WireType::EndGroup {
            return Ok(());
        }
        is.skip_field(wire_type)?;
    }
}

/// Size of encoded unknown fields size.
pub fn unknown_fields_size(unknown_fields: &UnknownFields) -> u64 {
    let mut r = 0;
    for (number, value) in unknown_fields {
        r += tag_size(number);
        r += match value {
            UnknownValueRef::Fixed32(_) => 4,
            UnknownValueRef::Fixed64(_) => 8,
            UnknownValueRef::Varint(v) => compute_raw_varint64_size(v),
            UnknownValueRef::LengthDelimited(v) => bytes_size_no_tag(v),
        };
    }
    r
}

/// Handle unknown field in generated code.
/// Either store a value in unknown, or skip a group.
pub(crate) fn read_unknown_or_skip_group_with_tag_unpacked(
    field_number: u32,
    wire_type: WireType,
    is: &mut CodedInputStream,
    unknown_fields: &mut UnknownFields,
) -> crate::Result<()> {
    match wire_type {
        WireType::StartGroup => skip_group(is),
        _ => {
            let unknown = is.read_unknown(wire_type)?;
            unknown_fields.add_value(field_number, unknown);
            Ok(())
        }
    }
}

/// Handle unknown field in generated code.
/// Either store a value in unknown, or skip a group.
/// Return error if tag is incorrect.
pub fn read_unknown_or_skip_group(
    tag: u32,
    is: &mut CodedInputStream,
    unknown_fields: &mut UnknownFields,
) -> crate::Result<()> {
    let (field_humber, wire_type) = Tag::new(tag)?.unpack();
    read_unknown_or_skip_group_with_tag_unpacked(field_humber, wire_type, is, unknown_fields)
}

/// Skip field.
pub fn skip_field_for_tag(tag: u32, is: &mut CodedInputStream) -> crate::Result<()> {
    let (_field_humber, wire_type) = Tag::new(tag)?.unpack();
    is.skip_field(wire_type)
}
