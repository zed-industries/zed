use serde::de::{self, DeserializeSeed, IntoDeserializer, Visitor};

use crate::de::flavors::{Flavor, Slice};
use crate::error::{Error, Result};
use crate::varint::{max_of_last_byte, varint_max};
use core::marker::PhantomData;

/// A `serde` compatible deserializer, generic over “Flavors” of deserializing plugins.
///
/// Please note that postcard messages are not self-describing and therefore incompatible with
/// [internally tagged enums](https://serde.rs/enum-representations.html#internally-tagged).
pub struct Deserializer<'de, F: Flavor<'de>> {
    flavor: F,
    _plt: PhantomData<&'de ()>,
}

impl<'de, F> Deserializer<'de, F>
where
    F: Flavor<'de> + 'de,
{
    /// Obtain a Deserializer from a slice of bytes
    pub fn from_flavor(flavor: F) -> Self {
        Deserializer {
            flavor,
            _plt: PhantomData,
        }
    }

    /// Return the remaining (unused) bytes in the Deserializer along with any
    /// additional data provided by the [`Flavor`]
    pub fn finalize(self) -> Result<F::Remainder> {
        self.flavor.finalize()
    }
}

impl<'de> Deserializer<'de, Slice<'de>> {
    /// Obtain a Deserializer from a slice of bytes
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer {
            flavor: Slice::new(input),
            _plt: PhantomData,
        }
    }
}

impl<'de, F: Flavor<'de>> Deserializer<'de, F> {
    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    fn try_take_varint_usize(&mut self) -> Result<usize> {
        self.try_take_varint_u16().map(|u| u as usize)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn try_take_varint_usize(&mut self) -> Result<usize> {
        self.try_take_varint_u32().map(|u| u as usize)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn try_take_varint_usize(&mut self) -> Result<usize> {
        self.try_take_varint_u64().map(|u| u as usize)
    }

    #[inline]
    fn try_take_varint_u16(&mut self) -> Result<u16> {
        let mut out = 0;
        for i in 0..varint_max::<u16>() {
            let val = self.flavor.pop()?;
            let carry = (val & 0x7F) as u16;
            out |= carry << (7 * i);

            if (val & 0x80) == 0 {
                if i == varint_max::<u16>() - 1 && val > max_of_last_byte::<u16>() {
                    return Err(Error::DeserializeBadVarint);
                } else {
                    return Ok(out);
                }
            }
        }
        Err(Error::DeserializeBadVarint)
    }

    #[inline]
    fn try_take_varint_u32(&mut self) -> Result<u32> {
        let mut out = 0;
        for i in 0..varint_max::<u32>() {
            let val = self.flavor.pop()?;
            let carry = (val & 0x7F) as u32;
            out |= carry << (7 * i);

            if (val & 0x80) == 0 {
                if i == varint_max::<u32>() - 1 && val > max_of_last_byte::<u32>() {
                    return Err(Error::DeserializeBadVarint);
                } else {
                    return Ok(out);
                }
            }
        }
        Err(Error::DeserializeBadVarint)
    }

    #[inline]
    fn try_take_varint_u64(&mut self) -> Result<u64> {
        let mut out = 0;
        for i in 0..varint_max::<u64>() {
            let val = self.flavor.pop()?;
            let carry = (val & 0x7F) as u64;
            out |= carry << (7 * i);

            if (val & 0x80) == 0 {
                if i == varint_max::<u64>() - 1 && val > max_of_last_byte::<u64>() {
                    return Err(Error::DeserializeBadVarint);
                } else {
                    return Ok(out);
                }
            }
        }
        Err(Error::DeserializeBadVarint)
    }

    #[inline]
    fn try_take_varint_u128(&mut self) -> Result<u128> {
        let mut out = 0;
        for i in 0..varint_max::<u128>() {
            let val = self.flavor.pop()?;
            let carry = (val & 0x7F) as u128;
            out |= carry << (7 * i);

            if (val & 0x80) == 0 {
                if i == varint_max::<u128>() - 1 && val > max_of_last_byte::<u128>() {
                    return Err(Error::DeserializeBadVarint);
                } else {
                    return Ok(out);
                }
            }
        }
        Err(Error::DeserializeBadVarint)
    }
}

struct SeqAccess<'a, 'b, F: Flavor<'b>> {
    deserializer: &'a mut Deserializer<'b, F>,
    len: usize,
}

