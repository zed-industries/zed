use byteorder::{ByteOrder, LittleEndian};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::protocol::text::{ColumnFlags, ColumnType};
use crate::types::Type;
use crate::{MySql, MySqlTypeInfo, MySqlValueFormat, MySqlValueRef};

fn int_compatible(ty: &MySqlTypeInfo) -> bool {
    matches!(
        ty.r#type,
        ColumnType::Tiny
            | ColumnType::Short
            | ColumnType::Long
            | ColumnType::Int24
            | ColumnType::LongLong
    ) && !ty.flags.contains(ColumnFlags::UNSIGNED)
}

impl Type<MySql> for i8 {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Tiny)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        int_compatible(ty)
    }
}

impl Type<MySql> for i16 {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Short)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        int_compatible(ty)
    }
}

impl Type<MySql> for i32 {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Long)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        int_compatible(ty)
    }
}

impl Type<MySql> for i64 {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::LongLong)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        int_compatible(ty)
    }
}

impl Encode<'_, MySql> for i8 {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.to_le_bytes());

        Ok(IsNull::No)
    }
}

impl Encode<'_, MySql> for i16 {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.to_le_bytes());

        Ok(IsNull::No)
    }
}

impl Encode<'_, MySql> for i32 {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.to_le_bytes());

        Ok(IsNull::No)
    }
}

impl Encode<'_, MySql> for i64 {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.to_le_bytes());

        Ok(IsNull::No)
    }
}

fn int_decode(value: MySqlValueRef<'_>) -> Result<i64, BoxDynError> {
    Ok(match value.format() {
        MySqlValueFormat::Text => value.as_str()?.parse()?,
        MySqlValueFormat::Binary => {
            let buf = value.as_bytes()?;

            // Check conditions that could cause `read_int()` to panic.
            if buf.is_empty() {
                return Err("empty buffer".into());
            }

            if buf.len() > 8 {
                return Err(format!(
                    "expected no more than 8 bytes for integer value, got {}",
                    buf.len()
                )
                .into());
            }

            LittleEndian::read_int(buf, buf.len())
        }
    })
}

impl Decode<'_, MySql> for i8 {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        int_decode(value)?.try_into().map_err(Into::into)
    }
}

impl Decode<'_, MySql> for i16 {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        int_decode(value)?.try_into().map_err(Into::into)
    }
}

impl Decode<'_, MySql> for i32 {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        int_decode(value)?.try_into().map_err(Into::into)
    }
}

impl Decode<'_, MySql> for i64 {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        int_decode(value)
    }
}
