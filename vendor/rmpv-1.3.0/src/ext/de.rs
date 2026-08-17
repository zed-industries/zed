use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::iter::ExactSizeIterator;
use std::slice::Iter;
use std::vec::IntoIter;

use serde::de::{self, DeserializeSeed, IntoDeserializer, SeqAccess, Unexpected, Visitor};
use serde::forward_to_deserialize_any;
use serde::{self, Deserialize, Deserializer};

use crate::{IntPriv, Integer, Utf8String, Utf8StringRef, Value, ValueRef};

use super::{Error, ValueExt};
use crate::MSGPACK_EXT_STRUCT_NAME;

#[inline]
pub fn from_value<T>(val: Value) -> Result<T, Error>
    where T: for<'de> Deserialize<'de>
{
    deserialize_from(val)
}

#[inline]
pub fn deserialize_from<'de, T, D>(val: D) -> Result<T, Error>
    where T: Deserialize<'de>,
          D: Deserializer<'de, Error = Error>
{
    Deserialize::deserialize(val)
}

impl de::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Self {
        Error::Syntax(format!("{msg}"))
    }
}

impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct ValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ValueVisitor {
            type Value = Value;

            #[cold]
            fn expecting(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
                "any valid MessagePack value".fmt(fmt)
            }

            #[inline]
            fn visit_some<D>(self, de: D) -> Result<Value, D::Error>
                where D: de::Deserializer<'de>
            {
                Deserialize::deserialize(de)
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Value, E> {
                Ok(Value::Nil)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Value, E> {
                Ok(Value::Nil)
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                Ok(Value::Boolean(value))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
                Ok(Value::from(value))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                Ok(Value::from(value))
            }

            #[inline]
            fn visit_f32<E>(self, value: f32) -> Result<Value, E> {
                Ok(Value::F32(value))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                Ok(Value::F64(value))
            }

            #[inline]
            fn visit_string<E>(self, value: String) -> Result<Value, E> {
                Ok(Value::String(Utf8String::from(value)))
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Value, E>
                where E: de::Error
            {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
                where V: SeqAccess<'de>
            {
                let mut vec = Vec::new();
                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }
                Ok(Value::Array(vec))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(Value::Binary(v.to_owned()))
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(Value::Binary(v))
            }

            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
                where V: de::MapAccess<'de>
            {
                let mut pairs = vec![];

                while let Some(key) = visitor.next_key()? {
                    let val = visitor.next_value()?;
                    pairs.push((key, val));
                }

                Ok(Value::Map(pairs))
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where D: Deserializer<'de>,
            {
                struct ExtValueVisitor;
                impl<'de> serde::de::Visitor<'de> for ExtValueVisitor {
                    type Value = Value;

                    #[cold]
                    fn expecting(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
                        "a valid MessagePack Ext".fmt(fmt)
                    }

                    #[inline]
                    fn visit_seq<V>(self, mut seq: V) -> Result<Value, V::Error>
                        where V: SeqAccess<'de>
                    {
                        let tag = seq.next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let bytes: serde_bytes::ByteBuf = seq.next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                        Ok(Value::Ext(tag, bytes.to_vec()))
                    }
                }

                deserializer.deserialize_tuple(2, ExtValueVisitor)
            }
        }

        de.deserialize_any(ValueVisitor)
    }
}

impl<'de> Deserialize<'de> for ValueRef<'de> {
    #[inline]
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct ValueVisitor;

        impl<'de> de::Visitor<'de> for ValueVisitor {
            type Value = ValueRef<'de>;

            #[cold]
            fn expecting(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
                "any valid MessagePack value".fmt(fmt)
            }

            #[inline]
            fn visit_some<D>(self, de: D) -> Result<Self::Value, D::Error>
                where D: Deserializer<'de>
            {
                Deserialize::deserialize(de)
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(ValueRef::Nil)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(ValueRef::Nil)
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
                Ok(ValueRef::Boolean(value))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(ValueRef::from(value))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(ValueRef::from(value))
            }

            #[inline]
            fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
                Ok(ValueRef::F32(value))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(ValueRef::F64(value))
            }

            #[inline]
            fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(ValueRef::String(Utf8StringRef::from(value)))
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
                where V: SeqAccess<'de>
            {
                let mut vec = Vec::new();

                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }

                Ok(ValueRef::Array(vec))
            }

            #[inline]
            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(ValueRef::Binary(v))
            }

            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
                where V: de::MapAccess<'de>
            {
                let mut vec = Vec::new();

                while let Some(key) = visitor.next_key()? {
                    let val = visitor.next_value()?;
                    vec.push((key, val));
                }

                Ok(ValueRef::Map(vec))
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where D: Deserializer<'de>,
            {
                struct ExtValueRefVisitor;
                impl<'de> serde::de::Visitor<'de> for ExtValueRefVisitor {
                    type Value = ValueRef<'de>;

                    fn expecting(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
                        "a valid MessagePack Ext".fmt(fmt)
                    }

                    #[inline]
                    fn visit_seq<V>(self, mut seq: V) -> Result<ValueRef<'de>, V::Error>
                        where V: SeqAccess<'de>
                    {
                        let tag = seq.next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &"invalid ext sequence"))?;
                        let bytes: &[u8] = seq.next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &"invalid ext sequence"))?;

                        Ok(ValueRef::Ext(tag, bytes))
                    }
                }

                deserializer.deserialize_tuple(2, ExtValueRefVisitor)
            }
        }

        de.deserialize_any(ValueVisitor)
    }
}