impl<'a, 'b: 'a, F: Flavor<'b>> serde::de::SeqAccess<'b> for SeqAccess<'a, 'b, F> {
    type Error = Error;

    #[inline]
    fn next_element_seed<V: DeserializeSeed<'b>>(&mut self, seed: V) -> Result<Option<V::Value>> {
        if self.len > 0 {
            self.len -= 1;
            Ok(Some(DeserializeSeed::deserialize(
                seed,
                &mut *self.deserializer,
            )?))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        match self.deserializer.flavor.size_hint() {
            Some(size) if size < self.len => None,
            _ => Some(self.len),
        }
    }
}

struct MapAccess<'a, 'b, F: Flavor<'b>> {
    deserializer: &'a mut Deserializer<'b, F>,
    len: usize,
}

impl<'a, 'b: 'a, F: Flavor<'b>> serde::de::MapAccess<'b> for MapAccess<'a, 'b, F> {
    type Error = Error;

    #[inline]
    fn next_key_seed<K: DeserializeSeed<'b>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        if self.len > 0 {
            self.len -= 1;
            Ok(Some(DeserializeSeed::deserialize(
                seed,
                &mut *self.deserializer,
            )?))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn next_value_seed<V: DeserializeSeed<'b>>(&mut self, seed: V) -> Result<V::Value> {
        DeserializeSeed::deserialize(seed, &mut *self.deserializer)
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'de, F: Flavor<'de>> de::Deserializer<'de> for &mut Deserializer<'de, F> {
    type Error = Error;

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }

    // Postcard does not support structures not known at compile time
    #[inline]
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // We wont ever support this.
        Err(Error::WontImplement)
    }

    // Take a boolean encoded as a u8
    #[inline]
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let val = match self.flavor.pop()? {
            0 => false,
            1 => true,
            _ => return Err(Error::DeserializeBadBool),
        };
        visitor.visit_bool(val)
    }

    #[inline]
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.flavor.pop()? as i8)
    }

    #[inline]
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.try_take_varint_u16()?;
        visitor.visit_i16(de_zig_zag_i16(v))
    }

    #[inline]
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.try_take_varint_u32()?;
        visitor.visit_i32(de_zig_zag_i32(v))
    }

    #[inline]
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.try_take_varint_u64()?;
        visitor.visit_i64(de_zig_zag_i64(v))
    }

    #[inline]
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.try_take_varint_u128()?;
        visitor.visit_i128(de_zig_zag_i128(v))
    }

    #[inline]
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.flavor.pop()?)
    }

    #[inline]
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.try_take_varint_u16()?;
        visitor.visit_u16(v)
    }

    #[inline]
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.try_take_varint_u32()?;
        visitor.visit_u32(v)
    }

    #[inline]
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.try_take_varint_u64()?;
        visitor.visit_u64(v)
    }

    #[inline]
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.try_take_varint_u128()?;
        visitor.visit_u128(v)
    }

    #[inline]
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let bytes = self.flavor.try_take_n_temp(4)?;
        let mut buf = [0u8; 4];
        buf.copy_from_slice(bytes);
        visitor.visit_f32(f32::from_bits(u32::from_le_bytes(buf)))
    }

    #[inline]
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let bytes = self.flavor.try_take_n_temp(8)?;
        let mut buf = [0u8; 8];
        buf.copy_from_slice(bytes);
        visitor.visit_f64(f64::from_bits(u64::from_le_bytes(buf)))
    }

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let sz = self.try_take_varint_usize()?;
        if sz > 4 {
            return Err(Error::DeserializeBadChar);
        }
        let bytes: &[u8] = self.flavor.try_take_n_temp(sz)?;
        // we pass the character through string conversion because
        // this handles transforming the array of code units to a
        // codepoint. we can't use char::from_u32() because it expects
        // an already-processed codepoint.
        let character = core::str::from_utf8(bytes)
            .map_err(|_| Error::DeserializeBadChar)?
            .chars()
            .next()
            .ok_or(Error::DeserializeBadChar)?;
        visitor.visit_char(character)
    }

    #[inline]
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let sz = self.try_take_varint_usize()?;
        let bytes: &'de [u8] = self.flavor.try_take_n(sz)?;
        let str_sl = core::str::from_utf8(bytes).map_err(|_| Error::DeserializeBadUtf8)?;

        visitor.visit_borrowed_str(str_sl)
    }

    #[inline]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let sz = self.try_take_varint_usize()?;
        let bytes: &[u8] = self.flavor.try_take_n_temp(sz)?;
        let str_sl = core::str::from_utf8(bytes).map_err(|_| Error::DeserializeBadUtf8)?;

        visitor.visit_str(str_sl)
    }

    #[inline]
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let sz = self.try_take_varint_usize()?;
        let bytes: &'de [u8] = self.flavor.try_take_n(sz)?;
        visitor.visit_borrowed_bytes(bytes)
    }

    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let sz = self.try_take_varint_usize()?;
        let bytes: &[u8] = self.flavor.try_take_n_temp(sz)?;
        visitor.visit_bytes(bytes)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.flavor.pop()? {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => Err(Error::DeserializeBadOption),
        }
    }

    // In Serde, unit means an anonymous value containing no data.
    // Unit is not actually encoded in Postcard.
    #[inline]
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    // Unit struct means a named value containing no data.
    // Unit structs are not actually encoded in Postcard.
    #[inline]
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.try_take_varint_usize()?;

        visitor.visit_seq(SeqAccess {
            deserializer: self,
            len,
        })
    }

    #[inline]
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess {
            deserializer: self,
            len,
        })
    }

    #[inline]
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    #[inline]
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.try_take_varint_usize()?;

        visitor.visit_map(MapAccess {
            deserializer: self,
            len,
        })
    }

    #[inline]
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    // As a binary format, Postcard does not encode identifiers
    #[inline]
    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Will not support
        Err(Error::WontImplement)
    }

    #[inline]
    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Will not support
        Err(Error::WontImplement)
    }
}

