use mac_address::MacAddress;

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};

impl Type<Postgres> for MacAddress {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::MACADDR
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        *ty == PgTypeInfo::MACADDR
    }
}

impl PgHasArrayType for MacAddress {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::MACADDR_ARRAY
    }
}

impl Encode<'_, Postgres> for MacAddress {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        buf.extend_from_slice(&self.bytes()); // write just the address
        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        6
    }
}

impl Decode<'_, Postgres> for MacAddress {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let bytes = match value.format() {
            PgValueFormat::Binary => value.as_bytes()?,
            PgValueFormat::Text => {
                return Ok(value.as_str()?.parse()?);
            }
        };

        if bytes.len() == 6 {
            return Ok(MacAddress::new(bytes.try_into().unwrap()));
        }

        Err("invalid data received when expecting an MACADDR".into())
    }
}
