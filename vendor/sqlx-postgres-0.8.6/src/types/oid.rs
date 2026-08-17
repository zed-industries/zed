use byteorder::{BigEndian, ByteOrder};
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};

/// The PostgreSQL [`OID`] type stores an object identifier,
/// used internally by PostgreSQL as primary keys for various system tables.
///
/// [`OID`]: https://www.postgresql.org/docs/current/datatype-oid.html
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct Oid(
    /// The raw unsigned integer value sent over the wire
    pub u32,
);

impl Type<Postgres> for Oid {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::OID
    }
}

impl PgHasArrayType for Oid {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::OID_ARRAY
    }
}

impl Encode<'_, Postgres> for Oid {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.0.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Decode<'_, Postgres> for Oid {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(Self(match value.format() {
            PgValueFormat::Binary => BigEndian::read_u32(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        }))
    }
}

impl Serialize for Oid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Oid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        u32::deserialize(deserializer).map(Self)
    }
}
