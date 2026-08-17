use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, Visitor};

use std::{marker::PhantomData, str};

#[cfg(unix)]
use std::os::fd::AsFd;

use crate::{
    Basic, Error, ObjectPath, Result, Signature,
    de::{DeserializerCommon, ValueParseStage},
    serialized::{Context, Format},
    utils::*,
};

/// Our D-Bus deserialization implementation.
#[derive(Debug)]
pub(crate) struct Deserializer<'de, 'sig, 'f, F>(pub(crate) DeserializerCommon<'de, 'sig, 'f, F>);

#[allow(clippy::needless_lifetimes)]
impl<'de, 'sig, 'f, F> Deserializer<'de, 'sig, 'f, F> {
    /// Create a Deserializer struct instance.
    ///
    /// On Windows, there is no `fds` argument.
    pub fn new<'r: 'de>(
        bytes: &'r [u8],
        #[cfg(unix)] fds: Option<&'f [F]>,
        signature: &'sig Signature,
        ctxt: Context,
    ) -> Result<Self> {
        assert_eq!(ctxt.format(), Format::DBus);

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
    ($method:ident $read_method:ident $visitor_method:ident($type:ty)) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            let v = self
                .0
                .ctxt
                .endian()
                .$read_method(self.0.next_const_size_slice::<$type>()?);

            visitor.$visitor_method(v)
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

impl<'de, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> de::Deserializer<'de>
    for &mut Deserializer<'de, '_, '_, F>
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        crate::de::deserialize_any::<Self, V>(self, self.0.signature, visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self
            .0
            .ctxt
            .endian()
            .read_u32(self.0.next_const_size_slice::<bool>()?);
        let b = match v {
            1 => true,
            0 => false,
            // As per D-Bus spec, only 0 and 1 values are allowed
            _ => {
                return Err(de::Error::invalid_value(
                    de::Unexpected::Unsigned(v as u64),
                    &"0 or 1",
                ));
            }
        };

        visitor.visit_bool(b)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i16(visitor)
    }

    deserialize_basic!(deserialize_i16 read_i16 visit_i16(i16));
    deserialize_basic!(deserialize_i64 read_i64 visit_i64(i64));
    deserialize_basic!(deserialize_u16 read_u16 visit_u16(u16));
    deserialize_basic!(deserialize_u32 read_u32 visit_u32(u32));
    deserialize_basic!(deserialize_u64 read_u64 visit_u64(u64));
    deserialize_basic!(deserialize_f64 read_f64 visit_f64(f64));

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

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = match &self.0.signature {
            #[cfg(unix)]
            Signature::Fd => {
                let alignment = u32::alignment(Format::DBus);
                self.0.parse_padding(alignment)?;
                let idx = self.0.ctxt.endian().read_u32(self.0.next_slice(alignment)?);
                self.0.get_fd(idx)?
            }
            _ => self
                .0
                .ctxt
                .endian()
                .read_i32(self.0.next_const_size_slice::<i32>()?),
        };

        visitor.visit_i32(v)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Endianness is irrelevant for single bytes.
        visitor.visit_u8(self.0.next_const_size_slice::<u8>().map(|bytes| bytes[0])?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self
            .0
            .ctxt
            .endian()
            .read_f64(self.0.next_const_size_slice::<f64>()?);

        if v.is_finite() && v > (f32::MAX as f64) {
            return Err(de::Error::invalid_value(
                de::Unexpected::Float(v),
                &"Too large for f32",
            ));
        }
        visitor.visit_f32(v as f32)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = match self.0.signature {
            Signature::Signature | Signature::Variant => {
                let len_slice = self.0.next_slice(1)?;

                len_slice[0] as usize
            }
            Signature::Str | Signature::ObjectPath => {
                let alignment = u32::alignment(Format::DBus);
                self.0.parse_padding(alignment)?;
                let len_slice = self.0.next_slice(alignment)?;

                self.0.ctxt.endian().read_u32(len_slice) as usize
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
        let slice = self.0.next_slice(len)?;
        if slice.contains(&0) {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Char('\0'),
                &"D-Bus string type must not contain interior null bytes",
            ));
        }
        self.0.pos += 1; // skip trailing null byte
        let s = str::from_utf8(slice).map_err(Error::Utf8)?;

        visitor.visit_borrowed_str(s)
    }

    fn deserialize_option<V>(self, #[allow(unused)] visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "option-as-array")]
        {
            // This takes care of parsing all the padding and getting the byte length.
            let ad = ArrayDeserializer::new(self)?;
            let len = ad.len;
            let array_signature = ad.array_signature;

            let v = if len == 0 {
                visitor.visit_none()
            } else {
                visitor.visit_some(&mut *self)
            };
            self.0.container_depths = self.0.container_depths.dec_array();
            self.0.signature = array_signature;

            v
        }

        #[cfg(not(feature = "option-as-array"))]
        Err(de::Error::custom(
            "Can only decode Option<T> from D-Bus format if `option-as-array` feature is enabled",
        ))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
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
        let alignment = self.0.signature.alignment(Format::DBus);
        self.0.parse_padding(alignment)?;

        match self.0.signature {
            Signature::Variant => {
                let value_de = ValueDeserializer::new(self);

                visitor.visit_seq(value_de)
            }
            Signature::Array(_) => {
                let array_de = ArrayDeserializer::new(self)?;
                visitor.visit_seq(ArraySeqDeserializer(array_de))
            }
            Signature::Dict { .. } => visitor.visit_map(ArrayMapDeserializer::new(self)?),
            Signature::Structure(_) => visitor.visit_seq(StructureDeserializer::new(self)?),
            Signature::U8 => {
                // Empty struct: encoded as a `0u8`.
                let _: u8 = serde::Deserialize::deserialize(&mut *self)?;

                visitor.visit_seq(StructureDeserializer {
                    de: self,
                    field_idx: 0,
                    num_fields: 0,
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

        visitor.visit_enum(crate::de::Enum {
            de: self,
            name,
            _phantom: PhantomData,
        })
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

struct ArrayDeserializer<'d, 'de, 'sig, 'f, F> {
    de: &'d mut Deserializer<'de, 'sig, 'f, F>,
    len: usize,
    start: usize,
    // alignment of element
    element_alignment: usize,
    array_signature: &'sig Signature,
}

impl<'d, 'de, 'sig, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F>
    ArrayDeserializer<'d, 'de, 'sig, 'f, F>
{
    fn new(de: &'d mut Deserializer<'de, 'sig, 'f, F>) -> Result<Self> {
        de.0.parse_padding(ARRAY_ALIGNMENT_DBUS)?;
        de.0.container_depths = de.0.container_depths.inc_array()?;

        let len = de.0.ctxt.endian().read_u32(de.0.next_slice(4)?) as usize;

        // D-Bus expects us to add padding for the first element even when there is no first
        // element (i-e empty array) so we parse padding already.
        let (element_alignment, child_signature) = match de.0.signature {
            Signature::Array(child) => (child.alignment(de.0.ctxt.format()), child.signature()),
            Signature::Dict { key, .. } => (DICT_ENTRY_ALIGNMENT_DBUS, key.signature()),
            _ => {
                return Err(Error::SignatureMismatch(
                    de.0.signature.clone(),
                    "an array or dict".to_string(),
                ));
            }
        };
        de.0.parse_padding(element_alignment)?;

        // In case of an array, we'll only be serializing the array's child elements from now on and
        // in case of a dict, we'll swap key and value signatures during serlization of each entry,
        // so let's assume the element signature for array and key signature for dict, from now on.
        // We restore the original signature at the end of deserialization.
        let array_signature = de.0.signature;
        de.0.signature = child_signature;
        let start = de.0.pos;

        Ok(Self {
            de,
            len,
            start,
            element_alignment,
            array_signature,
        })
    }

    fn next<T>(&mut self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        let v = seed.deserialize(&mut *self.de);

        if self.de.0.pos > self.start + self.len {
            return Err(serde::de::Error::invalid_length(
                self.len,
                &format!(">= {}", self.de.0.pos - self.start).as_str(),
            ));
        }

        v
    }

    fn next_element<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.done() {
            self.end();

            return Ok(None);
        }
        // Redundant for normal arrays but dict requires each entry to be padded by 8 bytes.
        self.de.0.parse_padding(self.element_alignment)?;

        self.next(seed).map(Some)
    }

    fn done(&self) -> bool {
        self.de.0.pos == self.start + self.len
    }

    fn end(&mut self) {
        self.de.0.container_depths = self.de.0.container_depths.dec_array();
        self.de.0.signature = self.array_signature;
    }
}

fn deserialize_ay<'de, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F>(
    de: &mut Deserializer<'de, '_, '_, F>,
) -> Result<&'de [u8]> {
    if !matches!(de.0.signature, Signature::Array(child) if child.signature() == &Signature::U8) {
        return Err(de::Error::invalid_type(de::Unexpected::Seq, &"ay"));
    }

    let mut ad = ArrayDeserializer::new(de)?;
    let len = ad.len;
    ad.end();

    de.0.next_slice(len)
}

struct ArraySeqDeserializer<'d, 'de, 'sig, 'f, F>(ArrayDeserializer<'d, 'de, 'sig, 'f, F>);

impl<'de, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> SeqAccess<'de>
    for ArraySeqDeserializer<'_, 'de, '_, '_, F>
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        self.0.next_element(seed)
    }
}

struct ArrayMapDeserializer<'d, 'de, 'sig, 'f, F> {
    ad: ArrayDeserializer<'d, 'de, 'sig, 'f, F>,
    key_signature: &'sig Signature,
    value_signature: &'sig Signature,
}
impl<'d, 'de, 'sig, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F>
    ArrayMapDeserializer<'d, 'de, 'sig, 'f, F>
{
    fn new(de: &'d mut Deserializer<'de, 'sig, 'f, F>) -> Result<Self> {
        let (key_signature, value_signature) = match de.0.signature {
            Signature::Dict { key, value } => (key.signature(), value.signature()),
            _ => {
                return Err(Error::SignatureMismatch(
                    de.0.signature.clone(),
                    "a dict".to_string(),
                ));
            }
        };
        let ad = ArrayDeserializer::new(de)?;

        Ok(Self {
            ad,
            key_signature,
            value_signature,
        })
    }
}

impl<'de, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> MapAccess<'de>
    for ArrayMapDeserializer<'_, 'de, '_, '_, F>
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        self.ad.next_element(seed)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        self.ad.de.0.signature = self.value_signature;
        let v = self.ad.next(seed);
        self.ad.de.0.signature = self.key_signature;

        v
    }
}

