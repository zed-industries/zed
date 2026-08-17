//! YAML Serialization
//!
//! This module provides YAML serialization with the type `Serializer`.

use crate::error::{self, Error, ErrorImpl};
use crate::libyaml;
use crate::libyaml::emitter::{Emitter, Event, Mapping, Scalar, ScalarStyle, Sequence};
use crate::value::tagged::{self, MaybeTag};
use serde::de::Visitor;
use serde::ser::{self, Serializer as _};
use std::fmt::{self, Display};
use std::io;
use std::marker::PhantomData;
use std::mem;
use std::num;
use std::str;

type Result<T, E = Error> = std::result::Result<T, E>;

/// A structure for serializing Rust values into YAML.
///
/// # Example
///
/// ```
/// use anyhow::Result;
/// use serde::Serialize;
/// use std::collections::BTreeMap;
///
/// fn main() -> Result<()> {
///     let mut buffer = Vec::new();
///     let mut ser = serde_yaml::Serializer::new(&mut buffer);
///
///     let mut object = BTreeMap::new();
///     object.insert("k", 107);
///     object.serialize(&mut ser)?;
///
///     object.insert("J", 74);
///     object.serialize(&mut ser)?;
///
///     assert_eq!(buffer, b"k: 107\n---\nJ: 74\nk: 107\n");
///     Ok(())
/// }
/// ```
pub struct Serializer<W> {
    depth: usize,
    state: State,
    emitter: Emitter<'static>,
    writer: PhantomData<W>,
}

enum State {
    NothingInParticular,
    CheckForTag,
    CheckForDuplicateTag,
    FoundTag(String),
    AlreadyTagged,
}

impl<W> Serializer<W>
where
    W: io::Write,
{
    /// Creates a new YAML serializer.
    pub fn new(writer: W) -> Self {
        let mut emitter = Emitter::new({
            let writer = Box::new(writer);
            unsafe { mem::transmute::<Box<dyn io::Write>, Box<dyn io::Write>>(writer) }
        });
        emitter.emit(Event::StreamStart).unwrap();
        Serializer {
            depth: 0,
            state: State::NothingInParticular,
            emitter,
            writer: PhantomData,
        }
    }

    /// Calls [`.flush()`](io::Write::flush) on the underlying `io::Write`
    /// object.
    pub fn flush(&mut self) -> Result<()> {
        self.emitter.flush()?;
        Ok(())
    }

    /// Unwrap the underlying `io::Write` object from the `Serializer`.
    pub fn into_inner(mut self) -> Result<W> {
        self.emitter.emit(Event::StreamEnd)?;
        self.emitter.flush()?;
        let writer = self.emitter.into_inner();
        Ok(*unsafe { Box::from_raw(Box::into_raw(writer).cast::<W>()) })
    }

    fn emit_scalar(&mut self, mut scalar: Scalar) -> Result<()> {
        self.flush_mapping_start()?;
        if let Some(tag) = self.take_tag() {
            scalar.tag = Some(tag);
        }
        self.value_start()?;
        self.emitter.emit(Event::Scalar(scalar))?;
        self.value_end()
    }

    fn emit_sequence_start(&mut self) -> Result<()> {
        self.flush_mapping_start()?;
        self.value_start()?;
        let tag = self.take_tag();
        self.emitter.emit(Event::SequenceStart(Sequence { tag }))?;
        Ok(())
    }

    fn emit_sequence_end(&mut self) -> Result<()> {
        self.emitter.emit(Event::SequenceEnd)?;
        self.value_end()
    }

    fn emit_mapping_start(&mut self) -> Result<()> {
        self.flush_mapping_start()?;
        self.value_start()?;
        let tag = self.take_tag();
        self.emitter.emit(Event::MappingStart(Mapping { tag }))?;
        Ok(())
    }

    fn emit_mapping_end(&mut self) -> Result<()> {
        self.emitter.emit(Event::MappingEnd)?;
        self.value_end()
    }

    fn value_start(&mut self) -> Result<()> {
        if self.depth == 0 {
            self.emitter.emit(Event::DocumentStart)?;
        }
        self.depth += 1;
        Ok(())
    }

    fn value_end(&mut self) -> Result<()> {
        self.depth -= 1;
        if self.depth == 0 {
            self.emitter.emit(Event::DocumentEnd)?;
        }
        Ok(())
    }

    fn take_tag(&mut self) -> Option<String> {
        let state = mem::replace(&mut self.state, State::NothingInParticular);
        if let State::FoundTag(mut tag) = state {
            if !tag.starts_with('!') {
                tag.insert(0, '!');
            }
            Some(tag)
        } else {
            self.state = state;
            None
        }
    }

    fn flush_mapping_start(&mut self) -> Result<()> {
        if let State::CheckForTag = self.state {
            self.state = State::NothingInParticular;
            self.emit_mapping_start()?;
        } else if let State::CheckForDuplicateTag = self.state {
            self.state = State::NothingInParticular;
        }
        Ok(())
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: if v { "true" } else { "false" },
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_i128(self, v: i128) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_u128(self, v: u128) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: itoa::Buffer::new().format(v),
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        let mut buffer = ryu::Buffer::new();
        self.emit_scalar(Scalar {
            tag: None,
            value: match v.classify() {
                num::FpCategory::Infinite if v.is_sign_positive() => ".inf",
                num::FpCategory::Infinite => "-.inf",
                num::FpCategory::Nan => ".nan",
                _ => buffer.format_finite(v),
            },
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        let mut buffer = ryu::Buffer::new();
        self.emit_scalar(Scalar {
            tag: None,
            value: match v.classify() {
                num::FpCategory::Infinite if v.is_sign_positive() => ".inf",
                num::FpCategory::Infinite => "-.inf",
                num::FpCategory::Nan => ".nan",
                _ => buffer.format_finite(v),
            },
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_char(self, value: char) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: value.encode_utf8(&mut [0u8; 4]),
            style: ScalarStyle::SingleQuoted,
        })
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        struct InferScalarStyle;

        impl<'de> Visitor<'de> for InferScalarStyle {
            type Value = ScalarStyle;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("I wonder")
            }

            fn visit_bool<E>(self, _v: bool) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }

            fn visit_i64<E>(self, _v: i64) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }

            fn visit_i128<E>(self, _v: i128) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }

            fn visit_u64<E>(self, _v: u64) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }

            fn visit_u128<E>(self, _v: u128) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }

            fn visit_f64<E>(self, _v: f64) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
                Ok(if crate::de::digits_but_not_number(v) {
                    ScalarStyle::SingleQuoted
                } else {
                    ScalarStyle::Any
                })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(ScalarStyle::SingleQuoted)
            }
        }

        let style = if value.contains('\n') {
            ScalarStyle::Literal
        } else {
            let result = crate::de::visit_untagged_scalar(
                InferScalarStyle,
                value,
                None,
                libyaml::parser::ScalarStyle::Plain,
            );
            result.unwrap_or(ScalarStyle::Any)
        };

        self.emit_scalar(Scalar {
            tag: None,
            value,
            style,
        })
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(error::new(ErrorImpl::BytesUnsupported))
    }

    fn serialize_unit(self) -> Result<()> {
        self.emit_scalar(Scalar {
            tag: None,
            value: "null",
            style: ScalarStyle::Plain,
        })
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        if let State::FoundTag(_) = self.state {
            return Err(error::new(ErrorImpl::SerializeNestedEnum));
        }
        self.state = State::FoundTag(variant.to_owned());
        value.serialize(&mut *self)
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<V>(self, value: &V) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.emit_sequence_start()?;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        self.emit_sequence_start()?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.emit_sequence_start()?;
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _enm: &'static str,
        _idx: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        if let State::FoundTag(_) = self.state {
            return Err(error::new(ErrorImpl::SerializeNestedEnum));
        }
        self.state = State::FoundTag(variant.to_owned());
        self.emit_sequence_start()?;
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if len == Some(1) {
            self.state = if let State::FoundTag(_) = self.state {
                self.emit_mapping_start()?;
                State::CheckForDuplicateTag
            } else {
                State::CheckForTag
            };
        } else {
            self.emit_mapping_start()?;
        }
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.emit_mapping_start()?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _enm: &'static str,
        _idx: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        if let State::FoundTag(_) = self.state {
            return Err(error::new(ErrorImpl::SerializeNestedEnum));
        }
        self.state = State::FoundTag(variant.to_owned());
        self.emit_mapping_start()?;
        Ok(self)
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        let string = if let State::CheckForTag | State::CheckForDuplicateTag = self.state {
            match tagged::check_for_tag(value) {
                MaybeTag::NotTag(string) => string,
                MaybeTag::Tag(string) => {
                    return if let State::CheckForDuplicateTag = self.state {
                        Err(error::new(ErrorImpl::SerializeNestedEnum))
                    } else {
                        self.state = State::FoundTag(string);
                        Ok(())
                    };
                }
            }
        } else {
            value.to_string()
        };

        self.serialize_str(&string)
    }
}

