use crate::error::{self, Error, ErrorImpl};
use crate::libyaml::error::Mark;
use crate::libyaml::parser::{MappingStart, Scalar, ScalarStyle, SequenceStart};
use crate::libyaml::tag::Tag;
use crate::loader::{Document, Loader};
use crate::path::Path;
use serde::de::value::StrDeserializer;
use serde::de::{
    self, Deserialize, DeserializeOwned, DeserializeSeed, Expected, IgnoredAny, Unexpected, Visitor,
};
use std::fmt;
use std::io;
use std::mem;
use std::num::ParseIntError;
use std::str;
use std::sync::Arc;

type Result<T, E = Error> = std::result::Result<T, E>;

/// A structure that deserializes YAML into Rust values.
///
/// # Examples
///
/// Deserializing a single document:
///
/// ```
/// use anyhow::Result;
/// use serde::Deserialize;
/// use serde_yaml::Value;
///
/// fn main() -> Result<()> {
///     let input = "k: 107\n";
///     let de = serde_yaml::Deserializer::from_str(input);
///     let value = Value::deserialize(de)?;
///     println!("{:?}", value);
///     Ok(())
/// }
/// ```
///
/// Deserializing multi-doc YAML:
///
/// ```
/// use anyhow::Result;
/// use serde::Deserialize;
/// use serde_yaml::Value;
///
/// fn main() -> Result<()> {
///     let input = "---\nk: 107\n...\n---\nj: 106\n";
///
///     for document in serde_yaml::Deserializer::from_str(input) {
///         let value = Value::deserialize(document)?;
///         println!("{:?}", value);
///     }
///
///     Ok(())
/// }
/// ```
pub struct Deserializer<'de> {
    progress: Progress<'de>,
}

pub(crate) enum Progress<'de> {
    Str(&'de str),
    Slice(&'de [u8]),
    Read(Box<dyn io::Read + 'de>),
    Iterable(Loader<'de>),
    Document(Document<'de>),
    Fail(Arc<ErrorImpl>),
}

impl<'de> Deserializer<'de> {
    /// Creates a YAML deserializer from a `&str`.
    pub fn from_str(s: &'de str) -> Self {
        let progress = Progress::Str(s);
        Deserializer { progress }
    }

    /// Creates a YAML deserializer from a `&[u8]`.
    pub fn from_slice(v: &'de [u8]) -> Self {
        let progress = Progress::Slice(v);
        Deserializer { progress }
    }

    /// Creates a YAML deserializer from an `io::Read`.
    ///
    /// Reader-based deserializers do not support deserializing borrowed types
    /// like `&str`, since the `std::io::Read` trait has no non-copying methods
    /// -- everything it does involves copying bytes out of the data source.
    pub fn from_reader<R>(rdr: R) -> Self
    where
        R: io::Read + 'de,
    {
        let progress = Progress::Read(Box::new(rdr));
        Deserializer { progress }
    }

    fn de<T>(
        self,
        f: impl for<'document> FnOnce(&mut DeserializerFromEvents<'de, 'document>) -> Result<T>,
    ) -> Result<T> {
        let mut pos = 0;
        let mut jumpcount = 0;

        match self.progress {
            Progress::Iterable(_) => return Err(error::new(ErrorImpl::MoreThanOneDocument)),
            Progress::Document(document) => {
                let t = f(&mut DeserializerFromEvents {
                    document: &document,
                    pos: &mut pos,
                    jumpcount: &mut jumpcount,
                    path: Path::Root,
                    remaining_depth: 128,
                    current_enum: None,
                })?;
                if let Some(parse_error) = document.error {
                    return Err(error::shared(parse_error));
                }
                return Ok(t);
            }
            _ => {}
        }

        let mut loader = Loader::new(self.progress)?;
        let document = match loader.next_document() {
            Some(document) => document,
            None => return Err(error::new(ErrorImpl::EndOfStream)),
        };
        let t = f(&mut DeserializerFromEvents {
            document: &document,
            pos: &mut pos,
            jumpcount: &mut jumpcount,
            path: Path::Root,
            remaining_depth: 128,
            current_enum: None,
        })?;
        if let Some(parse_error) = document.error {
            return Err(error::shared(parse_error));
        }
        if loader.next_document().is_none() {
            Ok(t)
        } else {
            Err(error::new(ErrorImpl::MoreThanOneDocument))
        }
    }
}

impl<'de> Iterator for Deserializer<'de> {
    type Item = Self;

    fn next(&mut self) -> Option<Self> {
        match &mut self.progress {
            Progress::Iterable(loader) => {
                let document = loader.next_document()?;
                return Some(Deserializer {
                    progress: Progress::Document(document),
                });
            }
            Progress::Document(_) => return None,
            Progress::Fail(err) => {
                return Some(Deserializer {
                    progress: Progress::Fail(Arc::clone(err)),
                });
            }
            _ => {}
        }

        let dummy = Progress::Str("");
        let input = mem::replace(&mut self.progress, dummy);
        match Loader::new(input) {
            Ok(loader) => {
                self.progress = Progress::Iterable(loader);
                self.next()
            }
            Err(err) => {
                let fail = err.shared();
                self.progress = Progress::Fail(Arc::clone(&fail));
                Some(Deserializer {
                    progress: Progress::Fail(fail),
                })
            }
        }
    }
}

impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_any(visitor))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_bool(visitor))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_i8(visitor))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_i16(visitor))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_i32(visitor))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_i64(visitor))
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_i128(visitor))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_u8(visitor))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_u16(visitor))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_u32(visitor))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_u64(visitor))
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_u128(visitor))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_f32(visitor))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_f64(visitor))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_char(visitor))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_str(visitor))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_string(visitor))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_bytes(visitor))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_byte_buf(visitor))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_option(visitor))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_unit(visitor))
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_unit_struct(name, visitor))
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_newtype_struct(name, visitor))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_seq(visitor))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_tuple(len, visitor))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_tuple_struct(name, len, visitor))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_map(visitor))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_struct(name, fields, visitor))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_enum(name, variants, visitor))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_identifier(visitor))
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de(|state| state.deserialize_ignored_any(visitor))
    }
}

