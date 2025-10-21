use serde::Serializer;

/// Serializes an f32 value with 2 decimal places of precision.
///
/// This function rounds the value to 2 decimal places and formats it as a string,
/// then parses it back to f64 before serialization. This ensures clean JSON output
/// without IEEE 754 floating-point artifacts.
///
/// # Arguments
///
/// * `value` - The f32 value to serialize
/// * `serializer` - The serde serializer to use
///
/// # Returns
///
/// Result of the serialization operation
///
/// # Usage
///
/// This function can be used with Serde's `serialize_with` attribute:
/// ```
/// use serde::Serialize;
/// use settings::serialize_f32_with_two_decimal_places;
///
/// #[derive(Serialize)]
/// struct ExampleStruct(#[serde(serialize_with = "serialize_f32_with_two_decimal_places")] f32);
/// ```
pub fn serialize_f32_with_two_decimal_places<S>(
    value: &f32,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let rounded = (value * 100.0).round() / 100.0;
    let formatted = format!("{:.2}", rounded);
    let clean_value: f64 = formatted.parse().unwrap_or(rounded as f64);
    serializer.serialize_f64(clean_value)
}

/// Serializes an optional f32 value with 2 decimal places of precision.
///
/// This function handles `Option<f32>` types, serializing `Some` values with 2 decimal
/// places of precision and `None` values as null. For `Some` values, it rounds to 2 decimal
/// places and formats as a string, then parses back to f64 before serialization. This ensures
/// clean JSON output without IEEE 754 floating-point artifacts.
///
/// # Arguments
///
/// * `value` - The optional f32 value to serialize
/// * `serializer` - The serde serializer to use
///
/// # Returns
///
/// Result of the serialization operation
///
/// # Behavior
///
/// * `Some(v)` - Serializes the value rounded to 2 decimal places
/// * `None` - Serializes as JSON null
///
/// # Usage
///
/// This function can be used with Serde's `serialize_with` attribute:
/// ```
/// use serde::Serialize;
/// use settings::serialize_optional_f32_with_two_decimal_places;
///
/// #[derive(Serialize)]
/// struct ExampleStruct {
///     #[serde(serialize_with = "serialize_optional_f32_with_two_decimal_places")]
///     optional_value: Option<f32>,
/// }
/// ```
pub fn serialize_optional_f32_with_two_decimal_places<S>(
    value: &Option<f32>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => {
            let rounded = (v * 100.0).round() / 100.0;
            let formatted = format!("{:.2}", rounded);
            let clean_value: f64 = formatted.parse().unwrap_or(rounded as f64);
            serializer.serialize_some(&clean_value)
        }
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct TestOptional {
        #[serde(serialize_with = "serialize_optional_f32_with_two_decimal_places")]
        value: Option<f32>,
    }

    #[derive(Serialize, Deserialize)]
    struct TestNonOptional {
        #[serde(serialize_with = "serialize_f32_with_two_decimal_places")]
        value: f32,
    }

    #[test]
    fn test_serialize_optional_f32_with_two_decimal_places() {
        let cases = [
            (Some(123.456789), r#"{"value":123.46}"#),
            (Some(1.2), r#"{"value":1.2}"#),
            (Some(300.00000), r#"{"value":300.0}"#),
        ];
        for (value, expected) in cases {
            let value = TestOptional { value };
            assert_eq!(serde_json::to_string(&value).unwrap(), expected);
        }
    }

    #[test]
    fn test_serialize_f32_with_two_decimal_places() {
        let cases = [
            (123.456789, r#"{"value":123.46}"#),
            (1.200, r#"{"value":1.2}"#),
            (300.00000, r#"{"value":300.0}"#),
        ];
        for (value, expected) in cases {
            let value = TestNonOptional { value };
            assert_eq!(serde_json::to_string(&value).unwrap(), expected);
        }
    }
}
