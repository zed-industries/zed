//! Function for reading TIFF tags

use std::collections::HashMap;
use std::io::{self, Read, Seek};
use std::mem;
use std::str;

use super::stream::{ByteOrder, EndianReader};
use crate::tags::{IfdPointer, Tag, Type, ValueBuffer};
use crate::{TiffError, TiffFormatError, TiffResult};

use self::Value::{
    Ascii, Byte, Double, Float, Ifd, IfdBig, List, Rational, SRational, Short, Signed, SignedBig,
    SignedByte, SignedShort, Unsigned, UnsignedBig,
};

#[allow(unused_qualifications)]
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Value {
    Byte(u8),
    Short(u16),
    SignedByte(i8),
    SignedShort(i16),
    Signed(i32),
    SignedBig(i64),
    Unsigned(u32),
    UnsignedBig(u64),
    Float(f32),
    Double(f64),
    List(Vec<Value>),
    Rational(u32, u32),
    #[deprecated(
        note = "Not implemented in BigTIFF with a standard tag value",
        since = "0.11.1"
    )]
    RationalBig(u64, u64),
    SRational(i32, i32),
    #[deprecated(
        note = "Not implemented in BigTIFF with a standard tag value",
        since = "0.11.1"
    )]
    SRationalBig(i64, i64),
    Ascii(String),
    Ifd(u32),
    IfdBig(u64),
}

