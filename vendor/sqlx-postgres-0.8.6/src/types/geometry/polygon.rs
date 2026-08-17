use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::{PgPoint, Type};
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};
use sqlx_core::bytes::Buf;
use sqlx_core::Error;
use std::mem;
use std::str::FromStr;

const BYTE_WIDTH: usize = mem::size_of::<f64>();

/// ## Postgres Geometric Polygon type
///
/// Description: Polygon (similar to closed polygon)
/// Representation: `((x1,y1),...)`
///
/// Polygons are represented by lists of points (the vertexes of the polygon). Polygons are very similar to closed paths; the essential semantic difference is that a polygon is considered to include the area within it, while a path is not.
/// An important implementation difference between polygons and paths is that the stored representation of a polygon includes its smallest bounding box. This speeds up certain search operations, although computing the bounding box adds overhead while constructing new polygons.
/// Values of type polygon are specified using any of the following syntaxes:
///
/// ```text
/// ( ( x1 , y1 ) , ... , ( xn , yn ) )
///   ( x1 , y1 ) , ... , ( xn , yn )
///   ( x1 , y1   , ... ,   xn , yn )
///     x1 , y1   , ... ,   xn , yn
/// ```
///
/// where the points are the end points of the line segments comprising the boundary of the polygon.
///
/// See [Postgres Manual, Section 8.8.6, Geometric Types - Polygons][PG.S.8.8.6] for details.
///
/// [PG.S.8.8.6]: https://www.postgresql.org/docs/current/datatype-geometric.html#DATATYPE-POLYGON
///
#[derive(Debug, Clone, PartialEq)]
pub struct PgPolygon {
    pub points: Vec<PgPoint>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Header {
    length: usize,
}

impl Type<Postgres> for PgPolygon {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("polygon")
    }
}

impl PgHasArrayType for PgPolygon {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_polygon")
    }
}

impl<'r> Decode<'r, Postgres> for PgPolygon {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match value.format() {
            PgValueFormat::Text => Ok(PgPolygon::from_str(value.as_str()?)?),
            PgValueFormat::Binary => Ok(PgPolygon::from_bytes(value.as_bytes()?)?),
        }
    }
}

impl<'q> Encode<'q, Postgres> for PgPolygon {
    fn produces(&self) -> Option<PgTypeInfo> {
        Some(PgTypeInfo::with_name("polygon"))
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        self.serialize(buf)?;
        Ok(IsNull::No)
    }
}

impl FromStr for PgPolygon {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sanitised = s.replace(['(', ')', '[', ']', ' '], "");
        let parts = sanitised.split(',').collect::<Vec<_>>();

        let mut points = vec![];

        if parts.len() % 2 != 0 {
            return Err(Error::Decode(
                format!("Unmatched pair in POLYGON: {}", s).into(),
            ));
        }

        for chunk in parts.chunks_exact(2) {
            if let [x_str, y_str] = chunk {
                let x = parse_float_from_str(x_str, "could not get x")?;
                let y = parse_float_from_str(y_str, "could not get y")?;

                let point = PgPoint { x, y };
                points.push(point);
            }
        }

        if !points.is_empty() {
            return Ok(PgPolygon { points });
        }

        Err(Error::Decode(
            format!("could not get polygon from {}", s).into(),
        ))
    }
}

impl PgPolygon {
    fn header(&self) -> Header {
        Header {
            length: self.points.len(),
        }
    }

    fn from_bytes(mut bytes: &[u8]) -> Result<Self, BoxDynError> {
        let header = Header::try_read(&mut bytes)?;

        if bytes.len() != header.data_size() {
            return Err(format!(
                "expected {} bytes after header, got {}",
                header.data_size(),
                bytes.len()
            )
            .into());
        }

        if bytes.len() % BYTE_WIDTH * 2 != 0 {
            return Err(format!(
                "data length not divisible by pairs of {BYTE_WIDTH}: {}",
                bytes.len()
            )
            .into());
        }

        let mut out_points = Vec::with_capacity(bytes.len() / (BYTE_WIDTH * 2));
        while bytes.has_remaining() {
            let point = PgPoint {
                x: bytes.get_f64(),
                y: bytes.get_f64(),
            };
            out_points.push(point)
        }
        Ok(PgPolygon { points: out_points })
    }

