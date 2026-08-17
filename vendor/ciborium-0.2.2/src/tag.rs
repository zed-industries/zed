//! Contains helper types for dealing with CBOR tags

use serde::{de, de::Error as _, forward_to_deserialize_any, ser, Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename = "@@TAG@@")]
enum Internal<T> {
    #[serde(rename = "@@UNTAGGED@@")]
    Untagged(T),

    #[serde(rename = "@@TAGGED@@")]
    Tagged(u64, T),
}

/// An optional CBOR tag and its data item
///
/// No semantic evaluation of the tag is made.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Captured<V>(pub Option<u64>, pub V);

impl<'de, V: Deserialize<'de>> Deserialize<'de> for Captured<V> {
    #[inline]
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Internal::deserialize(deserializer)? {
            Internal::Tagged(t, v) => Ok(Captured(Some(t), v)),
            Internal::Untagged(v) => Ok(Captured(None, v)),
        }
    }
}

impl<V: Serialize> Serialize for Captured<V> {
    #[inline]
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Some(tag) => Internal::Tagged(tag, &self.1).serialize(serializer),
            None => Internal::Untagged(&self.1).serialize(serializer),
        }
    }
}

/// A required CBOR tag
///
/// This data type indicates that the specified tag, and **only** that tag,
/// is required during deserialization. If the tag is missing, deserialization
/// will fail. The tag will always be emitted during serialization.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Required<V, const TAG: u64>(pub V);

impl<'de, V: Deserialize<'de>, const TAG: u64> Deserialize<'de> for Required<V, TAG> {
    #[inline]
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Internal::deserialize(deserializer)? {
            Internal::Tagged(t, v) if t == TAG => Ok(Required(v)),
            _ => Err(de::Error::custom("required tag not found")),
        }
    }
}

impl<V: Serialize, const TAG: u64> Serialize for Required<V, TAG> {
    #[inline]
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Internal::Tagged(TAG, &self.0).serialize(serializer)
    }
}

/// An optional CBOR tag
///
/// This data type indicates that the specified tag, and **only** that tag,
/// is accepted, but not required, during deserialization. The tag will always
/// be emitted during serialization.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Accepted<V, const TAG: u64>(pub V);

impl<'de, V: Deserialize<'de>, const TAG: u64> Deserialize<'de> for Accepted<V, TAG> {
    #[inline]
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Internal::deserialize(deserializer)? {
            Internal::Tagged(t, v) if t == TAG => Ok(Accepted(v)),
            Internal::Untagged(v) => Ok(Accepted(v)),
            _ => Err(de::Error::custom("required tag not found")),
        }
    }
}

impl<V: Serialize, const TAG: u64> Serialize for Accepted<V, TAG> {
    #[inline]
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Internal::Tagged(TAG, &self.0).serialize(serializer)
    }
}

pub(crate) struct TagAccess<D> {
    parent: Option<D>,
    state: usize,
    tag: Option<u64>,
}

impl<D> TagAccess<D> {
    pub fn new(parent: D, tag: Option<u64>) -> Self {
        Self {
            parent: Some(parent),
            state: 0,
            tag,
        }
    }
}

impl<'de, D: de::Deserializer<'de>> de::Deserializer<'de> for &mut TagAccess<D> {
    type Error = D::Error;

    #[inline]
    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.state += 1;

        match self.state {
            1 => visitor.visit_str(match self.tag {
                Some(..) => "@@TAGGED@@",
                None => "@@UNTAGGED@@",
            }),

            _ => visitor.visit_u64(self.tag.unwrap()),
        }
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128
        u8 u16 u32 u64 u128
        bool f32 f64
        char str string
        bytes byte_buf
        seq map
        struct tuple tuple_struct
        identifier ignored_any
        option unit unit_struct newtype_struct enum
    }
}

impl<'de, D: de::Deserializer<'de>> de::EnumAccess<'de> for TagAccess<D> {
    type Error = D::Error;
    type Variant = Self;

    #[inline]
    fn variant_seed<V: de::DeserializeSeed<'de>>(
        mut self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), Self::Error> {
        let variant = seed.deserialize(&mut self)?;
        Ok((variant, self))
    }
}

