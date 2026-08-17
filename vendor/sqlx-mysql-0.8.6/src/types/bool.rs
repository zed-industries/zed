use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{
    protocol::text::{ColumnFlags, ColumnType},
    MySql, MySqlTypeInfo, MySqlValueRef,
};

impl Type<MySql> for bool {
    fn type_info() -> MySqlTypeInfo {
        // MySQL has no actual `BOOLEAN` type, the type is an alias of `TINYINT(1)`
        MySqlTypeInfo {
            flags: ColumnFlags::BINARY | ColumnFlags::UNSIGNED,
            max_size: Some(1),
            r#type: ColumnType::Tiny,
        }
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        matches!(
            ty.r#type,
            ColumnType::Tiny
                | ColumnType::Short
                | ColumnType::Long
                | ColumnType::Int24
                | ColumnType::LongLong
                | ColumnType::Bit
        )
    }
}

impl Encode<'_, MySql> for bool {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        <i8 as Encode<MySql>>::encode(*self as i8, buf)
    }
}

impl Decode<'_, MySql> for bool {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(<i8 as Decode<MySql>>::decode(value)? != 0)
    }
}
