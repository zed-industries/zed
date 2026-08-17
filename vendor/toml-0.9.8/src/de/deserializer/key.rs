use serde_core::de::IntoDeserializer;

use crate::de::DeString;
use crate::de::Error;

pub(crate) struct KeyDeserializer<'i> {
    span: Option<core::ops::Range<usize>>,
    key: DeString<'i>,
}

impl<'i> KeyDeserializer<'i> {
    pub(crate) fn new(key: DeString<'i>, span: Option<core::ops::Range<usize>>) -> Self {
        KeyDeserializer { span, key }
    }
}

impl<'de> IntoDeserializer<'de, Error> for KeyDeserializer<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> serde_core::de::Deserializer<'de> for KeyDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        self.key.into_deserializer().deserialize_any(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let key: bool = self.key.parse().map_err(serde_core::de::Error::custom)?;
        key.into_deserializer().deserialize_bool(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let key: i8 = self.key.parse().map_err(serde_core::de::Error::custom)?;
        key.into_deserializer().deserialize_i8(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let key: i16 = self.key.parse().map_err(serde_core::de::Error::custom)?;
        key.into_deserializer().deserialize_i16(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let key: i32 = self.key.parse().map_err(serde_core::de::Error::custom)?;
        key.into_deserializer().deserialize_i32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let key: i64 = self.key.parse().map_err(serde_core::de::Error::custom)?;
        key.into_deserializer().deserialize_i64(visitor)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let key: i128 = self.key.parse().map_err(serde_core::de::Error::custom)?;
        key.into_deserializer().deserialize_i128(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let key: u8 = self.key.parse().map_err(serde_core::de::Error::custom)?;
        key.into_deserializer().deserialize_u8(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let key: u16 = self.key.parse().map_err(serde_core::de::Error::custom)?;
        key.into_deserializer().deserialize_u16(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let key: u32 = self.key.parse().map_err(serde_core::de::Error::custom)?;
        key.into_deserializer().deserialize_u32(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let key: u64 = self.key.parse().map_err(serde_core::de::Error::custom)?;
        key.into_deserializer().deserialize_u64(visitor)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let key: u128 = self.key.parse().map_err(serde_core::de::Error::custom)?;
        key.into_deserializer().deserialize_u128(visitor)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let key: char = self.key.parse().map_err(serde_core::de::Error::custom)?;
        key.into_deserializer().deserialize_char(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        let _ = name;
        let _ = variants;
        visitor.visit_enum(self)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        if serde_spanned::de::is_spanned(name) {
            if let Some(span) = self.span.clone() {
                return visitor.visit_map(super::SpannedDeserializer::new(self.key, span));
            } else {
                return Err(Error::custom("value is missing a span", None));
            }
        }
        self.deserialize_any(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    serde_core::forward_to_deserialize_any! {
        f32 f64 str string seq
        bytes byte_buf map option unit
        ignored_any unit_struct tuple_struct tuple identifier
    }
}

impl<'de> serde_core::de::EnumAccess<'de> for KeyDeserializer<'de> {
    type Error = Error;
    type Variant = UnitOnly<Self::Error>;

    fn variant_seed<T>(self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error>
    where
        T: serde_core::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self).map(unit_only)
    }
}

pub(crate) struct UnitOnly<E> {
    marker: core::marker::PhantomData<E>,
}

fn unit_only<T, E>(t: T) -> (T, UnitOnly<E>) {
    (
        t,
        UnitOnly {
            marker: core::marker::PhantomData,
        },
    )
}

impl<'de, E> serde_core::de::VariantAccess<'de> for UnitOnly<E>
where
    E: serde_core::de::Error,
{
    type Error = E;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde_core::de::DeserializeSeed<'de>,
    {
        Err(serde_core::de::Error::invalid_type(
            serde_core::de::Unexpected::UnitVariant,
            &"newtype variant",
        ))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        Err(serde_core::de::Error::invalid_type(
            serde_core::de::Unexpected::UnitVariant,
            &"tuple variant",
        ))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        Err(serde_core::de::Error::invalid_type(
            serde_core::de::Unexpected::UnitVariant,
            &"struct variant",
        ))
    }
}
