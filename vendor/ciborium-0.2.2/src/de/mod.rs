// SPDX-License-Identifier: Apache-2.0

//! Serde deserialization support for CBOR

mod error;

pub use error::Error;

use alloc::{string::String, vec::Vec};

use ciborium_io::Read;
use ciborium_ll::*;
use serde::{de, de::Deserializer as _, forward_to_deserialize_any};

trait Expected<E: de::Error> {
    fn expected(self, kind: &'static str) -> E;
}

impl<E: de::Error> Expected<E> for Header {
    #[inline]
    fn expected(self, kind: &'static str) -> E {
        de::Error::invalid_type(
            match self {
                Header::Positive(x) => de::Unexpected::Unsigned(x),
                Header::Negative(x) => de::Unexpected::Signed(x as i64 ^ !0),
                Header::Bytes(..) => de::Unexpected::Other("bytes"),
                Header::Text(..) => de::Unexpected::Other("string"),

                Header::Array(..) => de::Unexpected::Seq,
                Header::Map(..) => de::Unexpected::Map,

                Header::Tag(..) => de::Unexpected::Other("tag"),

                Header::Simple(simple::FALSE) => de::Unexpected::Bool(false),
                Header::Simple(simple::TRUE) => de::Unexpected::Bool(true),
                Header::Simple(simple::NULL) => de::Unexpected::Other("null"),
                Header::Simple(simple::UNDEFINED) => de::Unexpected::Other("undefined"),
                Header::Simple(..) => de::Unexpected::Other("simple"),

                Header::Float(x) => de::Unexpected::Float(x),
                Header::Break => de::Unexpected::Other("break"),
            },
            &kind,
        )
    }
}

struct Deserializer<'b, R: Read> {
    decoder: Decoder<R>,
    scratch: &'b mut [u8],
    recurse: usize,
}

impl<'a, R: Read> Deserializer<'a, R>
where
    R::Error: core::fmt::Debug,
{
    #[inline]
    fn recurse<V, F: FnOnce(&mut Self) -> Result<V, Error<R::Error>>>(
        &mut self,
        func: F,
    ) -> Result<V, Error<R::Error>> {
        if self.recurse == 0 {
            return Err(Error::RecursionLimitExceeded);
        }

        self.recurse -= 1;
        let result = func(self);
        self.recurse += 1;
        result
    }

    #[inline]
    fn integer(&mut self, mut header: Option<Header>) -> Result<(bool, u128), Error<R::Error>> {
        loop {
            let header = match header.take() {
                Some(h) => h,
                None => self.decoder.pull()?,
            };

            let neg = match header {
                Header::Positive(x) => return Ok((false, x.into())),
                Header::Negative(x) => return Ok((true, x.into())),
                Header::Tag(tag::BIGPOS) => false,
                Header::Tag(tag::BIGNEG) => true,
                Header::Tag(..) => continue,
                header => return Err(header.expected("integer")),
            };

            let mut buffer = [0u8; 16];
            let mut value = [0u8; 16];
            let mut index = 0usize;

            return match self.decoder.pull()? {
                Header::Bytes(len) => {
                    let mut segments = self.decoder.bytes(len);
                    while let Some(mut segment) = segments.pull()? {
                        while let Some(chunk) = segment.pull(&mut buffer)? {
                            for b in chunk {
                                match index {
                                    16 => return Err(de::Error::custom("bigint too large")),
                                    0 if *b == 0 => continue, // Skip leading zeros
                                    _ => value[index] = *b,
                                }

                                index += 1;
                            }
                        }
                    }

                    value[..index].reverse();
                    Ok((neg, u128::from_le_bytes(value)))
                }

                h => Err(h.expected("bytes")),
            };
        }
    }
}