    fn serialize(&self, buff: &mut PgArgumentBuffer) -> Result<(), BoxDynError> {
        let header = self.header();
        buff.reserve(header.data_size());
        header.try_write(buff)?;

        for point in &self.points {
            buff.extend_from_slice(&point.x.to_be_bytes());
            buff.extend_from_slice(&point.y.to_be_bytes());
        }
        Ok(())
    }

    #[cfg(test)]
    fn serialize_to_vec(&self) -> Vec<u8> {
        let mut buff = PgArgumentBuffer::default();
        self.serialize(&mut buff).unwrap();
        buff.to_vec()
    }
}

impl Header {
    const HEADER_WIDTH: usize = mem::size_of::<i8>() + mem::size_of::<i32>();

    fn data_size(&self) -> usize {
        self.length * BYTE_WIDTH * 2
    }

    fn try_read(buf: &mut &[u8]) -> Result<Self, String> {
        if buf.len() < Self::HEADER_WIDTH {
            return Err(format!(
                "expected polygon data to contain at least {} bytes, got {}",
                Self::HEADER_WIDTH,
                buf.len()
            ));
        }

        let length = buf.get_i32();

        let length = usize::try_from(length).ok().ok_or_else(|| {
            format!(
                "received polygon with length: {length}. Expected length between 0 and  {}",
                usize::MAX
            )
        })?;

        Ok(Self { length })
    }

    fn try_write(&self, buff: &mut PgArgumentBuffer) -> Result<(), String> {
        let length = i32::try_from(self.length).map_err(|_| {
            format!(
                "polygon length exceeds allowed maximum ({} > {})",
                self.length,
                i32::MAX
            )
        })?;

        buff.extend(length.to_be_bytes());

        Ok(())
    }
}

fn parse_float_from_str(s: &str, error_msg: &str) -> Result<f64, Error> {
    s.parse().map_err(|_| Error::Decode(error_msg.into()))
}

#[cfg(test)]
mod polygon_tests {

    use std::str::FromStr;

    use crate::types::PgPoint;

    use super::PgPolygon;

    const POLYGON_BYTES: &[u8] = &[
        0, 0, 0, 12, 192, 0, 0, 0, 0, 0, 0, 0, 192, 8, 0, 0, 0, 0, 0, 0, 191, 240, 0, 0, 0, 0, 0,
        0, 192, 8, 0, 0, 0, 0, 0, 0, 191, 240, 0, 0, 0, 0, 0, 0, 191, 240, 0, 0, 0, 0, 0, 0, 63,
        240, 0, 0, 0, 0, 0, 0, 63, 240, 0, 0, 0, 0, 0, 0, 63, 240, 0, 0, 0, 0, 0, 0, 64, 8, 0, 0,
        0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 8, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 192,
        8, 0, 0, 0, 0, 0, 0, 63, 240, 0, 0, 0, 0, 0, 0, 192, 8, 0, 0, 0, 0, 0, 0, 63, 240, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 191, 240, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 191,
        240, 0, 0, 0, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 192, 0, 0, 0, 0,
        0, 0, 0,
    ];

    #[test]
    fn can_deserialise_polygon_type_bytes() {
        let polygon = PgPolygon::from_bytes(POLYGON_BYTES).unwrap();
        assert_eq!(
            polygon,
            PgPolygon {
                points: vec![
                    PgPoint { x: -2., y: -3. },
                    PgPoint { x: -1., y: -3. },
                    PgPoint { x: -1., y: -1. },
                    PgPoint { x: 1., y: 1. },
                    PgPoint { x: 1., y: 3. },
                    PgPoint { x: 2., y: 3. },
                    PgPoint { x: 2., y: -3. },
                    PgPoint { x: 1., y: -3. },
                    PgPoint { x: 1., y: 0. },
                    PgPoint { x: -1., y: 0. },
                    PgPoint { x: -1., y: -2. },
                    PgPoint { x: -2., y: -2. }
                ]
            }
        )
    }

