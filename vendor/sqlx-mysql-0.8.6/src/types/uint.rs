use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::protocol::text::{ColumnFlags, ColumnType};
use crate::types::Type;
use crate::{MySql, MySqlTypeInfo, MySqlValueFormat, MySqlValueRef};
use byteorder::{ByteOrder, LittleEndian};

fn uint_type_info(ty: ColumnType) -> MySqlTypeInfo {
    MySqlTypeInfo {
        r#type: ty,
        flags: ColumnFlags::BINARY | ColumnFlags::UNSIGNED,
        max_size: None,
    }
}

fn uint_compatible(ty: &MySqlTypeInfo) -> bool {
    matches!(
        ty.r#type,
        ColumnType::Tiny
            | ColumnType::Short
            | ColumnType::Long
            | ColumnType::Int24
            | ColumnType::LongLong
            | ColumnType::Year
            | ColumnType::Bit
    ) && ty.flags.contains(ColumnFlags::UNSIGNED)
}

impl Type<MySql> for u8 {
    fn type_info() -> MySqlTypeInfo {
        uint_type_info(ColumnType::Tiny)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        uint_compatible(ty)
    }
}

impl Type<MySql> for u16 {
    fn type_info() -> MySqlTypeInfo {
        uint_type_info(ColumnType::Short)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        uint_compatible(ty)
    }
}

impl Type<MySql> for u32 {
    fn type_info() -> MySqlTypeInfo {
        uint_type_info(ColumnType::Long)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        uint_compatible(ty)
    }
}

impl Type<MySql> for u64 {
    fn type_info() -> MySqlTypeInfo {
        uint_type_info(ColumnType::LongLong)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        uint_compatible(ty)
    }
}

impl Encode<'_, MySql> for u8 {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.to_le_bytes());

        Ok(IsNull::No)
    }
}

impl Encode<'_, MySql> for u16 {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.to_le_bytes());

        Ok(IsNull::No)
    }
}

impl Encode<'_, MySql> for u32 {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.to_le_bytes());

        Ok(IsNull::No)
    }
}

impl Encode<'_, MySql> for u64 {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.to_le_bytes());

        Ok(IsNull::No)
    }
}

fn uint_decode(value: MySqlValueRef<'_>) -> Result<u64, BoxDynError> {
    if value.type_info.r#type == ColumnType::Bit {
        // NOTE: Regardless of the value format, there is raw binary data here

        let buf = value.as_bytes()?;
        let mut value: u64 = 0;

        for b in buf {
            value = (*b as u64) | (value << 8);
        }

        return Ok(value);
    }

    Ok(match value.format() {
        MySqlValueFormat::Text => value.as_str()?.parse()?,

        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;

            // Check conditions that could cause `read_uint()` to panic.
            if buf.is_empty() {
                return Err("empty buffer".into());
            }

            if buf.len() > 8 {
                return Err(format!(
                    "expected no more than 8 bytes for unsigned integer value, got {}",
                    buf.len()
                )
                .into());
            }

            LittleEndian::read_uint(buf, buf.len())
        }
    })
}

impl Decode<'_, MySql> for u8 {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        uint_decode(value)?.try_into().map_err(Into::into)
    }
}

impl Decode<'_, MySql> for u16 {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        uint_decode(value)?.try_into().map_err(Into::into)
    }
}

impl Decode<'_, MySql> for u32 {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        uint_decode(value)?.try_into().map_err(Into::into)
    }
}

impl Decode<'_, MySql> for u64 {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        uint_decode(value)
    }
}
