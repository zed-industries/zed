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

/// ## Postgres Geometric Path type
///
/// Description: Open path or Closed path (similar to polygon)
/// Representation: Open `[(x1,y1),...]`, Closed `((x1,y1),...)`
///
/// Paths are represented by lists of connected points. Paths can be open, where the first and last points in the list are considered not connected, or closed, where the first and last points are considered connected.
/// Values of type path are specified using any of the following syntaxes:
/// ```text
/// [ ( x1 , y1 ) , ... , ( xn , yn ) ]
/// ( ( x1 , y1 ) , ... , ( xn , yn ) )
///   ( x1 , y1 ) , ... , ( xn , yn )
///   ( x1 , y1   , ... ,   xn , yn )
///     x1 , y1   , ... ,   xn , yn
/// ```
/// where the points are the end points of the line segments comprising the path. Square brackets `([])` indicate an open path, while parentheses `(())` indicate a closed path.
/// When the outermost parentheses are omitted, as in the third through fifth syntaxes, a closed path is assumed.
///
/// See [Postgres Manual, Section 8.8.5, Geometric Types - Paths][PG.S.8.8.5] for details.
///
/// [PG.S.8.8.5]: https://www.postgresql.org/docs/current/datatype-geometric.html#DATATYPE-GEOMETRIC-PATHS
///
#[derive(Debug, Clone, PartialEq)]
pub struct PgPath {
    pub closed: bool,
    pub points: Vec<PgPoint>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Header {
    is_closed: bool,
    length: usize,
}

impl Type<Postgres> for PgPath {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("path")
    }
}

impl PgHasArrayType for PgPath {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_path")
    }
}

impl<'r> Decode<'r, Postgres> for PgPath {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match value.format() {
            PgValueFormat::Text => Ok(PgPath::from_str(value.as_str()?)?),
            PgValueFormat::Binary => Ok(PgPath::from_bytes(value.as_bytes()?)?),
        }
    }
}

impl<'q> Encode<'q, Postgres> for PgPath {
    fn produces(&self) -> Option<PgTypeInfo> {
        Some(PgTypeInfo::with_name("path"))
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        self.serialize(buf)?;
        Ok(IsNull::No)
    }
}

impl FromStr for PgPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let closed = !s.contains('[');
        let sanitised = s.replace(['(', ')', '[', ']', ' '], "");
        let parts = sanitised.split(',').collect::<Vec<_>>();

        let mut points = vec![];

        if parts.len() % 2 != 0 {
            return Err(Error::Decode(
                format!("Unmatched pair in PATH: {}", s).into(),
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
            return Ok(PgPath { points, closed });
        }

        Err(Error::Decode(
            format!("could not get path from {}", s).into(),
        ))
    }
}

impl PgPath {
    fn header(&self) -> Header {
        Header {
            is_closed: self.closed,
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
        Ok(PgPath {
            closed: header.is_closed,
            points: out_points,
        })
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
                "expected PATH data to contain at least {} bytes, got {}",
                Self::HEADER_WIDTH,
                buf.len()
            ));
        }

        let is_closed = buf.get_i8();
        let length = buf.get_i32();

        let length = usize::try_from(length).ok().ok_or_else(|| {
            format!(
                "received PATH data length: {length}. Expected length between 0 and {}",
                usize::MAX
            )
        })?;

        Ok(Self {
            is_closed: is_closed != 0,
            length,
        })
    }

    fn try_write(&self, buff: &mut PgArgumentBuffer) -> Result<(), String> {
        let is_closed = self.is_closed as i8;

        let length = i32::try_from(self.length).map_err(|_| {
            format!(
                "PATH length exceeds allowed maximum ({} > {})",
                self.length,
                i32::MAX
            )
        })?;

        buff.extend(is_closed.to_be_bytes());
        buff.extend(length.to_be_bytes());

        Ok(())
    }
}

fn parse_float_from_str(s: &str, error_msg: &str) -> Result<f64, Error> {
    s.parse().map_err(|_| Error::Decode(error_msg.into()))
}

#[cfg(test)]
mod path_tests {

    use std::str::FromStr;

    use crate::types::PgPoint;

    use super::PgPath;