impl<'de, 'a, 'b, R: Read> de::Deserializer<'de> for &'a mut Deserializer<'b, R>
where
    R::Error: core::fmt::Debug,
{
    type Error = Error<R::Error>;

    #[inline]
    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let header = self.decoder.pull()?;
        self.decoder.push(header);

        match header {
            Header::Positive(..) => self.deserialize_u64(visitor),
            Header::Negative(x) => match i64::try_from(x) {
                Ok(..) => self.deserialize_i64(visitor),
                Err(..) => self.deserialize_i128(visitor),
            },

            Header::Bytes(len) => match len {
                Some(len) if len <= self.scratch.len() => self.deserialize_bytes(visitor),
                _ => self.deserialize_byte_buf(visitor),
            },

            Header::Text(len) => match len {
                Some(len) if len <= self.scratch.len() => self.deserialize_str(visitor),
                _ => self.deserialize_string(visitor),
            },

            Header::Array(..) => self.deserialize_seq(visitor),
            Header::Map(..) => self.deserialize_map(visitor),

            Header::Tag(tag) => {
                let _: Header = self.decoder.pull()?;

                // Peek at the next item.
                let header = self.decoder.pull()?;
                self.decoder.push(header);

                // If it is bytes, capture the length.
                let len = match header {
                    Header::Bytes(x) => x,
                    _ => None,
                };

                match (tag, len) {
                    (tag::BIGPOS, Some(len)) | (tag::BIGNEG, Some(len)) if len <= 16 => {
                        let result = match self.integer(Some(Header::Tag(tag)))? {
                            (false, raw) => return visitor.visit_u128(raw),
                            (true, raw) => i128::try_from(raw).map(|x| x ^ !0),
                        };

                        match result {
                            Ok(x) => visitor.visit_i128(x),
                            Err(..) => Err(de::Error::custom("integer too large")),
                        }
                    }

                    _ => self.recurse(|me| {
                        let access = crate::tag::TagAccess::new(me, Some(tag));
                        visitor.visit_enum(access)
                    }),
                }
            }

            Header::Float(..) => self.deserialize_f64(visitor),

            Header::Simple(simple::FALSE) => self.deserialize_bool(visitor),
            Header::Simple(simple::TRUE) => self.deserialize_bool(visitor),
            Header::Simple(simple::NULL) => self.deserialize_option(visitor),
            Header::Simple(simple::UNDEFINED) => self.deserialize_option(visitor),
            h @ Header::Simple(..) => Err(h.expected("known simple value")),

            h @ Header::Break => Err(h.expected("non-break")),
        }
    }

    #[inline]
    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        loop {
            let offset = self.decoder.offset();

            return match self.decoder.pull()? {
                Header::Tag(..) => continue,
                Header::Simple(simple::FALSE) => visitor.visit_bool(false),
                Header::Simple(simple::TRUE) => visitor.visit_bool(true),
                _ => Err(Error::semantic(offset, "expected bool")),
            };
        }
    }

    #[inline]
    fn deserialize_f32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_f64(visitor)
    }

    #[inline]
    fn deserialize_f64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        loop {
            return match self.decoder.pull()? {
                Header::Tag(..) => continue,
                Header::Float(x) => visitor.visit_f64(x),
                h => Err(h.expected("float")),
            };
        }
    }

    fn deserialize_i8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let result = match self.integer(None)? {
            (false, raw) => i64::try_from(raw),
            (true, raw) => i64::try_from(raw).map(|x| x ^ !0),
        };

        match result {
            Ok(x) => visitor.visit_i64(x),
            Err(..) => Err(de::Error::custom("integer too large")),
        }
    }

    fn deserialize_i128<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let result = match self.integer(None)? {
            (false, raw) => i128::try_from(raw),
            (true, raw) => i128::try_from(raw).map(|x| x ^ !0),
        };

        match result {
            Ok(x) => visitor.visit_i128(x),
            Err(..) => Err(de::Error::custom("integer too large")),
        }
    }

    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let result = match self.integer(None)? {
            (false, raw) => u64::try_from(raw),
            (true, ..) => return Err(de::Error::custom("unexpected negative integer")),
        };

        match result {
            Ok(x) => visitor.visit_u64(x),
            Err(..) => Err(de::Error::custom("integer too large")),
        }
    }

    fn deserialize_u128<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.integer(None)? {
            (false, raw) => visitor.visit_u128(raw),
            (true, ..) => Err(de::Error::custom("unexpected negative integer")),
        }
    }

    fn deserialize_char<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        loop {
            let offset = self.decoder.offset();
            let header = self.decoder.pull()?;

            return match header {
                Header::Tag(..) => continue,

                Header::Text(Some(len)) if len <= 4 => {
                    let mut buf = [0u8; 4];
                    self.decoder.read_exact(&mut buf[..len])?;

                    match core::str::from_utf8(&buf[..len]) {
                        Ok(s) => match s.chars().count() {
                            1 => visitor.visit_char(s.chars().next().unwrap()),
                            _ => Err(header.expected("char")),
                        },
                        Err(..) => Err(Error::Syntax(offset)),
                    }
                }

                _ => Err(header.expected("char")),
            };
        }
    }

    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        loop {
            let offset = self.decoder.offset();

            return match self.decoder.pull()? {
                Header::Tag(..) => continue,

                Header::Text(Some(len)) if len <= self.scratch.len() => {
                    self.decoder.read_exact(&mut self.scratch[..len])?;

                    match core::str::from_utf8(&self.scratch[..len]) {
                        Ok(s) => visitor.visit_str(s),
                        Err(..) => Err(Error::Syntax(offset)),
                    }
                }

                header => Err(header.expected("str")),
            };
        }
    }

    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        loop {
            return match self.decoder.pull()? {
                Header::Tag(..) => continue,

                Header::Text(len) => {
                    let mut buffer = String::new();

                    let mut segments = self.decoder.text(len);
                    while let Some(mut segment) = segments.pull()? {
                        while let Some(chunk) = segment.pull(self.scratch)? {
                            buffer.push_str(chunk);
                        }
                    }

                    visitor.visit_string(buffer)
                }

                header => Err(header.expected("string")),
            };
        }
    }

    fn deserialize_bytes<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        loop {
            return match self.decoder.pull()? {
                Header::Tag(..) => continue,

                Header::Bytes(Some(len)) if len <= self.scratch.len() => {
                    self.decoder.read_exact(&mut self.scratch[..len])?;
                    visitor.visit_bytes(&self.scratch[..len])
                }

                Header::Array(len) => self.recurse(|me| {
                    let access = Access(me, len);
                    visitor.visit_seq(access)
                }),

                header => Err(header.expected("bytes")),
            };
        }
    }

    fn deserialize_byte_buf<V: de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        loop {
            return match self.decoder.pull()? {
                Header::Tag(..) => continue,

                Header::Bytes(len) => {
                    let mut buffer = Vec::new();

                    let mut segments = self.decoder.bytes(len);
                    while let Some(mut segment) = segments.pull()? {
                        while let Some(chunk) = segment.pull(self.scratch)? {
                            buffer.extend_from_slice(chunk);
                        }
                    }

                    visitor.visit_byte_buf(buffer)
                }

                Header::Array(len) => self.recurse(|me| {
                    let access = Access(me, len);
                    visitor.visit_seq(access)
                }),

                header => Err(header.expected("byte buffer")),
            };
        }
    }

    fn deserialize_seq<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        loop {
            return match self.decoder.pull()? {
                Header::Tag(..) => continue,

                Header::Array(len) => self.recurse(|me| {
                    let access = Access(me, len);
                    visitor.visit_seq(access)
                }),

                Header::Bytes(len) => {
                    let mut buffer = Vec::new();

                    let mut segments = self.decoder.bytes(len);
                    while let Some(mut segment) = segments.pull()? {
                        while let Some(chunk) = segment.pull(self.scratch)? {
                            buffer.extend_from_slice(chunk);
                        }
                    }

                    visitor.visit_seq(BytesAccess::<R>(0, buffer, core::marker::PhantomData))
                }

                header => Err(header.expected("array")),
            };
        }
    }

    fn deserialize_map<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        loop {
            return match self.decoder.pull()? {
                Header::Tag(..) => continue,

                Header::Map(len) => self.recurse(|me| {
                    let access = Access(me, len);
                    visitor.visit_map(access)
                }),

                header => Err(header.expected("map")),
            };
        }
    }

    fn deserialize_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_map(visitor)
    }

    fn deserialize_tuple<V: de::Visitor<'de>>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_identifier<V: de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        loop {
            let offset = self.decoder.offset();

            return match self.decoder.pull()? {
                Header::Tag(..) => continue,

                Header::Text(Some(len)) if len <= self.scratch.len() => {
                    self.decoder.read_exact(&mut self.scratch[..len])?;

                    match core::str::from_utf8(&self.scratch[..len]) {
                        Ok(s) => visitor.visit_str(s),
                        Err(..) => Err(Error::Syntax(offset)),
                    }
                }
                Header::Bytes(Some(len)) if len <= self.scratch.len() => {
                    self.decoder.read_exact(&mut self.scratch[..len])?;
                    visitor.visit_bytes(&self.scratch[..len])
                }

                header => Err(header.expected("str or bytes")),
            };
        }
    }

    fn deserialize_ignored_any<V: de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    #[inline]
    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.decoder.pull()? {
            Header::Simple(simple::UNDEFINED) => visitor.visit_none(),
            Header::Simple(simple::NULL) => visitor.visit_none(),
            header => {
                self.decoder.push(header);
                visitor.visit_some(self)
            }
        }
    }

    #[inline]
    fn deserialize_unit<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        loop {
            return match self.decoder.pull()? {
                Header::Simple(simple::UNDEFINED) => visitor.visit_unit(),
                Header::Simple(simple::NULL) => visitor.visit_unit(),
                Header::Tag(..) => continue,
                header => Err(header.expected("unit")),
            };
        }
    }

    #[inline]
    fn deserialize_unit_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_unit(visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if name == "@@TAG@@" {
            let tag = match self.decoder.pull()? {
                Header::Tag(x) => Some(x),
                header => {
                    self.decoder.push(header);
                    None
                }
            };

            return self.recurse(|me| {
                let access = crate::tag::TagAccess::new(me, tag);
                visitor.visit_enum(access)
            });
        }

        loop {
            match self.decoder.pull()? {
                Header::Tag(..) => continue,
                Header::Map(Some(1)) => (),
                header @ Header::Text(..) => self.decoder.push(header),
                header => return Err(header.expected("enum")),
            }

            return self.recurse(|me| {
                let access = Access(me, Some(0));
                visitor.visit_enum(access)
            });
        }
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

struct Access<'a, 'b, R: Read>(&'a mut Deserializer<'b, R>, Option<usize>);

impl<'de, 'a, 'b, R: Read> de::SeqAccess<'de> for Access<'a, 'b, R>
where
    R::Error: core::fmt::Debug,
{
    type Error = Error<R::Error>;

    #[inline]
    fn next_element_seed<U: de::DeserializeSeed<'de>>(
        &mut self,
        seed: U,
    ) -> Result<Option<U::Value>, Self::Error> {
        match self.1 {
            Some(0) => return Ok(None),
            Some(x) => self.1 = Some(x - 1),
            None => match self.0.decoder.pull()? {
                Header::Break => return Ok(None),
                header => self.0.decoder.push(header),
            },
        }

        seed.deserialize(&mut *self.0).map(Some)
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        self.1
    }
}