impl<'de> Deserializer<'de> for Value {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        match self {
            Value::Nil => visitor.visit_unit(),
            Value::Boolean(v) => visitor.visit_bool(v),
            Value::Integer(Integer { n }) => match n {
                IntPriv::PosInt(v) => visitor.visit_u64(v),
                IntPriv::NegInt(v) => visitor.visit_i64(v),
            },
            Value::F32(v) => visitor.visit_f32(v),
            Value::F64(v) => visitor.visit_f64(v),
            Value::String(v) => match v.s {
                Ok(v) => visitor.visit_string(v),
                Err(v) => visitor.visit_byte_buf(v.0),
            },
            Value::Binary(v) => visitor.visit_byte_buf(v),
            Value::Array(v) => {
                let len = v.len();
                let mut de = SeqDeserializer::new(v.into_iter());
                let seq = visitor.visit_seq(&mut de)?;
                if de.iter.len() == 0 {
                    Ok(seq)
                } else {
                    Err(de::Error::invalid_length(len, &"fewer elements in array"))
                }
            }
            Value::Map(v) => {
                let len = v.len();
                let mut de = MapDeserializer::new(v.into_iter());
                let map = visitor.visit_map(&mut de)?;
                if de.iter.len() == 0 {
                    Ok(map)
                } else {
                    Err(de::Error::invalid_length(len, &"fewer elements in map"))
                }
            }
            Value::Ext(tag, data) => {
                let de = ExtDeserializer::new_owned(tag, data);
                visitor.visit_newtype_struct(de)
            }
        }
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        ValueBase::deserialize_option(self, visitor)
    }

    #[inline]
    fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        ValueBase::deserialize_enum(self, visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        if name == MSGPACK_EXT_STRUCT_NAME {
            match self {
                Value::Ext(tag, data) => {
                    let ext_de = ExtDeserializer::new_owned(tag, data);
                    return visitor.visit_newtype_struct(ext_de);
                }
                other => return Err(de::Error::invalid_type(other.unexpected(), &"expected Ext")),
            }
        }

        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        ValueBase::deserialize_unit_struct(self, visitor)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit seq
        bytes byte_buf map tuple_struct struct
        identifier tuple ignored_any
    }
}

