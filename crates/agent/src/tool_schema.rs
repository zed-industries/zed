use anyhow::Result;
use language_model::LanguageModelToolSchemaFormat;
use schemars::{
    JsonSchema, Schema,
    generate::SchemaSettings,
    transform::{Transform, transform_subschemas},
};
use serde_json::Value;

pub(crate) fn root_schema_for<T: JsonSchema>(format: LanguageModelToolSchemaFormat) -> Schema {
    let mut generator = match format {
        LanguageModelToolSchemaFormat::JsonSchema => SchemaSettings::draft07().into_generator(),
        LanguageModelToolSchemaFormat::JsonSchemaSubset => SchemaSettings::openapi3()
            .with(|settings| {
                settings.meta_schema = None;
                settings.inline_subschemas = true;
            })
            .with_transform(ToJsonSchemaSubsetTransform)
            .into_generator(),
    };
    generator.root_schema_for::<T>()
}

#[derive(Debug, Clone)]
struct ToJsonSchemaSubsetTransform;

impl Transform for ToJsonSchemaSubsetTransform {
    fn transform(&mut self, schema: &mut Schema) {
        // Ensure that the type field is not an array, this happens when we use
        // Option<T>, the type will be [T, "null"].
        if let Some(type_field) = schema.get_mut("type")
            && let Some(types) = type_field.as_array()
            && let Some(first_type) = types.first()
        {
            *type_field = first_type.clone();
        }

        // oneOf is not supported, use anyOf instead
        if let Some(one_of) = schema.remove("oneOf") {
            schema.insert("anyOf".to_string(), one_of);
        }

        transform_subschemas(self, schema);
    }
}

/// Tries to adapt a JSON schema representation to be compatible with the specified format.
///
/// If the json cannot be made compatible with the specified format, an error is returned.
pub fn adapt_schema_to_format(
    json: &mut Value,
    format: LanguageModelToolSchemaFormat,
) -> Result<()> {
    if let Value::Object(obj) = json {
        obj.remove("$schema");
        obj.remove("title");
    }

    match format {
        LanguageModelToolSchemaFormat::JsonSchema => preprocess_json_schema(json),
        LanguageModelToolSchemaFormat::JsonSchemaSubset => adapt_to_json_schema_subset(json),
    }
}

fn preprocess_json_schema(json: &mut Value) -> Result<()> {
    // `additionalProperties` defaults to `false` unless explicitly specified.
    // This prevents models from hallucinating tool parameters.
    if let Value::Object(obj) = json
        && matches!(obj.get("type"), Some(Value::String(s)) if s == "object")
    {
        if !obj.contains_key("additionalProperties") {
            obj.insert("additionalProperties".to_string(), Value::Bool(false));
        }

        // OpenAI API requires non-missing `properties`
        if !obj.contains_key("properties") {
            obj.insert("properties".to_string(), Value::Object(Default::default()));
        }
    }
    Ok(())
}

