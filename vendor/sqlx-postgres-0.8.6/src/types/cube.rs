use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};
use sqlx_core::bytes::Buf;
use sqlx_core::Error;
use std::mem;
use std::str::FromStr;

const BYTE_WIDTH: usize = 8;

/// <https://github.com/postgres/postgres/blob/e3ec9dc1bf4983fcedb6f43c71ea12ee26aefc7a/contrib/cube/cubedata.h#L7>
const MAX_DIMENSIONS: usize = 100;

const IS_POINT_FLAG: u32 = 1 << 31;

// FIXME(breaking): these variants are confusingly named and structured
// consider changing them or making this an opaque wrapper around `Vec<f64>`
#[derive(Debug, Clone, PartialEq)]
pub enum PgCube {
    /// A one-dimensional point.
    // FIXME: `Point1D(f64)`
    Point(f64),
    /// An N-dimensional point ("represented internally as a zero-volume cube").
    // FIXME: `PointND(f64)`
    ZeroVolume(Vec<f64>),

    /// A one-dimensional interval with starting and ending points.
    // FIXME: `Interval1D { start: f64, end: f64 }`
    OneDimensionInterval(f64, f64),

    // FIXME: add `Cube3D { lower_left: [f64; 3], upper_right: [f64; 3] }`?
    /// An N-dimensional cube with points representing lower-left and upper-right corners, respectively.
    // FIXME: `CubeND { lower_left: Vec<f64>, upper_right: Vec<f64> }`
    MultiDimension(Vec<Vec<f64>>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Header {
    dimensions: usize,
    is_point: bool,
}

#[derive(Debug, thiserror::Error)]
#[error("error decoding CUBE (is_point: {is_point}, dimensions: {dimensions})")]
struct DecodeError {
    is_point: bool,
    dimensions: usize,
    message: String,
}

impl Type<Postgres> for PgCube {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("cube")
    }
}

impl PgHasArrayType for PgCube {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_cube")
    }
}

impl<'r> Decode<'r, Postgres> for PgCube {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match value.format() {
            PgValueFormat::Text => Ok(PgCube::from_str(value.as_str()?)?),
            PgValueFormat::Binary => Ok(PgCube::from_bytes(value.as_bytes()?)?),
        }
    }
}

impl<'q> Encode<'q, Postgres> for PgCube {
    fn produces(&self) -> Option<PgTypeInfo> {
        Some(PgTypeInfo::with_name("cube"))
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        self.serialize(buf)?;
        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        self.header().encoded_size()
    }
}

impl FromStr for PgCube {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let content = s
            .trim_start_matches('(')
            .trim_start_matches('[')
            .trim_end_matches(')')
            .trim_end_matches(']')
            .replace(' ', "");

        if !content.contains('(') && !content.contains(',') {
            return parse_point(&content);
        }

        if !content.contains("),(") {
            return parse_zero_volume(&content);
        }

        let point_vecs = content.split("),(").collect::<Vec<&str>>();
        if point_vecs.len() == 2 && !point_vecs.iter().any(|pv| pv.contains(',')) {
            return parse_one_dimensional_interval(point_vecs);
        }

        parse_multidimensional_interval(point_vecs)
    }
}

impl PgCube {
    fn header(&self) -> Header {
        match self {
            PgCube::Point(..) => Header {
                is_point: true,
                dimensions: 1,
            },
            PgCube::ZeroVolume(values) => Header {
                is_point: true,
                dimensions: values.len(),
            },
            PgCube::OneDimensionInterval(..) => Header {
                is_point: false,
                dimensions: 1,
            },
            PgCube::MultiDimension(multi_values) => Header {
                is_point: false,
                dimensions: multi_values.first().map(|arr| arr.len()).unwrap_or(0),
            },
        }
    }

    fn from_bytes(mut bytes: &[u8]) -> Result<Self, BoxDynError> {
        let header = Header::try_read(&mut bytes)?;

        if bytes.len() != header.data_size() {
            return Err(DecodeError::new(
                &header,
                format!(
                    "expected {} bytes after header, got {}",
                    header.data_size(),
                    bytes.len()
                ),
            )
            .into());
        }

        match (header.is_point, header.dimensions) {
            (true, 1) => Ok(PgCube::Point(bytes.get_f64())),
            (true, _) => Ok(PgCube::ZeroVolume(
                read_vec(&mut bytes).map_err(|e| DecodeError::new(&header, e))?,
            )),
            (false, 1) => Ok(PgCube::OneDimensionInterval(
                bytes.get_f64(),
                bytes.get_f64(),
            )),
            (false, _) => Ok(PgCube::MultiDimension(read_cube(&header, bytes)?)),
        }
    }

