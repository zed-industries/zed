use serde::de::value::BorrowedStrDeserializer;
use serde::de::IntoDeserializer as _;

use super::Error;

pub(crate) struct SpannedDeserializer<'de, T: serde::de::IntoDeserializer<'de, Error>> {
    phantom_data: std::marker::PhantomData<&'de ()>,
    start: Option<usize>,
    end: Option<usize>,
    value: Option<T>,
}

impl<'de, T> SpannedDeserializer<'de, T>
where
    T: serde::de::IntoDeserializer<'de, Error>,
{
    pub(crate) fn new(value: T, span: std::ops::Range<usize>) -> Self {
        Self {
            phantom_data: Default::default(),
            start: Some(span.start),
            end: Some(span.end),
            value: Some(value),
        }
    }
}

impl<'de, T> serde::de::MapAccess<'de> for SpannedDeserializer<'de, T>
where
    T: serde::de::IntoDeserializer<'de, Error>,
{
    type Error = Error;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.start.is_some() {
            seed.deserialize(BorrowedStrDeserializer::new(
                serde_spanned::__unstable::START_FIELD,
            ))
            .map(Some)
        } else if self.end.is_some() {
            seed.deserialize(BorrowedStrDeserializer::new(
                serde_spanned::__unstable::END_FIELD,
            ))
            .map(Some)
        } else if self.value.is_some() {
            seed.deserialize(BorrowedStrDeserializer::new(
                serde_spanned::__unstable::VALUE_FIELD,
            ))
            .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if let Some(start) = self.start.take() {
            seed.deserialize(start.into_deserializer())
        } else if let Some(end) = self.end.take() {
            seed.deserialize(end.into_deserializer())
        } else if let Some(value) = self.value.take() {
            seed.deserialize(value.into_deserializer())
        } else {
            panic!("next_value_seed called before next_key_seed")
        }
    }
}
