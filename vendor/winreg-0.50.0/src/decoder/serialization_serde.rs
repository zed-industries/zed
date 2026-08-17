// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use super::{DecodeResult, Decoder, DecoderCursor, DecoderError, DECODER_SAM};
use crate::types::FromRegValue;
use serde::de::*;
use std::fmt;

impl Error for DecoderError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        DecoderError::DeserializerError(format!("{}", msg))
    }
}

impl<'de, 'a> Deserializer<'de> for &'a mut Decoder {
    type Error = DecoderError;
    fn deserialize_any<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        use super::DecoderCursor::*;
        let cursor = self.cursor.clone();
        match cursor {
            Start => self.deserialize_map(visitor),
            KeyName(..) | FieldName(..) => self.deserialize_string(visitor),
            FieldVal(index, name) => {
                use crate::enums::RegType::*;
                let v = self.key.get_raw_value(name)?;
                self.cursor = Field(index + 1);
                match v.vtype {
                    REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
                        visitor.visit_string(String::from_reg_value(&v)?)
                    }
                    REG_DWORD => visitor.visit_u32(u32::from_reg_value(&v)?),
                    REG_QWORD => visitor.visit_u64(u64::from_reg_value(&v)?),
                    REG_BINARY => visitor.visit_byte_buf(v.bytes),
                    _ => no_impl!("value type deserialization not implemented"),
                }
            }
            _ => no_impl!("deserialize_any"),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.read_value().map(|v: u32| v > 0)?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.read_value()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.read_value()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(parse_string!(self)?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(parse_string!(self)?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(parse_string!(self)?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(parse_string!(self)?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(parse_string!(self)?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(parse_string!(self)?)
    }

    fn deserialize_char<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_str<V>(self, _visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        no_impl!("deserialize_str")
    }

    fn deserialize_string<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        use super::DecoderCursor::*;
        let cursor = self.cursor.clone();
        match cursor {
            KeyName(index, name) => {
                self.cursor = DecoderCursor::KeyVal(index, name.clone());
                visitor.visit_string(name)
            }
            FieldName(index, name) => {
                self.cursor = DecoderCursor::FieldVal(index, name.clone());
                visitor.visit_string(name)
            }
            FieldVal(..) => visitor.visit_string(self.read_value()?),
            _ => Err(DecoderError::NoFieldName),
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        no_impl!("deserialize_bytes")
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_byte_buf(self.read_bytes()?)
    }

    fn deserialize_option<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = {
            use super::DecoderCursor::*;
            match self.cursor {
                FieldVal(_, ref name) => {
                    self.key.get_raw_value(name).map_err(DecoderError::IoError)
                }
                _ => Err(DecoderError::DeserializerError("Nothing found".to_owned())),
            }
        };
        match v {
            Ok(..) => visitor.visit_some(&mut *self),
            Err(..) => visitor.visit_none(),
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        no_impl!("deserialize_unit")
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        no_impl!("deserialize_unit_struct")
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        no_impl!("deserialize_newtype_struct")
    }

    fn deserialize_seq<V>(self, _visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        no_impl!("deserialize_seq")
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        no_impl!("deserialize_tuple")
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        no_impl!("deserialize_tuple_struct")
    }

    fn deserialize_map<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        no_impl!("deserialize_enum")
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

impl<'de> MapAccess<'de> for Decoder {
    type Error = DecoderError;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        use super::DecoderCursor::*;
        match self.cursor {
            Start => {
                self.cursor = Key(0);
                self.next_key_seed(seed)
            }
            Key(index) => match self.key.enum_key(index) {
                Some(res) => {
                    self.cursor = KeyName(index, res?);
                    seed.deserialize(&mut *self).map(Some)
                }
                None => {
                    self.cursor = Field(0);
                    self.next_key_seed(seed)
                }
            },
            Field(index) => {
                let next_value = self.key.enum_value(index);
                match next_value {
                    Some(res) => {
                        self.cursor = FieldName(index, res?.0);
                        seed.deserialize(&mut *self).map(Some)
                    }
                    None => Ok(None),
                }
            }
            _ => no_impl!("Wrong cursor state (key)"),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        use super::DecoderCursor::*;
        match self.cursor {
            KeyVal(index, ref name) => match self.key.open_subkey_with_flags(name, DECODER_SAM) {
                Ok(subkey) => {
                    let mut nested = Decoder::new(subkey);
                    self.cursor = Key(index + 1);
                    seed.deserialize(&mut nested)
                }
                Err(err) => Err(DecoderError::IoError(err)),
            },
            FieldVal(..) => seed.deserialize(&mut *self),
            _ => no_impl!("Wrong cursor state (field)"),
        }
    }
}
