use crate::Struct;

impl From<std::collections::HashMap<String, crate::Value>> for Struct {
    fn from(fields: std::collections::HashMap<String, crate::Value>) -> Self {
        Self { fields }
    }
}

impl FromIterator<(String, crate::Value)> for Struct {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (String, crate::Value)>,
    {
        Self {
            fields: iter.into_iter().collect(),
        }
    }
}

impl serde::Serialize for Struct {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.fields.serialize(ser)
    }
}

impl<'de> serde::Deserialize<'de> for Struct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(StructVisitor)
    }
}

struct StructVisitor;

impl<'de> serde::de::Visitor<'de> for StructVisitor {
    type Value = Struct;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("google.protobuf.Struct")
    }

    fn visit_map<A>(self, mut map_access: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut map = std::collections::HashMap::new();

        while let Some((key, value)) = map_access.next_entry()? {
            map.insert(key, value);
        }

        Ok(map.into())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let map: crate::Struct = std::collections::HashMap::from([
            (String::from("bool"), crate::Value::from(true)),
            (
                String::from("unit"),
                crate::value::Kind::NullValue(0).into(),
            ),
            (String::from("number"), 5.0.into()),
            (String::from("string"), "string".into()),
            (String::from("list"), vec![1.0.into(), 2.0.into()].into()),
            (
                String::from("map"),
                std::collections::HashMap::from([(String::from("key"), "value".into())]).into(),
            ),
        ])
        .into();

        assert_eq!(
            serde_json::to_value(map).unwrap(),
            serde_json::json!({
                "bool": true,
                "unit": null,
                "number": 5.0,
                "string": "string",
                "list": [1.0, 2.0],
                "map": {
                    "key": "value",
                }
            })
        );
    }
}
