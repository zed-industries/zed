// Copyright 2017, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use super::EncoderState::*;
use super::{EncodeResult, Encoder, EncoderError, ENCODER_SAM};
use serde::ser::*;
use std::fmt;
use std::mem;

impl Error for EncoderError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        EncoderError::SerializerError(format!("{}", msg))
    }
}

impl<'a> Serializer for &'a mut Encoder {
    type Ok = ();
    type Error = EncoderError;

    type SerializeSeq = SeqEncoder;
    type SerializeTuple = TupleEncoder;
    type SerializeTupleStruct = TupleStructEncoder;
    type SerializeTupleVariant = TupleVariantEncoder;
    type SerializeMap = StructMapEncoder<'a>;
    type SerializeStruct = StructMapEncoder<'a>;
    type SerializeStructVariant = StructVariantEncoder;

    fn serialize_bool(self, value: bool) -> EncodeResult<Self::Ok> {
        self.serialize_u32(value as u32)
    }

    fn serialize_i8(self, value: i8) -> EncodeResult<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i16(self, value: i16) -> EncodeResult<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i32(self, value: i32) -> EncodeResult<Self::Ok> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i64(self, value: i64) -> EncodeResult<Self::Ok> {
        let s = value.to_string();
        emit_value!(self, s)
    }

    fn serialize_u8(self, value: u8) -> EncodeResult<Self::Ok> {
        self.serialize_u32(value as u32)
    }

    fn serialize_u16(self, value: u16) -> EncodeResult<Self::Ok> {
        self.serialize_u32(value as u32)
    }

    fn serialize_u32(self, value: u32) -> EncodeResult<Self::Ok> {
        emit_value!(self, value)
    }

    fn serialize_u64(self, value: u64) -> EncodeResult<Self::Ok> {
        emit_value!(self, value)
    }

    fn serialize_f32(self, value: f32) -> EncodeResult<Self::Ok> {
        let s = value.to_string();
        emit_value!(self, s)
    }

    fn serialize_f64(self, value: f64) -> EncodeResult<Self::Ok> {
        let s = value.to_string();
        emit_value!(self, s)
    }

    fn serialize_char(self, value: char) -> EncodeResult<Self::Ok> {
        let mut s = String::new();
        s.push(value);
        emit_value!(self, s)
    }

    fn serialize_str(self, value: &str) -> EncodeResult<Self::Ok> {
        emit_value!(self, value)
    }

    fn serialize_bytes(self, _value: &[u8]) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_bytes")
    }

    fn serialize_none(self) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_none")
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_some")
    }

    fn serialize_unit(self) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_unit")
    }

    fn serialize_unit_struct(self, _name: &'static str) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_unit_struct")
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_unit_variant")
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_newtype_struct")
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> EncodeResult<Self::Ok> {
        no_impl!("serialize_newtype_variant")
    }

    fn serialize_seq(self, _len: Option<usize>) -> EncodeResult<Self::SerializeSeq> {
        no_impl!("serialize_seq")
    }

    fn serialize_tuple(self, _len: usize) -> EncodeResult<Self::SerializeTuple> {
        no_impl!("serialize_tuple")
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> EncodeResult<Self::SerializeTupleStruct> {
        no_impl!("serialize_tuple_struct")
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> EncodeResult<Self::SerializeTupleVariant> {
        no_impl!("serialize_tuple_variant")
    }

    fn serialize_map(self, _len: Option<usize>) -> EncodeResult<Self::SerializeMap> {
        match mem::replace(&mut self.state, Start) {
            // ---
            Start => {
                // root structure
                Ok(StructMapEncoder {
                    enc: self,
                    is_root: true,
                })
            }
            NextKey(ref s) => {
                // nested structure
                match self.keys[self.keys.len() - 1].create_subkey_transacted_with_flags(
                    &s,
                    &self.tr,
                    ENCODER_SAM,
                ) {
                    Ok((subkey, _disp)) => {
                        self.keys.push(subkey);
                        Ok(StructMapEncoder {
                            enc: self,
                            is_root: true,
                        })
                    }
                    Err(err) => Err(EncoderError::IoError(err)),
                }
            }
        }
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> EncodeResult<Self::SerializeStruct> {
        self.serialize_map(Some(_len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> EncodeResult<Self::SerializeStructVariant> {
        no_impl!("serialize_struct_variant")
    }
}

pub struct SeqEncoder {}

impl SerializeSeq for SeqEncoder {
    type Ok = ();
    type Error = EncoderError;
    fn serialize_element<T: ?Sized + Serialize>(&mut self, _value: &T) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeSeq::serialize_element")
    }
    fn end(self) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeSeq::end")
    }
}

pub struct TupleEncoder {}

impl SerializeTuple for TupleEncoder {
    type Ok = ();
    type Error = EncoderError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _value: &T) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeTuple::serialize_element")
    }

    fn end(self) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeTuple::end")
    }
}

pub struct TupleStructEncoder {}

impl SerializeTupleStruct for TupleStructEncoder {
    type Ok = ();
    type Error = EncoderError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _value: &T) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeTupleStruct::serialize_field")
    }

    fn end(self) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeTupleStruct::end")
    }
}

