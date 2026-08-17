//! Implementation of the deserializer from attributes

use std::borrow::Cow;

use serde::de::{DeserializeSeed, Deserializer, Error, IntoDeserializer, MapAccess, Visitor};
use serde::forward_to_deserialize_any;

use crate::de::key::QNameDeserializer;
use crate::de::SimpleTypeDeserializer;
use crate::errors::serialize::DeError;
use crate::events::attributes::Attributes;

impl<'i> Attributes<'i> {
    /// Converts this iterator into a serde's [`MapAccess`] trait to use with serde.
    /// The returned object also implements the [`Deserializer`] trait.
    ///
    /// # Parameters
    /// - `prefix`: a prefix of the field names in structs that should be stripped
    ///   to get the local attribute name. The [`crate::de::Deserializer`] uses `"@"`
    ///   as a prefix, but [`Self::into_deserializer()`] uses empy string, which mean
    ///   that we do not strip anything.
    ///
    /// # Example
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// use quick_xml::events::BytesStart;
    /// use serde::Deserialize;
    /// use serde::de::IntoDeserializer;
    ///
    /// #[derive(Debug, PartialEq, Deserialize)]
    /// struct MyData<'i> {
    ///     question: &'i str,
    ///     answer: u32,
    /// }
    ///
    /// #[derive(Debug, PartialEq, Deserialize)]
    /// struct MyDataPrefixed<'i> {
    ///     #[serde(rename = "@question")] question: &'i str,
    ///     #[serde(rename = "@answer")]   answer: u32,
    /// }
    ///
    /// let tag = BytesStart::from_content(
    ///     "tag
    ///         question = 'The Ultimate Question of Life, the Universe, and Everything'
    ///         answer = '42'",
    ///     3
    /// );
    /// // Strip nothing from the field names
    /// let de = tag.attributes().clone().into_deserializer();
    /// assert_eq!(
    ///     MyData::deserialize(de).unwrap(),
    ///     MyData {
    ///         question: "The Ultimate Question of Life, the Universe, and Everything",
    ///         answer: 42,
    ///     }
    /// );
    ///
    /// // Strip "@" from the field name
    /// let de = tag.attributes().into_map_access("@");
    /// assert_eq!(
    ///     MyDataPrefixed::deserialize(de).unwrap(),
    ///     MyDataPrefixed {
    ///         question: "The Ultimate Question of Life, the Universe, and Everything",
    ///         answer: 42,
    ///     }
    /// );
    /// ```
    #[inline]
    pub const fn into_map_access(self, prefix: &'static str) -> AttributesDeserializer<'i> {
        AttributesDeserializer {
            iter: self,
            value: None,
            prefix,
            key_buf: String::new(),
        }
    }
}

impl<'de> IntoDeserializer<'de, DeError> for Attributes<'de> {
    type Deserializer = AttributesDeserializer<'de>;

    #[inline]
    fn into_deserializer(self) -> Self::Deserializer {
        self.into_map_access("")
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// A deserializer used to make possible to pack all attributes into a struct.
/// It is created by [`Attributes::into_map_access`] or [`Attributes::into_deserializer`]
/// methods.
///
/// This deserializer always call [`Visitor::visit_map`] with self as [`MapAccess`].
///
/// # Lifetime
///
/// `'i` is a lifetime of the original buffer from which attributes were parsed.
/// In particular, when reader was created from a string, this is lifetime of the
/// string.
#[derive(Debug, Clone)]
pub struct AttributesDeserializer<'i> {
    iter: Attributes<'i>,
    /// The value of the attribute, read in last call to `next_key_seed`.
    value: Option<Cow<'i, [u8]>>,
    /// This prefix will be stripped from struct fields before match against attribute name.
    prefix: &'static str,
    /// Buffer to store attribute name as a field name exposed to serde consumers.
    /// Keeped in the serializer to avoid many small allocations
    key_buf: String,
}

impl<'de> Deserializer<'de> for AttributesDeserializer<'de> {
    type Error = DeError;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> MapAccess<'de> for AttributesDeserializer<'de> {
    type Error = DeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        debug_assert_eq!(self.value, None);

        match self.iter.next() {
            None => Ok(None),
            Some(Ok(attr)) => {
                self.value = Some(attr.value);
                self.key_buf.clear();
                self.key_buf.push_str(self.prefix);
                let de =
                    QNameDeserializer::from_attr(attr.key, self.iter.decoder(), &mut self.key_buf)?;
                seed.deserialize(de).map(Some)
            }
            Some(Err(err)) => Err(Error::custom(err)),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => {
                let de =
                    SimpleTypeDeserializer::from_part(&value, 0..value.len(), self.iter.decoder());
                seed.deserialize(de)
            }
            None => Err(DeError::KeyNotRead),
        }
    }
}
