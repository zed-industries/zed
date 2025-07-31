use serde::de::{self, Deserializer, Visitor};
use std::fmt;

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