impl<'de, 'a, 'b, R: Read> de::MapAccess<'de> for Access<'a, 'b, R>
where
    R::Error: core::fmt::Debug,
{
    type Error = Error<R::Error>;

    #[inline]
    fn next_key_seed<K: de::DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        match self.1 {
            Some(0) => return Ok(None),
            Some(x) => self.1 = Some(x - 1),
            None => match self.0.decoder.pull()? {
                Header::Break => return Ok(None),
                header => self.0.decoder.push(header),
            },
        }

        seed.deserialize(&mut *self.0).map(Some)
    }

    #[inline]
    fn next_value_seed<V: de::DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        seed.deserialize(&mut *self.0)
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        self.1
    }
}

impl<'de, 'a, 'b, R: Read> de::EnumAccess<'de> for Access<'a, 'b, R>
where
    R::Error: core::fmt::Debug,
{
    type Error = Error<R::Error>;
    type Variant = Self;

    #[inline]
    fn variant_seed<V: de::DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), Self::Error> {
        let variant = seed.deserialize(&mut *self.0)?;
        Ok((variant, self))
    }
}

impl<'de, 'a, 'b, R: Read> de::VariantAccess<'de> for Access<'a, 'b, R>
where
    R::Error: core::fmt::Debug,
{
    type Error = Error<R::Error>;

    #[inline]
    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn newtype_variant_seed<U: de::DeserializeSeed<'de>>(
        self,
        seed: U,
    ) -> Result<U::Value, Self::Error> {
        seed.deserialize(&mut *self.0)
    }

    #[inline]
    fn tuple_variant<V: de::Visitor<'de>>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_any(visitor)
    }

    #[inline]
    fn struct_variant<V: de::Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.deserialize_any(visitor)
    }
}