impl Value {
    pub fn into_u8(self) -> TiffResult<u8> {
        match self {
            Byte(val) => Ok(val),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }
    pub fn into_i8(self) -> TiffResult<i8> {
        match self {
            SignedByte(val) => Ok(val),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_u16(self) -> TiffResult<u16> {
        match self {
            Byte(val) => Ok(val.into()),
            Short(val) => Ok(val),
            Unsigned(val) => Ok(u16::try_from(val)?),
            UnsignedBig(val) => Ok(u16::try_from(val)?),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_i16(self) -> TiffResult<i16> {
        match self {
            SignedByte(val) => Ok(val.into()),
            SignedShort(val) => Ok(val),
            Signed(val) => Ok(i16::try_from(val)?),
            SignedBig(val) => Ok(i16::try_from(val)?),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_u32(self) -> TiffResult<u32> {
        match self {
            Byte(val) => Ok(val.into()),
            Short(val) => Ok(val.into()),
            Unsigned(val) => Ok(val),
            UnsignedBig(val) => Ok(u32::try_from(val)?),
            Ifd(val) => Ok(val),
            IfdBig(val) => Ok(u32::try_from(val)?),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_i32(self) -> TiffResult<i32> {
        match self {
            SignedByte(val) => Ok(val.into()),
            SignedShort(val) => Ok(val.into()),
            Signed(val) => Ok(val),
            SignedBig(val) => Ok(i32::try_from(val)?),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_u64(self) -> TiffResult<u64> {
        match self {
            Byte(val) => Ok(val.into()),
            Short(val) => Ok(val.into()),
            Unsigned(val) => Ok(val.into()),
            UnsignedBig(val) => Ok(val),
            Ifd(val) => Ok(val.into()),
            IfdBig(val) => Ok(val),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_i64(self) -> TiffResult<i64> {
        match self {
            SignedByte(val) => Ok(val.into()),
            SignedShort(val) => Ok(val.into()),
            Signed(val) => Ok(val.into()),
            SignedBig(val) => Ok(val),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_f32(self) -> TiffResult<f32> {
        match self {
            Float(val) => Ok(val),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_f64(self) -> TiffResult<f64> {
        match self {
            Double(val) => Ok(val),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    /// Turn this value into an `IfdPointer`.
    ///
    /// Notice that this does not take an argument, a 64-bit IFD is always allowed. If the
    /// difference is crucial and you do not want to be permissive you're expected to filter this
    /// out before.
    ///
    /// For compatibility the smaller sized tags should always be allowed i.e. you might use a
    /// non-bigtiff's directory and its tag types and move it straight to a bigtiff. For instance
    /// the SubIFD tag is defined as `LONG or IFD`:
    ///
    /// <https://web.archive.org/web/20181105221012/https://www.awaresystems.be/imaging/tiff/tifftags/subifds.html>
    pub fn into_ifd_pointer(self) -> TiffResult<IfdPointer> {
        match self {
            Unsigned(val) | Ifd(val) => Ok(IfdPointer(val.into())),
            IfdBig(val) => Ok(IfdPointer(val)),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_string(self) -> TiffResult<String> {
        match self {
            Ascii(val) => Ok(val),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_u32_vec(self) -> TiffResult<Vec<u32>> {
        match self {
            List(vec) => {
                let mut new_vec = Vec::with_capacity(vec.len());
                for v in vec {
                    new_vec.push(v.into_u32()?)
                }
                Ok(new_vec)
            }
            Byte(val) => Ok(vec![val.into()]),
            Short(val) => Ok(vec![val.into()]),
            Unsigned(val) => Ok(vec![val]),
            UnsignedBig(val) => Ok(vec![u32::try_from(val)?]),
            Rational(numerator, denominator) => Ok(vec![numerator, denominator]),
            #[expect(deprecated)]
            Value::RationalBig(numerator, denominator) => {
                Ok(vec![u32::try_from(numerator)?, u32::try_from(denominator)?])
            }
            Ifd(val) => Ok(vec![val]),
            IfdBig(val) => Ok(vec![u32::try_from(val)?]),
            Ascii(val) => Ok(val.chars().map(u32::from).collect()),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_u8_vec(self) -> TiffResult<Vec<u8>> {
        match self {
            List(vec) => {
                let mut new_vec = Vec::with_capacity(vec.len());
                for v in vec {
                    new_vec.push(v.into_u8()?)
                }
                Ok(new_vec)
            }
            Byte(val) => Ok(vec![val]),

            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_u16_vec(self) -> TiffResult<Vec<u16>> {
        match self {
            List(vec) => {
                let mut new_vec = Vec::with_capacity(vec.len());
                for v in vec {
                    new_vec.push(v.into_u16()?)
                }
                Ok(new_vec)
            }
            Byte(val) => Ok(vec![val.into()]),
            Short(val) => Ok(vec![val]),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_i32_vec(self) -> TiffResult<Vec<i32>> {
        match self {
            List(vec) => {
                let mut new_vec = Vec::with_capacity(vec.len());
                for v in vec {
                    match v {
                        SRational(numerator, denominator) => {
                            new_vec.push(numerator);
                            new_vec.push(denominator);
                        }
                        #[expect(deprecated)]
                        Value::SRationalBig(numerator, denominator) => {
                            new_vec.push(i32::try_from(numerator)?);
                            new_vec.push(i32::try_from(denominator)?);
                        }
                        _ => new_vec.push(v.into_i32()?),
                    }
                }
                Ok(new_vec)
            }
            SignedByte(val) => Ok(vec![val.into()]),
            SignedShort(val) => Ok(vec![val.into()]),
            Signed(val) => Ok(vec![val]),
            SignedBig(val) => Ok(vec![i32::try_from(val)?]),
            SRational(numerator, denominator) => Ok(vec![numerator, denominator]),
            #[expect(deprecated)]
            Value::SRationalBig(numerator, denominator) => {
                Ok(vec![i32::try_from(numerator)?, i32::try_from(denominator)?])
            }
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_f32_vec(self) -> TiffResult<Vec<f32>> {
        match self {
            List(vec) => {
                let mut new_vec = Vec::with_capacity(vec.len());
                for v in vec {
                    new_vec.push(v.into_f32()?)
                }
                Ok(new_vec)
            }
            Float(val) => Ok(vec![val]),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_f64_vec(self) -> TiffResult<Vec<f64>> {
        match self {
            List(vec) => {
                let mut new_vec = Vec::with_capacity(vec.len());
                for v in vec {
                    new_vec.push(v.into_f64()?)
                }
                Ok(new_vec)
            }
            Double(val) => Ok(vec![val]),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_u64_vec(self) -> TiffResult<Vec<u64>> {
        match self {
            List(vec) => {
                let mut new_vec = Vec::with_capacity(vec.len());
                for v in vec {
                    new_vec.push(v.into_u64()?)
                }
                Ok(new_vec)
            }
            Byte(val) => Ok(vec![val.into()]),
            Short(val) => Ok(vec![val.into()]),
            Unsigned(val) => Ok(vec![val.into()]),
            UnsignedBig(val) => Ok(vec![val]),
            Rational(numerator, denominator) => Ok(vec![numerator.into(), denominator.into()]),
            #[expect(deprecated)]
            Value::RationalBig(numerator, denominator) => Ok(vec![numerator, denominator]),
            Ifd(val) => Ok(vec![val.into()]),
            IfdBig(val) => Ok(vec![val]),
            Ascii(val) => Ok(val.chars().map(u32::from).map(u64::from).collect()),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_i64_vec(self) -> TiffResult<Vec<i64>> {
        match self {
            List(vec) => {
                let mut new_vec = Vec::with_capacity(vec.len());
                for v in vec {
                    match v {
                        SRational(numerator, denominator) => {
                            new_vec.push(numerator.into());
                            new_vec.push(denominator.into());
                        }
                        #[expect(deprecated)]
                        Value::SRationalBig(numerator, denominator) => {
                            new_vec.push(numerator);
                            new_vec.push(denominator);
                        }
                        _ => new_vec.push(v.into_i64()?),
                    }
                }
                Ok(new_vec)
            }
            SignedByte(val) => Ok(vec![val.into()]),
            SignedShort(val) => Ok(vec![val.into()]),
            Signed(val) => Ok(vec![val.into()]),
            SignedBig(val) => Ok(vec![val]),
            SRational(numerator, denominator) => Ok(vec![numerator.into(), denominator.into()]),
            #[expect(deprecated)]
            Value::SRationalBig(numerator, denominator) => Ok(vec![numerator, denominator]),
            _ => Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        }
    }

    pub fn into_ifd_vec(self) -> TiffResult<Vec<IfdPointer>> {
        let vec = match self {
            Unsigned(val) | Ifd(val) => return Ok(vec![IfdPointer(val.into())]),
            IfdBig(val) => return Ok(vec![IfdPointer(val)]),
            List(vec) => vec,
            _ => return Err(TiffError::FormatError(TiffFormatError::InvalidTypeForTag)),
        };

        vec.into_iter().map(Self::into_ifd_pointer).collect()
    }
}

/// A combination of type, count, and offset.
///
/// In a TIFF the data offset portion of an entry is used for inline data in case the length of the
/// encoded value does not exceed the size of the offset field. Since the size of the offset field
/// depends on the file kind (4 bytes for standard TIFF, 8 bytes for BigTIFF) the interpretation of
/// this struct is only complete in combination with file metadata.
#[derive(Clone)]
pub struct Entry {
    type_: Type,
    count: u64,
    offset: [u8; 8],
}

impl ::std::fmt::Debug for Entry {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        fmt.write_str(&format!(
            "Entry {{ type_: {:?}, count: {:?}, offset: {:?} }}",
            self.type_, self.count, &self.offset
        ))
    }
}

impl Entry {
    /// Create a new entry fit to be added to a standard TIFF IFD.
    pub fn new(type_: Type, count: u32, offset: [u8; 4]) -> Entry {
        let mut entry_off = [0u8; 8];
        entry_off[..4].copy_from_slice(&offset);
        Entry::new_u64(type_, count.into(), entry_off)
    }

    /// Create a new entry with data for a Big TIFF IFD.
    pub fn new_u64(type_: Type, count: u64, offset: [u8; 8]) -> Entry {
        Entry {
            type_,
            count,
            offset,
        }
    }

    pub fn field_type(&self) -> Type {
        self.type_
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub(crate) fn offset(&self) -> &[u8] {
        &self.offset
    }

    /// Returns a mem_reader for the offset/value field
    pub(crate) fn offset_field_reader(
        &self,
        byte_order: ByteOrder,
    ) -> EndianReader<io::Cursor<Vec<u8>>> {
        EndianReader::new(io::Cursor::new(self.offset.to_vec()), byte_order)
    }

    pub(crate) fn val<R: Read + Seek>(
        &self,
        limits: &super::Limits,
        bigtiff: bool,
        reader: &mut EndianReader<R>,
    ) -> TiffResult<Value> {
        // Case 1: there are no values so we can return immediately.
        if self.count == 0 {
            return Ok(List(Vec::new()));
        }

        let bo = reader.byte_order;
        let value_bytes = self.type_.value_bytes(self.count)?;

        // Case 2: there is one value.
        if self.count == 1 {
            // 2a: the value is 5-8 bytes and we're in BigTiff mode.
            if bigtiff && value_bytes > 4 && value_bytes <= 8 {
                return Ok(match self.type_ {
                    Type::LONG8 => UnsignedBig(self.offset_field_reader(bo).read_u64()?),
                    Type::SLONG8 => SignedBig(self.offset_field_reader(bo).read_i64()?),
                    Type::DOUBLE => Double(self.offset_field_reader(bo).read_f64()?),
                    Type::RATIONAL => {
                        let mut r = self.offset_field_reader(bo);
                        Rational(r.read_u32()?, r.read_u32()?)
                    }
                    Type::SRATIONAL => {
                        let mut r = self.offset_field_reader(bo);
                        SRational(r.read_i32()?, r.read_i32()?)
                    }
                    Type::IFD8 => IfdBig(self.offset_field_reader(bo).read_u64()?),
                    Type::BYTE
                    | Type::SBYTE
                    | Type::ASCII
                    | Type::UNDEFINED
                    | Type::SHORT
                    | Type::SSHORT
                    | Type::LONG
                    | Type::SLONG
                    | Type::FLOAT
                    | Type::IFD => unreachable!(),
                });
            }

            // 2b: the value is at most 4 bytes or doesn't fit in the offset field.
            return Ok(match self.type_ {
                Type::BYTE => Byte(self.offset[0]),
                Type::SBYTE => SignedByte(self.offset[0] as i8),
                Type::UNDEFINED => Byte(self.offset[0]),
                Type::SHORT => Short(self.offset_field_reader(bo).read_u16()?),
                Type::SSHORT => SignedShort(self.offset_field_reader(bo).read_i16()?),
                Type::LONG => Unsigned(self.offset_field_reader(bo).read_u32()?),
                Type::SLONG => Signed(self.offset_field_reader(bo).read_i32()?),
                Type::FLOAT => Float(self.offset_field_reader(bo).read_f32()?),
                Type::ASCII => {
                    if self.offset[0] == 0 {
                        Ascii("".to_string())
                    } else {
                        return Err(TiffError::FormatError(TiffFormatError::InvalidTag));
                    }
                }
                Type::LONG8 => {
                    reader.goto_offset(self.offset_field_reader(bo).read_u32()?.into())?;
                    UnsignedBig(reader.read_u64()?)
                }
                Type::SLONG8 => {
                    reader.goto_offset(self.offset_field_reader(bo).read_u32()?.into())?;
                    SignedBig(reader.read_i64()?)
                }
                Type::DOUBLE => {
                    reader.goto_offset(self.offset_field_reader(bo).read_u32()?.into())?;
                    Double(reader.read_f64()?)
                }
                Type::RATIONAL => {
                    reader.goto_offset(self.offset_field_reader(bo).read_u32()?.into())?;
                    Rational(reader.read_u32()?, reader.read_u32()?)
                }
                Type::SRATIONAL => {
                    reader.goto_offset(self.offset_field_reader(bo).read_u32()?.into())?;
                    SRational(reader.read_i32()?, reader.read_i32()?)
                }
                Type::IFD => Ifd(self.offset_field_reader(bo).read_u32()?),
                Type::IFD8 => {
                    reader.goto_offset(self.offset_field_reader(bo).read_u32()?.into())?;
                    IfdBig(reader.read_u64()?)
                }
            });
        }

        // Case 3: There is more than one value, but it fits in the offset field.
        if value_bytes <= 4 || bigtiff && value_bytes <= 8 {
            match self.type_ {
                Type::BYTE => return offset_to_bytes(self.count as usize, self),
                Type::SBYTE => return offset_to_sbytes(self.count as usize, self),
                Type::ASCII => {
                    let mut buf = vec![0; self.count as usize];
                    buf.copy_from_slice(&self.offset[..self.count as usize]);
                    if buf.is_ascii() && buf.ends_with(&[0]) {
                        let v = str::from_utf8(&buf)?;
                        let v = v.trim_matches(char::from(0));
                        return Ok(Ascii(v.into()));
                    } else {
                        return Err(TiffError::FormatError(TiffFormatError::InvalidTag));
                    }
                }
                Type::UNDEFINED => {
                    return Ok(List(
                        self.offset[0..self.count as usize]
                            .iter()
                            .map(|&b| Byte(b))
                            .collect(),
                    ));
                }
                Type::SHORT => {
                    let mut r = self.offset_field_reader(bo);
                    let mut v = Vec::new();
                    for _ in 0..self.count {
                        v.push(Short(r.read_u16()?));
                    }
                    return Ok(List(v));
                }
                Type::SSHORT => {
                    let mut r = self.offset_field_reader(bo);
                    let mut v = Vec::new();
                    for _ in 0..self.count {
                        v.push(SignedShort(r.read_i16()?));
                    }
                    return Ok(List(v));
                }
                Type::LONG => {
                    let mut r = self.offset_field_reader(bo);
                    let mut v = Vec::new();
                    for _ in 0..self.count {
                        v.push(Unsigned(r.read_u32()?));
                    }
                    return Ok(List(v));
                }
                Type::SLONG => {
                    let mut r = self.offset_field_reader(bo);
                    let mut v = Vec::new();
                    for _ in 0..self.count {
                        v.push(Signed(r.read_i32()?));
                    }
                    return Ok(List(v));
                }
                Type::FLOAT => {
                    let mut r = self.offset_field_reader(bo);
                    let mut v = Vec::new();
                    for _ in 0..self.count {
                        v.push(Float(r.read_f32()?));
                    }
                    return Ok(List(v));
                }
                Type::IFD => {
                    let mut r = self.offset_field_reader(bo);
                    let mut v = Vec::new();
                    for _ in 0..self.count {
                        v.push(Ifd(r.read_u32()?));
                    }
                    return Ok(List(v));
                }
                Type::LONG8
                | Type::SLONG8
                | Type::RATIONAL
                | Type::SRATIONAL
                | Type::DOUBLE
                | Type::IFD8 => {
                    unreachable!()
                }
            }
        }

        // Case 4: there is more than one value, and it doesn't fit in the offset field.
        let mut v;
        self.set_reader_offset_relative(bigtiff, reader, 0)?;

        match self.type_ {
            Type::BYTE | Type::UNDEFINED => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(bytes.iter().copied().map(Byte))
                })
            }
            Type::SBYTE => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(bytes.iter().copied().map(|v| SignedByte(v as i8)))
                })
            }
            Type::SHORT => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(
                        bytes
                            .chunks_exact(2)
                            .map(|ch| Short(u16::from_ne_bytes(ch.try_into().unwrap()))),
                    )
                })
            }
            Type::SSHORT => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(
                        bytes
                            .chunks_exact(2)
                            .map(|ch| SignedShort(i16::from_ne_bytes(ch.try_into().unwrap()))),
                    )
                })
            }
            Type::LONG => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(
                        bytes
                            .chunks_exact(4)
                            .map(|ch| Unsigned(u32::from_ne_bytes(ch.try_into().unwrap()))),
                    )
                })
            }
            Type::SLONG => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(
                        bytes
                            .chunks_exact(4)
                            .map(|ch| Signed(i32::from_ne_bytes(ch.try_into().unwrap()))),
                    )
                })
            }
            Type::FLOAT => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(
                        bytes
                            .chunks_exact(4)
                            .map(|ch| Float(f32::from_ne_bytes(ch.try_into().unwrap()))),
                    )
                })
            }
            Type::DOUBLE => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(
                        bytes
                            .chunks_exact(8)
                            .map(|ch| Double(f64::from_ne_bytes(ch.try_into().unwrap()))),
                    )
                })
            }
            Type::RATIONAL => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(bytes.chunks_exact(8).map(|ch| {
                        Rational(
                            u32::from_ne_bytes(ch[..4].try_into().unwrap()),
                            u32::from_ne_bytes(ch[4..].try_into().unwrap()),
                        )
                    }))
                })
            }
            Type::SRATIONAL => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(bytes.chunks_exact(8).map(|ch| {
                        SRational(
                            i32::from_ne_bytes(ch[..4].try_into().unwrap()),
                            i32::from_ne_bytes(ch[4..].try_into().unwrap()),
                        )
                    }))
                })
            }
            Type::LONG8 => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(
                        bytes
                            .chunks_exact(8)
                            .map(|ch| UnsignedBig(u64::from_ne_bytes(ch.try_into().unwrap()))),
                    )
                })
            }
            Type::SLONG8 => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(
                        bytes
                            .chunks_exact(8)
                            .map(|ch| SignedBig(i64::from_ne_bytes(ch.try_into().unwrap()))),
                    )
                })
            }
            Type::IFD => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(
                        bytes
                            .chunks_exact(4)
                            .map(|ch| Ifd(u32::from_ne_bytes(ch.try_into().unwrap()))),
                    )
                })
            }
            Type::IFD8 => {
                v = Self::vec_with_capacity(self.count, limits)?;
                self.decode_values(self.count, self.type_, reader, |bytes| {
                    v.extend(
                        bytes
                            .chunks_exact(8)
                            .map(|ch| IfdBig(u64::from_ne_bytes(ch.try_into().unwrap()))),
                    )
                })
            }
            Type::ASCII => {
                let n = usize::try_from(self.count)?;

                if n > limits.decoding_buffer_size {
                    return Err(dbg!(TiffError::LimitsExceeded));
                }

                let mut out = vec![0; n];
                reader.inner().read_exact(&mut out)?;
                // Strings may be null-terminated, so we trim anything downstream of the null byte
                if let Some(first) = out.iter().position(|&b| b == 0) {
                    out.truncate(first);
                }

                return Ok(Ascii(String::from_utf8(out)?));
            }
        }?;

        Ok(List(v))
    }

    pub(crate) fn buffered_value<R: Read + Seek>(
        &self,
        buf: &mut ValueBuffer,
        limits: &super::Limits,
        bigtiff: bool,
        reader: &mut EndianReader<R>,
    ) -> TiffResult<()> {
        if self.count == 0 {
            buf.assume_type(self.type_, 0, reader.byte_order);
            return Ok(());
        }

        let value_bytes = self.buffer_with_capacity(buf, limits)?;

        // Case 1: the value fits in the offset field.
        if value_bytes <= 4 || bigtiff && value_bytes <= 8 {
            let src = &self.offset[..value_bytes];
            buf.raw_bytes_mut()[..value_bytes].copy_from_slice(src);
            buf.assume_type(self.type_, self.count, reader.byte_order);

            return Ok(());
        }

        // Case 2: the value is stored in the reader at an offset.
        self.set_reader_offset_relative(bigtiff, reader, 0)?;

        // In case of an error we set the type and endianess.
        buf.assume_type(self.type_, 0, reader.byte_order);
        let target = &mut buf.raw_bytes_mut()[..value_bytes];
        // FIXME: if the read fails we have already grown to full size, which is not great.
        reader.inner().read_exact(target)?;
        buf.assume_type(self.type_, self.count, reader.byte_order);

        Ok(())
    }

    pub(crate) fn raw_value_at<R: Read + Seek>(
        &self,
        buf: &mut [u8],
        bigtiff: bool,
        reader: &mut EndianReader<R>,
        at: u64,
    ) -> TiffResult<usize> {
        if self.count == 0 {
            return Ok(0);
        }

        // We have no limits to handle, we do not allocate.
        let value_bytes = self.type_.value_bytes(self.count)?;

        // No bytes to fill into the buffer.
        if at >= value_bytes {
            return Ok(0);
        }

        // Case 1: the value fits in the offset field.
        if value_bytes <= 4 || bigtiff && value_bytes <= 8 {
            // `at < value_bytes` and `value_bytes <= 8` so casting is mathematical
            let src = &self.offset[..value_bytes as usize][at as usize..];
            let len = src.len().min(buf.len());
            buf[..len].copy_from_slice(&src[..len]);
            return Ok(value_bytes as usize);
        }

        // Case 2: the value is stored in the reader at an offset. We will find the offset
        // encoded in the entry, apply the relative start position and seek there.
        self.set_reader_offset_relative(bigtiff, reader, at)?;

        let remainder = value_bytes - at;
        let len = usize::try_from(remainder)
            .unwrap_or(usize::MAX)
            .min(buf.len());

        let target = &mut buf[..len];
        reader.inner().read_exact(target)?;

        // Design note: in a previous draft we would consume the rest of the bytes of this value
        // here (into a stack buffer if need be) to verify the stream itself. But in the end we
        // have `Seek` so we better verify this by seeking over the rest of the bytes, finding if
        // the stream continues that far. Even that is maybe bad if we wanted to provide a
        // async-adaptor that `WouldBlock` errors to fill back a read window then the seek is
        // poison to that, too.

        // So a really simple choice: The caller is responsible for handling the fact that this did
        // not verify the whole value. Attempt a 1-byte read at the end of the value instead?
        Ok(len)
    }

    // Returns `Ok(bytes)` if our value's bytes through type and count fit into `usize` and are
    // within the limits. Extends the buffer to that many bytes.
    fn buffer_with_capacity(
        &self,
        buf: &mut ValueBuffer,
        limits: &super::Limits,
    ) -> TiffResult<usize> {
        let bytes = self.type_.value_bytes(self.count())?;

        let allowed_length = usize::try_from(bytes)
            .ok()
            .filter(|&n| n <= limits.decoding_buffer_size)
            .ok_or(TiffError::LimitsExceeded)?;

        buf.prepare_length(allowed_length);

        Ok(allowed_length)
    }

    fn vec_with_capacity(
        value_count: u64,
        limits: &super::Limits,
    ) -> Result<Vec<Value>, TiffError> {
        let value_count = usize::try_from(value_count)?;

        if value_count > limits.decoding_buffer_size / mem::size_of::<Value>() {
            return Err(TiffError::LimitsExceeded);
        }

        Ok(Vec::with_capacity(value_count))
    }

    /// Seek to an offset within a value stored in the offset defined by this entry.
    fn set_reader_offset_relative<R>(
        &self,
        bigtiff: bool,
        reader: &mut EndianReader<R>,
        at: u64,
    ) -> TiffResult<()>
    where
        R: Read + Seek,
    {
        let bo = reader.byte_order;

        let offset = if bigtiff {
            self.offset_field_reader(bo).read_u64()?
        } else {
            self.offset_field_reader(bo).read_u32()?.into()
        };

        // FIXME: `at` should be within `self.type_.value_bytes(self.count)` and that itself should
        // be within the bounds of the stream. But we do not check this eagerly so this below will
        // fail sometimes differently for exotic streams, depending on the method by which we read
        // (at once or through multiple raw into-byte-slice reads).
        let offset = offset.checked_add(at).ok_or(TiffError::FormatError(
            TiffFormatError::InconsistentSizesEncountered,
        ))?;

        reader.goto_offset(offset)?;

        Ok(())
    }

    #[inline]
    fn decode_values<R, F>(
        &self,
        value_count: u64,
        type_: Type,
        reader: &mut EndianReader<R>,
        mut collect: F,
    ) -> TiffResult<()>
    where
        R: Read + Seek,
        F: FnMut(&[u8]),
    {
        let mut total_bytes = type_.value_bytes(value_count)?;
        let mut buffer = [0u8; 512];

        let buf_unit = usize::from(type_.byte_len());
        let mul_of_ty = buffer.len() / buf_unit * buf_unit;

        let cls = type_.endian_bytes();
        let native = ByteOrder::native();

        while total_bytes > 0 {
            // `now <= mul_of_ty < 512` so casting is mathematical
            let now = total_bytes.min(mul_of_ty as u64);
            total_bytes -= now;

            let buffer = &mut buffer[..now as usize];
            reader.inner().read_exact(buffer)?;

            reader.byte_order.convert_endian_bytes(cls, buffer, native);
            collect(buffer);
        }

        Ok(())
    }
}

/// Extracts a list of BYTE tags stored in an offset
#[inline]
fn offset_to_bytes(n: usize, entry: &Entry) -> TiffResult<Value> {
    Ok(List(
        entry.offset[0..n]
            .iter()
            .map(|&e| Unsigned(u32::from(e)))
            .collect(),
    ))
}

/// Extracts a list of SBYTE tags stored in an offset
#[inline]
fn offset_to_sbytes(n: usize, entry: &Entry) -> TiffResult<Value> {
    Ok(List(
        entry.offset[0..n]
            .iter()
            .map(|&e| Signed(i32::from(e as i8)))
            .collect(),
    ))
}

/// Type representing an Image File Directory
#[doc(hidden)]
#[deprecated = "Use struct `tiff::Directory` instead which contains all fields relevant to an Image File Directory, including the offset to the next directory"]
pub type Directory = HashMap<Tag, Entry>;