#[derive(Debug)]
pub(crate) enum Event<'de> {
    Alias(usize),
    Scalar(Scalar<'de>),
    SequenceStart(SequenceStart),
    SequenceEnd,
    MappingStart(MappingStart),
    MappingEnd,
    Void,
}

struct DeserializerFromEvents<'de, 'document> {
    document: &'document Document<'de>,
    pos: &'document mut usize,
    jumpcount: &'document mut usize,
    path: Path<'document>,
    remaining_depth: u8,
    current_enum: Option<CurrentEnum<'document>>,
}

#[derive(Copy, Clone)]
struct CurrentEnum<'document> {
    name: Option<&'static str>,
    tag: &'document str,
}

impl<'de, 'document> DeserializerFromEvents<'de, 'document> {
    fn peek_event(&self) -> Result<&'document Event<'de>> {
        self.peek_event_mark().map(|(event, _mark)| event)
    }

    fn peek_event_mark(&self) -> Result<(&'document Event<'de>, Mark)> {
        match self.document.events.get(*self.pos) {
            Some((event, mark)) => Ok((event, *mark)),
            None => Err(match &self.document.error {
                Some(parse_error) => error::shared(Arc::clone(parse_error)),
                None => error::new(ErrorImpl::EndOfStream),
            }),
        }
    }

    fn next_event(&mut self) -> Result<&'document Event<'de>> {
        self.next_event_mark().map(|(event, _mark)| event)
    }

    fn next_event_mark(&mut self) -> Result<(&'document Event<'de>, Mark)> {
        self.peek_event_mark().map(|(event, mark)| {
            *self.pos += 1;
            self.current_enum = None;
            (event, mark)
        })
    }

    fn jump<'anchor>(
        &'anchor mut self,
        pos: &'anchor mut usize,
    ) -> Result<DeserializerFromEvents<'de, 'anchor>> {
        *self.jumpcount += 1;
        if *self.jumpcount > self.document.events.len() * 100 {
            return Err(error::new(ErrorImpl::RepetitionLimitExceeded));
        }
        match self.document.aliases.get(pos) {
            Some(found) => {
                *pos = *found;
                Ok(DeserializerFromEvents {
                    document: self.document,
                    pos,
                    jumpcount: self.jumpcount,
                    path: Path::Alias { parent: &self.path },
                    remaining_depth: self.remaining_depth,
                    current_enum: None,
                })
            }
            None => panic!("unresolved alias: {}", *pos),
        }
    }

    fn ignore_any(&mut self) -> Result<()> {
        enum Nest {
            Sequence,
            Mapping,
        }

        let mut stack = Vec::new();

        loop {
            match self.next_event()? {
                Event::Alias(_) | Event::Scalar(_) | Event::Void => {}
                Event::SequenceStart(_) => {
                    stack.push(Nest::Sequence);
                }
                Event::MappingStart(_) => {
                    stack.push(Nest::Mapping);
                }
                Event::SequenceEnd => match stack.pop() {
                    Some(Nest::Sequence) => {}
                    None | Some(Nest::Mapping) => {
                        panic!("unexpected end of sequence");
                    }
                },
                Event::MappingEnd => match stack.pop() {
                    Some(Nest::Mapping) => {}
                    None | Some(Nest::Sequence) => {
                        panic!("unexpected end of mapping");
                    }
                },
            }
            if stack.is_empty() {
                return Ok(());
            }
        }
    }

    fn visit_sequence<V>(&mut self, visitor: V, mark: Mark) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (value, len) = self.recursion_check(mark, |de| {
            let mut seq = SeqAccess {
                empty: false,
                de,
                len: 0,
            };
            let value = visitor.visit_seq(&mut seq)?;
            Ok((value, seq.len))
        })?;
        self.end_sequence(len)?;
        Ok(value)
    }

    fn visit_mapping<V>(&mut self, visitor: V, mark: Mark) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (value, len) = self.recursion_check(mark, |de| {
            let mut map = MapAccess {
                empty: false,
                de,
                len: 0,
                key: None,
            };
            let value = visitor.visit_map(&mut map)?;
            Ok((value, map.len))
        })?;
        self.end_mapping(len)?;
        Ok(value)
    }

    fn end_sequence(&mut self, len: usize) -> Result<()> {
        let total = {
            let mut seq = SeqAccess {
                empty: false,
                de: self,
                len,
            };
            while de::SeqAccess::next_element::<IgnoredAny>(&mut seq)?.is_some() {}
            seq.len
        };
        match self.next_event()? {
            Event::SequenceEnd | Event::Void => {}
            _ => panic!("expected a SequenceEnd event"),
        }
        if total == len {
            Ok(())
        } else {
            struct ExpectedSeq(usize);
            impl Expected for ExpectedSeq {
                fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    if self.0 == 1 {
                        write!(formatter, "sequence of 1 element")
                    } else {
                        write!(formatter, "sequence of {} elements", self.0)
                    }
                }
            }
            Err(de::Error::invalid_length(total, &ExpectedSeq(len)))
        }
    }

    fn end_mapping(&mut self, len: usize) -> Result<()> {
        let total = {
            let mut map = MapAccess {
                empty: false,
                de: self,
                len,
                key: None,
            };
            while de::MapAccess::next_entry::<IgnoredAny, IgnoredAny>(&mut map)?.is_some() {}
            map.len
        };
        match self.next_event()? {
            Event::MappingEnd | Event::Void => {}
            _ => panic!("expected a MappingEnd event"),
        }
        if total == len {
            Ok(())
        } else {
            struct ExpectedMap(usize);
            impl Expected for ExpectedMap {
                fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    if self.0 == 1 {
                        write!(formatter, "map containing 1 entry")
                    } else {
                        write!(formatter, "map containing {} entries", self.0)
                    }
                }
            }
            Err(de::Error::invalid_length(total, &ExpectedMap(len)))
        }
    }

    fn recursion_check<F: FnOnce(&mut Self) -> Result<T>, T>(
        &mut self,
        mark: Mark,
        f: F,
    ) -> Result<T> {
        let previous_depth = self.remaining_depth;
        self.remaining_depth = match previous_depth.checked_sub(1) {
            Some(depth) => depth,
            None => return Err(error::new(ErrorImpl::RecursionLimitExceeded(mark))),
        };
        let result = f(self);
        self.remaining_depth = previous_depth;
        result
    }
}