struct BytesAccess<R: Read>(usize, Vec<u8>, core::marker::PhantomData<R>);

impl<'de, R: Read> de::SeqAccess<'de> for BytesAccess<R>
where
    R::Error: core::fmt::Debug,
{
    type Error = Error<R::Error>;

    #[inline]
    fn next_element_seed<U: de::DeserializeSeed<'de>>(
        &mut self,
        seed: U,
    ) -> Result<Option<U::Value>, Self::Error> {
        use de::IntoDeserializer;

        if self.0 < self.1.len() {
            let byte = self.1[self.0];
            self.0 += 1;
            seed.deserialize(byte.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        Some(self.1.len() - self.0)
    }
}

struct TagAccess<'a, 'b, R: Read>(&'a mut Deserializer<'b, R>, usize);

impl<'de, 'a, 'b, R: Read> de::Deserializer<'de> for &mut TagAccess<'a, 'b, R>
where
    R::Error: core::fmt::Debug,
{
    type Error = Error<R::Error>;

    #[inline]
    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let offset = self.0.decoder.offset();

        match self.0.decoder.pull()? {
            Header::Tag(x) => visitor.visit_u64(x),
            _ => Err(Error::semantic(offset, "expected tag")),
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

impl<'de, 'a, 'b, R: Read> de::SeqAccess<'de> for TagAccess<'a, 'b, R>
where
    R::Error: core::fmt::Debug,
{
    type Error = Error<R::Error>;

    #[inline]
    fn next_element_seed<U: de::DeserializeSeed<'de>>(
        &mut self,
        seed: U,
    ) -> Result<Option<U::Value>, Self::Error> {
        self.1 += 1;

        match self.1 {
            1 => seed.deserialize(self).map(Some),
            2 => seed.deserialize(&mut *self.0).map(Some),
            _ => Ok(None),
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        Some(match self.1 {
            0 => 2,
            1 => 1,
            _ => 0,
        })
    }
}

/// Deserializes as CBOR from a type with [`impl
/// ciborium_io::Read`](ciborium_io::Read) using a 4KB buffer on the stack.
///
/// If you want to deserialize faster at the cost of more memory, consider using
/// [`from_reader_with_buffer`](from_reader_with_buffer) with a larger buffer,
/// for example 64KB.
#[inline]
pub fn from_reader<T: de::DeserializeOwned, R: Read>(reader: R) -> Result<T, Error<R::Error>>
where
    R::Error: core::fmt::Debug,
{
    let mut scratch = [0; 4096];
    from_reader_with_buffer(reader, &mut scratch)
}

/// Deserializes as CBOR from a type with [`impl
/// ciborium_io::Read`](ciborium_io::Read), using a caller-specific buffer as a
/// temporary scratch space.
#[inline]
pub fn from_reader_with_buffer<T: de::DeserializeOwned, R: Read>(
    reader: R,
    scratch_buffer: &mut [u8],
) -> Result<T, Error<R::Error>>
where
    R::Error: core::fmt::Debug,
{
    let mut reader = Deserializer {
        decoder: reader.into(),
        scratch: scratch_buffer,
        recurse: 256,
    };

    T::deserialize(&mut reader)
}

/// Deserializes as CBOR from a type with [`impl ciborium_io::Read`](ciborium_io::Read), with
/// a specified maximum recursion limit.  Inputs that are nested beyond the specified limit
/// will result in [`Error::RecursionLimitExceeded`] .
///
/// Set a high recursion limit at your own risk (of stack exhaustion)!
#[inline]
pub fn from_reader_with_recursion_limit<T: de::DeserializeOwned, R: Read>(
    reader: R,
    recurse_limit: usize,
) -> Result<T, Error<R::Error>>
where
    R::Error: core::fmt::Debug,
{
    let mut scratch = [0; 4096];

    let mut reader = Deserializer {
        decoder: reader.into(),
        scratch: &mut scratch,
        recurse: recurse_limit,
    };

    T::deserialize(&mut reader)
}
