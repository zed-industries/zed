// SPDX-License-Identifier: Apache-2.0

//! Serde serialization support for CBOR

mod error;

pub use error::Error;

use alloc::string::ToString;

use ciborium_io::Write;
use ciborium_ll::*;
use serde::{ser, Serialize as _};

struct Serializer<W: Write>(Encoder<W>);

impl<W: Write> From<W> for Serializer<W> {
    #[inline]
    fn from(writer: W) -> Self {
        Self(writer.into())
    }
}

impl<W: Write> From<Encoder<W>> for Serializer<W> {
    #[inline]
    fn from(writer: Encoder<W>) -> Self {
        Self(writer)
    }
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W>
where
    W::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = Error<W::Error>;

    type SerializeSeq = CollectionSerializer<'a, W>;
    type SerializeTuple = CollectionSerializer<'a, W>;
    type SerializeTupleStruct = CollectionSerializer<'a, W>;
    type SerializeTupleVariant = CollectionSerializer<'a, W>;
    type SerializeMap = CollectionSerializer<'a, W>;
    type SerializeStruct = CollectionSerializer<'a, W>;
    type SerializeStructVariant = CollectionSerializer<'a, W>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<(), Self::Error> {
        Ok(self.0.push(match v {
            false => Header::Simple(simple::FALSE),
            true => Header::Simple(simple::TRUE),
        })?)
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<(), Self::Error> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<(), Self::Error> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<(), Self::Error> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<(), Self::Error> {
        Ok(self.0.push(match v.is_negative() {
            false => Header::Positive(v as u64),
            true => Header::Negative(v as u64 ^ !0),
        })?)
    }

    #[inline]
    fn serialize_i128(self, v: i128) -> Result<(), Self::Error> {
        let (tag, raw) = match v.is_negative() {
            false => (tag::BIGPOS, v as u128),
            true => (tag::BIGNEG, v as u128 ^ !0),
        };

        match (tag, u64::try_from(raw)) {
            (tag::BIGPOS, Ok(x)) => return Ok(self.0.push(Header::Positive(x))?),
            (tag::BIGNEG, Ok(x)) => return Ok(self.0.push(Header::Negative(x))?),
            _ => {}
        }

        let bytes = raw.to_be_bytes();

        // Skip leading zeros.
        let mut slice = &bytes[..];
        while !slice.is_empty() && slice[0] == 0 {
            slice = &slice[1..];
        }

        self.0.push(Header::Tag(tag))?;
        self.0.push(Header::Bytes(Some(slice.len())))?;
        Ok(self.0.write_all(slice)?)
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<(), Self::Error> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<(), Self::Error> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<(), Self::Error> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<(), Self::Error> {
        Ok(self.0.push(Header::Positive(v))?)
    }

    #[inline]
    fn serialize_u128(self, v: u128) -> Result<(), Self::Error> {
        if let Ok(x) = u64::try_from(v) {
            return self.serialize_u64(x);
        }

        let bytes = v.to_be_bytes();

        // Skip leading zeros.
        let mut slice = &bytes[..];
        while !slice.is_empty() && slice[0] == 0 {
            slice = &slice[1..];
        }

        self.0.push(Header::Tag(tag::BIGPOS))?;
        self.0.push(Header::Bytes(Some(slice.len())))?;
        Ok(self.0.write_all(slice)?)
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<(), Self::Error> {
        self.serialize_f64(v.into())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<(), Self::Error> {
        Ok(self.0.push(Header::Float(v))?)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<(), Self::Error> {
        self.serialize_str(&v.to_string())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<(), Self::Error> {
        let bytes = v.as_bytes();
        self.0.push(Header::Text(bytes.len().into()))?;
        Ok(self.0.write_all(bytes)?)
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<(), Self::Error> {
        self.0.push(Header::Bytes(v.len().into()))?;
        Ok(self.0.write_all(v)?)
    }

    #[inline]
    fn serialize_none(self) -> Result<(), Self::Error> {
        Ok(self.0.push(Header::Simple(simple::NULL))?)
    }

    #[inline]
    fn serialize_some<U: ?Sized + ser::Serialize>(self, value: &U) -> Result<(), Self::Error> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<(), Self::Error> {
        self.serialize_none()
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<(), Self::Error> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<U: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        value: &U,
    ) -> Result<(), Self::Error> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<U: ?Sized + ser::Serialize>(
        self,
        name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &U,
    ) -> Result<(), Self::Error> {
        if name != "@@TAG@@" || variant != "@@UNTAGGED@@" {
            self.0.push(Header::Map(Some(1)))?;
            self.serialize_str(variant)?;
        }

        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, length: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.0.push(Header::Array(length))?;
        Ok(CollectionSerializer {
            encoder: self,
            ending: length.is_none(),
            tag: false,
        })
    }

    #[inline]
    fn serialize_tuple(self, length: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(length))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        length: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(length))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _index: u32,
        variant: &'static str,
        length: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        match (name, variant) {
            ("@@TAG@@", "@@TAGGED@@") => Ok(CollectionSerializer {
                encoder: self,
                ending: false,
                tag: true,
            }),

            _ => {
                self.0.push(Header::Map(Some(1)))?;
                self.serialize_str(variant)?;
                self.0.push(Header::Array(Some(length)))?;
                Ok(CollectionSerializer {
                    encoder: self,
                    ending: false,
                    tag: false,
                })
            }
        }
    }

    #[inline]
    fn serialize_map(self, length: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.0.push(Header::Map(length))?;
        Ok(CollectionSerializer {
            encoder: self,
            ending: length.is_none(),
            tag: false,
        })
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        length: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.0.push(Header::Map(Some(length)))?;
        Ok(CollectionSerializer {
            encoder: self,
            ending: false,
            tag: false,
        })
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        length: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.0.push(Header::Map(Some(1)))?;
        self.serialize_str(variant)?;
        self.0.push(Header::Map(Some(length)))?;
        Ok(CollectionSerializer {
            encoder: self,
            ending: false,
            tag: false,
        })
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

macro_rules! end {
    () => {
        #[inline]
        fn end(self) -> Result<(), Self::Error> {
            if self.ending {
                self.encoder.0.push(Header::Break)?;
            }

            Ok(())
        }
    };
}

struct CollectionSerializer<'a, W: Write> {
    encoder: &'a mut Serializer<W>,
    ending: bool,
    tag: bool,
}

impl<'a, W: Write> ser::SerializeSeq for CollectionSerializer<'a, W>
where
    W::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = Error<W::Error>;

    #[inline]
    fn serialize_element<U: ?Sized + ser::Serialize>(
        &mut self,
        value: &U,
    ) -> Result<(), Self::Error> {
        value.serialize(&mut *self.encoder)
    }

    end!();
}

impl<'a, W: Write> ser::SerializeTuple for CollectionSerializer<'a, W>
where
    W::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = Error<W::Error>;

    #[inline]
    fn serialize_element<U: ?Sized + ser::Serialize>(
        &mut self,
        value: &U,
    ) -> Result<(), Self::Error> {
        value.serialize(&mut *self.encoder)
    }

    end!();
}

impl<'a, W: Write> ser::SerializeTupleStruct for CollectionSerializer<'a, W>
where
    W::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = Error<W::Error>;

    #[inline]
    fn serialize_field<U: ?Sized + ser::Serialize>(
        &mut self,
        value: &U,
    ) -> Result<(), Self::Error> {
        value.serialize(&mut *self.encoder)
    }

    end!();
}

impl<'a, W: Write> ser::SerializeTupleVariant for CollectionSerializer<'a, W>
where
    W::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = Error<W::Error>;

    #[inline]
    fn serialize_field<U: ?Sized + ser::Serialize>(
        &mut self,
        value: &U,
    ) -> Result<(), Self::Error> {
        if !self.tag {
            return value.serialize(&mut *self.encoder);
        }

        self.tag = false;
        match value.serialize(crate::tag::Serializer) {
            Ok(x) => Ok(self.encoder.0.push(Header::Tag(x))?),
            _ => Err(Error::Value("expected tag".into())),
        }
    }

    end!();
}

impl<'a, W: Write> ser::SerializeMap for CollectionSerializer<'a, W>
where
    W::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = Error<W::Error>;