struct SeqAccess<'de, 'document, 'seq> {
    empty: bool,
    de: &'seq mut DeserializerFromEvents<'de, 'document>,
    len: usize,
}

impl<'de, 'document, 'seq> de::SeqAccess<'de> for SeqAccess<'de, 'document, 'seq> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.empty {
            return Ok(None);
        }
        match self.de.peek_event()? {
            Event::SequenceEnd | Event::Void => Ok(None),
            _ => {
                let mut element_de = DeserializerFromEvents {
                    document: self.de.document,
                    pos: self.de.pos,
                    jumpcount: self.de.jumpcount,
                    path: Path::Seq {
                        parent: &self.de.path,
                        index: self.len,
                    },
                    remaining_depth: self.de.remaining_depth,
                    current_enum: None,
                };
                self.len += 1;
                seed.deserialize(&mut element_de).map(Some)
            }
        }
    }
}

struct MapAccess<'de, 'document, 'map> {
    empty: bool,
    de: &'map mut DeserializerFromEvents<'de, 'document>,
    len: usize,
    key: Option<&'document [u8]>,
}

impl<'de, 'document, 'map> de::MapAccess<'de> for MapAccess<'de, 'document, 'map> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.empty {
            return Ok(None);
        }
        match self.de.peek_event()? {
            Event::MappingEnd | Event::Void => Ok(None),
            Event::Scalar(scalar) => {
                self.len += 1;
                self.key = Some(&scalar.value);
                seed.deserialize(&mut *self.de).map(Some)
            }
            _ => {
                self.len += 1;
                self.key = None;
                seed.deserialize(&mut *self.de).map(Some)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let mut value_de = DeserializerFromEvents {
            document: self.de.document,
            pos: self.de.pos,
            jumpcount: self.de.jumpcount,
            path: if let Some(key) = self.key.and_then(|key| str::from_utf8(key).ok()) {
                Path::Map {
                    parent: &self.de.path,
                    key,
                }
            } else {
                Path::Unknown {
                    parent: &self.de.path,
                }
            },
            remaining_depth: self.de.remaining_depth,
            current_enum: None,
        };
        seed.deserialize(&mut value_de)
    }
}

struct EnumAccess<'de, 'document, 'variant> {
    de: &'variant mut DeserializerFromEvents<'de, 'document>,
    name: Option<&'static str>,
    tag: &'document str,
}

impl<'de, 'document, 'variant> de::EnumAccess<'de> for EnumAccess<'de, 'document, 'variant> {
    type Error = Error;
    type Variant = DeserializerFromEvents<'de, 'variant>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let str_de = StrDeserializer::<Error>::new(self.tag);
        let variant = seed.deserialize(str_de)?;
        let visitor = DeserializerFromEvents {
            document: self.de.document,
            pos: self.de.pos,
            jumpcount: self.de.jumpcount,
            path: self.de.path,
            remaining_depth: self.de.remaining_depth,
            current_enum: Some(CurrentEnum {
                name: self.name,
                tag: self.tag,
            }),
        };
        Ok((variant, visitor))
    }
}

impl<'de, 'document> de::VariantAccess<'de> for DeserializerFromEvents<'de, 'document> {
    type Error = Error;

    fn unit_variant(mut self) -> Result<()> {
        Deserialize::deserialize(&mut self)
    }

    fn newtype_variant_seed<T>(mut self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut self)
    }

    fn tuple_variant<V>(mut self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(&mut self, visitor)
    }

    fn struct_variant<V>(mut self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(&mut self, "", fields, visitor)
    }
}

struct UnitVariantAccess<'de, 'document, 'variant> {
    de: &'variant mut DeserializerFromEvents<'de, 'document>,
}

impl<'de, 'document, 'variant> de::EnumAccess<'de> for UnitVariantAccess<'de, 'document, 'variant> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        Ok((seed.deserialize(&mut *self.de)?, self))
    }
}

