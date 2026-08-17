use crate::NullValue;

impl From<()> for NullValue {
    fn from(_: ()) -> Self {
        Self::NullValue
    }
}

impl serde::Serialize for NullValue {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ().serialize(ser)
    }
}

impl<'de> serde::Deserialize<'de> for NullValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_unit(NullValueVisitor)
    }
}

struct NullValueVisitor;

impl<'de> serde::de::Visitor<'de> for NullValueVisitor {
    type Value = NullValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("google.protobuf.NullValue")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NullValue::NullValue)
    }
}
