use serde::de::value::BorrowedStrDeserializer;
use serde::de::IntoDeserializer;

use crate::de::Error;

pub(crate) struct DatetimeDeserializer {
    date: Option<crate::Datetime>,
}

impl DatetimeDeserializer {
    pub(crate) fn new(date: crate::Datetime) -> Self {
        Self { date: Some(date) }
    }
}

impl<'de> serde::de::MapAccess<'de> for DatetimeDeserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.date.is_some() {
            seed.deserialize(BorrowedStrDeserializer::new(
                toml_datetime::__unstable::FIELD,
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
        if let Some(date) = self.date.take() {
            seed.deserialize(date.to_string().into_deserializer())
        } else {
            panic!("next_value_seed called before next_key_seed")
        }
    }
}