impl<'de> Deserializer<'de> for ValueRef<'de> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        match self {
            ValueRef::Nil => visitor.visit_unit(),
            ValueRef::Boolean(v) => visitor.visit_bool(v),
            ValueRef::Integer(Integer { n }) => match n {
                IntPriv::PosInt(v) => visitor.visit_u64(v),
                IntPriv::NegInt(v) => visitor.visit_i64(v),
            },
            ValueRef::F32(v) => visitor.visit_f32(v),
            ValueRef::F64(v) => visitor.visit_f64(v),
            ValueRef::String(v) => match v.s {
                Ok(v) => visitor.visit_borrowed_str(v),
                Err(v) => visitor.visit_borrowed_bytes(v.0),
            },
            ValueRef::Binary(v) => visitor.visit_borrowed_bytes(v),
            ValueRef::Array(v) => {
                let len = v.len();
                let mut de = SeqDeserializer::new(v.into_iter());
                let seq = visitor.visit_seq(&mut de)?;
                if de.iter.len() == 0 {
                    Ok(seq)
                } else {
                    Err(de::Error::invalid_length(len, &"fewer elements in array"))
                }
            }
            ValueRef::Map(v) => {
                let len = v.len();
                let mut de = MapDeserializer::new(v.into_iter());
                let map = visitor.visit_map(&mut de)?;
                if de.iter.len() == 0 {
                    Ok(map)
                } else {
                    Err(de::Error::invalid_length(len, &"fewer elements in map"))
                }
            }
            ValueRef::Ext(tag, data) => {
                let de = ExtDeserializer::new_ref(tag, data);
                visitor.visit_newtype_struct(de)
            }
        }
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        ValueBase::deserialize_option(self, visitor)
    }

    #[inline]
    fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        ValueBase::deserialize_enum(self, visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        if name == MSGPACK_EXT_STRUCT_NAME {
            match self {
                ValueRef::Ext(tag, data) => {
                    let ext_de = ExtDeserializer::new_ref(tag, data);
                    return visitor.visit_newtype_struct(ext_de);
                }
                other => return Err(de::Error::invalid_type(other.unexpected(), &"expected Ext")),
            }
        }

        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        ValueBase::deserialize_unit_struct(self, visitor)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit seq
        bytes byte_buf map tuple_struct struct
        identifier tuple ignored_any
    }
}

impl<'de> Deserializer<'de> for &'de ValueRef<'de> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        match *self {
            ValueRef::Nil => visitor.visit_unit(),
            ValueRef::Boolean(v) => visitor.visit_bool(v),
            ValueRef::Integer(Integer { n }) => match n {
                IntPriv::PosInt(v) => visitor.visit_u64(v),
                IntPriv::NegInt(v) => visitor.visit_i64(v),
            },
            ValueRef::F32(v) => visitor.visit_f32(v),
            ValueRef::F64(v) => visitor.visit_f64(v),
            ValueRef::String(v) => match v.s {
                Ok(v) => visitor.visit_borrowed_str(v),
                Err(v) => visitor.visit_borrowed_bytes(v.0),
            },
            ValueRef::Binary(v) => visitor.visit_borrowed_bytes(v),
            ValueRef::Array(ref v) => {
                let len = v.len();
                let mut de = SeqDeserializer::new(v.iter());
                let seq = visitor.visit_seq(&mut de)?;
                if de.iter.len() == 0 {
                    Ok(seq)
                } else {
                    Err(de::Error::invalid_length(len, &"fewer elements in array"))
                }
            }
            ValueRef::Map(ref v) => {
                let len = v.len();
                let mut de = MapRefDeserializer::new(v.iter());
                let map = visitor.visit_map(&mut de)?;
                if de.iter.len() == 0 {
                    Ok(map)
                } else {
                    Err(de::Error::invalid_length(len, &"fewer elements in map"))
                }
            }
            ValueRef::Ext(tag, data) => {
                let de = ExtDeserializer::new_ref(tag, data);
                visitor.visit_newtype_struct(de)
            }
        }
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        if let ValueRef::Nil = *self {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    #[inline]
    fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        match self {
            ValueRef::Array(v) => {
                let len = v.len();
                let mut iter = v.iter();
                if !(len == 1 || len == 2) {
                    return Err(de::Error::invalid_length(len, &"array with one or two elements"));
                }

                let id = match iter.next() {
                    Some(id) => deserialize_from(id)?,
                    None => {
                        return Err(de::Error::invalid_length(len, &"array with one or two elements"));
                    }
                };

                visitor.visit_enum(EnumRefDeserializer::new(id, iter.next()))
            }
            other => Err(de::Error::invalid_type(other.unexpected(), &"array, map or int")),
        }
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        if name == MSGPACK_EXT_STRUCT_NAME {
            match self {
                ValueRef::Ext(tag, data) => {
                    let ext_de = ExtDeserializer::new_ref(*tag, data);
                    return visitor.visit_newtype_struct(ext_de);
                }
                other => return Err(de::Error::invalid_type(other.unexpected(), &"expected Ext")),
            }
        }

        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        match self {
            ValueRef::Array(v) => {
                if v.is_empty() {
                    visitor.visit_unit()
                } else {
                    Err(de::Error::invalid_length(v.len(), &"empty array"))
                }
            }
            other => Err(de::Error::invalid_type(other.unexpected(), &"empty array")),
        }
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit seq
        bytes byte_buf map tuple_struct struct
        identifier tuple ignored_any
    }
}