#[derive(Debug)]
struct StructureDeserializer<'d, 'de, 'sig, 'f, F> {
    de: &'d mut Deserializer<'de, 'sig, 'f, F>,
    /// Index of the next field to serialize.
    field_idx: usize,
    /// The number of fields in the structure.
    num_fields: usize,
}

impl<'d, 'de, 'sig, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F>
    StructureDeserializer<'d, 'de, 'sig, 'f, F>
{
    fn new(de: &'d mut Deserializer<'de, 'sig, 'f, F>) -> Result<Self> {
        let num_fields = match de.0.signature {
            Signature::Structure(fields) => fields.iter().count(),
            _ => unreachable!("Incorrect signature for struct"),
        };
        de.0.parse_padding(STRUCT_ALIGNMENT_DBUS)?;
        de.0.container_depths = de.0.container_depths.inc_structure()?;

        Ok(Self {
            de,
            field_idx: 0,
            num_fields,
        })
    }
}

impl<'de, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> SeqAccess<'de>
    for StructureDeserializer<'_, 'de, '_, '_, F>
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.field_idx == self.num_fields {
            return Ok(None);
        }

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

        let mut de = Deserializer::<F>(DeserializerCommon {
            ctxt: self.de.0.ctxt,
            signature: field_signature,
            fds: self.de.0.fds,
            bytes: self.de.0.bytes,
            pos: self.de.0.pos,
            container_depths: self.de.0.container_depths,
        });
        let v = seed.deserialize(&mut de)?;
        self.de.0.pos = de.0.pos;

        if self.field_idx == self.num_fields {
            // All fields have been deserialized.
            self.de.0.container_depths = self.de.0.container_depths.dec_structure();
        }

        Ok(Some(v))
    }
}

