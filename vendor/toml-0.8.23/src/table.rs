use serde::de;
use serde::ser;

use crate::map::Map;
use crate::Value;

/// Type representing a TOML table, payload of the `Value::Table` variant.
///
/// By default it entries are stored in
/// [lexicographic order](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord-for-str)
/// of the keys. Enable the `preserve_order` feature to store entries in the order they appear in
/// the source file.
pub type Table = Map<String, Value>;

impl Table {
    /// Convert a `T` into `toml::Table`.
    ///
    /// This conversion can fail if `T`'s implementation of `Serialize` decides to
    /// fail, or if `T` contains a map with non-string keys.
    pub fn try_from<T>(value: T) -> Result<Self, crate::ser::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(crate::value::TableSerializer)
    }

    /// Interpret a `toml::Table` as an instance of type `T`.
    ///
    /// This conversion can fail if the structure of the `Table` does not match the structure
    /// expected by `T`, for example if `T` is a bool which can't be mapped to a `Table`. It can
    /// also fail if the structure is correct but `T`'s implementation of `Deserialize` decides
    /// that something is wrong with the data, for example required struct fields are missing from
    /// the TOML map or some number is too big to fit in the expected primitive type.
    pub fn try_into<'de, T>(self) -> Result<T, crate::de::Error>
    where
        T: de::Deserialize<'de>,
    {
        de::Deserialize::deserialize(self)
    }
}

#[cfg(feature = "display")]
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::ser::to_string(self)
            .expect("Unable to represent value as string")
            .fmt(f)
    }
}

#[cfg(feature = "parse")]
impl std::str::FromStr for Table {
    type Err = crate::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::from_str(s)
    }
}

impl<'de> de::Deserializer<'de> for Table {
    type Error = crate::de::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, crate::de::Error>
    where
        V: de::Visitor<'de>,
    {
        Value::Table(self).deserialize_any(visitor)
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, crate::de::Error>
    where
        V: de::Visitor<'de>,
    {
        Value::Table(self).deserialize_enum(name, variants, visitor)
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, crate::de::Error>
    where
        V: de::Visitor<'de>,
    {
        Value::Table(self).deserialize_option(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, crate::de::Error>
    where
        V: de::Visitor<'de>,
    {
        Value::Table(self).deserialize_newtype_struct(name, visitor)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit seq
        bytes byte_buf map unit_struct tuple_struct struct
        tuple ignored_any identifier
    }
}

impl de::IntoDeserializer<'_, crate::de::Error> for Table {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}