impl<'de, 'document, 'variant> de::VariantAccess<'de>
    for UnitVariantAccess<'de, 'document, 'variant>
{
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"newtype variant",
        ))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"tuple variant",
        ))
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"struct variant",
        ))
    }
}

fn visit_scalar<'de, V>(visitor: V, scalar: &Scalar<'de>, tagged_already: bool) -> Result<V::Value>
where
    V: Visitor<'de>,
{
    let v = match str::from_utf8(&scalar.value) {
        Ok(v) => v,
        Err(_) => {
            return Err(de::Error::invalid_type(
                Unexpected::Bytes(&scalar.value),
                &visitor,
            ))
        }
    };
    if let (Some(tag), false) = (&scalar.tag, tagged_already) {
        if tag == Tag::BOOL {
            return match parse_bool(v) {
                Some(v) => visitor.visit_bool(v),
                None => Err(de::Error::invalid_value(Unexpected::Str(v), &"a boolean")),
            };
        } else if tag == Tag::INT {
            return match visit_int(visitor, v) {
                Ok(result) => result,
                Err(_) => Err(de::Error::invalid_value(Unexpected::Str(v), &"an integer")),
            };
        } else if tag == Tag::FLOAT {
            return match parse_f64(v) {
                Some(v) => visitor.visit_f64(v),
                None => Err(de::Error::invalid_value(Unexpected::Str(v), &"a float")),
            };
        } else if tag == Tag::NULL {
            return match parse_null(v.as_bytes()) {
                Some(()) => visitor.visit_unit(),
                None => Err(de::Error::invalid_value(Unexpected::Str(v), &"null")),
            };
        } else if tag.starts_with("!") && scalar.style == ScalarStyle::Plain {
            return visit_untagged_scalar(visitor, v, scalar.repr, scalar.style);
        }
    } else if scalar.style == ScalarStyle::Plain {
        return visit_untagged_scalar(visitor, v, scalar.repr, scalar.style);
    }
    if let Some(borrowed) = parse_borrowed_str(v, scalar.repr, scalar.style) {
        visitor.visit_borrowed_str(borrowed)
    } else {
        visitor.visit_str(v)
    }
}

fn parse_borrowed_str<'de>(
    utf8_value: &str,
    repr: Option<&'de [u8]>,
    style: ScalarStyle,
) -> Option<&'de str> {
    let borrowed_repr = repr?;
    let expected_offset = match style {
        ScalarStyle::Plain => 0,
        ScalarStyle::SingleQuoted | ScalarStyle::DoubleQuoted => 1,
        ScalarStyle::Literal | ScalarStyle::Folded => return None,
    };
    let expected_end = borrowed_repr.len().checked_sub(expected_offset)?;
    let expected_start = expected_end.checked_sub(utf8_value.len())?;
    let borrowed_bytes = borrowed_repr.get(expected_start..expected_end)?;
    if borrowed_bytes == utf8_value.as_bytes() {
        return Some(unsafe { str::from_utf8_unchecked(borrowed_bytes) });
    }
    None
}

fn parse_null(scalar: &[u8]) -> Option<()> {
    match scalar {
        b"null" | b"Null" | b"NULL" | b"~" => Some(()),
        _ => None,
    }
}

fn parse_bool(scalar: &str) -> Option<bool> {
    match scalar {
        "true" | "True" | "TRUE" => Some(true),
        "false" | "False" | "FALSE" => Some(false),
        _ => None,
    }
}

fn parse_unsigned_int<T>(
    scalar: &str,
    from_str_radix: fn(&str, radix: u32) -> Result<T, ParseIntError>,
) -> Option<T> {
    let unpositive = scalar.strip_prefix('+').unwrap_or(scalar);
    if let Some(rest) = unpositive.strip_prefix("0x") {
        if rest.starts_with(['+', '-']) {
            return None;
        }
        if let Ok(int) = from_str_radix(rest, 16) {
            return Some(int);
        }
    }
    if let Some(rest) = unpositive.strip_prefix("0o") {
        if rest.starts_with(['+', '-']) {
            return None;
        }
        if let Ok(int) = from_str_radix(rest, 8) {
            return Some(int);
        }
    }
    if let Some(rest) = unpositive.strip_prefix("0b") {
        if rest.starts_with(['+', '-']) {
            return None;
        }
        if let Ok(int) = from_str_radix(rest, 2) {
            return Some(int);
        }
    }
    if unpositive.starts_with(['+', '-']) {
        return None;
    }
    if digits_but_not_number(scalar) {
        return None;
    }
    from_str_radix(unpositive, 10).ok()
}

fn parse_signed_int<T>(
    scalar: &str,
    from_str_radix: fn(&str, radix: u32) -> Result<T, ParseIntError>,
) -> Option<T> {
    let unpositive = if let Some(unpositive) = scalar.strip_prefix('+') {
        if unpositive.starts_with(['+', '-']) {
            return None;
        }
        unpositive
    } else {
        scalar
    };
    if let Some(rest) = unpositive.strip_prefix("0x") {
        if rest.starts_with(['+', '-']) {
            return None;
        }
        if let Ok(int) = from_str_radix(rest, 16) {
            return Some(int);
        }
    }
    if let Some(rest) = scalar.strip_prefix("-0x") {
        let negative = format!("-{}", rest);
        if let Ok(int) = from_str_radix(&negative, 16) {
            return Some(int);
        }
    }
    if let Some(rest) = unpositive.strip_prefix("0o") {
        if rest.starts_with(['+', '-']) {
            return None;
        }
        if let Ok(int) = from_str_radix(rest, 8) {
            return Some(int);
        }
    }
    if let Some(rest) = scalar.strip_prefix("-0o") {
        let negative = format!("-{}", rest);
        if let Ok(int) = from_str_radix(&negative, 8) {
            return Some(int);
        }
    }
    if let Some(rest) = unpositive.strip_prefix("0b") {
        if rest.starts_with(['+', '-']) {
            return None;
        }
        if let Ok(int) = from_str_radix(rest, 2) {
            return Some(int);
        }
    }
    if let Some(rest) = scalar.strip_prefix("-0b") {
        let negative = format!("-{}", rest);
        if let Ok(int) = from_str_radix(&negative, 2) {
            return Some(int);
        }
    }
    if digits_but_not_number(scalar) {
        return None;
    }
    from_str_radix(unpositive, 10).ok()
}

