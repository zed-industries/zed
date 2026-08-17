use crate::ListValue;

impl From<Vec<crate::Value>> for ListValue {
    fn from(values: Vec<crate::Value>) -> Self {
        Self { values }
    }
}

impl<const N: usize> From<[crate::Value; N]> for ListValue {
    fn from(values: [crate::Value; N]) -> Self {
        Self {
            values: values.into(),
        }
    }
}

impl FromIterator<crate::value::Kind> for ListValue {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = crate::value::Kind>,
    {
        Self {
            values: iter.into_iter().map(Into::into).collect(),
        }
    }
}

impl FromIterator<crate::Value> for ListValue {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = crate::Value>,
    {
        Self {
            values: iter.into_iter().collect(),
        }
    }
}

impl serde::Serialize for ListValue {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.values.serialize(ser)
    }
}

impl<'de> serde::Deserialize<'de> for ListValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(ListValueVisitor)
    }
}

struct ListValueVisitor;

impl<'de> serde::de::Visitor<'de> for ListValueVisitor {
    type Value = ListValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("google.protobuf.ListValue")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut list = Vec::new();

        while let Some(value) = seq.next_element()? {
            list.push(value);
        }

        Ok(list.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;

    #[test]
    fn mixed_types() {
        assert_eq!(
            serde_json::to_value(Value::from([true.into(), "HELLO".into(), false.into()])).unwrap(),
            serde_json::json!([true, "HELLO", false])
        );
    }

    #[test]
    fn list_value() {
        assert_eq!(
            serde_json::to_value(Value::from([false.into(), true.into(), false.into()])).unwrap(),
            serde_json::json!([false, true, false])
        );
        assert_eq!(
            serde_json::to_value(Value::from(true)).unwrap(),
            serde_json::json!(true)
        );
    }
}
