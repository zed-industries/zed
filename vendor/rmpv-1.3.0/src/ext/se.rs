use std::fmt::Display;

use serde::ser::{
    self, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple, SerializeTupleStruct,
};
use serde::Serialize;
use serde_bytes::Bytes;

use crate::{IntPriv, Integer, Value};

use super::Error;
use crate::MSGPACK_EXT_STRUCT_NAME;

impl Serialize for Value {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        match *self {
            Value::Nil => s.serialize_unit(),
            Value::Boolean(v) => s.serialize_bool(v),
            Value::Integer(Integer { n }) => match n {
                IntPriv::PosInt(n) => s.serialize_u64(n),
                IntPriv::NegInt(n) => s.serialize_i64(n),
            },
            Value::F32(v) => s.serialize_f32(v),
            Value::F64(v) => s.serialize_f64(v),
            Value::String(ref v) => match v.s {
                Ok(ref v) => s.serialize_str(v),
                Err(ref v) => Bytes::new(&v.0[..]).serialize(s),
            },
            Value::Binary(ref v) => Bytes::new(&v[..]).serialize(s),
            Value::Array(ref array) => {
                let mut state = s.serialize_seq(Some(array.len()))?;
                for item in array {
                    state.serialize_element(item)?;
                }
                state.end()
            }
            Value::Map(ref map) => {
                let mut state = s.serialize_map(Some(map.len()))?;
                for (key, val) in map {
                    state.serialize_entry(key, val)?;
                }
                state.end()
            }
            Value::Ext(ty, ref buf) => {
                let value = (ty, Bytes::new(&buf[..]));
                s.serialize_newtype_struct(MSGPACK_EXT_STRUCT_NAME, &value)
            }
        }
    }
}

impl ser::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Self {
        Error::Syntax(format!("{msg}"))
    }
}

struct Serializer;

/// Convert a `T` into `rmpv::Value` which is an enum that can represent any valid MessagePack data.
///
/// This conversion can fail if `T`'s implementation of `Serialize` decides to fail.
///
/// ```rust
/// # use rmpv::Value;
///
/// let val = rmpv::ext::to_value("John Smith").unwrap();
///
/// assert_eq!(Value::String("John Smith".into()), val);
/// ```
#[inline]
pub fn to_value<T: Serialize>(value: T) -> Result<Value, Error> {
    value.serialize(Serializer)
}

impl ser::Serializer for Serializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SerializeVec;
    type SerializeTuple = SerializeVec;
    type SerializeTupleStruct = SerializeVec;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = DefaultSerializeMap;
    type SerializeStruct = SerializeVec;
    type SerializeStructVariant = SerializeStructVariant;

    #[inline]
    fn serialize_bool(self, val: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Boolean(val))
    }

    #[inline]
    fn serialize_i8(self, val: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(val))
    }

    #[inline]
    fn serialize_i16(self, val: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(val))
    }

    #[inline]
    fn serialize_i32(self, val: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(val))
    }

    #[inline]
    fn serialize_i64(self, val: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(val))
    }

    #[inline]
    fn serialize_u8(self, val: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(val))
    }

    #[inline]
    fn serialize_u16(self, val: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(val))
    }

    #[inline]
    fn serialize_u32(self, val: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(val))
    }

    #[inline]
    fn serialize_u64(self, val: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(val))
    }

    #[inline]
    fn serialize_f32(self, val: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::F32(val))
    }

    #[inline]
    fn serialize_f64(self, val: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::F64(val))
    }

    #[inline]
    fn serialize_char(self, val: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = String::new();
        buf.push(val);
        self.serialize_str(&buf)
    }

    #[inline]
    fn serialize_str(self, val: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(val.into()))
    }

    #[inline]
    fn serialize_bytes(self, val: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Binary(val.into()))
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Nil)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(Vec::new()))
    }

    #[inline]
    fn serialize_unit_variant(self, _name: &'static str, idx: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        let vec = vec![
            Value::from(idx),
            Value::Array(Vec::new())
        ];
        Ok(Value::Array(vec))
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        if name == MSGPACK_EXT_STRUCT_NAME {
            let mut ext_se = ExtSerializer::new();
            value.serialize(&mut ext_se)?;

            return ext_se.value();
        }

        to_value(value)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, idx: u32, _variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        let vec = vec![
            Value::from(idx),
            Value::Array(vec![to_value(value)?]),
        ];
        Ok(Value::Array(vec))
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let se = SerializeVec {
            vec: Vec::with_capacity(len.unwrap_or(0)),
        };
        Ok(se)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Error> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(self, _name: &'static str, idx: u32, _variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Error> {
        let se = SerializeTupleVariant {
            idx,
            vec: Vec::with_capacity(len),
        };
        Ok(se)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        let se = DefaultSerializeMap {
            map: Vec::with_capacity(len.unwrap_or(0)),
            next_key: None,
        };
        Ok(se)
    }

    #[inline]
    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Error> {
        self.serialize_tuple_struct(name, len)
    }

    #[inline]
    fn serialize_struct_variant(self, _name: &'static str, idx: u32, _variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Error> {
        let se = SerializeStructVariant {
            idx,
            vec: Vec::with_capacity(len),
        };
        Ok(se)
    }
}

pub struct ExtSerializer {
    fields_se: Option<ExtFieldSerializer>,
}