/// Tries to adapt the json schema so that it is compatible with https://ai.google.dev/api/caching#Schema
fn adapt_to_json_schema_subset(json: &mut Value) -> Result<()> {
    if let Value::Object(obj) = json {
        const UNSUPPORTED_KEYS: [&str; 4] = ["if", "then", "else", "$ref"];

        for key in UNSUPPORTED_KEYS {
            anyhow::ensure!(
                !obj.contains_key(key),
                "Schema cannot be made compatible because it contains \"{key}\""
            );
        }

        const KEYS_TO_REMOVE: [(&str, fn(&Value) -> bool); 5] = [
            ("format", |value| value.is_string()),
            ("additionalProperties", |value| value.is_boolean()),
            ("exclusiveMinimum", |value| value.is_number()),
            ("exclusiveMaximum", |value| value.is_number()),
            ("optional", |value| value.is_boolean()),
        ];
        for (key, predicate) in KEYS_TO_REMOVE {
            if let Some(value) = obj.get(key)
                && predicate(value)
            {
                obj.remove(key);
            }
        }

        // If a type is not specified for an input parameter, add a default type
        if matches!(obj.get("description"), Some(Value::String(_)))
            && !obj.contains_key("type")
            && !(obj.contains_key("anyOf")
                || obj.contains_key("oneOf")
                || obj.contains_key("allOf"))
        {
            obj.insert("type".to_string(), Value::String("string".to_string()));
        }

        // Handle oneOf -> anyOf conversion
        if let Some(subschemas) = obj.get_mut("oneOf")
            && subschemas.is_array()
        {
            let subschemas_clone = subschemas.clone();
            obj.remove("oneOf");
            obj.insert("anyOf".to_string(), subschemas_clone);
        }

        // Recursively process all nested objects and arrays
        for (_, value) in obj.iter_mut() {
            if let Value::Object(_) | Value::Array(_) = value {
                adapt_to_json_schema_subset(value)?;
            }
        }
    } else if let Value::Array(arr) = json {
        for item in arr.iter_mut() {
            adapt_to_json_schema_subset(item)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_transform_adds_type_when_missing() {
        let mut json = json!({
            "description": "A test field without type"
        });

        adapt_to_json_schema_subset(&mut json).unwrap();

        assert_eq!(
            json,
            json!({
                "description": "A test field without type",
                "type": "string"
            })
        );

        // Ensure that we do not add a type if it is an object
        let mut json = json!({
            "description": {
                "value": "abc",
                "type": "string"
            }
        });

        adapt_to_json_schema_subset(&mut json).unwrap();

        assert_eq!(
            json,
            json!({
                "description": {
                    "value": "abc",
                    "type": "string"
                }
            })
        );
    }

    #[test]
    fn test_transform_removes_unsupported_keys() {
        let mut json = json!({
            "description": "A test field",
            "type": "integer",
            "format": "uint32",
            "exclusiveMinimum": 0,
            "exclusiveMaximum": 100,
            "additionalProperties": false,
            "optional": true
        });

        adapt_to_json_schema_subset(&mut json).unwrap();

        assert_eq!(
            json,
            json!({
                "description": "A test field",
                "type": "integer"
            })
        );

        // Ensure that we do not remove keys that are actually supported (e.g. "format" can just be used as another property)
        let mut json = json!({
            "description": "A test field",
            "type": "integer",
            "format": {},
        });

        adapt_to_json_schema_subset(&mut json).unwrap();

        assert_eq!(
            json,
            json!({
                "description": "A test field",
                "type": "integer",
                "format": {},
            })
        );
    }

    #[test]
    fn test_transform_one_of_to_any_of() {
        let mut json = json!({
            "description": "A test field",
            "oneOf": [
                { "type": "string" },
                { "type": "integer" }
            ]
        });

        adapt_to_json_schema_subset(&mut json).unwrap();

        assert_eq!(
            json,
            json!({
                "description": "A test field",
                "anyOf": [
                    { "type": "string" },
                    { "type": "integer" }
                ]
            })
        );
    }

    #[test]
    fn test_transform_nested_objects() {
        let mut json = json!({
            "type": "object",
            "properties": {
                "nested": {
                    "oneOf": [
                        { "type": "string" },
                        { "type": "null" }
                    ],
                    "format": "email"
                }
            }
        });

        adapt_to_json_schema_subset(&mut json).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "object",
                "properties": {
                    "nested": {
                        "anyOf": [
                            { "type": "string" },
                            { "type": "null" }
                        ]
                    }
                }
            })
        );
    }

    #[test]
    fn test_transform_fails_if_unsupported_keys_exist() {
        let mut json = json!({
            "type": "object",
            "properties": {
                "$ref": "#/definitions/User",
            }
        });

        assert!(adapt_to_json_schema_subset(&mut json).is_err());

        let mut json = json!({
            "type": "object",
            "properties": {
                "if": "...",
            }
        });

        assert!(adapt_to_json_schema_subset(&mut json).is_err());

        let mut json = json!({
            "type": "object",
            "properties": {
                "then": "...",
            }
        });

        assert!(adapt_to_json_schema_subset(&mut json).is_err());

        let mut json = json!({
            "type": "object",
            "properties": {
                "else": "...",
            }
        });

        assert!(adapt_to_json_schema_subset(&mut json).is_err());
    }

    #[test]
    fn test_preprocess_json_schema_adds_additional_properties() {
        let mut json = json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string"
                }
            }
        });

        preprocess_json_schema(&mut json).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string"
                    }
                },
                "additionalProperties": false
            })
        );
    }

    #[test]
    fn test_preprocess_json_schema_preserves_additional_properties() {
        let mut json = json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string"
                }
            },
            "additionalProperties": true
        });

        preprocess_json_schema(&mut json).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string"
                    }
                },
                "additionalProperties": true
            })
        );
    }
}