    fn serialize(&self, buff: &mut PgArgumentBuffer) -> Result<(), String> {
        let header = self.header();

        buff.reserve(header.data_size());

        header.try_write(buff)?;

        match self {
            PgCube::Point(value) => {
                buff.extend_from_slice(&value.to_be_bytes());
            }
            PgCube::ZeroVolume(values) => {
                buff.extend(values.iter().flat_map(|v| v.to_be_bytes()));
            }
            PgCube::OneDimensionInterval(x, y) => {
                buff.extend_from_slice(&x.to_be_bytes());
                buff.extend_from_slice(&y.to_be_bytes());
            }
            PgCube::MultiDimension(multi_values) => {
                if multi_values.len() != 2 {
                    return Err(format!("invalid CUBE value: {self:?}"));
                }

                buff.extend(
                    multi_values
                        .iter()
                        .flat_map(|point| point.iter().flat_map(|scalar| scalar.to_be_bytes())),
                );
            }
        };
        Ok(())
    }

    #[cfg(test)]
    fn serialize_to_vec(&self) -> Vec<u8> {
        let mut buff = PgArgumentBuffer::default();
        self.serialize(&mut buff).unwrap();
        buff.to_vec()
    }
}

fn read_vec(bytes: &mut &[u8]) -> Result<Vec<f64>, String> {
    if bytes.len() % BYTE_WIDTH != 0 {
        return Err(format!(
            "data length not divisible by {BYTE_WIDTH}: {}",
            bytes.len()
        ));
    }

    let mut out = Vec::with_capacity(bytes.len() / BYTE_WIDTH);

    while bytes.has_remaining() {
        out.push(bytes.get_f64());
    }

    Ok(out)
}

fn read_cube(header: &Header, mut bytes: &[u8]) -> Result<Vec<Vec<f64>>, String> {
    if bytes.len() != header.data_size() {
        return Err(format!(
            "expected {} bytes, got {}",
            header.data_size(),
            bytes.len()
        ));
    }

    let mut out = Vec::with_capacity(2);

    // Expecting exactly 2 N-dimensional points
    for _ in 0..2 {
        let mut point = Vec::new();

        for _ in 0..header.dimensions {
            point.push(bytes.get_f64());
        }

        out.push(point);
    }

    Ok(out)
}

fn parse_float_from_str(s: &str, error_msg: &str) -> Result<f64, Error> {
    s.parse().map_err(|_| Error::Decode(error_msg.into()))
}

fn parse_point(str: &str) -> Result<PgCube, Error> {
    Ok(PgCube::Point(parse_float_from_str(
        str,
        "Failed to parse point",
    )?))
}

fn parse_zero_volume(content: &str) -> Result<PgCube, Error> {
    content
        .split(',')
        .map(|p| parse_float_from_str(p, "Failed to parse into zero-volume cube"))
        .collect::<Result<Vec<_>, _>>()
        .map(PgCube::ZeroVolume)
}

fn parse_one_dimensional_interval(point_vecs: Vec<&str>) -> Result<PgCube, Error> {
    let x = parse_float_from_str(
        &remove_parentheses(point_vecs.first().ok_or(Error::Decode(
            format!("Could not decode cube interval x: {:?}", point_vecs).into(),
        ))?),
        "Failed to parse X in one-dimensional interval",
    )?;
    let y = parse_float_from_str(
        &remove_parentheses(point_vecs.get(1).ok_or(Error::Decode(
            format!("Could not decode cube interval y: {:?}", point_vecs).into(),
        ))?),
        "Failed to parse Y in one-dimensional interval",
    )?;
    Ok(PgCube::OneDimensionInterval(x, y))
}

