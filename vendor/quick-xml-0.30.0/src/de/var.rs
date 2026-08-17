use crate::{
    de::key::QNameDeserializer,
    de::resolver::EntityResolver,
    de::simple_type::SimpleTypeDeserializer,
    de::{DeEvent, Deserializer, XmlRead, TEXT_KEY},
    errors::serialize::DeError,
};
use serde::de::value::BorrowedStrDeserializer;
use serde::de::{self, DeserializeSeed, Deserializer as _, Visitor};

/// An enum access
pub struct EnumAccess<'de, 'a, R, E>
where
    R: XmlRead<'de>,
    E: EntityResolver,
{
    de: &'a mut Deserializer<'de, R, E>,
}

impl<'de, 'a, R, E> EnumAccess<'de, 'a, R, E>
where
    R: XmlRead<'de>,
    E: EntityResolver,
{
    pub fn new(de: &'a mut Deserializer<'de, R, E>) -> Self {
        EnumAccess { de }
    }
}

impl<'de, 'a, R, E> de::EnumAccess<'de> for EnumAccess<'de, 'a, R, E>
where
    R: XmlRead<'de>,
    E: EntityResolver,
{
    type Error = DeError;
    type Variant = VariantAccess<'de, 'a, R, E>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, VariantAccess<'de, 'a, R, E>), DeError>
    where
        V: DeserializeSeed<'de>,
    {
        let decoder = self.de.reader.decoder();
        let (name, is_text) = match self.de.peek()? {
            DeEvent::Start(e) => (
                seed.deserialize(QNameDeserializer::from_elem(e.raw_name(), decoder)?)?,
                false,
            ),
            DeEvent::Text(_) => (
                seed.deserialize(BorrowedStrDeserializer::<DeError>::new(TEXT_KEY))?,
                true,
            ),
            DeEvent::End(e) => return Err(DeError::UnexpectedEnd(e.name().into_inner().to_vec())),
            DeEvent::Eof => return Err(DeError::UnexpectedEof),
        };
        Ok((
            name,
            VariantAccess {
                de: self.de,
                is_text,
            },
        ))
    }
}

pub struct VariantAccess<'de, 'a, R, E>
where
    R: XmlRead<'de>,
    E: EntityResolver,
{
    de: &'a mut Deserializer<'de, R, E>,
    /// `true` if variant should be deserialized from a textual content
    /// and `false` if from tag
    is_text: bool,
}

impl<'de, 'a, R, E> de::VariantAccess<'de> for VariantAccess<'de, 'a, R, E>
where
    R: XmlRead<'de>,
    E: EntityResolver,
{
    type Error = DeError;

    fn unit_variant(self) -> Result<(), DeError> {
        match self.de.next()? {
            // Consume subtree
            DeEvent::Start(e) => self.de.read_to_end(e.name()),
            // Does not needed to deserialize using SimpleTypeDeserializer, because
            // it returns `()` when `deserialize_unit()` is requested
            DeEvent::Text(_) => Ok(()),
            // SAFETY: the other events are filtered in `variant_seed()`
            _ => unreachable!("Only `Start` or `Text` events are possible here"),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, DeError>
    where
        T: DeserializeSeed<'de>,
    {
        if self.is_text {
            match self.de.next()? {
                DeEvent::Text(e) => seed.deserialize(SimpleTypeDeserializer::from_text_content(e)),
                // SAFETY: the other events are filtered in `variant_seed()`
                _ => unreachable!("Only `Text` events are possible here"),
            }
        } else {
            seed.deserialize(&mut *self.de)
        }
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, DeError>
    where
        V: Visitor<'de>,
    {
        if self.is_text {
            match self.de.next()? {
                DeEvent::Text(e) => {
                    SimpleTypeDeserializer::from_text_content(e).deserialize_tuple(len, visitor)
                }
                // SAFETY: the other events are filtered in `variant_seed()`
                _ => unreachable!("Only `Text` events are possible here"),
            }
        } else {
            self.de.deserialize_tuple(len, visitor)
        }
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, DeError>
    where
        V: Visitor<'de>,
    {
        if self.is_text {
            match self.de.next()? {
                DeEvent::Text(e) => SimpleTypeDeserializer::from_text_content(e)
                    .deserialize_struct("", fields, visitor),
                // SAFETY: the other events are filtered in `variant_seed()`
                _ => unreachable!("Only `Text` events are possible here"),
            }
        } else {
            self.de.deserialize_struct("", fields, visitor)
        }
    }
}