struct ExtDeserializer<'de> {
    tag: Option<i8>,
    data: Option<Cow<'de, [u8]>>,
}

impl<'de> ExtDeserializer<'de> {
    fn new_owned(tag: i8, data: Vec<u8>) -> Self {
        ExtDeserializer {
            tag: Some(tag),
            data: Some(Cow::Owned(data)),
        }
    }

    fn new_ref(tag: i8, data: &'de [u8]) -> Self {
        ExtDeserializer {
            tag: Some(tag),
            data: Some(Cow::Borrowed(data)),
        }
    }
}

impl<'de> SeqAccess<'de> for ExtDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.tag.is_some() || self.data.is_some() {
            return Ok(Some(seed.deserialize(self)?));
        }

        Ok(None)
    }
}

/// Deserializer for Ext (expecting sequence)
impl<'a, 'de: 'a> Deserializer<'de> for ExtDeserializer<'de> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        visitor.visit_seq(self)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        struct identifier tuple enum ignored_any tuple_struct
    }
}

/// Deserializer for Ext `SeqAccess` elements
impl<'a, 'de: 'a> Deserializer<'de> for &'a mut ExtDeserializer<'de> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        if self.tag.is_some() {
            let tag = self.tag.take().unwrap();
            visitor.visit_i8(tag)
        } else if self.data.is_some() {
            let data = self.data.take().unwrap();
            match data {
                Cow::Owned(data) => visitor.visit_byte_buf(data),
                Cow::Borrowed(data) => visitor.visit_borrowed_bytes(data),
            }
        } else {
            debug_assert!(false, "ext seq only has two elements");
            Err(Error::Syntax(String::new()))
        }
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        tuple_struct struct identifier tuple enum ignored_any
    }
}

struct SeqDeserializer<I> {
    iter: I,
}

impl<I> SeqDeserializer<I> {
    fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<'de, I, U> SeqAccess<'de> for SeqDeserializer<I>
    where I: Iterator<Item = U>,
          U: Deserializer<'de, Error = Error>
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where T: de::DeserializeSeed<'de>
    {
        match self.iter.next() {
            Some(val) => seed.deserialize(val).map(Some),
            None => Ok(None),
        }
    }
}

impl<'de, I, U> Deserializer<'de> for SeqDeserializer<I>
    where I: ExactSizeIterator<Item = U>,
          U: Deserializer<'de, Error = Error>
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        let len = self.iter.len();
        if len == 0 {
            visitor.visit_unit()
        } else {
            let ret = visitor.visit_seq(&mut self)?;
            let rem = self.iter.len();
            if rem == 0 {
                Ok(ret)
            } else {
                Err(de::Error::invalid_length(len, &"fewer elements in array"))
            }
        }
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        tuple_struct struct identifier tuple enum ignored_any
    }
}

