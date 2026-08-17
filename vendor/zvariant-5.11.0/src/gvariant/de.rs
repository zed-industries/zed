use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, Visitor};

use std::{marker::PhantomData, str};

#[cfg(unix)]
use std::os::fd::AsFd;

use crate::{
    Basic, Error, ObjectPath, Result, Signature,
    de::{DeserializerCommon, ValueParseStage},
    framing_offset_size::FramingOffsetSize,
    framing_offsets::FramingOffsets,
    serialized::{Context, Format},
    utils::*,
};

/// Our GVariant deserialization implementation.
#[derive(Debug)]
pub(crate) struct Deserializer<'de, 'sig, 'f, F>(pub(crate) DeserializerCommon<'de, 'sig, 'f, F>);

impl<'de, 'sig, 'f, F> Deserializer<'de, 'sig, 'f, F> {
    /// Create a Deserializer struct instance.
    ///
    /// On Windows, the function doesn't have `fds` argument.
    pub fn new<'r: 'de>(
        bytes: &'r [u8],
        #[cfg(unix)] fds: Option<&'f [F]>,
        signature: &'sig Signature,
        ctxt: Context,
    ) -> Result<Self> {
        assert_eq!(ctxt.format(), Format::GVariant);

        Ok(Self(DeserializerCommon {
            ctxt,
            signature,
            bytes,
            #[cfg(unix)]
            fds,
            #[cfg(not(unix))]
            fds: PhantomData,
            pos: 0,
            container_depths: Default::default(),
        }))
    }
}

macro_rules! deserialize_basic {
    ($method:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            let ctxt = Context::new_dbus(self.0.ctxt.endian(), self.0.ctxt.position() + self.0.pos);

            let mut dbus_de = crate::dbus::Deserializer::<F>(DeserializerCommon::<F> {
                ctxt,
                signature: self.0.signature,
                bytes: subslice(self.0.bytes, self.0.pos..)?,
                fds: self.0.fds,
                pos: 0,
                container_depths: self.0.container_depths,
            });

            let v = dbus_de.$method(visitor)?;
            self.0.signature = dbus_de.0.signature;
            self.0.pos += dbus_de.0.pos;
            // Basic types don't have anything to do with container depths so not updating it here.

            Ok(v)
        }
    };
}

macro_rules! deserialize_as {
    ($method:ident => $as:ident) => {
        deserialize_as!($method() => $as());
    };
    ($method:ident($($in_arg:ident: $type:ty),*) => $as:ident($($as_arg:expr),*)) => {
        #[inline]
        fn $method<V>(self, $($in_arg: $type,)* visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            self.$as($($as_arg,)* visitor)
        }
    }
}

