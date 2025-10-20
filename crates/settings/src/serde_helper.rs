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
/// struct ExampleStruct(#[serde(serialize_with = "serialize_f32_with_two_decimal_places")] f32);
/// ```
pub fn serialize_f32_with_two_decimal_places<S>(value: &f32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let rounded = (value * 100.0).round() / 100.0;
    let formatted = format!("{:.2}", rounded);
    let clean_value: f64 = formatted.parse().unwrap_or(rounded as f64);
    serializer.serialize_f64(clean_value)
}