pub struct TupleVariantEncoder {}

impl SerializeTupleVariant for TupleVariantEncoder {
    type Ok = ();
    type Error = EncoderError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _value: &T) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeTupleVariant::serialize_field")
    }

    fn end(self) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeTupleVariant::end")
    }
}

struct MapKeySerializer;

impl serde::Serializer for MapKeySerializer {
    type Ok = String;
    type Error = EncoderError;

    type SerializeSeq = Impossible<String, EncoderError>;
    type SerializeTuple = Impossible<String, EncoderError>;
    type SerializeTupleStruct = Impossible<String, EncoderError>;
    type SerializeTupleVariant = Impossible<String, EncoderError>;
    type SerializeMap = Impossible<String, EncoderError>;
    type SerializeStruct = Impossible<String, EncoderError>;
    type SerializeStructVariant = Impossible<String, EncoderError>;

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> EncodeResult<Self::Ok> {
        Ok(variant.to_owned())
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> EncodeResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_bool(self, _value: bool) -> EncodeResult<Self::Ok> {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_i8(self, value: i8) -> EncodeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_i16(self, value: i16) -> EncodeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_i32(self, value: i32) -> EncodeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_i64(self, value: i64) -> EncodeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_u8(self, value: u8) -> EncodeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_u16(self, value: u16) -> EncodeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_u32(self, value: u32) -> EncodeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_u64(self, value: u64) -> EncodeResult<Self::Ok> {
        Ok(value.to_string())
    }

    fn serialize_f32(self, _value: f32) -> EncodeResult<Self::Ok> {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_f64(self, _value: f64) -> EncodeResult<Self::Ok> {
        Err(EncoderError::KeyMustBeAString)
    }

    #[inline]
    fn serialize_char(self, value: char) -> EncodeResult<Self::Ok> {
        Ok({
            let mut s = String::new();
            s.push(value);
            s
        })
    }

    #[inline]
    fn serialize_str(self, value: &str) -> EncodeResult<Self::Ok> {
        Ok(value.to_owned())
    }

    fn serialize_bytes(self, _value: &[u8]) -> EncodeResult<Self::Ok> {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_unit(self) -> EncodeResult<Self::Ok> {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> EncodeResult<Self::Ok> {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> EncodeResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_none(self) -> EncodeResult<Self::Ok> {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_some<T>(self, _value: &T) -> EncodeResult<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_seq(self, _len: Option<usize>) -> EncodeResult<Self::SerializeSeq> {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_tuple(self, _len: usize) -> EncodeResult<Self::SerializeTuple> {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> EncodeResult<Self::SerializeTupleStruct> {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> EncodeResult<Self::SerializeTupleVariant> {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_map(self, _len: Option<usize>) -> EncodeResult<Self::SerializeMap> {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> EncodeResult<Self::SerializeStruct> {
        Err(EncoderError::KeyMustBeAString)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> EncodeResult<Self::SerializeStructVariant> {
        Err(EncoderError::KeyMustBeAString)
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> EncodeResult<String>
    where
        T: fmt::Display,
    {
        Ok(value.to_string())
    }
}

pub struct StructMapEncoder<'a> {
    enc: &'a mut Encoder,
    is_root: bool,
}

impl<'a> SerializeStruct for StructMapEncoder<'a> {
    type Ok = ();
    type Error = EncoderError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> EncodeResult<Self::Ok> {
        self.enc.state = NextKey(String::from(key));
        value.serialize(&mut *self.enc)
    }

    fn end(self) -> EncodeResult<Self::Ok> {
        if self.is_root {
            self.enc.keys.pop();
        }
        Ok(())
    }
}

impl<'a> SerializeMap for StructMapEncoder<'a> {
    type Ok = ();
    type Error = EncoderError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> EncodeResult<Self::Ok> {
        self.enc.state = NextKey(key.serialize(MapKeySerializer)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> EncodeResult<Self::Ok> {
        value.serialize(&mut *self.enc)
    }

    fn end(self) -> EncodeResult<Self::Ok> {
        if self.is_root {
            self.enc.keys.pop();
        }
        Ok(())
    }
}

pub struct StructVariantEncoder {}

impl SerializeStructVariant for StructVariantEncoder {
    type Ok = ();
    type Error = EncoderError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeStructVariant::serialize_field")
    }

    fn end(self) -> EncodeResult<Self::Ok> {
        no_impl!("SerializeStructVariant::end")
    }
}
