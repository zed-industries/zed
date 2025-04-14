use schemars::{
    SchemaGenerator,
    schema::{ArrayValidation, InstanceType, Schema, SchemaObject, SingleOrVec, StringValidation},
};
use serde::de::{self, Deserializer, Visitor};
use std::fmt;

/// Generates a JSON schema for a non-empty string array.
pub fn non_empty_string_vec_json_schema(_: &mut SchemaGenerator) -> Schema {
    Schema::Object(SchemaObject {
        instance_type: Some(InstanceType::Array.into()),
        array: Some(Box::new(ArrayValidation {
            unique_items: Some(true),
            items: Some(SingleOrVec::Single(Box::new(Schema::Object(
                SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    string: Some(Box::new(StringValidation {
                        min_length: Some(1), // Ensures string in the array is non-empty
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            )))),
            ..Default::default()
        })),
        format: Some("vec-of-non-empty-strings".to_string()), // Use a custom format keyword
        ..Default::default()
    })
}

/// Deserializes a non-empty string array.
pub fn non_empty_string_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct NonEmptyStringVecVisitor;

    impl<'de> Visitor<'de> for NonEmptyStringVecVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a list of non-empty strings")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Vec<String>, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element::<String>()? {
                if value.is_empty() {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Str(&value),
                        &"a non-empty string",
                    ));
                }
                vec.push(value);
            }
            Ok(vec)
        }
    }

    deserializer.deserialize_seq(NonEmptyStringVecVisitor)
}