impl<'de, 'd, 'sig, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> de::Deserializer<'de>
    for &'d mut Deserializer<'de, 'sig, 'f, F>
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        crate::de::deserialize_any::<Self, V>(self, self.0.signature, visitor)
    }

    deserialize_basic!(deserialize_i8);
    deserialize_basic!(deserialize_i16);
    deserialize_basic!(deserialize_i32);
    deserialize_basic!(deserialize_i64);
    deserialize_basic!(deserialize_u8);
    deserialize_basic!(deserialize_u16);
    deserialize_basic!(deserialize_u32);
    deserialize_basic!(deserialize_u64);
    deserialize_basic!(deserialize_f32);
    deserialize_basic!(deserialize_f64);

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // bool is encoded as a single byte in GVariant
        let value = *subslice(self.0.bytes, self.0.pos)? != 0;
        self.0.pos += 1;
        visitor.visit_bool(value)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let bytes = deserialize_ay(self)?;
        visitor.visit_byte_buf(bytes.into())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let bytes = deserialize_ay(self)?;
        visitor.visit_borrowed_bytes(bytes)
    }

    deserialize_as!(deserialize_char => deserialize_str);
    deserialize_as!(deserialize_string => deserialize_str);
    deserialize_as!(deserialize_tuple(_l: usize) => deserialize_struct("", &[]));
    deserialize_as!(deserialize_tuple_struct(n: &'static str, _l: usize) => deserialize_struct(n, &[]));
    deserialize_as!(deserialize_struct(_n: &'static str, _f: &'static [&'static str]) => deserialize_seq());
    deserialize_as!(deserialize_map => deserialize_seq);
    deserialize_as!(deserialize_ignored_any => deserialize_any);

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let slice = subslice(self.0.bytes, self.0.pos..)?;

        let s = match self.0.signature {
            Signature::Str | Signature::Signature | Signature::ObjectPath => {
                self.0.pos += slice.len();
                // Get rid of the trailing nul byte (if any)
                let slice = if !slice.is_empty() && slice[slice.len() - 1] == 0 {
                    &slice[..slice.len() - 1]
                } else {
                    slice
                };
                if slice.contains(&0) {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Char('\0'),
                        &"GVariant string type must not contain interior null bytes",
                    ));
                }

                str::from_utf8(slice).map_err(Error::Utf8)?
            }
            _ => {
                let expected = format!(
                    "`{}`, `{}`, `{}` or `{}`",
                    <&str>::SIGNATURE_STR,
                    Signature::SIGNATURE_STR,
                    ObjectPath::SIGNATURE_STR,
                    VARIANT_SIGNATURE_CHAR,
                );
                return Err(Error::SignatureMismatch(self.0.signature.clone(), expected));
            }
        };
        visitor.visit_borrowed_str(s)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let alignment = self.0.signature.alignment(self.0.ctxt.format());
        self.0.parse_padding(alignment)?;

        let child_signature = match self.0.signature {
            Signature::Maybe(child) => child.signature(),
            _ => {
                return Err(Error::SignatureMismatch(
                    self.0.signature.clone(),
                    "a maybe".to_string(),
                ));
            }
        };
        let fixed_sized_child = child_signature.is_fixed_sized();

        if self.0.pos == self.0.bytes.len() {
            visitor.visit_none()
        } else {
            let ctxt = Context::new(
                self.0.ctxt.format(),
                self.0.ctxt.endian(),
                self.0.ctxt.position() + self.0.pos,
            );
            let end = if fixed_sized_child {
                self.0.bytes.len()
            } else {
                self.0.bytes.len() - 1
            };

            let mut de = Deserializer::<F>(DeserializerCommon {
                ctxt,
                signature: child_signature,
                bytes: subslice(self.0.bytes, self.0.pos..end)?,
                fds: self.0.fds,
                pos: 0,
                container_depths: self.0.container_depths.inc_maybe()?,
            });

            let v = visitor.visit_some(&mut de)?;
            self.0.pos += de.0.pos;
            // No need for retaking the container depths as the underlying type can't be incomplete.

            if !fixed_sized_child {
                let byte = *subslice(self.0.bytes, self.0.pos)?;
                if byte != 0 {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Bytes(&byte.to_le_bytes()),
                        &"0 byte expected at end of Maybe value",
                    ));
                }

                self.0.pos += 1;
            }

            Ok(v)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let byte = *subslice(self.0.bytes, self.0.pos)?;
        if byte != 0 {
            return Err(de::Error::invalid_value(
                de::Unexpected::Bytes(&byte.to_le_bytes()),
                &"0 byte expected for empty tuples (unit type)",
            ));
        }

        self.0.pos += 1;

        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let alignment = self.0.signature.alignment(Format::GVariant);
        self.0.parse_padding(alignment)?;

        match self.0.signature {
            Signature::Variant => {
                let value_de = ValueDeserializer::new(self)?;

                visitor.visit_seq(value_de)
            }
            Signature::Array(_) => {
                let array_de = ArrayDeserializer::new(self)?;
                visitor.visit_seq(array_de)
            }
            Signature::Dict { .. } => visitor.visit_map(ArrayDeserializer::new(self)?),
            Signature::Structure(_) => visitor.visit_seq(StructureDeserializer::new(self)?),
            Signature::U8 => {
                // Empty struct: encoded as a `0u8`.
                let _: u8 = serde::Deserialize::deserialize(&mut *self)?;

                let start = self.0.pos;
                let end = self.0.bytes.len();
                visitor.visit_seq(StructureDeserializer {
                    de: self,
                    start,
                    end,
                    field_idx: 0,
                    num_fields: 0,
                    offsets_len: 0,
                    offset_size: FramingOffsetSize::U8,
                })
            }
            _ => Err(Error::SignatureMismatch(
                self.0.signature.clone(),
                "a variant, array, dict, structure or u8".to_string(),
            )),
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let alignment = self.0.signature.alignment(self.0.ctxt.format());
        self.0.parse_padding(alignment)?;

        let v = visitor.visit_enum(crate::de::Enum {
            de: &mut *self,
            name,
            _phantom: PhantomData,
        })?;

        Ok(v)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.0.signature {
            Signature::Str | Signature::ObjectPath | Signature::Signature => {
                self.deserialize_str(visitor)
            }
            Signature::U32 => self.deserialize_u32(visitor),
            Signature::Structure(fields) => {
                let mut fields = fields.iter();
                let index_signature = fields.next().ok_or_else(|| {
                    Error::SignatureMismatch(
                        self.0.signature.clone(),
                        "a structure with 2 fields and u32 as its first field".to_string(),
                    )
                })?;
                self.0.signature = index_signature;
                let v = self.deserialize_u32(visitor);

                self.0.signature = fields.next().ok_or_else(|| {
                    Error::SignatureMismatch(
                        self.0.signature.clone(),
                        "a structure with 2 fields and u32 as its first field".to_string(),
                    )
                })?;

                v
            }
            _ => Err(Error::SignatureMismatch(
                self.0.signature.clone(),
                "a string, object path or signature".to_string(),
            )),
        }
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

fn deserialize_ay<'de, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F>(
    de: &mut Deserializer<'de, '_, '_, F>,
) -> Result<&'de [u8]> {
    if !matches!(de.0.signature, Signature::Array(child) if child.signature() == &Signature::U8) {
        return Err(de::Error::invalid_type(de::Unexpected::Seq, &"ay"));
    }

    let ad = ArrayDeserializer::new(de)?;
    let len = ad.len;
    de.0.container_depths = de.0.container_depths.dec_array();

    de.0.next_slice(len)
}

struct ArrayDeserializer<'d, 'de, 'sig, 'f, F> {
    de: &'d mut Deserializer<'de, 'sig, 'f, F>,
    len: usize,
    start: usize,
    // alignment of element
    element_alignment: usize,
    // Element signature in case of normal array, key signature in case of dict.
    child_signature: &'sig Signature,
    value_signature: Option<&'sig Signature>,
    // All offsets (GVariant-specific)
    offsets: Option<FramingOffsets>,
    // Length of all the offsets after the array
    offsets_len: usize,
    // size of the framing offset of last dict-entry key read (GVariant-specific)
    key_offset_size: Option<FramingOffsetSize>,
}

impl<'d, 'de, 'sig, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F>
    ArrayDeserializer<'d, 'de, 'sig, 'f, F>
{
    fn new(de: &'d mut Deserializer<'de, 'sig, 'f, F>) -> Result<Self> {
        de.0.container_depths = de.0.container_depths.inc_array()?;
        let alignment = de.0.signature.alignment(Format::GVariant);
        de.0.parse_padding(alignment)?;
        let mut len = de.0.bytes.len() - de.0.pos;

        let (child_signature, value_signature, fixed_sized_key, fixed_sized_child) =
            match de.0.signature {
                Signature::Array(child) => (child.signature(), None, false, child.is_fixed_sized()),
                Signature::Dict { key, value } => (
                    key.signature(),
                    Some(value.signature()),
                    key.is_fixed_sized(),
                    key.is_fixed_sized() && value.is_fixed_sized(),
                ),
                _ => {
                    return Err(Error::SignatureMismatch(
                        de.0.signature.clone(),
                        "an array or dict".to_string(),
                    ));
                }
            };

        let (offsets, offsets_len, key_offset_size) = if !fixed_sized_child {
            let (array_offsets, offsets_len) =
                FramingOffsets::from_encoded_array(subslice(de.0.bytes, de.0.pos..)?)?;
            len -= offsets_len;
            let key_offset_size = if !fixed_sized_key {
                // The actual offset for keys is calculated per key later, this is just to
                // put Some value to indicate at key is not fixed sized and thus uses
                // offsets.
                Some(FramingOffsetSize::U8)
            } else {
                None
            };

            (Some(array_offsets), offsets_len, key_offset_size)
        } else {
            (None, 0, None)
        };
        let start = de.0.pos;

        Ok(Self {
            de,
            len,
            start,
            element_alignment: alignment,
            child_signature,
            value_signature,
            offsets,
            offsets_len,
            key_offset_size,
        })
    }

    fn element_end(&mut self, pop: bool) -> Result<usize> {
        match self.offsets.as_mut() {
            Some(offsets) => {
                assert_eq!(self.de.0.ctxt.format(), Format::GVariant);

                let offset = if pop { offsets.pop() } else { offsets.peek() };
                match offset {
                    Some(offset) => Ok(self.start + offset),
                    None => Err(Error::MissingFramingOffset),
                }
            }
            None => Ok(self.start + self.len),
        }
    }

    fn done(&self) -> bool {
        match self.offsets.as_ref() {
            // If all offsets have been popped/used, we're already at the end
            Some(offsets) => offsets.is_empty(),
            None => self.de.0.pos == self.start + self.len,
        }
    }
}

impl<'d, 'de, 'sig, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> SeqAccess<'de>
    for ArrayDeserializer<'d, 'de, 'sig, 'f, F>
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.done() {
            self.de.0.pos += self.offsets_len;
            self.de.0.container_depths = self.de.0.container_depths.dec_array();

            return Ok(None);
        }

        let ctxt = Context::new(
            self.de.0.ctxt.format(),
            self.de.0.ctxt.endian(),
            self.de.0.ctxt.position() + self.de.0.pos,
        );
        let end = self.element_end(true)?;

        let mut de = Deserializer::<F>(DeserializerCommon {
            ctxt,
            signature: self.child_signature,
            bytes: subslice(self.de.0.bytes, self.de.0.pos..end)?,
            fds: self.de.0.fds,
            pos: 0,
            container_depths: self.de.0.container_depths,
        });

        let v = seed.deserialize(&mut de).map(Some);
        self.de.0.pos += de.0.pos;
        // No need for retaking the container depths as the child can't be incomplete.

        if self.de.0.pos > self.start + self.len {
            return Err(serde::de::Error::invalid_length(
                self.len,
                &format!(">= {}", self.de.0.pos - self.start).as_str(),
            ));
        }

        v
    }
}

