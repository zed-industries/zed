use core::str;
use std::marker::PhantomData;

use serde::de::{Deserializer, SeqAccess, Visitor};

use crate::{Signature, Type};

/// A wrapper to deserialize a value to `T: Type + serde::Deserialize`.
///
/// When the type of a value is well-known, you may avoid the cost and complexity of wrapping to a
/// generic [`enum@crate::Value`] and instead use this wrapper.
///
/// ```
/// # use zvariant::{to_bytes, serialized::Context, as_value::{Deserialize, Serialize}, LE};
/// #
/// # let ctxt = Context::new_dbus(LE, 0);
/// # let array = [0, 1, 2];
/// # let v = Serialize(&array);
/// # let encoded = to_bytes(ctxt, &v).unwrap();
/// let decoded: Deserialize<[u8; 3]> = encoded.deserialize().unwrap().0;
/// # assert_eq!(decoded.0, array);
/// ```
pub struct Deserialize<'de, T: Type + serde::Deserialize<'de>>(
    pub T,
    std::marker::PhantomData<&'de T>,
);

impl<'de, T: Type + serde::Deserialize<'de>> serde::Deserialize<'de> for Deserialize<'de, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["signature", "value"];
        Ok(Deserialize(
            deserializer.deserialize_struct(
                "Variant",
                FIELDS,
                DeserializeValueVisitor(PhantomData),
            )?,
            PhantomData,
        ))
    }
}

struct DeserializeValueVisitor<T>(PhantomData<T>);

impl<'de, T: Type + serde::Deserialize<'de>> Visitor<'de> for DeserializeValueVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Variant")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let sig: Signature = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        if T::SIGNATURE != &sig {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&sig.to_string()),
                &"the value signature",
            ));
        }

        seq.next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))
    }
}

impl<'de, T: Type + serde::Deserialize<'de>> Type for Deserialize<'de, T> {
    const SIGNATURE: &'static Signature = &Signature::Variant;
}

/// Deserialize a value as a [`enum@zvariant::Value`].
pub fn deserialize<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: serde::Deserialize<'de> + Type + 'de,
{
    use serde::Deserialize as _;

    Deserialize::deserialize(deserializer).map(|v| v.0)
}

/// Deserialize an optional value as a [`enum@zvariant::Value`].
pub fn deserialize_optional<'de, T, D>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::Deserialize<'de> + Type + 'de,
{
    deserialize(deserializer).map(Some)
}
