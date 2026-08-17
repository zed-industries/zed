use core::marker::PhantomData;

use serde::{
    de::{DeserializeSeed, MapAccess, Visitor},
    Deserialize, Deserializer,
};

/// Deserializes a color with an attached alpha value. The alpha value is
/// expected to be found alongside the other values in a flattened structure.
pub(crate) struct AlphaDeserializer<'a, D, A> {
    pub inner: D,
    pub alpha: &'a mut Option<A>,
}

impl<'de, 'a, D, A> Deserializer<'de> for AlphaDeserializer<'a, D, A>
where
    D: Deserializer<'de>,
    A: Deserialize<'de>,
{
    type Error = D::Error;

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_seq(AlphaSeqVisitor {
            inner: visitor,
            alpha: self.alpha,
        })
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_tuple(
            len + 1,
            AlphaMapVisitor {
                inner: visitor,
                alpha: self.alpha,
                field_count: Some(len),
            },
        )
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_tuple_struct(
            name,
            len + 1,
            AlphaMapVisitor {
                inner: visitor,
                alpha: self.alpha,
                field_count: Some(len),
            },
        )
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_map(AlphaMapVisitor {
            inner: visitor,
            alpha: self.alpha,
            field_count: None,
        })
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_struct(
            name,
            fields, // We can't add to the expected fields so we just hope it works anyway.
            AlphaMapVisitor {
                inner: visitor,
                alpha: self.alpha,
                field_count: Some(fields.len()),
            },
        )
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_ignored_any(AlphaSeqVisitor {
            inner: visitor,
            alpha: self.alpha,
        })
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_tuple(
            1,
            AlphaMapVisitor {
                inner: visitor,
                alpha: self.alpha,
                field_count: None,
            },
        )
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.inner.deserialize_newtype_struct(
            name,
            AlphaMapVisitor {
                inner: visitor,
                alpha: self.alpha,
                field_count: Some(0),
            },
        )
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple_struct(name, 1, visitor)
    }

    // Unsupported methods:

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        alpha_deserializer_error()
    }
}

fn alpha_deserializer_error() -> ! {
    unimplemented!("AlphaDeserializer can only deserialize structs, maps and sequences")
}

/// Deserializes a sequence with the alpha value last.
struct AlphaSeqVisitor<'a, D, A> {
    inner: D,
    alpha: &'a mut Option<A>,
}

impl<'de, 'a, D, A> Visitor<'de> for AlphaSeqVisitor<'a, D, A>
where
    D: Visitor<'de>,
    A: Deserialize<'de>,
{
    type Value = D::Value;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.inner.expecting(formatter)?;
        write!(formatter, " with an alpha value")
    }

    fn visit_seq<T>(self, mut seq: T) -> Result<Self::Value, T::Error>
    where
        T: serde::de::SeqAccess<'de>,
    {
        let color = self.inner.visit_seq(&mut seq)?;
        *self.alpha = seq.next_element()?;

        Ok(color)
    }
}

/// Deserializes a map or a struct with an "alpha" key, or a tuple with the
/// alpha value as the last value.
struct AlphaMapVisitor<'a, D, A> {
    inner: D,
    alpha: &'a mut Option<A>,
    field_count: Option<usize>,
}

impl<'de, 'a, D, A> Visitor<'de> for AlphaMapVisitor<'a, D, A>
where
    D: Visitor<'de>,
    A: Deserialize<'de>,
{
    type Value = D::Value;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.inner.expecting(formatter)?;
        write!(formatter, " with an alpha value")
    }

    fn visit_seq<T>(self, mut seq: T) -> Result<Self::Value, T::Error>
    where
        T: serde::de::SeqAccess<'de>,
    {
        let color = if self.field_count.is_none() {
            self.inner.visit_unit()?
        } else {
            self.inner.visit_seq(&mut seq)?
        };
        *self.alpha = seq.next_element()?;

        Ok(color)
    }

    fn visit_map<T>(self, map: T) -> Result<Self::Value, T::Error>
    where
        T: serde::de::MapAccess<'de>,
    {
        self.inner.visit_map(MapWrapper {
            inner: map,
            alpha: self.alpha,
            field_count: self.field_count,
        })
    }

    fn visit_newtype_struct<T>(self, deserializer: T) -> Result<Self::Value, T::Error>
    where
        T: Deserializer<'de>,
    {
        *self.alpha = Some(A::deserialize(deserializer)?);
        self.inner.visit_unit()
    }
}

/// Intercepts map deserializing to catch the alpha value while deserializing
/// the entries.
struct MapWrapper<'a, T, A> {
    inner: T,
    alpha: &'a mut Option<A>,
    field_count: Option<usize>,
}