struct MapDeserializer<I, U> {
    val: Option<U>,
    iter: I,
}

impl<I, U> MapDeserializer<I, U> {
    fn new(iter: I) -> Self {
        Self { val: None, iter }
    }
}

impl<'de, I, U> de::MapAccess<'de> for MapDeserializer<I, U>
    where I: Iterator<Item = (U, U)>,
          U: ValueBase<'de>
{
    type Error = Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where T: DeserializeSeed<'de>
    {
        match self.iter.next() {
            Some((key, val)) => {
                self.val = Some(val);
                seed.deserialize(key).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
        where T: DeserializeSeed<'de>
    {
        match self.val.take() {
            Some(val) => seed.deserialize(val),
            None => Err(de::Error::custom("value is missing")),
        }
    }
}

impl<'de, I, U> Deserializer<'de> for MapDeserializer<I, U>
    where I: Iterator<Item = (U, U)>,
          U: ValueBase<'de>
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        tuple_struct struct identifier tuple enum ignored_any
    }
}

struct EnumDeserializer<U> {
    id: u32,
    value: Option<U>,
}

impl<U> EnumDeserializer<U> {
    pub fn new(id: u32, value: Option<U>) -> Self {
        Self { id, value }
    }
}

impl<'de, U: ValueBase<'de> + ValueExt> de::EnumAccess<'de> for EnumDeserializer<U> {
    type Error = Error;
    type Variant = VariantDeserializer<U>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
        where V: de::DeserializeSeed<'de>
    {
        let variant = self.id.into_deserializer();
        let visitor = VariantDeserializer { value: self.value };
        seed.deserialize(variant).map(|v| (v, visitor))
    }
}

struct VariantDeserializer<U> {
    value: Option<U>,
}

