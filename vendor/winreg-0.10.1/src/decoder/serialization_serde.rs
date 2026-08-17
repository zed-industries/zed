// Copyright 2017, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use super::super::FromRegValue;
use super::{
    DecodeResult, Decoder, DecoderEnumerationState, DecoderError, DecoderReadingState, DECODER_SAM,
};
use serde::de::*;
use std::fmt;
use std::mem;

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
        use self::DecoderEnumerationState::*;
        match self.enumeration_state {
            EnumeratingKeys(..) => no_impl!("deserialize_any for keys"),
            EnumeratingValues(..) => {
                let s = self.f_name.as_ref().ok_or(DecoderError::NoFieldName)?;
                let v = self.key.get_raw_value(s)?;
                use RegType::*;
                match v.vtype {
                    REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
                        visitor.visit_string(String::from_reg_value(&v)?)
                    }
                    REG_DWORD => visitor.visit_u32(u32::from_reg_value(&v)?),
                    REG_QWORD => visitor.visit_u64(u64::from_reg_value(&v)?),
                    _ => Err(DecoderError::DecodeNotImplemented(
                        "value type deserialization not implemented".to_owned(),
                    )),
                }
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(read_value!(self).map(|v: u32| v > 0)?)
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
        visitor.visit_u32(read_value!(self)?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(read_value!(self)?)
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
        use self::DecoderReadingState::*;
        match self.reading_state {
            WaitingForKey => {
                let s = self.f_name.as_ref().ok_or(DecoderError::NoFieldName)?;
                visitor.visit_string(s.clone())
            }
            WaitingForValue => visitor.visit_string(read_value!(self)?),
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        no_impl!("deserialize_bytes")
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        no_impl!("deserialize_byte_buf")
    }

    fn deserialize_option<V>(self, visitor: V) -> DecodeResult<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = {
            let s = self.f_name.as_ref().ok_or(DecoderError::NoFieldName)?;
            self.key.get_raw_value(s)
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

impl<'de, 'a> MapAccess<'de> for Decoder {
    type Error = DecoderError;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        self.reading_state = DecoderReadingState::WaitingForKey;
        use self::DecoderEnumerationState::*;
        match self.enumeration_state {
            EnumeratingKeys(index) => match self.key.enum_key(index) {
                Some(res) => {
                    self.f_name = Some(res?);
                    self.enumeration_state = EnumeratingKeys(index + 1);
                    seed.deserialize(&mut *self).map(Some)
                }
                None => {
                    self.enumeration_state = EnumeratingValues(0);
                    self.next_key_seed(seed)
                }
            },
            EnumeratingValues(index) => {
                let next_value = self.key.enum_value(index);
                match next_value {
                    Some(res) => {
                        self.f_name = Some(res?.0);
                        self.enumeration_state = EnumeratingValues(index + 1);
                        seed.deserialize(&mut *self).map(Some)
                    }
                    None => Ok(None),
                }
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.reading_state = DecoderReadingState::WaitingForValue;
        use self::DecoderEnumerationState::*;
        match self.enumeration_state {
            EnumeratingKeys(..) => {
                let f_name = self.f_name.as_ref().ok_or(DecoderError::NoFieldName)?;
                match self.key.open_subkey_with_flags(f_name, DECODER_SAM) {
                    Ok(subkey) => {
                        let mut nested = Decoder::new(subkey);
                        seed.deserialize(&mut nested)
                    }
                    Err(err) => Err(DecoderError::IoError(err)),
                }
            }
            EnumeratingValues(..) => seed.deserialize(&mut *self),
        }
    }
}