impl<'de, D: de::Deserializer<'de>> de::VariantAccess<'de> for TagAccess<D> {
    type Error = D::Error;

    #[inline]
    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(Self::Error::custom("expected tag"))
    }

    #[inline]
    fn newtype_variant_seed<U: de::DeserializeSeed<'de>>(
        mut self,
        seed: U,
    ) -> Result<U::Value, Self::Error> {
        seed.deserialize(self.parent.take().unwrap())
    }

    #[inline]
    fn tuple_variant<V: de::Visitor<'de>>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_seq(self)
    }

    #[inline]
    fn struct_variant<V: de::Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Err(Self::Error::custom("expected tag"))
    }
}

impl<'de, D: de::Deserializer<'de>> de::SeqAccess<'de> for TagAccess<D> {
    type Error = D::Error;

    #[inline]
    fn next_element_seed<T: de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error> {
        if self.state < 2 {
            return Ok(Some(seed.deserialize(self)?));
        }

        Ok(match self.parent.take() {
            Some(x) => Some(seed.deserialize(x)?),
            None => None,
        })
    }
}

#[derive(Debug)]
pub(crate) struct Error;

impl core::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ser::StdError for Error {}

impl ser::Error for Error {
    fn custom<U: core::fmt::Display>(_msg: U) -> Self {
        Error
    }
}

pub(crate) struct Serializer;

impl ser::Serializer for Serializer {
    type Ok = u64;
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    #[inline]
    fn serialize_bool(self, _: bool) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_i8(self, _: i8) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_i16(self, _: i16) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_i32(self, _: i32) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_i64(self, _: i64) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_i128(self, _: i128) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<u64, Self::Error> {
        Ok(v.into())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<u64, Self::Error> {
        Ok(v.into())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<u64, Self::Error> {
        Ok(v.into())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<u64, Self::Error> {
        Ok(v)
    }

    #[inline]
    fn serialize_u128(self, _: u128) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_f32(self, _: f32) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_f64(self, _: f64) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_char(self, _: char) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_str(self, _: &str) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_bytes(self, _: &[u8]) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_none(self) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_some<U: ?Sized + ser::Serialize>(self, _: &U) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_unit(self) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        _variant: &'static str,
    ) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_newtype_struct<U: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        _value: &U,
    ) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_newtype_variant<U: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        _index: u32,
        _variant: &'static str,
        _value: &U,
    ) -> Result<u64, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_seq(self, _length: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_tuple(self, _length: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        _variant: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_map(self, _length: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        _variant: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

impl ser::SerializeSeq for Serializer {
    type Ok = u64;
    type Error = Error;

    #[inline]
    fn serialize_element<U: ?Sized + ser::Serialize>(&mut self, _value: &U) -> Result<(), Error> {
        Err(Error)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error)
    }
}

impl ser::SerializeTuple for Serializer {
    type Ok = u64;
    type Error = Error;

    #[inline]
    fn serialize_element<U: ?Sized + ser::Serialize>(&mut self, _value: &U) -> Result<(), Error> {
        Err(Error)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error)
    }
}

impl ser::SerializeTupleStruct for Serializer {
    type Ok = u64;
    type Error = Error;

    #[inline]
    fn serialize_field<U: ?Sized + ser::Serialize>(&mut self, _value: &U) -> Result<(), Error> {
        Err(Error)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error)
    }
}

impl ser::SerializeTupleVariant for Serializer {
    type Ok = u64;
    type Error = Error;

    #[inline]
    fn serialize_field<U: ?Sized + ser::Serialize>(&mut self, _value: &U) -> Result<(), Error> {
        Err(Error)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error)
    }
}

impl ser::SerializeMap for Serializer {
    type Ok = u64;
    type Error = Error;

    #[inline]
    fn serialize_key<U: ?Sized + ser::Serialize>(&mut self, _key: &U) -> Result<(), Error> {
        Err(Error)
    }

    #[inline]
    fn serialize_value<U: ?Sized + ser::Serialize>(&mut self, _value: &U) -> Result<(), Error> {
        Err(Error)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error)
    }
}

impl ser::SerializeStruct for Serializer {
    type Ok = u64;
    type Error = Error;

    #[inline]
    fn serialize_field<U: ?Sized + ser::Serialize>(
        &mut self,
        _key: &'static str,
        _value: &U,
    ) -> Result<(), Error> {
        Err(Error)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error)
    }
}

impl ser::SerializeStructVariant for Serializer {
    type Ok = u64;
    type Error = Error;

    #[inline]
    fn serialize_field<U: ?Sized + ser::Serialize>(
        &mut self,
        _key: &'static str,
        _value: &U,
    ) -> Result<(), Self::Error> {
        Err(Error)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error)
    }
}
