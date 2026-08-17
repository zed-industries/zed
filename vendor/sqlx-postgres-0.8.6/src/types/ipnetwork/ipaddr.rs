use std::net::IpAddr;

use ipnetwork::IpNetwork;

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef, Postgres};

impl Type<Postgres> for IpAddr
where
    IpNetwork: Type<Postgres>,
{
    fn type_info() -> PgTypeInfo {
        IpNetwork::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        IpNetwork::compatible(ty)
    }
}

impl PgHasArrayType for IpAddr {
    fn array_type_info() -> PgTypeInfo {
        <IpNetwork as PgHasArrayType>::array_type_info()
    }

    fn array_compatible(ty: &PgTypeInfo) -> bool {
        <IpNetwork as PgHasArrayType>::array_compatible(ty)
    }
}

impl<'db> Encode<'db, Postgres> for IpAddr
where
    IpNetwork: Encode<'db, Postgres>,
{
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        IpNetwork::from(*self).encode_by_ref(buf)
    }

    fn size_hint(&self) -> usize {
        IpNetwork::from(*self).size_hint()
    }
}

impl<'db> Decode<'db, Postgres> for IpAddr
where
    IpNetwork: Decode<'db, Postgres>,
{
    fn decode(value: PgValueRef<'db>) -> Result<Self, BoxDynError> {
        let ipnetwork = IpNetwork::decode(value)?;

        if ipnetwork.is_ipv4() && ipnetwork.prefix() != 32
            || ipnetwork.is_ipv6() && ipnetwork.prefix() != 128
        {
            Err("lossy decode from inet/cidr")?
        }

        Ok(ipnetwork.ip())
    }
}