impl<'d, 'de, 'sig, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> MapAccess<'de>
    for ArrayDeserializer<'d, 'de, 'sig, 'f, F>
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.done() {
            self.de.0.pos += self.offsets_len;
            self.de.0.container_depths = self.de.0.container_depths.dec_array();

            return Ok(None);
        }

        self.de.0.parse_padding(self.element_alignment)?;

        let ctxt = Context::new(
            self.de.0.ctxt.format(),
            self.de.0.ctxt.endian(),
            self.de.0.ctxt.position() + self.de.0.pos,
        );
        let element_end = self.element_end(false)?;

        let key_end = match self.key_offset_size {
            Some(_) => {
                if self.de.0.pos > element_end {
                    return Err(serde::de::Error::invalid_length(
                        self.de.0.pos,
                        &format!("< {}", element_end).as_str(),
                    ));
                }
                let offset_size =
                    FramingOffsetSize::for_encoded_container(element_end - self.de.0.pos);
                self.key_offset_size.replace(offset_size);

                self.de.0.pos
                    + offset_size
                        .read_last_offset_from_buffer(&self.de.0.bytes[self.de.0.pos..element_end])
            }
            None => element_end,
        };

        let mut de = Deserializer::<F>(DeserializerCommon {
            ctxt,
            signature: self.child_signature,
            bytes: subslice(self.de.0.bytes, self.de.0.pos..key_end)?,
            fds: self.de.0.fds,
            pos: 0,
            container_depths: self.de.0.container_depths,
        });
        let v = seed.deserialize(&mut de).map(Some);
        self.de.0.pos += de.0.pos;
        // No need for retaking the container depths as the key can't be incomplete.

        if self.de.0.pos > self.start + self.len {
            return Err(serde::de::Error::invalid_length(
                self.len,
                &format!(">= {}", self.de.0.pos - self.start).as_str(),
            ));
        }

        v
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let ctxt = Context::new(
            self.de.0.ctxt.format(),
            self.de.0.ctxt.endian(),
            self.de.0.ctxt.position() + self.de.0.pos,
        );
        let element_end = self.element_end(true)?;
        let value_end = match self.key_offset_size {
            Some(key_offset_size) => {
                if key_offset_size as usize > element_end {
                    return Err(serde::de::Error::invalid_length(
                        key_offset_size as usize,
                        &format!("< {}", element_end).as_str(),
                    ));
                }
                element_end - key_offset_size as usize
            }
            None => element_end,
        };

        let mut de = Deserializer::<F>(DeserializerCommon {
            ctxt,
            signature: self.value_signature.as_ref().unwrap(),
            bytes: subslice(self.de.0.bytes, self.de.0.pos..value_end)?,
            fds: self.de.0.fds,
            pos: 0,
            container_depths: self.de.0.container_depths,
        });
        let v = seed.deserialize(&mut de);
        self.de.0.pos += de.0.pos;
        // No need for retaking the container depths as the value can't be incomplete.

        if let Some(key_offset_size) = self.key_offset_size {
            self.de.0.pos += key_offset_size as usize;
        }

        if self.de.0.pos > self.start + self.len {
            return Err(serde::de::Error::invalid_length(
                self.len,
                &format!(">= {}", self.de.0.pos - self.start).as_str(),
            ));
        }

        v
    }
}

