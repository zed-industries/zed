use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::io::MySqlBufMutExt;
use crate::types::Type;
use crate::{MySql, MySqlTypeInfo, MySqlValueRef};

impl Type<MySql> for Ipv4Addr {
    fn type_info() -> MySqlTypeInfo {
        <&str as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <&str as Type<MySql>>::compatible(ty)
    }
}

impl Encode<'_, MySql> for Ipv4Addr {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.put_str_lenenc(&self.to_string());

        Ok(IsNull::No)
    }
}

impl Decode<'_, MySql> for Ipv4Addr {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        // delegate to the &str type to decode from MySQL
        let text = <&str as Decode<MySql>>::decode(value)?;

        // parse a Ipv4Addr from the text
        text.parse().map_err(Into::into)
    }
}

impl Type<MySql> for Ipv6Addr {
    fn type_info() -> MySqlTypeInfo {
        <&str as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <&str as Type<MySql>>::compatible(ty)
    }
}

impl Encode<'_, MySql> for Ipv6Addr {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.put_str_lenenc(&self.to_string());

        Ok(IsNull::No)
    }
}

impl Decode<'_, MySql> for Ipv6Addr {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        // delegate to the &str type to decode from MySQL
        let text = <&str as Decode<MySql>>::decode(value)?;

        // parse a Ipv6Addr from the text
        text.parse().map_err(Into::into)
    }
}

impl Type<MySql> for IpAddr {
    fn type_info() -> MySqlTypeInfo {
        <&str as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <&str as Type<MySql>>::compatible(ty)
    }
}

impl Encode<'_, MySql> for IpAddr {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.put_str_lenenc(&self.to_string());

        Ok(IsNull::No)
    }
}

impl Decode<'_, MySql> for IpAddr {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        // delegate to the &str type to decode from MySQL
        let text = <&str as Decode<MySql>>::decode(value)?;

        // parse a IpAddr from the text
        text.parse().map_err(Into::into)
    }
}