    #[inline]
    fn serialize_key<U: ?Sized + ser::Serialize>(&mut self, key: &U) -> Result<(), Self::Error> {
        key.serialize(&mut *self.encoder)
    }

    #[inline]
    fn serialize_value<U: ?Sized + ser::Serialize>(
        &mut self,
        value: &U,
    ) -> Result<(), Self::Error> {
        value.serialize(&mut *self.encoder)
    }

    end!();
}

impl<'a, W: Write> ser::SerializeStruct for CollectionSerializer<'a, W>
where
    W::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = Error<W::Error>;

    #[inline]
    fn serialize_field<U: ?Sized + ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &U,
    ) -> Result<(), Self::Error> {
        key.serialize(&mut *self.encoder)?;
        value.serialize(&mut *self.encoder)?;
        Ok(())
    }

    end!();
}

impl<'a, W: Write> ser::SerializeStructVariant for CollectionSerializer<'a, W>
where
    W::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = Error<W::Error>;

    #[inline]
    fn serialize_field<U: ?Sized + ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &U,
    ) -> Result<(), Self::Error> {
        key.serialize(&mut *self.encoder)?;
        value.serialize(&mut *self.encoder)
    }

    end!();
}

/// Serializes as CBOR into a type with [`impl ciborium_io::Write`](ciborium_io::Write)
#[inline]
pub fn into_writer<T: ?Sized + ser::Serialize, W: Write>(
    value: &T,
    writer: W,
) -> Result<(), Error<W::Error>>
where
    W::Error: core::fmt::Debug,
{
    let mut encoder = Serializer::from(writer);
    value.serialize(&mut encoder)
}