impl<'de, F: Flavor<'de>> serde::de::VariantAccess<'de> for &mut Deserializer<'de, F> {
    type Error = Error;

    #[inline]
    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn newtype_variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<V::Value> {
        DeserializeSeed::deserialize(seed, self)
    }

    #[inline]
    fn tuple_variant<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value> {
        serde::de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    #[inline]
    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}

impl<'de, F: Flavor<'de>> serde::de::EnumAccess<'de> for &mut Deserializer<'de, F> {
    type Error = Error;
    type Variant = Self;

    #[inline]
    fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self)> {
        let varint = self.try_take_varint_u32()?;
        let v = DeserializeSeed::deserialize(seed, varint.into_deserializer())?;
        Ok((v, self))
    }
}

fn de_zig_zag_i16(n: u16) -> i16 {
    ((n >> 1) as i16) ^ (-((n & 0b1) as i16))
}

fn de_zig_zag_i32(n: u32) -> i32 {
    ((n >> 1) as i32) ^ (-((n & 0b1) as i32))
}

fn de_zig_zag_i64(n: u64) -> i64 {
    ((n >> 1) as i64) ^ (-((n & 0b1) as i64))
}

fn de_zig_zag_i128(n: u128) -> i128 {
    ((n >> 1) as i128) ^ (-((n & 0b1) as i128))
}
