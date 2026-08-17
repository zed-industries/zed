use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};
use sqlx_core::bytes::Buf;
use std::str::FromStr;

const ERROR: &str = "error decoding LSEG";

/// ## Postgres Geometric Line Segment type
///
/// Description: Finite line segment
/// Representation: `((start_x,start_y),(end_x,end_y))`
///
///
/// Line segments are represented by pairs of points that are the endpoints of the segment. Values of type lseg are specified using any of the following syntaxes:
/// ```text
/// [ ( start_x , start_y ) , ( end_x , end_y ) ]
/// ( ( start_x , start_y ) , ( end_x , end_y ) )
///   ( start_x , start_y ) , ( end_x , end_y )
///     start_x , start_y   ,   end_x , end_y
/// ```
/// where `(start_x,start_y) and (end_x,end_y)` are the end points of the line segment.
///
/// See [Postgres Manual, Section 8.8.3, Geometric Types - Line Segments][PG.S.8.8.3] for details.
///
/// [PG.S.8.8.3]: https://www.postgresql.org/docs/current/datatype-geometric.html#DATATYPE-LSEG
///
#[doc(alias = "line segment")]
#[derive(Debug, Clone, PartialEq)]
pub struct PgLSeg {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
}

impl Type<Postgres> for PgLSeg {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("lseg")
    }
}

impl PgHasArrayType for PgLSeg {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_lseg")
    }
}

impl<'r> Decode<'r, Postgres> for PgLSeg {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match value.format() {
            PgValueFormat::Text => Ok(PgLSeg::from_str(value.as_str()?)?),
            PgValueFormat::Binary => Ok(PgLSeg::from_bytes(value.as_bytes()?)?),
        }
    }
}

impl<'q> Encode<'q, Postgres> for PgLSeg {
    fn produces(&self) -> Option<PgTypeInfo> {
        Some(PgTypeInfo::with_name("lseg"))
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        self.serialize(buf)?;
        Ok(IsNull::No)
    }
}

impl FromStr for PgLSeg {
    type Err = BoxDynError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sanitised = s.replace(['(', ')', '[', ']', ' '], "");
        let mut parts = sanitised.split(',');

        let start_x = parts
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| format!("{}: could not get start_x from {}", ERROR, s))?;

        let start_y = parts
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| format!("{}: could not get start_y from {}", ERROR, s))?;

        let end_x = parts
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| format!("{}: could not get end_x from {}", ERROR, s))?;

        let end_y = parts
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| format!("{}: could not get end_y from {}", ERROR, s))?;

        if parts.next().is_some() {
            return Err(format!("{}: too many numbers inputted in {}", ERROR, s).into());
        }

        Ok(PgLSeg {
            start_x,
            start_y,
            end_x,
            end_y,
        })
    }
}

impl PgLSeg {
    fn from_bytes(mut bytes: &[u8]) -> Result<PgLSeg, BoxDynError> {
        let start_x = bytes.get_f64();
        let start_y = bytes.get_f64();
        let end_x = bytes.get_f64();
        let end_y = bytes.get_f64();

        Ok(PgLSeg {
            start_x,
            start_y,
            end_x,
            end_y,
        })
    }

    fn serialize(&self, buff: &mut PgArgumentBuffer) -> Result<(), BoxDynError> {
        buff.extend_from_slice(&self.start_x.to_be_bytes());
        buff.extend_from_slice(&self.start_y.to_be_bytes());
        buff.extend_from_slice(&self.end_x.to_be_bytes());
        buff.extend_from_slice(&self.end_y.to_be_bytes());
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
mod lseg_tests {

    use std::str::FromStr;

    use super::PgLSeg;

    const LINE_SEGMENT_BYTES: &[u8] = &[
        63, 241, 153, 153, 153, 153, 153, 154, 64, 1, 153, 153, 153, 153, 153, 154, 64, 10, 102,
        102, 102, 102, 102, 102, 64, 17, 153, 153, 153, 153, 153, 154,
    ];

    #[test]
    fn can_deserialise_lseg_type_bytes() {
        let lseg = PgLSeg::from_bytes(LINE_SEGMENT_BYTES).unwrap();
        assert_eq!(
            lseg,
            PgLSeg {
                start_x: 1.1,
                start_y: 2.2,
                end_x: 3.3,
                end_y: 4.4
            }
        )
    }

    #[test]
    fn can_deserialise_lseg_type_str_first_syntax() {
        let lseg = PgLSeg::from_str("[( 1, 2), (3, 4 )]").unwrap();
        assert_eq!(
            lseg,
            PgLSeg {
                start_x: 1.,
                start_y: 2.,
                end_x: 3.,
                end_y: 4.
            }
        );
    }
    #[test]
    fn can_deserialise_lseg_type_str_second_syntax() {
        let lseg = PgLSeg::from_str("(( 1, 2), (3, 4 ))").unwrap();
        assert_eq!(
            lseg,
            PgLSeg {
                start_x: 1.,
                start_y: 2.,
                end_x: 3.,
                end_y: 4.
            }
        );
    }

    #[test]
    fn can_deserialise_lseg_type_str_third_syntax() {
        let lseg = PgLSeg::from_str("(1, 2), (3, 4 )").unwrap();
        assert_eq!(
            lseg,
            PgLSeg {
                start_x: 1.,
                start_y: 2.,
                end_x: 3.,
                end_y: 4.
            }
        );
    }

    #[test]
    fn can_deserialise_lseg_type_str_fourth_syntax() {
        let lseg = PgLSeg::from_str("1, 2, 3, 4").unwrap();
        assert_eq!(
            lseg,
            PgLSeg {
                start_x: 1.,
                start_y: 2.,
                end_x: 3.,
                end_y: 4.
            }
        );
    }

    #[test]
    fn can_deserialise_too_many_numbers() {
        let input_str = "1, 2, 3, 4, 5";
        let lseg = PgLSeg::from_str(input_str);
        assert!(lseg.is_err());
        if let Err(err) = lseg {
            assert_eq!(
                err.to_string(),
                format!("error decoding LSEG: too many numbers inputted in {input_str}")
            )
        }
    }

    #[test]
    fn can_deserialise_too_few_numbers() {
        let input_str = "1, 2, 3";
        let lseg = PgLSeg::from_str(input_str);
        assert!(lseg.is_err());
        if let Err(err) = lseg {
            assert_eq!(
                err.to_string(),
                format!("error decoding LSEG: could not get end_y from {input_str}")
            )
        }
    }

    #[test]
    fn can_deserialise_invalid_numbers() {
        let input_str = "1, 2, 3, FOUR";
        let lseg = PgLSeg::from_str(input_str);
        assert!(lseg.is_err());
        if let Err(err) = lseg {
            assert_eq!(
                err.to_string(),
                format!("error decoding LSEG: could not get end_y from {input_str}")
            )
        }
    }

    #[test]
    fn can_deserialise_lseg_type_str_float() {
        let lseg = PgLSeg::from_str("(1.1, 2.2), (3.3, 4.4)").unwrap();
        assert_eq!(
            lseg,
            PgLSeg {
                start_x: 1.1,
                start_y: 2.2,
                end_x: 3.3,
                end_y: 4.4
            }
        );
    }

    #[test]
    fn can_serialise_lseg_type() {
        let lseg = PgLSeg {
            start_x: 1.1,
            start_y: 2.2,
            end_x: 3.3,
            end_y: 4.4,
        };
        assert_eq!(lseg.serialize_to_vec(), LINE_SEGMENT_BYTES,)
    }
}