fn parse_negative_int<T>(
    scalar: &str,
    from_str_radix: fn(&str, radix: u32) -> Result<T, ParseIntError>,
) -> Option<T> {
    if let Some(rest) = scalar.strip_prefix("-0x") {
        let negative = format!("-{}", rest);
        if let Ok(int) = from_str_radix(&negative, 16) {
            return Some(int);
        }
    }
    if let Some(rest) = scalar.strip_prefix("-0o") {
        let negative = format!("-{}", rest);
        if let Ok(int) = from_str_radix(&negative, 8) {
            return Some(int);
        }
    }
    if let Some(rest) = scalar.strip_prefix("-0b") {
        let negative = format!("-{}", rest);
        if let Ok(int) = from_str_radix(&negative, 2) {
            return Some(int);
        }
    }
    if digits_but_not_number(scalar) {
        return None;
    }
    from_str_radix(scalar, 10).ok()
}

pub(crate) fn parse_f64(scalar: &str) -> Option<f64> {
    let unpositive = if let Some(unpositive) = scalar.strip_prefix('+') {
        if unpositive.starts_with(['+', '-']) {
            return None;
        }
        unpositive
    } else {
        scalar
    };
    if let ".inf" | ".Inf" | ".INF" = unpositive {
        return Some(f64::INFINITY);
    }
    if let "-.inf" | "-.Inf" | "-.INF" = scalar {
        return Some(f64::NEG_INFINITY);
    }
    if let ".nan" | ".NaN" | ".NAN" = scalar {
        return Some(f64::NAN.copysign(1.0));
    }
    if let Ok(float) = unpositive.parse::<f64>() {
        if float.is_finite() {
            return Some(float);
        }
    }
    None
}

pub(crate) fn digits_but_not_number(scalar: &str) -> bool {
    // Leading zero(s) followed by numeric characters is a string according to
    // the YAML 1.2 spec. https://yaml.org/spec/1.2/spec.html#id2761292
    let scalar = scalar.strip_prefix(['-', '+']).unwrap_or(scalar);
    scalar.len() > 1 && scalar.starts_with('0') && scalar[1..].bytes().all(|b| b.is_ascii_digit())
}

pub(crate) fn visit_int<'de, V>(visitor: V, v: &str) -> Result<Result<V::Value>, V>
where
    V: Visitor<'de>,
{
    if let Some(int) = parse_unsigned_int(v, u64::from_str_radix) {
        return Ok(visitor.visit_u64(int));
    }
    if let Some(int) = parse_negative_int(v, i64::from_str_radix) {
        return Ok(visitor.visit_i64(int));
    }
    if let Some(int) = parse_unsigned_int(v, u128::from_str_radix) {
        return Ok(visitor.visit_u128(int));
    }
    if let Some(int) = parse_negative_int(v, i128::from_str_radix) {
        return Ok(visitor.visit_i128(int));
    }
    Err(visitor)
}

pub(crate) fn visit_untagged_scalar<'de, V>(
    visitor: V,
    v: &str,
    repr: Option<&'de [u8]>,
    style: ScalarStyle,
) -> Result<V::Value>
where
    V: Visitor<'de>,
{
    if v.is_empty() || parse_null(v.as_bytes()) == Some(()) {
        return visitor.visit_unit();
    }
    if let Some(boolean) = parse_bool(v) {
        return visitor.visit_bool(boolean);
    }
    let visitor = match visit_int(visitor, v) {
        Ok(result) => return result,
        Err(visitor) => visitor,
    };
    if !digits_but_not_number(v) {
        if let Some(float) = parse_f64(v) {
            return visitor.visit_f64(float);
        }
    }
    if let Some(borrowed) = parse_borrowed_str(v, repr, style) {
        visitor.visit_borrowed_str(borrowed)
    } else {
        visitor.visit_str(v)
    }
}

fn is_plain_or_tagged_literal_scalar(
    expected: &str,
    scalar: &Scalar,
    tagged_already: bool,
) -> bool {
    match (scalar.style, &scalar.tag, tagged_already) {
        (ScalarStyle::Plain, _, _) => true,
        (ScalarStyle::Literal, Some(tag), false) => tag == expected,
        _ => false,
    }
}

fn invalid_type(event: &Event, exp: &dyn Expected) -> Error {
    enum Void {}

    struct InvalidType<'a> {
        exp: &'a dyn Expected,
    }

    impl<'de, 'a> Visitor<'de> for InvalidType<'a> {
        type Value = Void;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            self.exp.fmt(formatter)
        }
    }

    match event {
        Event::Alias(_) => unreachable!(),
        Event::Scalar(scalar) => {
            let get_type = InvalidType { exp };
            match visit_scalar(get_type, scalar, false) {
                Ok(void) => match void {},
                Err(invalid_type) => invalid_type,
            }
        }
        Event::SequenceStart(_) => de::Error::invalid_type(Unexpected::Seq, exp),
        Event::MappingStart(_) => de::Error::invalid_type(Unexpected::Map, exp),
        Event::SequenceEnd => panic!("unexpected end of sequence"),
        Event::MappingEnd => panic!("unexpected end of mapping"),
        Event::Void => error::new(ErrorImpl::EndOfStream),
    }
}

