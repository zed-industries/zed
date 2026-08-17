use uuid::{
    fmt::{Hyphenated, Simple},
    Uuid,
};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::io::MySqlBufMutExt;
use crate::types::Type;
use crate::{MySql, MySqlTypeInfo, MySqlValueRef};

impl Type<MySql> for Uuid {
    fn type_info() -> MySqlTypeInfo {
        <&[u8] as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <&[u8] as Type<MySql>>::compatible(ty)
    }
}

impl Encode<'_, MySql> for Uuid {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.put_bytes_lenenc(self.as_bytes());

        Ok(IsNull::No)
    }
}

impl Decode<'_, MySql> for Uuid {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        // delegate to the &[u8] type to decode from MySQL
        let bytes = <&[u8] as Decode<MySql>>::decode(value)?;

        if bytes.len() != 16 {
            return Err(format!(
                "Expected 16 bytes, got {}; `Uuid` uses binary format for MySQL/MariaDB. \
                 For text-formatted UUIDs, use `uuid::fmt::Hyphenated` instead of `Uuid`.",
                bytes.len(),
            )
            .into());
        }

        // construct a Uuid from the returned bytes
        Uuid::from_slice(bytes).map_err(Into::into)
    }
}

impl Type<MySql> for Hyphenated {
    fn type_info() -> MySqlTypeInfo {
        <&str as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <&str as Type<MySql>>::compatible(ty)
    }
}

impl Encode<'_, MySql> for Hyphenated {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.put_str_lenenc(&self.to_string());

        Ok(IsNull::No)
    }
}

impl Decode<'_, MySql> for Hyphenated {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        // delegate to the &str type to decode from MySQL
        let text = <&str as Decode<MySql>>::decode(value)?;

        // parse a UUID from the text
        Uuid::parse_str(text)
            .map_err(Into::into)
            .map(|u| u.hyphenated())
    }
}

impl Type<MySql> for Simple {
    fn type_info() -> MySqlTypeInfo {
        <&str as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <&str as Type<MySql>>::compatible(ty)
    }
}

impl Encode<'_, MySql> for Simple {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.put_str_lenenc(&self.to_string());

        Ok(IsNull::No)
    }
}

impl Decode<'_, MySql> for Simple {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        // delegate to the &str type to decode from MySQL
        let text = <&str as Decode<MySql>>::decode(value)?;

        // parse a UUID from the text
        Uuid::parse_str(text)
            .map_err(Into::into)
            .map(|u| u.simple())
    }
}