impl<'de, U: ValueBase<'de> + ValueExt> de::VariantAccess<'de> for VariantDeserializer<U> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        // Can accept only [u32].
        match self.value {
            Some(v) => {
                match v.into_iter() {
                    Ok(ref v) if v.len() == 0 => Ok(()),
                    Ok(..) => Err(de::Error::invalid_value(Unexpected::Seq, &"empty array")),
                    Err(v) => Err(de::Error::invalid_value(v.unexpected(), &"empty array")),
                }
            }
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
        where T: de::DeserializeSeed<'de>
    {
        // Can accept both [u32, T...] and [u32, [T]] cases.
        match self.value {
            Some(v) => {
                match v.into_iter() {
                    Ok(mut iter) => {
                        if iter.len() > 1 {
                            seed.deserialize(SeqDeserializer::new(iter))
                        } else {
                            let val = match iter.next() {
                                Some(val) => seed.deserialize(val),
                                None => return Err(de::Error::invalid_value(Unexpected::Seq, &"array with one element")),
                            };

                            if iter.next().is_some() {
                                Err(de::Error::invalid_value(Unexpected::Seq, &"array with one element"))
                            } else {
                                val
                            }
                        }
                    }
                    Err(v) => seed.deserialize(v),
                }
            }
            None => Err(de::Error::invalid_type(Unexpected::UnitVariant, &"newtype variant")),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
        where V: Visitor<'de>
    {
        // Can accept [u32, [T...]].
        match self.value {
            Some(v) => {
                match v.into_iter() {
                    Ok(v) => Deserializer::deserialize_any(SeqDeserializer::new(v), visitor),
                    Err(v) => Err(de::Error::invalid_type(v.unexpected(), &"tuple variant")),
                }
            }
            None => Err(de::Error::invalid_type(Unexpected::UnitVariant, &"tuple variant"))
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Error>
        where V: Visitor<'de>,
    {
        match self.value {
            Some(v) => {
                match v.into_iter() {
                    Ok(iter) => Deserializer::deserialize_any(SeqDeserializer::new(iter), visitor),
                    Err(v) => {
                        match v.into_map_iter() {
                            Ok(iter) => Deserializer::deserialize_any(MapDeserializer::new(iter), visitor),
                            Err(v) => Err(de::Error::invalid_type(v.unexpected(), &"struct variant")),
                        }
                    }
                }
            }
            None => Err(de::Error::invalid_type(Unexpected::UnitVariant, &"struct variant"))
        }
    }
}

pub struct MapRefDeserializer<'de> {
    val: Option<&'de ValueRef<'de>>,
    iter: Iter<'de, (ValueRef<'de>, ValueRef<'de>)>,
}

impl<'de> MapRefDeserializer<'de> {
    fn new(iter: Iter<'de, (ValueRef<'de>, ValueRef<'de>)>) -> Self {
        Self { val: None, iter }
    }
}

impl<'de> de::MapAccess<'de> for MapRefDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where T: DeserializeSeed<'de>
    {
        match self.iter.next() {
            Some((key, val)) => {
                self.val = Some(val);
                seed.deserialize(key).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
        where T: DeserializeSeed<'de>
    {
        match self.val.take() {
            Some(val) => seed.deserialize(val),
            None => Err(de::Error::custom("value is missing")),
        }
    }
}

impl<'de> Deserializer<'de> for MapRefDeserializer<'de> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        tuple_struct struct identifier tuple enum ignored_any
    }
}

pub struct EnumRefDeserializer<'de> {
    id: u32,
    value: Option<&'de ValueRef<'de>>,
}

impl<'de> EnumRefDeserializer<'de> {
    #[must_use] pub fn new(id: u32, value: Option<&'de ValueRef<'de>>) -> Self {
        Self { id, value }
    }
}

impl<'de> de::EnumAccess<'de> for EnumRefDeserializer<'de> {
    type Error = Error;
    type Variant = VariantRefDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
        where V: de::DeserializeSeed<'de>
    {
        let variant = self.id.into_deserializer();
        let visitor = VariantRefDeserializer { value: self.value };
        seed.deserialize(variant).map(|v| (v, visitor))
    }
}

pub struct VariantRefDeserializer<'de> {
    value: Option<&'de ValueRef<'de>>,
}