#[derive(Debug)]
struct ValueDeserializer<'d, 'de, 'sig, 'f, F> {
    de: &'d mut Deserializer<'de, 'sig, 'f, F>,
    stage: ValueParseStage,
    sig_start: usize,
}

impl<'d, 'de, 'sig, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F>
    ValueDeserializer<'d, 'de, 'sig, 'f, F>
{
    fn new(de: &'d mut Deserializer<'de, 'sig, 'f, F>) -> Self {
        let sig_start = de.0.pos;
        ValueDeserializer::<F> {
            de,
            stage: ValueParseStage::Signature,
            sig_start,
        }
    }
}

impl<'de, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> SeqAccess<'de>
    for ValueDeserializer<'_, 'de, '_, '_, F>
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.stage {
            ValueParseStage::Signature => {
                self.stage = ValueParseStage::Value;

                let signature = self.de.0.signature;
                self.de.0.signature = &Signature::Signature;
                let ret = seed.deserialize(&mut *self.de).map(Some);
                self.de.0.signature = signature;

                ret
            }
            ValueParseStage::Value => {
                self.stage = ValueParseStage::Done;

                let sig_len = self.de.0.bytes[self.sig_start] as usize;
                // skip length byte
                let sig_start = self.sig_start + 1;
                let sig_end = sig_start + sig_len;
                // Skip trailing nul byte
                let value_start = sig_end + 1;

                let slice = subslice(self.de.0.bytes, sig_start..sig_end)?;
                let signature = Signature::from_bytes(slice)?;

                let ctxt = Context::new(
                    Format::DBus,
                    self.de.0.ctxt.endian(),
                    self.de.0.ctxt.position() + value_start,
                );
                let mut de = Deserializer::<F>(DeserializerCommon {
                    ctxt,
                    signature: &signature,
                    bytes: subslice(self.de.0.bytes, value_start..)?,
                    fds: self.de.0.fds,
                    pos: 0,
                    container_depths: self.de.0.container_depths.inc_variant()?,
                });

                let v = seed.deserialize(&mut de).map(Some);
                self.de.0.pos += de.0.pos;

                v
            }
            ValueParseStage::Done => Ok(None),
        }
    }
}

impl<'de, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> EnumAccess<'de>
    for crate::de::Enum<&mut Deserializer<'de, '_, '_, F>, F>
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
