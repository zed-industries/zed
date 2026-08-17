use crate::error::{self, Error, ErrorImpl};
use crate::value::tagged::{self, MaybeTag};
use crate::value::{to_value, Mapping, Number, Sequence, Tag, TaggedValue, Value};
use serde::ser::{self, Serialize};
use std::fmt::Display;
use std::mem;

type Result<T, E = Error> = std::result::Result<T, E>;

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(n) => n.serialize(serializer),
            Value::String(s) => serializer.serialize_str(s),
            Value::Sequence(seq) => seq.serialize(serializer),
            Value::Mapping(mapping) => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(mapping.len()))?;
                for (k, v) in mapping {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            Value::Tagged(tagged) => tagged.serialize(serializer),
        }
    }
}

/// Serializer whose output is a `Value`.
///
/// This is the serializer that backs [`serde_yaml::to_value`][crate::to_value].
/// Unlike the main serde_yaml serializer which goes from some serializable
/// value of type `T` to YAML text, this one goes from `T` to
/// `serde_yaml::Value`.
///
/// The `to_value` function is implementable as:
///
/// ```
/// use serde::Serialize;
/// use serde_yaml::{Error, Value};
///
/// pub fn to_value<T>(input: T) -> Result<Value, Error>
/// where
///     T: Serialize,
/// {
///     input.serialize(serde_yaml::value::Serializer)
/// }
/// ```
pub struct Serializer;