impl<'a, 'de, T, A> MapAccess<'de> for MapWrapper<'a, T, A>
where
    T: MapAccess<'de>,
    A: Deserialize<'de>,
{
    type Error = T::Error;

    fn next_key_seed<K>(&mut self, mut seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        // Look for and extract the alpha value if its key is found, then return
        // the next key after that. The first key that isn't alpha is
        // immediately returned to the wrapped type's visitor.
        loop {
            seed = match self.inner.next_key_seed(AlphaFieldDeserializerSeed {
                inner: seed,
                field_count: self.field_count,
            }) {
                Ok(Some(AlphaField::Alpha(seed))) => {
                    // We found the alpha value, so deserialize it...
                    if self.alpha.is_some() {
                        return Err(serde::de::Error::duplicate_field("alpha"));
                    }
                    *self.alpha = Some(self.inner.next_value()?);

                    // ...then give the seed back for the next key
                    seed
                }
                Ok(Some(AlphaField::Other(other))) => return Ok(Some(other)),
                Ok(None) => return Ok(None),
                Err(error) => return Err(error),
            };
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        self.inner.next_value_seed(seed)
    }
}

struct AlphaFieldDeserializerSeed<T> {
    inner: T,
    field_count: Option<usize>,
}

impl<'de, T> DeserializeSeed<'de> for AlphaFieldDeserializerSeed<T>
where
    T: DeserializeSeed<'de>,
{
    type Value = AlphaField<T, T::Value>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(AlphaFieldVisitor {
            inner: self.inner,
            field_count: self.field_count,
        })
    }
}

/// An alpha struct field or another struct field.
enum AlphaField<A, O> {
    Alpha(A),
    Other(O),
}

/// A struct field name that hasn't been serialized yet.
enum StructField<'de> {
    Unsigned(u64),
    Str(&'de str),
    Bytes(&'de [u8]),
}

struct AlphaFieldVisitor<T> {
    inner: T,
    field_count: Option<usize>,
}

impl<'de, T> Visitor<'de> for AlphaFieldVisitor<T>
where
    T: DeserializeSeed<'de>,
{
    type Value = AlphaField<T, T::Value>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "alpha field")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // We need the field count here to get the last tuple field. No field
        // count implies that we definitely expected a struct or a map.
        let field_count = self.field_count.ok_or_else(|| {
            serde::de::Error::invalid_type(
                serde::de::Unexpected::Unsigned(v),
                &"map key or struct field",
            )
        })?;

        // Assume that it's the alpha value if it's after the expected number of
        // fields. Otherwise, pass on to the wrapped type's deserializer.
        if v == field_count as u64 {
            Ok(AlphaField::Alpha(self.inner))
        } else {
            Ok(AlphaField::Other(self.inner.deserialize(
                StructFieldDeserializer {
                    struct_field: StructField::Unsigned(v),
                    error: PhantomData,
                },
            )?))
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // Assume that it's the alpha value if it's named "alpha". Otherwise,
        // pass on to the wrapped type's deserializer.
        if v == "alpha" {
            Ok(AlphaField::Alpha(self.inner))
        } else {
            Ok(AlphaField::Other(self.inner.deserialize(
                StructFieldDeserializer {
                    struct_field: StructField::Str(v),
                    error: PhantomData,
                },
            )?))
        }
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // Assume that it's the alpha value if it's named "alpha". Otherwise,
        // pass on to the wrapped type's deserializer.
        if v == b"alpha" {
            Ok(AlphaField::Alpha(self.inner))
        } else {
            Ok(AlphaField::Other(self.inner.deserialize(
                StructFieldDeserializer {
                    struct_field: StructField::Bytes(v),
                    error: PhantomData,
                },
            )?))
        }
    }
}

/// Deserializes a non-alpha struct field name.
struct StructFieldDeserializer<'a, E> {
    struct_field: StructField<'a>,
    error: PhantomData<fn() -> E>,
}

impl<'a, 'de, E> Deserializer<'de> for StructFieldDeserializer<'a, E>
where
    E: serde::de::Error,
{
    type Error = E;

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.struct_field {
            StructField::Unsigned(v) => visitor.visit_u64(v),
            StructField::Str(v) => visitor.visit_str(v),
            StructField::Bytes(v) => visitor.visit_bytes(v),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_identifier(visitor)
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_identifier(visitor)
    }

    // Unsupported methods::

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct_field_deserializer_error()
    }
}

fn struct_field_deserializer_error() -> ! {
    unimplemented!("StructFieldDeserializer can only deserialize identifiers")
}