impl<'a, W> ser::SerializeSeq for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, elem: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        elem.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.emit_sequence_end()
    }
}

impl<'a, W> ser::SerializeTuple for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, elem: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        elem.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.emit_sequence_end()
    }
}

impl<'a, W> ser::SerializeTupleStruct for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<V>(&mut self, value: &V) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.emit_sequence_end()
    }
}

impl<'a, W> ser::SerializeTupleVariant for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<V>(&mut self, v: &V) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        v.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.emit_sequence_end()
    }
}

impl<'a, W> ser::SerializeMap for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.flush_mapping_start()?;
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + ser::Serialize,
        V: ?Sized + ser::Serialize,
    {
        key.serialize(&mut **self)?;
        let tagged = matches!(self.state, State::FoundTag(_));
        value.serialize(&mut **self)?;
        if tagged {
            self.state = State::AlreadyTagged;
        }
        Ok(())
    }

    fn end(self) -> Result<()> {
        if let State::CheckForTag = self.state {
            self.emit_mapping_start()?;
        }
        if !matches!(self.state, State::AlreadyTagged) {
            self.emit_mapping_end()?;
        }
        self.state = State::NothingInParticular;
        Ok(())
    }
}

impl<'a, W> ser::SerializeStruct for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<V>(&mut self, key: &'static str, value: &V) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        self.serialize_str(key)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.emit_mapping_end()
    }
}

impl<'a, W> ser::SerializeStructVariant for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<V>(&mut self, field: &'static str, v: &V) -> Result<()>
    where
        V: ?Sized + ser::Serialize,
    {
        self.serialize_str(field)?;
        v.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.emit_mapping_end()
    }
}

/// Serialize the given data structure as YAML into the IO stream.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// return an error.
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ?Sized + ser::Serialize,
{
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)
}

/// Serialize the given data structure as a String of YAML.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// return an error.
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + ser::Serialize,
{
    let mut vec = Vec::with_capacity(128);
    to_writer(&mut vec, value)?;
    String::from_utf8(vec).map_err(|error| error::new(ErrorImpl::FromUtf8(error)))
}
