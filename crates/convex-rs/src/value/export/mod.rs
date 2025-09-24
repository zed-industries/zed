use serde_json::{
    json,
    Value as JsonValue,
};

use crate::Value;

#[cfg(any(test, feature = "testing"))]
pub mod roundtrip;

impl Value {
    /// Converts this value to a JSON value in the `json` export format.
    /// <https://docs.convex.dev/database/types>
    ///
    /// It is possible for distinct Convex values to be serialized to the same
    /// JSON value by this method. For instance, strings and binary values are
    /// both exported as JSON strings. However, it is possible to convert the
    /// exported value back to a unique Convex value if you also have the `Type`
    /// value associated with the original Convex value (see `roundtrip.rs`).
    ///
    /// # Example
    /// ```
    /// use convex::Value;
    /// use serde_json::{
    ///     json,
    ///     Value as JsonValue,
    /// };
    ///
    /// let value = Value::Bytes(vec![0b00000000, 0b00010000, 0b10000011]);
    /// assert_eq!(JsonValue::from(value.clone()), json!({ "$bytes": "ABCD" }));
    /// assert_eq!(value.export(), json!("ABCD"));
    /// ```
    pub fn export(self) -> JsonValue {
        match self {
            Value::Null => JsonValue::Null,
            Value::Int64(value) => JsonValue::String(value.to_string()),
            Value::Float64(value) => {
                if value.is_nan() {
                    json!("NaN")
                } else if value.is_infinite() {
                    if value.is_sign_positive() {
                        json!("Infinity")
                    } else {
                        json!("-Infinity")
                    }
                } else {
                    value.into()
                }
            },
            Value::Boolean(value) => JsonValue::Bool(value),
            Value::String(value) => JsonValue::String(value),
            Value::Bytes(value) => JsonValue::String(base64::encode(value)),
            Value::Array(values) => {
                JsonValue::Array(values.into_iter().map(|x| x.export()).collect())
            },
            Value::Object(map) => JsonValue::Object(
                map.into_iter()
                    .map(|(key, value)| (key, value.export()))
                    .collect(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use maplit::btreemap;
    use serde_json::json;

    use super::*;

    #[test]
    fn export_rustdoc_example() {
        let value = Value::Bytes(vec![0b00000000, 0b00010000, 0b10000011]);
        assert_eq!(JsonValue::from(value.clone()), json!({ "$bytes": "ABCD" }));
        assert_eq!(value.export(), json!("ABCD"));
    }

    #[test]
    fn nulls_are_exported_as_null() {
        assert_eq!(Value::Null.export(), JsonValue::Null)
    }

    #[test]
    fn booleans_are_exported_as_booleans() {
        assert_eq!(Value::Boolean(true).export(), json!(true));
        assert_eq!(Value::Boolean(false).export(), json!(false));
    }

    #[test]
    fn ints_are_exported_as_strings() {
        assert_eq!(Value::Int64(1234).export(), json!("1234"));

        assert_eq!(Value::Int64(-314).export(), json!("-314"));

        assert_eq!(Value::Int64(0).export(), json!("0"));

        assert_eq!(
            Value::Int64(i64::MIN).export(),
            json!("-9223372036854775808")
        );

        assert_eq!(
            Value::Int64(i64::MAX).export(),
            json!("9223372036854775807")
        );
    }

    #[test]
    fn finite_floats_are_exported_as_numbers() {
        assert_eq!(Value::Float64(12.34).export(), json!(12.34));
    }

    #[test]
    fn pos_zero_is_exported_as_number() {
        let json = Value::Float64(0.0).export();
        assert_eq!(json, json!(0.0));
        assert!(json.as_f64().unwrap().is_sign_positive());
        assert!(!json.as_f64().unwrap().is_sign_negative());
    }

    #[test]
    fn neg_zero_is_exported_as_number() {
        let json = Value::Float64(-0.0).export();
        assert_eq!(json, json!(-0.0));
        assert!(json.as_f64().unwrap().is_sign_negative());
        assert!(!json.as_f64().unwrap().is_sign_positive());
    }

    #[test]
    fn infinite_floats_are_exported_as_strings() {
        assert_eq!(Value::Float64(f64::INFINITY).export(), json!("Infinity"));
        assert_eq!(
            Value::Float64(f64::NEG_INFINITY).export(),
            json!("-Infinity")
        );
    }

    #[test]
    fn nan_is_exported_as_string() {
        assert_eq!(Value::Float64(f64::NAN).export(), json!("NaN"));
    }

    #[test]
    fn strings_are_exported_as_strings() {
        assert_eq!(Value::Null.export(), JsonValue::Null);
    }

    #[test]
    fn bytes_are_exported_as_base64() {
        let vec: Vec<u8> = vec![
            0b00000000, 0b00010000, 0b10000011, 0b00010000, 0b01010001, 0b10000111, 0b00100000,
            0b10010010, 0b10001011, 0b00110000, 0b11010011, 0b10001111, 0b01000001, 0b00010100,
            0b10010011, 0b01010001, 0b01010101, 0b10010111, 0b01100001, 0b10010110, 0b10011011,
            0b01110001, 0b11010111, 0b10011111, 0b10000010, 0b00011000, 0b10100011, 0b10010010,
            0b01011001, 0b10100111, 0b10100010, 0b10011010, 0b10101011, 0b10110010, 0b11011011,
            0b10101111, 0b11000011, 0b00011100, 0b10110011, 0b11010011, 0b01011101, 0b10110111,
            0b11100011, 0b10011110, 0b10111011, 0b11110011, 0b11011111, 0b10111111, 0b00000000,
        ];

        assert_eq!(
            Value::Bytes(vec).export(),
            json!("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/AA==")
        );
    }

    #[test]
    fn arrays_are_exported_as_arrays() {
        assert_eq!(
            Value::Array(vec![Value::Int64(1), Value::Int64(2), Value::Int64(3)]).export(),
            json!(["1", "2", "3"]),
        );
    }

    #[test]
    fn objects_are_exported_as_objects() {
        assert_eq!(
            Value::Object(btreemap! {
                "a".to_string() => 1.into(),
                "b".to_string() => 2.into(),
                "c".to_string() => 3.into(),
            })
            .export(),
            json!({
                "a": "1",
                "b": "2",
                "c": "3",
            }),
        );
    }
}