    const PATH_CLOSED_BYTES: &[u8] = &[
        1, 0, 0, 0, 2, 63, 240, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 8, 0, 0, 0, 0, 0, 0,
        64, 16, 0, 0, 0, 0, 0, 0,
    ];

    const PATH_OPEN_BYTES: &[u8] = &[
        0, 0, 0, 0, 2, 63, 240, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 8, 0, 0, 0, 0, 0, 0,
        64, 16, 0, 0, 0, 0, 0, 0,
    ];

    const PATH_UNEVEN_POINTS: &[u8] = &[
        0, 0, 0, 0, 2, 63, 240, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 8, 0, 0, 0, 0, 0, 0,
        64, 16, 0, 0,
    ];

    #[test]
    fn can_deserialise_path_type_bytes_closed() {
        let path = PgPath::from_bytes(PATH_CLOSED_BYTES).unwrap();
        assert_eq!(
            path,
            PgPath {
                closed: true,
                points: vec![PgPoint { x: 1.0, y: 2.0 }, PgPoint { x: 3.0, y: 4.0 }]
            }
        )
    }

    #[test]
    fn cannot_deserialise_path_type_uneven_point_bytes() {
        let path = PgPath::from_bytes(PATH_UNEVEN_POINTS);
        assert!(path.is_err());

        if let Err(err) = path {
            assert_eq!(
                err.to_string(),
                format!("expected 32 bytes after header, got 28")
            )
        }
    }

    #[test]
    fn can_deserialise_path_type_bytes_open() {
        let path = PgPath::from_bytes(PATH_OPEN_BYTES).unwrap();
        assert_eq!(
            path,
            PgPath {
                closed: false,
                points: vec![PgPoint { x: 1.0, y: 2.0 }, PgPoint { x: 3.0, y: 4.0 }]
            }
        )
    }

    #[test]
    fn can_deserialise_path_type_str_first_syntax() {
        let path = PgPath::from_str("[( 1, 2), (3, 4 )]").unwrap();
        assert_eq!(
            path,
            PgPath {
                closed: false,
                points: vec![PgPoint { x: 1., y: 2. }, PgPoint { x: 3., y: 4. }]
            }
        );
    }

    #[test]
    fn cannot_deserialise_path_type_str_uneven_points_first_syntax() {
        let input_str = "[( 1, 2), (3)]";
        let path = PgPath::from_str(input_str);

        assert!(path.is_err());

        if let Err(err) = path {
            assert_eq!(
                err.to_string(),
                format!("error occurred while decoding: Unmatched pair in PATH: {input_str}")
            )
        }
    }

    #[test]
    fn can_deserialise_path_type_str_second_syntax() {
        let path = PgPath::from_str("(( 1, 2), (3, 4 ))").unwrap();
        assert_eq!(
            path,
            PgPath {
                closed: true,
                points: vec![PgPoint { x: 1., y: 2. }, PgPoint { x: 3., y: 4. }]
            }
        );
    }

    #[test]
    fn can_deserialise_path_type_str_third_syntax() {
        let path = PgPath::from_str("(1, 2), (3, 4 )").unwrap();
        assert_eq!(
            path,
            PgPath {
                closed: true,
                points: vec![PgPoint { x: 1., y: 2. }, PgPoint { x: 3., y: 4. }]
            }
        );
    }

    #[test]
    fn can_deserialise_path_type_str_fourth_syntax() {
        let path = PgPath::from_str("1, 2, 3, 4").unwrap();
        assert_eq!(
            path,
            PgPath {
                closed: true,
                points: vec![PgPoint { x: 1., y: 2. }, PgPoint { x: 3., y: 4. }]
            }
        );
    }

    #[test]
    fn can_deserialise_path_type_str_float() {
        let path = PgPath::from_str("(1.1, 2.2), (3.3, 4.4)").unwrap();
        assert_eq!(
            path,
            PgPath {
                closed: true,
                points: vec![PgPoint { x: 1.1, y: 2.2 }, PgPoint { x: 3.3, y: 4.4 }]
            }
        );
    }

    #[test]
    fn can_serialise_path_type() {
        let path = PgPath {
            closed: true,
            points: vec![PgPoint { x: 1., y: 2. }, PgPoint { x: 3., y: 4. }],
        };
        assert_eq!(path.serialize_to_vec(), PATH_CLOSED_BYTES,)
    }
}