fn parse_tag(libyaml_tag: &Option<Tag>) -> Option<&str> {
    let mut bytes: &[u8] = libyaml_tag.as_ref()?;
    if let (b'!', rest) = bytes.split_first()? {
        if !rest.is_empty() {
            bytes = rest;
        }
        str::from_utf8(bytes).ok()
    } else {
        None
    }
}

impl<'de, 'document> de::Deserializer<'de> for &mut DeserializerFromEvents<'de, 'document> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let tagged_already = self.current_enum.is_some();
        let (next, mark) = self.next_event_mark()?;
        fn enum_tag(tag: &Option<Tag>, tagged_already: bool) -> Option<&str> {
            if tagged_already {
                return None;
            }
            parse_tag(tag)
        }
        loop {
            match next {
                Event::Alias(mut pos) => break self.jump(&mut pos)?.deserialize_any(visitor),
                Event::Scalar(scalar) => {
                    if let Some(tag) = enum_tag(&scalar.tag, tagged_already) {
                        *self.pos -= 1;
                        break visitor.visit_enum(EnumAccess {
                            de: self,
                            name: None,
                            tag,
                        });
                    }
                    break visit_scalar(visitor, scalar, tagged_already);
                }
                Event::SequenceStart(sequence) => {
                    if let Some(tag) = enum_tag(&sequence.tag, tagged_already) {
                        *self.pos -= 1;
                        break visitor.visit_enum(EnumAccess {
                            de: self,
                            name: None,
                            tag,
                        });
                    }
                    break self.visit_sequence(visitor, mark);
                }
                Event::MappingStart(mapping) => {
                    if let Some(tag) = enum_tag(&mapping.tag, tagged_already) {
                        *self.pos -= 1;
                        break visitor.visit_enum(EnumAccess {
                            de: self,
                            name: None,
                            tag,
                        });
                    }
                    break self.visit_mapping(visitor, mark);
                }
                Event::SequenceEnd => panic!("unexpected end of sequence"),
                Event::MappingEnd => panic!("unexpected end of mapping"),
                Event::Void => break visitor.visit_none(),
            }
        }
        // The de::Error impl creates errors with unknown line and column. Fill
        // in the position here by looking at the current index in the input.
        .map_err(|err| error::fix_mark(err, mark, self.path))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let tagged_already = self.current_enum.is_some();
        let (next, mark) = self.next_event_mark()?;
        loop {
            match next {
                Event::Alias(mut pos) => break self.jump(&mut pos)?.deserialize_bool(visitor),
                Event::Scalar(scalar)
                    if is_plain_or_tagged_literal_scalar(Tag::BOOL, scalar, tagged_already) =>
                {
                    if let Ok(value) = str::from_utf8(&scalar.value) {
                        if let Some(boolean) = parse_bool(value) {
                            break visitor.visit_bool(boolean);
                        }
                    }
                }
                _ => {}
            }
            break Err(invalid_type(next, &visitor));
        }
        .map_err(|err| error::fix_mark(err, mark, self.path))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let tagged_already = self.current_enum.is_some();
        let (next, mark) = self.next_event_mark()?;
        loop {
            match next {
                Event::Alias(mut pos) => break self.jump(&mut pos)?.deserialize_i64(visitor),
                Event::Scalar(scalar)
                    if is_plain_or_tagged_literal_scalar(Tag::INT, scalar, tagged_already) =>
                {
                    if let Ok(value) = str::from_utf8(&scalar.value) {
                        if let Some(int) = parse_signed_int(value, i64::from_str_radix) {
                            break visitor.visit_i64(int);
                        }
                    }
                }
                _ => {}
            }
            break Err(invalid_type(next, &visitor));
        }
        .map_err(|err| error::fix_mark(err, mark, self.path))
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let tagged_already = self.current_enum.is_some();
        let (next, mark) = self.next_event_mark()?;
        loop {
            match next {
                Event::Alias(mut pos) => break self.jump(&mut pos)?.deserialize_i128(visitor),
                Event::Scalar(scalar)
                    if is_plain_or_tagged_literal_scalar(Tag::INT, scalar, tagged_already) =>
                {
                    if let Ok(value) = str::from_utf8(&scalar.value) {
                        if let Some(int) = parse_signed_int(value, i128::from_str_radix) {
                            break visitor.visit_i128(int);
                        }
                    }
                }
                _ => {}
            }
            break Err(invalid_type(next, &visitor));
        }
        .map_err(|err| error::fix_mark(err, mark, self.path))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let tagged_already = self.current_enum.is_some();
        let (next, mark) = self.next_event_mark()?;
        loop {
            match next {
                Event::Alias(mut pos) => break self.jump(&mut pos)?.deserialize_u64(visitor),
                Event::Scalar(scalar)
                    if is_plain_or_tagged_literal_scalar(Tag::INT, scalar, tagged_already) =>
                {
                    if let Ok(value) = str::from_utf8(&scalar.value) {
                        if let Some(int) = parse_unsigned_int(value, u64::from_str_radix) {
                            break visitor.visit_u64(int);
                        }
                    }
                }
                _ => {}
            }
            break Err(invalid_type(next, &visitor));
        }
        .map_err(|err| error::fix_mark(err, mark, self.path))
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let tagged_already = self.current_enum.is_some();
        let (next, mark) = self.next_event_mark()?;
        loop {
            match next {
                Event::Alias(mut pos) => break self.jump(&mut pos)?.deserialize_u128(visitor),
                Event::Scalar(scalar)
                    if is_plain_or_tagged_literal_scalar(Tag::INT, scalar, tagged_already) =>
                {
                    if let Ok(value) = str::from_utf8(&scalar.value) {
                        if let Some(int) = parse_unsigned_int(value, u128::from_str_radix) {
                            break visitor.visit_u128(int);
                        }
                    }
                }
                _ => {}
            }
            break Err(invalid_type(next, &visitor));
        }
        .map_err(|err| error::fix_mark(err, mark, self.path))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let tagged_already = self.current_enum.is_some();
        let (next, mark) = self.next_event_mark()?;
        loop {
            match next {
                Event::Alias(mut pos) => break self.jump(&mut pos)?.deserialize_f64(visitor),
                Event::Scalar(scalar)
                    if is_plain_or_tagged_literal_scalar(Tag::FLOAT, scalar, tagged_already) =>
                {
                    if let Ok(value) = str::from_utf8(&scalar.value) {
                        if let Some(float) = parse_f64(value) {
                            break visitor.visit_f64(float);
                        }
                    }
                }
                _ => {}
            }
            break Err(invalid_type(next, &visitor));
        }
        .map_err(|err| error::fix_mark(err, mark, self.path))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (next, mark) = self.next_event_mark()?;
        match next {
            Event::Scalar(scalar) => {
                if let Ok(v) = str::from_utf8(&scalar.value) {
                    if let Some(borrowed) = parse_borrowed_str(v, scalar.repr, scalar.style) {
                        visitor.visit_borrowed_str(borrowed)
                    } else {
                        visitor.visit_str(v)
                    }
                } else {
                    Err(invalid_type(next, &visitor))
                }
            }
            Event::Alias(mut pos) => self.jump(&mut pos)?.deserialize_str(visitor),
            other => Err(invalid_type(other, &visitor)),
        }
        .map_err(|err: Error| error::fix_mark(err, mark, self.path))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(error::new(ErrorImpl::BytesUnsupported))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(error::new(ErrorImpl::BytesUnsupported))
    }

    /// Parses `null` as None and any other values as `Some(...)`.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let is_some = match self.peek_event()? {
            Event::Alias(mut pos) => {
                *self.pos += 1;
                return self.jump(&mut pos)?.deserialize_option(visitor);
            }
            Event::Scalar(scalar) => {
                let tagged_already = self.current_enum.is_some();
                if scalar.style != ScalarStyle::Plain {
                    true
                } else if let (Some(tag), false) = (&scalar.tag, tagged_already) {
                    if tag == Tag::NULL {
                        if let Some(()) = parse_null(&scalar.value) {
                            false
                        } else if let Ok(v) = str::from_utf8(&scalar.value) {
                            return Err(de::Error::invalid_value(Unexpected::Str(v), &"null"));
                        } else {
                            return Err(de::Error::invalid_value(
                                Unexpected::Bytes(&scalar.value),
                                &"null",
                            ));
                        }
                    } else {
                        true
                    }
                } else {
                    !scalar.value.is_empty() && parse_null(&scalar.value).is_none()
                }
            }
            Event::SequenceStart(_) | Event::MappingStart(_) => true,
            Event::SequenceEnd => panic!("unexpected end of sequence"),
            Event::MappingEnd => panic!("unexpected end of mapping"),
            Event::Void => false,
        };
        if is_some {
            visitor.visit_some(self)
        } else {
            *self.pos += 1;
            self.current_enum = None;
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let tagged_already = self.current_enum.is_some();
        let (next, mark) = self.next_event_mark()?;
        match next {
            Event::Scalar(scalar) => {
                let is_null = if scalar.style != ScalarStyle::Plain {
                    false
                } else if let (Some(tag), false) = (&scalar.tag, tagged_already) {
                    tag == Tag::NULL && parse_null(&scalar.value).is_some()
                } else {
                    scalar.value.is_empty() || parse_null(&scalar.value).is_some()
                };
                if is_null {
                    visitor.visit_unit()
                } else if let Ok(v) = str::from_utf8(&scalar.value) {
                    Err(de::Error::invalid_value(Unexpected::Str(v), &"null"))
                } else {
                    Err(de::Error::invalid_value(
                        Unexpected::Bytes(&scalar.value),
                        &"null",
                    ))
                }
            }
            Event::Alias(mut pos) => self.jump(&mut pos)?.deserialize_unit(visitor),
            Event::Void => visitor.visit_unit(),
            other => Err(invalid_type(other, &visitor)),
        }
        .map_err(|err| error::fix_mark(err, mark, self.path))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    /// Parses a newtype struct as the underlying value.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (_event, mark) = self.peek_event_mark()?;
        self.recursion_check(mark, |de| visitor.visit_newtype_struct(de))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (next, mark) = self.next_event_mark()?;
        match next {
            Event::Alias(mut pos) => self.jump(&mut pos)?.deserialize_seq(visitor),
            Event::SequenceStart(_) => self.visit_sequence(visitor, mark),
            other => {
                if match other {
                    Event::Void => true,
                    Event::Scalar(scalar) => {
                        scalar.value.is_empty() && scalar.style == ScalarStyle::Plain
                    }
                    _ => false,
                } {
                    visitor.visit_seq(SeqAccess {
                        empty: true,
                        de: self,
                        len: 0,
                    })
                } else {
                    Err(invalid_type(other, &visitor))
                }
            }
        }
        .map_err(|err| error::fix_mark(err, mark, self.path))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (next, mark) = self.next_event_mark()?;
        match next {
            Event::Alias(mut pos) => self.jump(&mut pos)?.deserialize_map(visitor),
            Event::MappingStart(_) => self.visit_mapping(visitor, mark),
            other => {
                if match other {
                    Event::Void => true,
                    Event::Scalar(scalar) => {
                        scalar.value.is_empty() && scalar.style == ScalarStyle::Plain
                    }
                    _ => false,
                } {
                    visitor.visit_map(MapAccess {
                        empty: true,
                        de: self,
                        len: 0,
                        key: None,
                    })
                } else {
                    Err(invalid_type(other, &visitor))
                }
            }
        }
        .map_err(|err| error::fix_mark(err, mark, self.path))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    /// Parses an enum as a single key:value pair where the key identifies the
    /// variant and the value gives the content. A String will also parse correctly
    /// to a unit enum value.
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (next, mark) = self.peek_event_mark()?;
        loop {
            if let Some(current_enum) = self.current_enum {
                if let Event::Scalar(scalar) = next {
                    if !scalar.value.is_empty() {
                        break visitor.visit_enum(UnitVariantAccess { de: self });
                    }
                }
                let message = if let Some(name) = current_enum.name {
                    format!(
                        "deserializing nested enum in {}::{} from YAML is not supported yet",
                        name, current_enum.tag,
                    )
                } else {
                    format!(
                        "deserializing nested enum in !{} from YAML is not supported yet",
                        current_enum.tag,
                    )
                };
                break Err(error::new(ErrorImpl::Message(message, None)));
            }
            break match next {
                Event::Alias(mut pos) => {
                    *self.pos += 1;
                    self.jump(&mut pos)?
                        .deserialize_enum(name, variants, visitor)
                }
                Event::Scalar(scalar) => {
                    if let Some(tag) = parse_tag(&scalar.tag) {
                        return visitor.visit_enum(EnumAccess {
                            de: self,
                            name: Some(name),
                            tag,
                        });
                    }
                    visitor.visit_enum(UnitVariantAccess { de: self })
                }
                Event::MappingStart(mapping) => {
                    if let Some(tag) = parse_tag(&mapping.tag) {
                        return visitor.visit_enum(EnumAccess {
                            de: self,
                            name: Some(name),
                            tag,
                        });
                    }
                    let err =
                        de::Error::invalid_type(Unexpected::Map, &"a YAML tag starting with '!'");
                    Err(error::fix_mark(err, mark, self.path))
                }
                Event::SequenceStart(sequence) => {
                    if let Some(tag) = parse_tag(&sequence.tag) {
                        return visitor.visit_enum(EnumAccess {
                            de: self,
                            name: Some(name),
                            tag,
                        });
                    }
                    let err =
                        de::Error::invalid_type(Unexpected::Seq, &"a YAML tag starting with '!'");
                    Err(error::fix_mark(err, mark, self.path))
                }
                Event::SequenceEnd => panic!("unexpected end of sequence"),
                Event::MappingEnd => panic!("unexpected end of mapping"),
                Event::Void => Err(error::new(ErrorImpl::EndOfStream)),
            };
        }
        .map_err(|err| error::fix_mark(err, mark, self.path))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.ignore_any()?;
        visitor.visit_unit()
    }
}

