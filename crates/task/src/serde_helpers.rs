use serde::Deserialize;
use serde::de::{self, DeserializeOwned, Deserializer, Visitor};
use std::fmt;

/// Deserializes `T` after first materializing the input into an owned
/// [`serde_json_lenient::Value`].
///
/// `serde_json_lenient` tolerates trailing commas in the values it actually
/// deserializes, but its value-skipping codepath (used for object fields the
/// target type does not model) rejects them. VS Code's `tasks.json` and
/// `launch.json` regularly contain fields Zed doesn't model — such as
/// `compounds` or `inputs` — and editors happily leave trailing commas inside
/// them, which would otherwise abort the entire parse. Building a `Value`
/// first tolerates those trailing commas, and the subsequent `from_value` has
/// nothing to skip mid-stream.
pub fn ignore_unknown_fields_lenient<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let value = serde_json_lenient::Value::deserialize(deserializer)?;
    serde_json_lenient::from_value(value).map_err(de::Error::custom)
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