impl ser::Serializer for &mut ExtSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = Self;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    #[inline]
    fn serialize_bytes(self, _val: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_bool(self, _val: bool) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_i8(self, _value: i8) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_i16(self, _val: i16) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_i32(self, _val: i32) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_i64(self, _val: i64) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_u8(self, _val: u8) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_u16(self, _val: u16) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_u32(self, _val: u32) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_u64(self, _val: u64) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_f32(self, _val: f32) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_f64(self, _val: f64) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_char(self, _val: char) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_str(self, _val: &str) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_unit_variant(self, _name: &'static str, _idx: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _idx: u32, _variant: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Error> {
        // FIXME check len
        self.fields_se = Some(ExtFieldSerializer::new());

        Ok(self)
    }

    #[inline]
    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_tuple_variant(self, _name: &'static str, _idx: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }

    #[inline]
    fn serialize_struct_variant(self, _name: &'static str, _idx: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Error> {
        Err(<Error as ser::Error>::custom("expected tuple"))
    }
}

impl SerializeTuple for &mut ExtSerializer {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where T: Serialize
    {
        match self.fields_se {
            Some(ref mut se) => value.serialize(&mut *se),
            None => {
                debug_assert!(false);
                Err(Error::Syntax(String::new()))
            }
        }
    }

    #[inline(always)]
    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct ExtFieldSerializer {
    tag: Option<i8>,
    binary: Option<Vec<u8>>,
}

impl ser::Serializer for &mut ExtFieldSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        if self.tag.is_none() {
            self.tag.replace(value);
            Ok(())
        } else {
            Err(<Error as ser::Error>::custom("received second i8"))
        }
    }

    #[inline]
    fn serialize_bytes(self, val: &[u8]) -> Result<Self::Ok, Self::Error> {
        if self.binary.is_none() {
            self.binary.replace(val.to_vec());

            Ok(())
        } else {
            Err(<Error as ser::Error>::custom("expected i8 and bytes"))
        }
    }


    #[inline]
    fn serialize_bool(self, _val: bool) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_i16(self, _val: i16) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_i32(self, _val: i32) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_i64(self, _val: i64) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_u8(self, _val: u8) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_u16(self, _val: u16) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_u32(self, _val: u32) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_u64(self, _val: u64) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_f32(self, _val: f32) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_f64(self, _val: f64) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_char(self, _val: char) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_str(self, _val: &str) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_unit_variant(self, _name: &'static str, _idx: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _idx: u32, _variant: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
        where T: Serialize
    {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_tuple_variant(self, _name: &'static str, _idx: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }

    #[inline]
    fn serialize_struct_variant(self, _name: &'static str, _idx: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Error> {
        Err(<Error as ser::Error>::custom("expected i8 and bytes"))
    }
}

impl ExtSerializer {
    #[inline]
    fn new() -> Self {
        Self { fields_se: None }
    }

    fn value(self) -> Result<Value, Error> {
        match self.fields_se {
            Some(fields_se) => fields_se.value(),
            None => Err(<Error as ser::Error>::custom("expected tuple"))
        }
    }
}

impl ExtFieldSerializer {
    #[inline]
    fn new() -> Self {
        Self {
            tag: None,
            binary: None,
        }
    }

    fn value(self) -> Result<Value, Error> {
        match (self.tag, self.binary) {
            (Some(tag), Some(binary)) => Ok(Value::Ext(tag, binary)),
            (Some(_), None) => Err(<Error as ser::Error>::custom("expected i8 and bytes")),
            (None, Some(_)) => Err(<Error as ser::Error>::custom("expected i8 and bytes")),
            (None, None) => Err(<Error as ser::Error>::custom("expected i8 and bytes")),
        }
    }
}

#[doc(hidden)]
pub struct SerializeVec {
    vec: Vec<Value>,
}

/// Default implementation for tuple variant serialization. It packs given enums as a tuple of an
/// index with a tuple of arguments.
#[doc(hidden)]
pub struct SerializeTupleVariant {
    idx: u32,
    vec: Vec<Value>,
}

#[doc(hidden)]
pub struct DefaultSerializeMap {
    map: Vec<(Value, Value)>,
    next_key: Option<Value>,
}

#[doc(hidden)]
pub struct SerializeStructVariant {
    idx: u32,
    vec: Vec<Value>,
}

impl SerializeSeq for SerializeVec {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where T: Serialize
    {
        self.vec.push(to_value(value)?);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Array(self.vec))
    }
}

impl SerializeTuple for SerializeVec {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where T: Serialize
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        ser::SerializeSeq::end(self)
    }
}

impl SerializeTupleStruct for SerializeVec {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where T: Serialize
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where T: Serialize
    {
        self.vec.push(to_value(value)?);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Array(vec![Value::from(self.idx), Value::Array(self.vec)]))
    }
}

impl ser::SerializeMap for DefaultSerializeMap {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Error>
        where T: Serialize
    {
        self.next_key = Some(to_value(key)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where T: ser::Serialize
    {
        // Panic because this indicates a bug in the program rather than an
        // expected failure.
        let key = self.next_key.take()
            .expect("`serialize_value` called before `serialize_key`");
        self.map.push((key, to_value(value)?));
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Map(self.map))
    }
}

impl SerializeStruct for SerializeVec {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Error>
        where T: Serialize
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Error>
        where T: Serialize
    {
        self.vec.push(to_value(value)?);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        Ok(Value::Array(vec![
            Value::from(self.idx),
            Value::Array(self.vec),
        ]))
    }
}
