use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};
use sqlx_core::bytes::Buf;
use sqlx_core::Error;
use std::str::FromStr;

/// ## Postgres Geometric Point type
///
/// Description: Point on a plane
/// Representation: `(x, y)`
///
/// Points are the fundamental two-dimensional building block for geometric types. Values of type point are specified using either of the following syntaxes:
/// ```text
/// ( x , y )
///  x , y
/// ````
/// where x and y are the respective coordinates, as floating-point numbers.
///
/// See [Postgres Manual, Section 8.8.1, Geometric Types - Points][PG.S.8.8.1] for details.
///
/// [PG.S.8.8.1]: https://www.postgresql.org/docs/current/datatype-geometric.html#DATATYPE-GEOMETRIC-POINTS
///
#[derive(Debug, Clone, PartialEq)]
pub struct PgPoint {
    pub x: f64,
    pub y: f64,
}

impl Type<Postgres> for PgPoint {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("point")
    }
}

impl PgHasArrayType for PgPoint {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_point")
    }
}

impl<'r> Decode<'r, Postgres> for PgPoint {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match value.format() {
            PgValueFormat::Text => Ok(PgPoint::from_str(value.as_str()?)?),
            PgValueFormat::Binary => Ok(PgPoint::from_bytes(value.as_bytes()?)?),
        }
    }
}

impl<'q> Encode<'q, Postgres> for PgPoint {
    fn produces(&self) -> Option<PgTypeInfo> {
        Some(PgTypeInfo::with_name("point"))
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        self.serialize(buf)?;
        Ok(IsNull::No)
    }
}

fn parse_float_from_str(s: &str, error_msg: &str) -> Result<f64, Error> {
    s.trim()
        .parse()
        .map_err(|_| Error::Decode(error_msg.into()))
}

impl FromStr for PgPoint {
    type Err = BoxDynError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s
            .trim_matches(|c| c == '(' || c == ')' || c == ' ')
            .split_once(',')
            .ok_or_else(|| format!("error decoding POINT: could not get x and y from {}", s))?;

        let x = parse_float_from_str(x_str, "error decoding POINT: could not get x")?;
        let y = parse_float_from_str(y_str, "error decoding POINT: could not get y")?;

        Ok(PgPoint { x, y })
    }
}

impl PgPoint {
    fn from_bytes(mut bytes: &[u8]) -> Result<PgPoint, BoxDynError> {
        let x = bytes.get_f64();
        let y = bytes.get_f64();
        Ok(PgPoint { x, y })
    }

    fn serialize(&self, buff: &mut PgArgumentBuffer) -> Result<(), BoxDynError> {
        buff.extend_from_slice(&self.x.to_be_bytes());
        buff.extend_from_slice(&self.y.to_be_bytes());
        Ok(())
    }

    #[cfg(test)]
    fn serialize_to_vec(&self) -> Vec<u8> {
        let mut buff = PgArgumentBuffer::default();
        self.serialize(&mut buff).unwrap();
        buff.to_vec()
    }
}

#[cfg(test)]
mod point_tests {

    use std::str::FromStr;

    use super::PgPoint;

    const POINT_BYTES: &[u8] = &[
        64, 0, 204, 204, 204, 204, 204, 205, 64, 20, 204, 204, 204, 204, 204, 205,
    ];

    #[test]
    fn can_deserialise_point_type_bytes() {
        let point = PgPoint::from_bytes(POINT_BYTES).unwrap();
        assert_eq!(point, PgPoint { x: 2.1, y: 5.2 })
    }

    #[test]
    fn can_deserialise_point_type_str() {
        let point = PgPoint::from_str("(2, 3)").unwrap();
        assert_eq!(point, PgPoint { x: 2., y: 3. });
    }

    #[test]
    fn can_deserialise_point_type_str_float() {
        let point = PgPoint::from_str("(2.5, 3.4)").unwrap();
        assert_eq!(point, PgPoint { x: 2.5, y: 3.4 });
    }

    #[test]
    fn can_serialise_point_type() {
        let point = PgPoint { x: 2.1, y: 5.2 };
        assert_eq!(point.serialize_to_vec(), POINT_BYTES,)
    }
}