impl<'de> de::VariantAccess<'de> for VariantRefDeserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        // Can accept only [u32].
        match self.value {
            Some(ValueRef::Array(v)) => {
                if v.is_empty() {
                    Ok(())
                } else {
                    Err(de::Error::invalid_value(Unexpected::Seq, &"empty array"))
                }
            }
            Some(v) => Err(de::Error::invalid_value(v.unexpected(), &"empty array")),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
        where T: de::DeserializeSeed<'de>
    {
        // Can accept both [u32, T...] and [u32, [T]] cases.
        match self.value {
            Some(ValueRef::Array(v)) => {
                let len = v.len();
                let mut iter = v.iter();
                if len > 1 {
                    seed.deserialize(SeqDeserializer::new(iter))
                } else {
                    let val = match iter.next() {
                        Some(val) => seed.deserialize(val),
                        None => return Err(de::Error::invalid_length(len, &"array with one element")),
                    };

                    if iter.next().is_some() {
                        Err(de::Error::invalid_length(len, &"array with one element"))
                    } else {
                        val
                    }
                }
            }
            Some(v) => seed.deserialize(v),
            None => Err(de::Error::invalid_type(Unexpected::UnitVariant, &"newtype variant")),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
        where V: Visitor<'de>
    {
        // Can accept [u32, [T...]].
        match self.value {
            Some(ValueRef::Array(v)) => {
                Deserializer::deserialize_any(SeqDeserializer::new(v.iter()), visitor)
            }
            Some(v) => Err(de::Error::invalid_type(v.unexpected(), &"tuple variant")),
            None => Err(de::Error::invalid_type(Unexpected::UnitVariant, &"tuple variant"))
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Error>
        where V: Visitor<'de>,
    {
        match self.value {
            Some(ValueRef::Array(v)) => {
                Deserializer::deserialize_any(SeqDeserializer::new(v.iter()), visitor)
            }
            Some(ValueRef::Map(v)) => {
                Deserializer::deserialize_any(MapRefDeserializer::new(v.iter()), visitor)
            }
            Some(v) => Err(de::Error::invalid_type(v.unexpected(), &"struct variant")),
            None => Err(de::Error::invalid_type(Unexpected::UnitVariant, &"struct variant"))
        }
    }
}

// TODO: Ugly hack. Needed for avoiding copy-pasting similar code, but I don't like it.
trait ValueBase<'de>: Deserializer<'de, Error = Error> + ValueExt {
    type Item: ValueBase<'de>;
    type Iter: ExactSizeIterator<Item = Self::Item>;
    type MapIter: Iterator<Item = (Self::Item, Self::Item)>;
    type MapDeserializer: Deserializer<'de>;

    fn is_nil(&self) -> bool;

    fn into_iter(self) -> Result<Self::Iter, Self::Item>;
    fn into_map_iter(self) -> Result<Self::MapIter, Self::Item>;

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        if self.is_nil() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    #[inline]
    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        match self.into_iter() {
            Ok(mut iter) => {
                if !(iter.len() == 1 || iter.len() == 2) {
                    return Err(de::Error::invalid_length(iter.len(), &"array with one or two elements"));
                }

                let id = match iter.next() {
                    Some(id) => deserialize_from(id)?,
                    None => {
                        return Err(de::Error::invalid_value(Unexpected::Seq, &"array with one or two elements"));
                    }
                };

                visitor.visit_enum(EnumDeserializer::new(id, iter.next()))
            }
            Err(other) => {
                Err(de::Error::invalid_type(other.unexpected(), &"array, map or int"))
            }
        }
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_unit_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: Visitor<'de>
    {
        match self.into_iter() {
            Ok(iter) => {
                if iter.len() == 0 {
                    visitor.visit_unit()
                } else {
                    Err(de::Error::invalid_type(Unexpected::Seq, &"empty array"))
                }
            }
            Err(other) => Err(de::Error::invalid_type(other.unexpected(), &"empty array")),
        }
    }
}

impl<'de> ValueBase<'de> for Value {
    type Item = Value;
    type Iter = IntoIter<Value>;
    type MapIter = IntoIter<(Value, Value)>;
    type MapDeserializer = MapDeserializer<Self::MapIter, Self::Item>;

    #[inline]
    fn is_nil(&self) -> bool {
        if let Value::Nil = *self {
            true
        } else {
            false
        }
    }

    #[inline]
    fn into_iter(self) -> Result<Self::Iter, Self::Item> {
        match self {
            Value::Array(v) => Ok(v.into_iter()),
            other => Err(other),
        }
    }

    #[inline]
    fn into_map_iter(self) -> Result<Self::MapIter, Self::Item> {
        match self {
            Value::Map(v) => Ok(v.into_iter()),
            other => Err(other),
        }
    }
}

impl<'de> ValueBase<'de> for ValueRef<'de> {
    type Item = ValueRef<'de>;
    type Iter = IntoIter<ValueRef<'de>>;
    type MapIter = IntoIter<(ValueRef<'de>, ValueRef<'de>)>;
    type MapDeserializer = MapDeserializer<Self::MapIter, Self::Item>;

    #[inline]
    fn is_nil(&self) -> bool {
        if let ValueRef::Nil = *self {
            true
        } else {
            false
        }
    }

    #[inline]
    fn into_iter(self) -> Result<Self::Iter, Self::Item> {
        match self {
            ValueRef::Array(v) => Ok(v.into_iter()),
            other => Err(other),
        }
    }

    #[inline]
    fn into_map_iter(self) -> Result<Self::MapIter, Self::Item> {
        match self {
            ValueRef::Map(v) => Ok(v.into_iter()),
            other => Err(other),
        }
    }
}
