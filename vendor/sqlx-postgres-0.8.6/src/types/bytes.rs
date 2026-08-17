use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};

impl PgHasArrayType for u8 {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::BYTEA
    }
}

impl PgHasArrayType for &'_ [u8] {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::BYTEA_ARRAY
    }
}

impl PgHasArrayType for Box<[u8]> {
    fn array_type_info() -> PgTypeInfo {
        <[&[u8]] as Type<Postgres>>::type_info()
    }
}

impl PgHasArrayType for Vec<u8> {
    fn array_type_info() -> PgTypeInfo {
        <[&[u8]] as Type<Postgres>>::type_info()
    }
}

impl<const N: usize> PgHasArrayType for [u8; N] {
    fn array_type_info() -> PgTypeInfo {
        <[&[u8]] as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for &'_ [u8] {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        buf.extend_from_slice(self);

        Ok(IsNull::No)
    }
}

impl Encode<'_, Postgres> for Box<[u8]> {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <&[u8] as Encode<Postgres>>::encode(self.as_ref(), buf)
    }
}

impl Encode<'_, Postgres> for Vec<u8> {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <&[u8] as Encode<Postgres>>::encode(self, buf)
    }
}

impl<const N: usize> Encode<'_, Postgres> for [u8; N] {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <&[u8] as Encode<Postgres>>::encode(self.as_slice(), buf)
    }
}

impl<'r> Decode<'r, Postgres> for &'r [u8] {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.format() {
            PgValueFormat::Binary => value.as_bytes(),
            PgValueFormat::Text => {
                Err("unsupported decode to `&[u8]` of BYTEA in a simple query; use a prepared query or decode to `Vec<u8>`".into())
            }
        }
    }
}

fn text_hex_decode_input(value: PgValueRef<'_>) -> Result<&[u8], BoxDynError> {
    // BYTEA is formatted as \x followed by hex characters
    value
        .as_bytes()?
        .strip_prefix(b"\\x")
        .ok_or("text does not start with \\x")
        .map_err(Into::into)
}

impl Decode<'_, Postgres> for Box<[u8]> {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            PgValueFormat::Binary => Box::from(value.as_bytes()?),
            PgValueFormat::Text => Box::from(hex::decode(text_hex_decode_input(value)?)?),
        })
    }
}

impl Decode<'_, Postgres> for Vec<u8> {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            PgValueFormat::Binary => value.as_bytes()?.to_owned(),
            PgValueFormat::Text => hex::decode(text_hex_decode_input(value)?)?,
        })
    }
}

impl<const N: usize> Decode<'_, Postgres> for [u8; N] {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let mut bytes = [0u8; N];
        match value.format() {
            PgValueFormat::Binary => {
                bytes = value.as_bytes()?.try_into()?;
            }
            PgValueFormat::Text => hex::decode_to_slice(text_hex_decode_input(value)?, &mut bytes)?,
        };
        Ok(bytes)
    }
}
