use std::net::IpAddr;

use ipnet::IpNet;

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef, Postgres};

impl Type<Postgres> for IpAddr
where
    IpNet: Type<Postgres>,
{
    fn type_info() -> PgTypeInfo {
        IpNet::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        IpNet::compatible(ty)
    }
}

impl PgHasArrayType for IpAddr {
    fn array_type_info() -> PgTypeInfo {
        <IpNet as PgHasArrayType>::array_type_info()
    }

    fn array_compatible(ty: &PgTypeInfo) -> bool {
        <IpNet as PgHasArrayType>::array_compatible(ty)
    }
}

impl<'db> Encode<'db, Postgres> for IpAddr
where
    IpNet: Encode<'db, Postgres>,
{
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        IpNet::from(*self).encode_by_ref(buf)
    }

    fn size_hint(&self) -> usize {
        IpNet::from(*self).size_hint()
    }
}

impl<'db> Decode<'db, Postgres> for IpAddr
where
    IpNet: Decode<'db, Postgres>,
{
    fn decode(value: PgValueRef<'db>) -> Result<Self, BoxDynError> {
        let ipnet = IpNet::decode(value)?;

        if matches!(ipnet, IpNet::V4(net) if net.prefix_len() != 32)
            || matches!(ipnet, IpNet::V6(net) if net.prefix_len() != 128)
        {
            Err("lossy decode from inet/cidr")?
        }

        Ok(ipnet.addr())
    }
}