fn parse_multidimensional_interval(point_vecs: Vec<&str>) -> Result<PgCube, Error> {
    point_vecs
        .iter()
        .map(|&point_vec| {
            point_vec
                .split(',')
                .map(|point| {
                    parse_float_from_str(
                        &remove_parentheses(point),
                        "Failed to parse into multi-dimension cube",
                    )
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .map(PgCube::MultiDimension)
}

fn remove_parentheses(s: &str) -> String {
    s.trim_matches(|c| c == '(' || c == ')').to_string()
}

impl Header {
    const PACKED_WIDTH: usize = mem::size_of::<u32>();

    fn encoded_size(&self) -> usize {
        Self::PACKED_WIDTH + self.data_size()
    }

    fn data_size(&self) -> usize {
        if self.is_point {
            self.dimensions * BYTE_WIDTH
        } else {
            self.dimensions * BYTE_WIDTH * 2
        }
    }

    fn try_write(&self, buff: &mut PgArgumentBuffer) -> Result<(), String> {
        if self.dimensions > MAX_DIMENSIONS {
            return Err(format!(
                "CUBE dimensionality exceeds allowed maximum ({} > {MAX_DIMENSIONS})",
                self.dimensions
            ));
        }

        // Cannot overflow thanks to the above check.
        #[allow(clippy::cast_possible_truncation)]
        let mut packed = self.dimensions as u32;

        // https://github.com/postgres/postgres/blob/e3ec9dc1bf4983fcedb6f43c71ea12ee26aefc7a/contrib/cube/cubedata.h#L18-L24
        if self.is_point {
            packed |= IS_POINT_FLAG;
        }

        buff.extend(packed.to_be_bytes());

        Ok(())
    }

    fn try_read(buf: &mut &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKED_WIDTH {
            return Err(format!(
                "expected CUBE data to contain at least {} bytes, got {}",
                Self::PACKED_WIDTH,
                buf.len()
            ));
        }

        let packed = buf.get_u32();

        let is_point = packed & IS_POINT_FLAG != 0;
        let dimensions = packed & !IS_POINT_FLAG;

        // can only overflow on 16-bit platforms
        let dimensions = usize::try_from(dimensions)
            .ok()
            .filter(|&it| it <= MAX_DIMENSIONS)
            .ok_or_else(|| format!("received CUBE data with higher than expected dimensionality: {dimensions} (is_point: {is_point})"))?;

        Ok(Self {
            is_point,
            dimensions,
        })
    }
}

impl DecodeError {
    fn new(header: &Header, message: String) -> Self {
        DecodeError {
            is_point: header.is_point,
            dimensions: header.dimensions,
            message,
        }
    }
}

#[cfg(test)]
mod cube_tests {

    use std::str::FromStr;

    use super::PgCube;

    const POINT_BYTES: &[u8] = &[128, 0, 0, 1, 64, 0, 0, 0, 0, 0, 0, 0];
    const ZERO_VOLUME_BYTES: &[u8] = &[
        128, 0, 0, 2, 64, 0, 0, 0, 0, 0, 0, 0, 64, 8, 0, 0, 0, 0, 0, 0,
    ];
    const ONE_DIMENSIONAL_INTERVAL_BYTES: &[u8] = &[
        0, 0, 0, 1, 64, 28, 0, 0, 0, 0, 0, 0, 64, 32, 0, 0, 0, 0, 0, 0,
    ];
    const MULTI_DIMENSION_2_DIM_BYTES: &[u8] = &[
        0, 0, 0, 2, 63, 240, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 64, 8, 0, 0, 0, 0, 0, 0,
        64, 16, 0, 0, 0, 0, 0, 0,
    ];
    const MULTI_DIMENSION_3_DIM_BYTES: &[u8] = &[
        0, 0, 0, 3, 64, 0, 0, 0, 0, 0, 0, 0, 64, 8, 0, 0, 0, 0, 0, 0, 64, 16, 0, 0, 0, 0, 0, 0, 64,
        20, 0, 0, 0, 0, 0, 0, 64, 24, 0, 0, 0, 0, 0, 0, 64, 28, 0, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn can_deserialise_point_type_byes() {
        let cube = PgCube::from_bytes(POINT_BYTES).unwrap();
        assert_eq!(cube, PgCube::Point(2.))
    }

    #[test]
    fn can_deserialise_point_type_str() {
        let cube_1 = PgCube::from_str("(2)").unwrap();
        assert_eq!(cube_1, PgCube::Point(2.));
        let cube_2 = PgCube::from_str("2").unwrap();
        assert_eq!(cube_2, PgCube::Point(2.));
    }

    #[test]
    fn can_serialise_point_type() {
        assert_eq!(PgCube::Point(2.).serialize_to_vec(), POINT_BYTES,)
    }
    #[test]
    fn can_deserialise_zero_volume_bytes() {
        let cube = PgCube::from_bytes(ZERO_VOLUME_BYTES).unwrap();
        assert_eq!(cube, PgCube::ZeroVolume(vec![2., 3.]));
    }

    #[test]
    fn can_deserialise_zero_volume_string() {
        let cube_1 = PgCube::from_str("(2,3,4)").unwrap();
        assert_eq!(cube_1, PgCube::ZeroVolume(vec![2., 3., 4.]));
        let cube_2 = PgCube::from_str("2,3,4").unwrap();
        assert_eq!(cube_2, PgCube::ZeroVolume(vec![2., 3., 4.]));
    }

    #[test]
    fn can_serialise_zero_volume() {
        assert_eq!(
            PgCube::ZeroVolume(vec![2., 3.]).serialize_to_vec(),
            ZERO_VOLUME_BYTES
        );
    }

    #[test]
    fn can_deserialise_one_dimension_interval_bytes() {
        let cube = PgCube::from_bytes(ONE_DIMENSIONAL_INTERVAL_BYTES).unwrap();
        assert_eq!(cube, PgCube::OneDimensionInterval(7., 8.))
    }

    #[test]
    fn can_deserialise_one_dimension_interval_string() {
        let cube_1 = PgCube::from_str("((7),(8))").unwrap();
        assert_eq!(cube_1, PgCube::OneDimensionInterval(7., 8.));
        let cube_2 = PgCube::from_str("(7),(8)").unwrap();
        assert_eq!(cube_2, PgCube::OneDimensionInterval(7., 8.));
    }

    #[test]
    fn can_serialise_one_dimension_interval() {
        assert_eq!(
            PgCube::OneDimensionInterval(7., 8.).serialize_to_vec(),
            ONE_DIMENSIONAL_INTERVAL_BYTES
        )
    }

    #[test]
    fn can_deserialise_multi_dimension_2_dimension_byte() {
        let cube = PgCube::from_bytes(MULTI_DIMENSION_2_DIM_BYTES).unwrap();
        assert_eq!(
            cube,
            PgCube::MultiDimension(vec![vec![1., 2.], vec![3., 4.]])
        )
    }

    #[test]
    fn can_deserialise_multi_dimension_2_dimension_string() {
        let cube_1 = PgCube::from_str("((1,2),(3,4))").unwrap();
        assert_eq!(
            cube_1,
            PgCube::MultiDimension(vec![vec![1., 2.], vec![3., 4.]])
        );
        let cube_2 = PgCube::from_str("((1, 2), (3, 4))").unwrap();
        assert_eq!(
            cube_2,
            PgCube::MultiDimension(vec![vec![1., 2.], vec![3., 4.]])
        );
        let cube_3 = PgCube::from_str("(1,2),(3,4)").unwrap();
        assert_eq!(
            cube_3,
            PgCube::MultiDimension(vec![vec![1., 2.], vec![3., 4.]])
        );
        let cube_4 = PgCube::from_str("(1, 2), (3, 4)").unwrap();
        assert_eq!(
            cube_4,
            PgCube::MultiDimension(vec![vec![1., 2.], vec![3., 4.]])
        )
    }

    #[test]
    fn can_serialise_multi_dimension_2_dimension() {
        assert_eq!(
            PgCube::MultiDimension(vec![vec![1., 2.], vec![3., 4.]]).serialize_to_vec(),
            MULTI_DIMENSION_2_DIM_BYTES
        )
    }

    #[test]
    fn can_deserialise_multi_dimension_3_dimension_bytes() {
        let cube = PgCube::from_bytes(MULTI_DIMENSION_3_DIM_BYTES).unwrap();
        assert_eq!(
            cube,
            PgCube::MultiDimension(vec![vec![2., 3., 4.], vec![5., 6., 7.]])
        )
    }

    #[test]
    fn can_deserialise_multi_dimension_3_dimension_string() {
        let cube = PgCube::from_str("((2,3,4),(5,6,7))").unwrap();
        assert_eq!(
            cube,
            PgCube::MultiDimension(vec![vec![2., 3., 4.], vec![5., 6., 7.]])
        );
        let cube_2 = PgCube::from_str("(2,3,4),(5,6,7)").unwrap();
        assert_eq!(
            cube_2,
            PgCube::MultiDimension(vec![vec![2., 3., 4.], vec![5., 6., 7.]])
        );
    }

    #[test]
    fn can_serialise_multi_dimension_3_dimension() {
        assert_eq!(
            PgCube::MultiDimension(vec![vec![2., 3., 4.], vec![5., 6., 7.]]).serialize_to_vec(),
            MULTI_DIMENSION_3_DIM_BYTES
        )
    }
}