    #[test]
    fn can_deserialise_polygon_type_str_first_syntax() {
        let polygon = PgPolygon::from_str("[( 1, 2), (3, 4 )]").unwrap();
        assert_eq!(
            polygon,
            PgPolygon {
                points: vec![PgPoint { x: 1., y: 2. }, PgPoint { x: 3., y: 4. }]
            }
        );
    }

    #[test]
    fn can_deserialise_polygon_type_str_second_syntax() {
        let polygon = PgPolygon::from_str("(( 1, 2), (3, 4 ))").unwrap();
        assert_eq!(
            polygon,
            PgPolygon {
                points: vec![PgPoint { x: 1., y: 2. }, PgPoint { x: 3., y: 4. }]
            }
        );
    }

    #[test]
    fn cannot_deserialise_polygon_type_str_uneven_points_first_syntax() {
        let input_str = "[( 1, 2), (3)]";
        let polygon = PgPolygon::from_str(input_str);

        assert!(polygon.is_err());

        if let Err(err) = polygon {
            assert_eq!(
                err.to_string(),
                format!("error occurred while decoding: Unmatched pair in POLYGON: {input_str}")
            )
        }
    }

    #[test]
    fn cannot_deserialise_polygon_type_str_invalid_numbers() {
        let input_str = "[( 1, 2), (2, three)]";
        let polygon = PgPolygon::from_str(input_str);

        assert!(polygon.is_err());

        if let Err(err) = polygon {
            assert_eq!(
                err.to_string(),
                format!("error occurred while decoding: could not get y")
            )
        }
    }

    #[test]
    fn can_deserialise_polygon_type_str_third_syntax() {
        let polygon = PgPolygon::from_str("(1, 2), (3, 4 )").unwrap();
        assert_eq!(
            polygon,
            PgPolygon {
                points: vec![PgPoint { x: 1., y: 2. }, PgPoint { x: 3., y: 4. }]
            }
        );
    }

    #[test]
    fn can_deserialise_polygon_type_str_fourth_syntax() {
        let polygon = PgPolygon::from_str("1, 2, 3, 4").unwrap();
        assert_eq!(
            polygon,
            PgPolygon {
                points: vec![PgPoint { x: 1., y: 2. }, PgPoint { x: 3., y: 4. }]
            }
        );
    }

    #[test]
    fn can_deserialise_polygon_type_str_float() {
        let polygon = PgPolygon::from_str("(1.1, 2.2), (3.3, 4.4)").unwrap();
        assert_eq!(
            polygon,
            PgPolygon {
                points: vec![PgPoint { x: 1.1, y: 2.2 }, PgPoint { x: 3.3, y: 4.4 }]
            }
        );
    }

    #[test]
    fn can_serialise_polygon_type() {
        let polygon = PgPolygon {
            points: vec![
                PgPoint { x: -2., y: -3. },
                PgPoint { x: -1., y: -3. },
                PgPoint { x: -1., y: -1. },
                PgPoint { x: 1., y: 1. },
                PgPoint { x: 1., y: 3. },
                PgPoint { x: 2., y: 3. },
                PgPoint { x: 2., y: -3. },
                PgPoint { x: 1., y: -3. },
                PgPoint { x: 1., y: 0. },
                PgPoint { x: -1., y: 0. },
                PgPoint { x: -1., y: -2. },
                PgPoint { x: -2., y: -2. },
            ],
        };
        assert_eq!(polygon.serialize_to_vec(), POLYGON_BYTES,)
    }
}
