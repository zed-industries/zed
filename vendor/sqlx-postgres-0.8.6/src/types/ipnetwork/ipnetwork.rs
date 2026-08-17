use std::net::{Ipv4Addr, Ipv6Addr};

use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};

// https://github.com/postgres/postgres/blob/574925bfd0a8175f6e161936ea11d9695677ba09/src/include/utils/inet.h#L39

// Technically this is a magic number here but it doesn't make sense to drag in the whole of `libc`
// just for one constant.
const PGSQL_AF_INET: u8 = 2; // AF_INET
const PGSQL_AF_INET6: u8 = PGSQL_AF_INET + 1;

impl Type<Postgres> for IpNetwork {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::INET
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        *ty == PgTypeInfo::CIDR || *ty == PgTypeInfo::INET
    }
}

impl PgHasArrayType for IpNetwork {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::INET_ARRAY
    }

    fn array_compatible(ty: &PgTypeInfo) -> bool {
        *ty == PgTypeInfo::CIDR_ARRAY || *ty == PgTypeInfo::INET_ARRAY
    }
}

impl Encode<'_, Postgres> for IpNetwork {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        // https://github.com/postgres/postgres/blob/574925bfd0a8175f6e161936ea11d9695677ba09/src/backend/utils/adt/network.c#L293
        // https://github.com/postgres/postgres/blob/574925bfd0a8175f6e161936ea11d9695677ba09/src/backend/utils/adt/network.c#L271

        match self {
            IpNetwork::V4(net) => {
                buf.push(PGSQL_AF_INET); // ip_family
                buf.push(net.prefix()); // ip_bits
                buf.push(0); // is_cidr
                buf.push(4); // nb (number of bytes)
                buf.extend_from_slice(&net.ip().octets()) // address
            }

            IpNetwork::V6(net) => {
                buf.push(PGSQL_AF_INET6); // ip_family
                buf.push(net.prefix()); // ip_bits
                buf.push(0); // is_cidr
                buf.push(16); // nb (number of bytes)
                buf.extend_from_slice(&net.ip().octets()); // address
            }
        }

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        match self {
            IpNetwork::V4(_) => 8,
            IpNetwork::V6(_) => 20,
        }
    }
}

impl Decode<'_, Postgres> for IpNetwork {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let bytes = match value.format() {
            PgValueFormat::Binary => value.as_bytes()?,
            PgValueFormat::Text => {
                return Ok(value.as_str()?.parse()?);
            }
        };

        if bytes.len() >= 8 {
            let family = bytes[0];
            let prefix = bytes[1];
            let _is_cidr = bytes[2] != 0;
            let len = bytes[3];

            match family {
                PGSQL_AF_INET => {
                    if bytes.len() == 8 && len == 4 {
                        let inet = Ipv4Network::new(
                            Ipv4Addr::new(bytes[4], bytes[5], bytes[6], bytes[7]),
                            prefix,
                        )?;

                        return Ok(IpNetwork::V4(inet));
                    }
                }

                PGSQL_AF_INET6 => {
                    if bytes.len() == 20 && len == 16 {
                        let inet = Ipv6Network::new(
                            Ipv6Addr::from([
                                bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9],
                                bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
                                bytes[16], bytes[17], bytes[18], bytes[19],
                            ]),
                            prefix,
                        )?;

                        return Ok(IpNetwork::V6(inet));
                    }
                }

                _ => {
                    return Err(format!("unknown ip family {family}").into());
                }
            }
        }

        Err("invalid data received when expecting an INET".into())
    }
}
