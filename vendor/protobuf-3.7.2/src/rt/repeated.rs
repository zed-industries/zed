use crate::CodedInputStream;
use crate::Enum;
use crate::EnumOrUnknown;

/// Read repeated enum field when the wire format is length-delimited.
pub fn read_repeated_packed_enum_or_unknown_into<E: Enum>(
    is: &mut CodedInputStream,
    target: &mut Vec<EnumOrUnknown<E>>,
) -> crate::Result<()> {
    let len = is.read_raw_varint64()?;
    let old_limit = is.push_limit(len)?;
    while !is.eof()? {
        target.push(is.read_enum_or_unknown()?);
    }
    is.pop_limit(old_limit);
    Ok(())
}