#[derive(Debug)]
struct StructureDeserializer<'d, 'de, 'sig, 'f, F> {
    de: &'d mut Deserializer<'de, 'sig, 'f, F>,
    start: usize,
    end: usize,
    /// Index of the next field to serialize.
    field_idx: usize,
    /// The number of fields in the structure.
    num_fields: usize,
    // Length of all the offsets after the array
    offsets_len: usize,
    // size of the framing offset
    offset_size: FramingOffsetSize,
}

impl<'d, 'de, 'sig, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F>
    StructureDeserializer<'d, 'de, 'sig, 'f, F>
{
    fn new(de: &'d mut Deserializer<'de, 'sig, 'f, F>) -> Result<Self> {
        let num_fields = match de.0.signature {
            Signature::Structure(fields) => fields.iter().count(),
            _ => unreachable!("Incorrect signature for struct"),
        };
        let alignment = de.0.signature.alignment(Format::GVariant);
        de.0.parse_padding(alignment)?;
        de.0.container_depths = de.0.container_depths.inc_structure()?;

        let start = de.0.pos;
        let end = de.0.bytes.len();
        let offset_size = FramingOffsetSize::for_encoded_container(end - start);

        Ok(Self {
            de,
            start,
            end,
            field_idx: 0,
            num_fields,
            offsets_len: 0,
            offset_size,
        })
    }
}

