use bigdecimal::BigDecimal;

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::io::MySqlBufMutExt;
use crate::protocol::text::ColumnType;
use crate::types::Type;
use crate::{MySql, MySqlTypeInfo, MySqlValueRef};

impl Type<MySql> for BigDecimal {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::NewDecimal)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        matches!(ty.r#type, ColumnType::Decimal | ColumnType::NewDecimal)
    }
}

impl Encode<'_, MySql> for BigDecimal {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.put_str_lenenc(&self.to_string());

        Ok(IsNull::No)
    }
}

impl Decode<'_, MySql> for BigDecimal {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(value.as_str()?.parse()?)
    }
}