/// Deserialize an instance of type `T` from a string of YAML text.
///
/// This conversion can fail if the structure of the Value does not match the
/// structure expected by `T`, for example if `T` is a struct type but the Value
/// contains something other than a YAML map. It can also fail if the structure
/// is correct but `T`'s implementation of `Deserialize` decides that something
/// is wrong with the data, for example required struct fields are missing from
/// the YAML map or some number is too big to fit in the expected primitive
/// type.
pub fn from_str<'de, T>(s: &'de str) -> Result<T>
where
    T: Deserialize<'de>,
{
    T::deserialize(Deserializer::from_str(s))
}

/// Deserialize an instance of type `T` from an IO stream of YAML.
///
/// This conversion can fail if the structure of the Value does not match the
/// structure expected by `T`, for example if `T` is a struct type but the Value
/// contains something other than a YAML map. It can also fail if the structure
/// is correct but `T`'s implementation of `Deserialize` decides that something
/// is wrong with the data, for example required struct fields are missing from
/// the YAML map or some number is too big to fit in the expected primitive
/// type.
pub fn from_reader<R, T>(rdr: R) -> Result<T>
where
    R: io::Read,
    T: DeserializeOwned,
{
    T::deserialize(Deserializer::from_reader(rdr))
}

/// Deserialize an instance of type `T` from bytes of YAML text.
///
/// This conversion can fail if the structure of the Value does not match the
/// structure expected by `T`, for example if `T` is a struct type but the Value
/// contains something other than a YAML map. It can also fail if the structure
/// is correct but `T`'s implementation of `Deserialize` decides that something
/// is wrong with the data, for example required struct fields are missing from
/// the YAML map or some number is too big to fit in the expected primitive
/// type.
pub fn from_slice<'de, T>(v: &'de [u8]) -> Result<T>
where
    T: Deserialize<'de>,
{
    T::deserialize(Deserializer::from_slice(v))
}