impl<'d, 'de, 'sig, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> SeqAccess<'de>
    for StructureDeserializer<'d, 'de, 'sig, 'f, F>
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.field_idx == self.num_fields {
            return Ok(None);
        }

        let ctxt = Context::new(
            self.de.0.ctxt.format(),
            self.de.0.ctxt.endian(),
            self.de.0.ctxt.position() + self.de.0.pos,
        );
        let signature = self.de.0.signature;
        let field_signature = match signature {
            Signature::Structure(fields) => {
                let signature = fields.iter().nth(self.field_idx).ok_or_else(|| {
                    Error::SignatureMismatch(signature.clone(), "a struct".to_string())
                })?;
                self.field_idx += 1;

                signature
            }
            _ => unreachable!("Incorrect signature for struct"),
        };
        let element_end = if !field_signature.is_fixed_sized() {
            if self.field_idx == self.num_fields {
                // This is the last item then and in GVariant format, we don't have offset for it
                // even if it's non-fixed-sized.
                self.end
            } else {
                let end = self
                    .offset_size
                    .read_last_offset_from_buffer(subslice(self.de.0.bytes, self.start..self.end)?)
                    + self.start;
                let offset_size = self.offset_size as usize;
                if offset_size > self.end {
                    return Err(serde::de::Error::invalid_length(
                        offset_size,
                        &format!("< {}", self.end).as_str(),
                    ));
                }

                self.end -= offset_size;
                self.offsets_len += offset_size;

                end
            }
        } else {
            self.end
        };

        let mut de = Deserializer::<F>(DeserializerCommon {
            ctxt,
            signature: field_signature,
            bytes: subslice(self.de.0.bytes, self.de.0.pos..element_end)?,
            fds: self.de.0.fds,
            pos: 0,
            container_depths: self.de.0.container_depths,
        });
        let v = seed.deserialize(&mut de).map(Some);
        self.de.0.pos += de.0.pos;
        // No need for retaking the container depths as the field can't be incomplete.

        if self.field_idx == self.num_fields {
            // All fields have been deserialized.
            self.de.0.container_depths = self.de.0.container_depths.dec_structure();

            // Skip over the framing offsets (if any)
            self.de.0.pos += self.offsets_len;
        }

        v
    }
}

