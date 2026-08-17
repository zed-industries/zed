use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};
use sqlx_core::bytes::Buf;
use sqlx_core::Error;
use std::str::FromStr;

const ERROR: &str = "error decoding CIRCLE";

/// ## Postgres Geometric Circle type
///
/// Description: Circle
/// Representation: `< (x, y), radius >` (center point and radius)
///
/// ```text
/// < ( x , y ) , radius >
/// ( ( x , y ) , radius )
///   ( x , y ) , radius
///     x , y   , radius
/// ```
/// where `(x,y)` is the center point.
///
/// See [Postgres Manual, Section 8.8.7, Geometric Types - Circles][PG.S.8.8.7] for details.
///
/// [PG.S.8.8.7]: https://www.postgresql.org/docs/current/datatype-geometric.html#DATATYPE-CIRCLE
///
#[derive(Debug, Clone, PartialEq)]
pub struct PgCircle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Type<Postgres> for PgCircle {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("circle")
    }
}

impl PgHasArrayType for PgCircle {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_circle")
    }
}

impl<'r> Decode<'r, Postgres> for PgCircle {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match value.format() {
            PgValueFormat::Text => Ok(PgCircle::from_str(value.as_str()?)?),
            PgValueFormat::Binary => Ok(PgCircle::from_bytes(value.as_bytes()?)?),
        }
    }
}

impl<'q> Encode<'q, Postgres> for PgCircle {
    fn produces(&self) -> Option<PgTypeInfo> {
        Some(PgTypeInfo::with_name("circle"))
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        self.serialize(buf)?;
        Ok(IsNull::No)
    }
}

impl FromStr for PgCircle {
    type Err = BoxDynError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sanitised = s.replace(['<', '>', '(', ')', ' '], "");
        let mut parts = sanitised.split(',');

        let x = parts
            .next()
            .and_then(|s| s.trim().parse::<f64>().ok())
            .ok_or_else(|| format!("{}: could not get x from {}", ERROR, s))?;

        let y = parts
            .next()
            .and_then(|s| s.trim().parse::<f64>().ok())
            .ok_or_else(|| format!("{}: could not get y from {}", ERROR, s))?;

        let radius = parts
            .next()
            .and_then(|s| s.trim().parse::<f64>().ok())
            .ok_or_else(|| format!("{}: could not get radius from {}", ERROR, s))?;

        if parts.next().is_some() {
            return Err(format!("{}: too many numbers inputted in {}", ERROR, s).into());
        }

        if radius < 0. {
            return Err(format!("{}: cannot have negative radius: {}", ERROR, s).into());
        }

        Ok(PgCircle { x, y, radius })
    }
}

impl PgCircle {
    fn from_bytes(mut bytes: &[u8]) -> Result<PgCircle, Error> {
        let x = bytes.get_f64();
        let y = bytes.get_f64();
        let r = bytes.get_f64();
        Ok(PgCircle { x, y, radius: r })
    }

    fn serialize(&self, buff: &mut PgArgumentBuffer) -> Result<(), Error> {
        buff.extend_from_slice(&self.x.to_be_bytes());
        buff.extend_from_slice(&self.y.to_be_bytes());
        buff.extend_from_slice(&self.radius.to_be_bytes());
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
mod circle_tests {

    use std::str::FromStr;

    use super::PgCircle;

    const CIRCLE_BYTES: &[u8] = &[
        63, 241, 153, 153, 153, 153, 153, 154, 64, 1, 153, 153, 153, 153, 153, 154, 64, 10, 102,
        102, 102, 102, 102, 102,
    ];

    #[test]
    fn can_deserialise_circle_type_bytes() {
        let circle = PgCircle::from_bytes(CIRCLE_BYTES).unwrap();
        assert_eq!(
            circle,
            PgCircle {
                x: 1.1,
                y: 2.2,
                radius: 3.3
            }
        )
    }

    #[test]
    fn can_deserialise_circle_type_str() {
        let circle = PgCircle::from_str("<(1, 2), 3 >").unwrap();
        assert_eq!(
            circle,
            PgCircle {
                x: 1.0,
                y: 2.0,
                radius: 3.0
            }
        );
    }

    #[test]
    fn can_deserialise_circle_type_str_second_syntax() {
        let circle = PgCircle::from_str("((1, 2), 3 )").unwrap();
        assert_eq!(
            circle,
            PgCircle {
                x: 1.0,
                y: 2.0,
                radius: 3.0
            }
        );
    }

    #[test]
    fn can_deserialise_circle_type_str_third_syntax() {
        let circle = PgCircle::from_str("(1, 2), 3 ").unwrap();
        assert_eq!(
            circle,
            PgCircle {
                x: 1.0,
                y: 2.0,
                radius: 3.0
            }
        );
    }

    #[test]
    fn can_deserialise_circle_type_str_fourth_syntax() {
        let circle = PgCircle::from_str("1, 2, 3 ").unwrap();
        assert_eq!(
            circle,
            PgCircle {
                x: 1.0,
                y: 2.0,
                radius: 3.0
            }
        );
    }

    #[test]
    fn cannot_deserialise_circle_invalid_numbers() {
        let input_str = "1, 2, Three";
        let circle = PgCircle::from_str(input_str);
        assert!(circle.is_err());
        if let Err(err) = circle {
            assert_eq!(
                err.to_string(),
                format!("error decoding CIRCLE: could not get radius from {input_str}")
            )
        }
    }

    #[test]
    fn cannot_deserialise_circle_negative_radius() {
        let input_str = "1, 2, -3";
        let circle = PgCircle::from_str(input_str);
        assert!(circle.is_err());
        if let Err(err) = circle {
            assert_eq!(
                err.to_string(),
                format!("error decoding CIRCLE: cannot have negative radius: {input_str}")
            )
        }
    }

    #[test]
    fn can_deserialise_circle_type_str_float() {
        let circle = PgCircle::from_str("<(1.1, 2.2), 3.3>").unwrap();
        assert_eq!(
            circle,
            PgCircle {
                x: 1.1,
                y: 2.2,
                radius: 3.3
            }
        );
    }

    #[test]
    fn can_serialise_circle_type() {
        let circle = PgCircle {
            x: 1.1,
            y: 2.2,
            radius: 3.3,
        };
        assert_eq!(circle.serialize_to_vec(), CIRCLE_BYTES,)
    }
}
