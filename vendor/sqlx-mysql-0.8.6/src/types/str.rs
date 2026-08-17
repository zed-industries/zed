use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::io::MySqlBufMutExt;
use crate::protocol::text::{ColumnFlags, ColumnType};
use crate::types::Type;
use crate::{MySql, MySqlTypeInfo, MySqlValueRef};
use std::borrow::Cow;

impl Type<MySql> for str {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo {
            r#type: ColumnType::VarString, // VARCHAR
            flags: ColumnFlags::empty(),
            max_size: None,
        }
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        // TODO: Support more collations being returned from SQL?
        matches!(
            ty.r#type,
            ColumnType::VarChar
                | ColumnType::Blob
                | ColumnType::TinyBlob
                | ColumnType::MediumBlob
                | ColumnType::LongBlob
                | ColumnType::String
                | ColumnType::VarString
                | ColumnType::Enum
        ) && !ty.flags.contains(ColumnFlags::BINARY)
    }
}

impl Encode<'_, MySql> for &'_ str {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.put_str_lenenc(self);

        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, MySql> for &'r str {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        value.as_str()
    }
}

impl Type<MySql> for Box<str> {
    fn type_info() -> MySqlTypeInfo {
        <&str as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <&str as Type<MySql>>::compatible(ty)
    }
}

impl Encode<'_, MySql> for Box<str> {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        <&str as Encode<MySql>>::encode(&**self, buf)
    }
}

impl<'r> Decode<'r, MySql> for Box<str> {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        <&str as Decode<MySql>>::decode(value).map(Box::from)
    }
}

impl Type<MySql> for String {
    fn type_info() -> MySqlTypeInfo {
        <str as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <str as Type<MySql>>::compatible(ty)
    }
}

impl Encode<'_, MySql> for String {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        <&str as Encode<MySql>>::encode(&**self, buf)
    }
}

impl Decode<'_, MySql> for String {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        <&str as Decode<MySql>>::decode(value).map(ToOwned::to_owned)
    }
}

impl Type<MySql> for Cow<'_, str> {
    fn type_info() -> MySqlTypeInfo {
        <&str as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <&str as Type<MySql>>::compatible(ty)
    }
}

impl Encode<'_, MySql> for Cow<'_, str> {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        match self {
            Cow::Borrowed(str) => <&str as Encode<MySql>>::encode(*str, buf),
            Cow::Owned(str) => <&str as Encode<MySql>>::encode(&**str, buf),
        }
    }
}

impl<'r> Decode<'r, MySql> for Cow<'r, str> {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        value.as_str().map(Cow::Borrowed)
    }
}