#[derive(Debug)]
struct ValueDeserializer<'d, 'de, 'sig, 'f, F> {
    de: &'d mut Deserializer<'de, 'sig, 'f, F>,
    stage: ValueParseStage,
    sig_start: usize,
    sig_end: usize,
    value_start: usize,
    value_end: usize,
}

impl<'d, 'de, 'sig, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F>
    ValueDeserializer<'d, 'de, 'sig, 'f, F>
{
    fn new(de: &'d mut Deserializer<'de, 'sig, 'f, F>) -> Result<Self> {
        de.0.parse_padding(VARIANT_ALIGNMENT_GVARIANT)?;

        // GVariant format has signature at the end
        let mut separator_pos = None;

        if de.0.bytes.is_empty() {
            return Err(de::Error::invalid_value(
                de::Unexpected::Other("end of byte stream"),
                &"nul byte separator between Variant's value & signature",
            ));
        }

        // Search for the nul byte separator
        for i in (de.0.pos..de.0.bytes.len() - 1).rev() {
            if de.0.bytes[i] == b'\0' {
                separator_pos = Some(i);

                break;
            }
        }

        let (sig_start, sig_end, value_start, value_end) = match separator_pos {
            None => {
                return Err(de::Error::invalid_value(
                    de::Unexpected::Bytes(&de.0.bytes[de.0.pos..]),
                    &"nul byte separator between Variant's value & signature",
                ));
            }
            Some(separator_pos) => (separator_pos + 1, de.0.bytes.len(), de.0.pos, separator_pos),
        };

        Ok(ValueDeserializer::<F> {
            de,
            stage: ValueParseStage::Signature,
            sig_start,
            sig_end,
            value_start,
            value_end,
        })
    }
}

impl<'d, 'de, 'sig, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> SeqAccess<'de>
    for ValueDeserializer<'d, 'de, 'sig, 'f, F>
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.stage {
            ValueParseStage::Signature => {
                self.stage = ValueParseStage::Value;

                let mut de = Deserializer::<F>(DeserializerCommon {
                    // No padding in signatures so just pass the same context
                    ctxt: self.de.0.ctxt,
                    signature: &Signature::Signature,
                    bytes: subslice(self.de.0.bytes, self.sig_start..self.sig_end)?,
                    fds: self.de.0.fds,
                    pos: 0,
                    container_depths: self.de.0.container_depths,
                });

                seed.deserialize(&mut de).map(Some)
            }
            ValueParseStage::Value => {
                self.stage = ValueParseStage::Done;

                let slice = subslice(self.de.0.bytes, self.sig_start..self.sig_end)?;
                let signature = Signature::from_bytes(slice)?;

                let ctxt = Context::new(
                    self.de.0.ctxt.format(),
                    self.de.0.ctxt.endian(),
                    self.de.0.ctxt.position() + self.value_start,
                );
                let mut de = Deserializer::<F>(DeserializerCommon {
                    ctxt,
                    signature: &signature,
                    bytes: subslice(self.de.0.bytes, self.value_start..self.value_end)?,
                    fds: self.de.0.fds,
                    pos: 0,
                    container_depths: self.de.0.container_depths.inc_variant()?,
                });

                let v = seed.deserialize(&mut de).map(Some);

                self.de.0.pos = self.sig_end;

                v
            }
            ValueParseStage::Done => Ok(None),
        }
    }
}

impl<'de, 'd, 'sig, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> EnumAccess<'de>
    for crate::de::Enum<&'d mut Deserializer<'de, 'sig, 'f, F>, F>
{
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de).map(|v| (v, self))
    }
}
