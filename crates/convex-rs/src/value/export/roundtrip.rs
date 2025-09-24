use std::collections::BTreeMap;

use anyhow::Context;
use serde_json::Value as JsonValue;

use crate::Value;

/// Type hint associated with a Convex value. This allows us to uniquely convert
/// the exported value back to the original Convex value.
#[allow(missing_docs)]
pub enum ExportContext {
    Null,
    Int64,
    Float64 {
        // Store the f64 value in the export context when it is NaN, because the export format
        // assumes a single NaN value. This ensures that we can fully roundtrip values.
        nan_value: Option<f64>,
    },
    Boolean,
    String,
    Bytes,
    Array(Vec<ExportContext>),
    Set,
    Map,
    Object(BTreeMap<String, ExportContext>),
}

impl ExportContext {
    /// Returns the export context of a Convex value
    pub fn of(value: &Value) -> ExportContext {
        match value {
            Value::Null => ExportContext::Null,
            Value::Int64(_) => ExportContext::Int64,
            Value::Float64(f) => ExportContext::Float64 {
                nan_value: f.is_nan().then_some(*f),
            },
            Value::Boolean(_) => ExportContext::Boolean,
            Value::String(_) => ExportContext::String,
            Value::Bytes(_) => ExportContext::Bytes,
            Value::Array(elements) => {
                ExportContext::Array(elements.iter().map(ExportContext::of).collect())
            },
            Value::Object(fields) => ExportContext::Object(
                fields
                    .iter()
                    .map(|(key, value)| (key.clone(), ExportContext::of(value)))
                    .collect(),
            ),
        }
    }
}

impl TryFrom<(JsonValue, &ExportContext)> for Value {
    type Error = anyhow::Error;

    fn try_from(
        (exported_value, type_hint): (JsonValue, &ExportContext),
    ) -> Result<Self, Self::Error> {
        match type_hint {
            ExportContext::Null => Ok(Value::Null),
            ExportContext::Int64 => match exported_value {
                JsonValue::String(str) => str
                    .parse::<i64>()
                    .map(Value::from)
                    .context("Unexpected string for i64"),
                _ => anyhow::bail!("Unexpected value for i64"),
            },
            ExportContext::Float64 {
                nan_value: Some(nan_value),
            } => {
                if !nan_value.is_nan() {
                    anyhow::bail!("Unexpected non-NaN value in the export context");
                }

                if exported_value != JsonValue::String(String::from("NaN")) {
                    anyhow::bail!("Unexpected serialization of a NaN value");
                }

                Ok((*nan_value).into())
            },
            ExportContext::Float64 { nan_value: None } => match exported_value {
                JsonValue::String(str) => match str.as_ref() {
                    "Infinity" => Ok(f64::INFINITY.into()),
                    "-Infinity" => Ok(f64::NEG_INFINITY.into()),
                    _ => anyhow::bail!("Unexpected string for f64"),
                },
                JsonValue::Number(n) => n
                    .as_f64()
                    .map(Value::from)
                    .context("Unexpected number for i64"),
                _ => anyhow::bail!("Unexpected value for f64"),
            },
            ExportContext::Boolean => match exported_value {
                JsonValue::Bool(value) => Ok(value.into()),
                _ => anyhow::bail!("Unexpected value for boolean"),
            },
            ExportContext::String => match exported_value {
                JsonValue::String(value) => Ok(value.into()),
                _ => anyhow::bail!("Unexpected value for string"),
            },
            ExportContext::Bytes => match exported_value {
                JsonValue::String(value) => base64::decode(value)
                    .map(Value::from)
                    .context("Unexpected string for bytes"),
                _ => anyhow::bail!("Unexpected value for bytes"),
            },
            ExportContext::Array(type_hints) => match exported_value {
                JsonValue::Array(exported_values) => {
                    if exported_values.len() != type_hints.len() {
                        anyhow::bail!("Array lengths do not match");
                    }

                    let values: anyhow::Result<Vec<Value>> = exported_values
                        .into_iter()
                        .zip(type_hints)
                        .map(Value::try_from)
                        .collect();

                    Ok(Value::Array(values?))
                },
                _ => anyhow::bail!("Unexpected value for array"),
            },
            ExportContext::Set | ExportContext::Map => Value::try_from(exported_value)
                .context("Couldn’t deserialize set/map from internal representation"),
            ExportContext::Object(type_hints) => match exported_value {
                JsonValue::Object(exported_values) => {
                    let entries: anyhow::Result<BTreeMap<String, Value>> = exported_values
                        .into_iter()
                        .map(|(key, value)| {
                            let Some(type_hint) = type_hints.get(&key) else {
                                anyhow::bail!("Missing export context for an object key");
                            };
                            Ok((key, (value, type_hint).try_into()?))
                        })
                        .collect();

                    Ok(Value::Object(entries?))
                },
                _ => anyhow::bail!("Unexpected value for object"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::{
        value::export::roundtrip::ExportContext,
        Value,
    };

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: None, ..ProptestConfig::default()
        })]
        #[test]
        fn export_roundtrips_with_type_hint(value in any::<Value>()) {
            let exported_value = value.clone().export();
            let type_hint = ExportContext::of(&value);

            prop_assert_eq!(
                value,
                Value::try_from((exported_value, &type_hint)).unwrap()
            );
        }
    }
}