impl ser::Serializer for Serializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SerializeArray;
    type SerializeTuple = SerializeArray;
    type SerializeTupleStruct = SerializeArray;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeStruct;
    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Value> {
        Ok(Value::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Value> {
        Ok(Value::Number(Number::from(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Value> {
        Ok(Value::Number(Number::from(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Value> {
        Ok(Value::Number(Number::from(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Value> {
        Ok(Value::Number(Number::from(v)))
    }

    fn serialize_i128(self, v: i128) -> Result<Value> {
        if let Ok(v) = u64::try_from(v) {
            self.serialize_u64(v)
        } else if let Ok(v) = i64::try_from(v) {
            self.serialize_i64(v)
        } else {
            Ok(Value::String(v.to_string()))
        }
    }

    fn serialize_u8(self, v: u8) -> Result<Value> {
        Ok(Value::Number(Number::from(v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Value> {
        Ok(Value::Number(Number::from(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Value> {
        Ok(Value::Number(Number::from(v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Value> {
        Ok(Value::Number(Number::from(v)))
    }

    fn serialize_u128(self, v: u128) -> Result<Value> {
        if let Ok(v) = u64::try_from(v) {
            self.serialize_u64(v)
        } else {
            Ok(Value::String(v.to_string()))
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Value> {
        Ok(Value::Number(Number::from(v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Value> {
        Ok(Value::Number(Number::from(v)))
    }

    fn serialize_char(self, value: char) -> Result<Value> {
        Ok(Value::String(value.to_string()))
    }

    fn serialize_str(self, value: &str) -> Result<Value> {
        Ok(Value::String(value.to_owned()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Value> {
        let vec = value
            .iter()
            .map(|&b| Value::Number(Number::from(b)))
            .collect();
        Ok(Value::Sequence(vec))
    }

    fn serialize_unit(self) -> Result<Value> {
        Ok(Value::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &str,
        _variant_index: u32,
        variant: &str,
    ) -> Result<Value> {
        Ok(Value::String(variant.to_owned()))
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Value>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &str,
        _variant_index: u32,
        variant: &str,
        value: &T,
    ) -> Result<Value>
    where
        T: ?Sized + ser::Serialize,
    {
        if variant.is_empty() {
            return Err(error::new(ErrorImpl::EmptyTag));
        }
        Ok(Value::Tagged(Box::new(TaggedValue {
            tag: Tag::new(variant),
            value: to_value(value)?,
        })))
    }

    fn serialize_none(self) -> Result<Value> {
        self.serialize_unit()
    }

    fn serialize_some<V>(self, value: &V) -> Result<Value>
    where
        V: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<SerializeArray> {
        let sequence = match len {
            None => Sequence::new(),
            Some(len) => Sequence::with_capacity(len),
        };
        Ok(SerializeArray { sequence })
    }

    fn serialize_tuple(self, len: usize) -> Result<SerializeArray> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<SerializeArray> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _enum: &'static str,
        _idx: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<SerializeTupleVariant> {
        if variant.is_empty() {
            return Err(error::new(ErrorImpl::EmptyTag));
        }
        Ok(SerializeTupleVariant {
            tag: variant,
            sequence: Sequence::with_capacity(len),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<SerializeMap> {
        if len == Some(1) {
            Ok(SerializeMap::CheckForTag)
        } else {
            Ok(SerializeMap::Untagged {
                mapping: Mapping::new(),
                next_key: None,
            })
        }
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<SerializeStruct> {
        Ok(SerializeStruct {
            mapping: Mapping::new(),
        })
    }

    fn serialize_struct_variant(
        self,
        _enum: &'static str,
        _idx: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<SerializeStructVariant> {
        if variant.is_empty() {
            return Err(error::new(ErrorImpl::EmptyTag));
        }
        Ok(SerializeStructVariant {
            tag: variant,
            mapping: Mapping::new(),
        })
    }
}

pub struct SerializeArray {
    sequence: Sequence,
}

impl ser::SerializeSeq for SerializeArray {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, elem: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.sequence.push(to_value(elem)?);
        Ok(())
    }

    fn end(self) -> Result<Value> {
        Ok(Value::Sequence(self.sequence))
    }
}

impl ser::SerializeTuple for SerializeArray {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, elem: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, elem)
    }

    fn end(self) -> Result<Value> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SerializeArray {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<V>(&mut self, value: &V) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value> {
        ser::SerializeSeq::end(self)
    }
}

pub struct SerializeTupleVariant {
    tag: &'static str,
    sequence: Sequence,
}

impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<V>(&mut self, v: &V) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        self.sequence.push(to_value(v)?);
        Ok(())
    }

    fn end(self) -> Result<Value> {
        Ok(Value::Tagged(Box::new(TaggedValue {
            tag: Tag::new(self.tag),
            value: Value::Sequence(self.sequence),
        })))
    }
}

pub enum SerializeMap {
    CheckForTag,
    Tagged(TaggedValue),
    Untagged {
        mapping: Mapping,
        next_key: Option<Value>,
    },
}

impl ser::SerializeMap for SerializeMap {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        let key = Some(to_value(key)?);
        match self {
            SerializeMap::CheckForTag => {
                *self = SerializeMap::Untagged {
                    mapping: Mapping::new(),
                    next_key: key,
                };
            }
            SerializeMap::Tagged(tagged) => {
                let mut mapping = Mapping::new();
                mapping.insert(
                    Value::String(tagged.tag.to_string()),
                    mem::take(&mut tagged.value),
                );
                *self = SerializeMap::Untagged {
                    mapping,
                    next_key: key,
                };
            }
            SerializeMap::Untagged { next_key, .. } => *next_key = key,
        }
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        let (mapping, key) = match self {
            SerializeMap::CheckForTag | SerializeMap::Tagged(_) => unreachable!(),
            SerializeMap::Untagged { mapping, next_key } => (mapping, next_key),
        };
        match key.take() {
            Some(key) => mapping.insert(key, to_value(value)?),
            None => panic!("serialize_value called before serialize_key"),
        };
        Ok(())
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<()>
    where
        K: ?Sized + ser::Serialize,
        V: ?Sized + ser::Serialize,
    {
        struct CheckForTag;
        struct NotTag<T> {
            delegate: T,
        }

        impl ser::Serializer for CheckForTag {
            type Ok = MaybeTag<Value>;
            type Error = Error;

            type SerializeSeq = NotTag<SerializeArray>;
            type SerializeTuple = NotTag<SerializeArray>;
            type SerializeTupleStruct = NotTag<SerializeArray>;
            type SerializeTupleVariant = NotTag<SerializeTupleVariant>;
            type SerializeMap = NotTag<SerializeMap>;
            type SerializeStruct = NotTag<SerializeStruct>;
            type SerializeStructVariant = NotTag<SerializeStructVariant>;

            fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
                Serializer.serialize_bool(v).map(MaybeTag::NotTag)
            }

            fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
                Serializer.serialize_i8(v).map(MaybeTag::NotTag)
            }

            fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
                Serializer.serialize_i16(v).map(MaybeTag::NotTag)
            }

            fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
                Serializer.serialize_i32(v).map(MaybeTag::NotTag)
            }

            fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
                Serializer.serialize_i64(v).map(MaybeTag::NotTag)
            }

            fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
                Serializer.serialize_i128(v).map(MaybeTag::NotTag)
            }

            fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
                Serializer.serialize_u8(v).map(MaybeTag::NotTag)
            }

            fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
                Serializer.serialize_u16(v).map(MaybeTag::NotTag)
            }

            fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
                Serializer.serialize_u32(v).map(MaybeTag::NotTag)
            }

            fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
                Serializer.serialize_u64(v).map(MaybeTag::NotTag)
            }

            fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
                Serializer.serialize_u128(v).map(MaybeTag::NotTag)
            }

            fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
                Serializer.serialize_f32(v).map(MaybeTag::NotTag)
            }

            fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
                Serializer.serialize_f64(v).map(MaybeTag::NotTag)
            }

            fn serialize_char(self, value: char) -> Result<Self::Ok> {
                Serializer.serialize_char(value).map(MaybeTag::NotTag)
            }

            fn serialize_str(self, value: &str) -> Result<Self::Ok> {
                Serializer.serialize_str(value).map(MaybeTag::NotTag)
            }

            fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
                Serializer.serialize_bytes(value).map(MaybeTag::NotTag)
            }

            fn serialize_unit(self) -> Result<Self::Ok> {
                Serializer.serialize_unit().map(MaybeTag::NotTag)
            }

            fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
                Serializer.serialize_unit_struct(name).map(MaybeTag::NotTag)
            }

            fn serialize_unit_variant(
                self,
                name: &'static str,
                variant_index: u32,
                variant: &'static str,
            ) -> Result<Self::Ok> {
                Serializer
                    .serialize_unit_variant(name, variant_index, variant)
                    .map(MaybeTag::NotTag)
            }

            fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok>
            where
                T: ?Sized + ser::Serialize,
            {
                Serializer
                    .serialize_newtype_struct(name, value)
                    .map(MaybeTag::NotTag)
            }

            fn serialize_newtype_variant<T>(
                self,
                name: &'static str,
                variant_index: u32,
                variant: &'static str,
                value: &T,
            ) -> Result<Self::Ok>
            where
                T: ?Sized + ser::Serialize,
            {
                Serializer
                    .serialize_newtype_variant(name, variant_index, variant, value)
                    .map(MaybeTag::NotTag)
            }

            fn serialize_none(self) -> Result<Self::Ok> {
                Serializer.serialize_none().map(MaybeTag::NotTag)
            }

            fn serialize_some<V>(self, value: &V) -> Result<Self::Ok>
            where
                V: ?Sized + ser::Serialize,
            {
                Serializer.serialize_some(value).map(MaybeTag::NotTag)
            }

            fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
                Ok(NotTag {
                    delegate: Serializer.serialize_seq(len)?,
                })
            }

            fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
                Ok(NotTag {
                    delegate: Serializer.serialize_tuple(len)?,
                })
            }

            fn serialize_tuple_struct(
                self,
                name: &'static str,
                len: usize,
            ) -> Result<Self::SerializeTupleStruct> {
                Ok(NotTag {
                    delegate: Serializer.serialize_tuple_struct(name, len)?,
                })
            }

            fn serialize_tuple_variant(
                self,
                name: &'static str,
                variant_index: u32,
                variant: &'static str,
                len: usize,
            ) -> Result<Self::SerializeTupleVariant> {
                Ok(NotTag {
                    delegate: Serializer.serialize_tuple_variant(
                        name,
                        variant_index,
                        variant,
                        len,
                    )?,
                })
            }

            fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
                Ok(NotTag {
                    delegate: Serializer.serialize_map(len)?,
                })
            }

            fn serialize_struct(
                self,
                name: &'static str,
                len: usize,
            ) -> Result<Self::SerializeStruct> {
                Ok(NotTag {
                    delegate: Serializer.serialize_struct(name, len)?,
                })
            }

            fn serialize_struct_variant(
                self,
                name: &'static str,
                variant_index: u32,
                variant: &'static str,
                len: usize,
            ) -> Result<Self::SerializeStructVariant> {
                Ok(NotTag {
                    delegate: Serializer.serialize_struct_variant(
                        name,
                        variant_index,
                        variant,
                        len,
                    )?,
                })
            }

            fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
            where
                T: ?Sized + Display,
            {
                Ok(match tagged::check_for_tag(value) {
                    MaybeTag::Tag(tag) => MaybeTag::Tag(tag),
                    MaybeTag::NotTag(string) => MaybeTag::NotTag(Value::String(string)),
                })
            }
        }

        impl ser::SerializeSeq for NotTag<SerializeArray> {
            type Ok = MaybeTag<Value>;
            type Error = Error;

            fn serialize_element<T>(&mut self, elem: &T) -> Result<()>
            where
                T: ?Sized + ser::Serialize,
            {
                self.delegate.serialize_element(elem)
            }

            fn end(self) -> Result<Self::Ok> {
                self.delegate.end().map(MaybeTag::NotTag)
            }
        }

        impl ser::SerializeTuple for NotTag<SerializeArray> {
            type Ok = MaybeTag<Value>;
            type Error = Error;

            fn serialize_element<T>(&mut self, elem: &T) -> Result<()>
            where
                T: ?Sized + ser::Serialize,
            {
                self.delegate.serialize_element(elem)
            }

            fn end(self) -> Result<Self::Ok> {
                self.delegate.end().map(MaybeTag::NotTag)
            }
        }

        impl ser::SerializeTupleStruct for NotTag<SerializeArray> {
            type Ok = MaybeTag<Value>;
            type Error = Error;

            fn serialize_field<V>(&mut self, value: &V) -> Result<()>
            where
                V: ?Sized + ser::Serialize,
            {
                self.delegate.serialize_field(value)
            }

            fn end(self) -> Result<Self::Ok> {
                self.delegate.end().map(MaybeTag::NotTag)
            }
        }

        impl ser::SerializeTupleVariant for NotTag<SerializeTupleVariant> {
            type Ok = MaybeTag<Value>;
            type Error = Error;

            fn serialize_field<V>(&mut self, v: &V) -> Result<()>
            where
                V: ?Sized + ser::Serialize,
            {
                self.delegate.serialize_field(v)
            }

            fn end(self) -> Result<Self::Ok> {
                self.delegate.end().map(MaybeTag::NotTag)
            }
        }

        impl ser::SerializeMap for NotTag<SerializeMap> {
            type Ok = MaybeTag<Value>;
            type Error = Error;

            fn serialize_key<T>(&mut self, key: &T) -> Result<()>
            where
                T: ?Sized + ser::Serialize,
            {
                self.delegate.serialize_key(key)
            }

            fn serialize_value<T>(&mut self, value: &T) -> Result<()>
            where
                T: ?Sized + ser::Serialize,
            {
                self.delegate.serialize_value(value)
            }

            fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<()>
            where
                K: ?Sized + ser::Serialize,
                V: ?Sized + ser::Serialize,
            {
                self.delegate.serialize_entry(key, value)
            }

            fn end(self) -> Result<Self::Ok> {
                self.delegate.end().map(MaybeTag::NotTag)
            }
        }

        impl ser::SerializeStruct for NotTag<SerializeStruct> {
            type Ok = MaybeTag<Value>;
            type Error = Error;

            fn serialize_field<V>(&mut self, key: &'static str, value: &V) -> Result<()>
            where
                V: ?Sized + ser::Serialize,
            {
                self.delegate.serialize_field(key, value)
            }

            fn end(self) -> Result<Self::Ok> {
                self.delegate.end().map(MaybeTag::NotTag)
            }
        }

        impl ser::SerializeStructVariant for NotTag<SerializeStructVariant> {
            type Ok = MaybeTag<Value>;
            type Error = Error;

            fn serialize_field<V>(&mut self, field: &'static str, v: &V) -> Result<()>
            where
                V: ?Sized + ser::Serialize,
            {
                self.delegate.serialize_field(field, v)
            }

            fn end(self) -> Result<Self::Ok> {
                self.delegate.end().map(MaybeTag::NotTag)
            }
        }

        match self {
            SerializeMap::CheckForTag => {
                let key = key.serialize(CheckForTag)?;
                let mut mapping = Mapping::new();
                *self = match key {
                    MaybeTag::Tag(string) => SerializeMap::Tagged(TaggedValue {
                        tag: Tag::new(string),
                        value: to_value(value)?,
                    }),
                    MaybeTag::NotTag(key) => {
                        mapping.insert(key, to_value(value)?);
                        SerializeMap::Untagged {
                            mapping,
                            next_key: None,
                        }
                    }
                };
            }
            SerializeMap::Tagged(tagged) => {
                let mut mapping = Mapping::new();
                mapping.insert(
                    Value::String(tagged.tag.to_string()),
                    mem::take(&mut tagged.value),
                );
                mapping.insert(to_value(key)?, to_value(value)?);
                *self = SerializeMap::Untagged {
                    mapping,
                    next_key: None,
                };
            }
            SerializeMap::Untagged { mapping, .. } => {
                mapping.insert(to_value(key)?, to_value(value)?);
            }
        }
        Ok(())
    }

    fn end(self) -> Result<Value> {
        Ok(match self {
            SerializeMap::CheckForTag => Value::Mapping(Mapping::new()),
            SerializeMap::Tagged(tagged) => Value::Tagged(Box::new(tagged)),
            SerializeMap::Untagged { mapping, .. } => Value::Mapping(mapping),
        })
    }
}

pub struct SerializeStruct {
    mapping: Mapping,
}

impl ser::SerializeStruct for SerializeStruct {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<V>(&mut self, key: &'static str, value: &V) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        self.mapping.insert(to_value(key)?, to_value(value)?);
        Ok(())
    }

    fn end(self) -> Result<Value> {
        Ok(Value::Mapping(self.mapping))
    }
}

pub struct SerializeStructVariant {
    tag: &'static str,
    mapping: Mapping,
}

impl ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<V>(&mut self, field: &'static str, v: &V) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        self.mapping.insert(to_value(field)?, to_value(v)?);
        Ok(())
    }

    fn end(self) -> Result<Value> {
        Ok(Value::Tagged(Box::new(TaggedValue {
            tag: Tag::new(self.tag),
            value: Value::Mapping(self.mapping),
        })))
    }
}
