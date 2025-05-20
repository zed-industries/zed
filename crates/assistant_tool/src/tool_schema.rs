use anyhow::Result;
use serde_json::Value;

use crate::LanguageModelToolSchemaFormat;

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
        LanguageModelToolSchemaFormat::JsonSchema => Ok(()),
        LanguageModelToolSchemaFormat::JsonSchemaSubset => adapt_to_json_schema_subset(json),
    }
}

/// Tries to adapt the json schema so that it is compatible with https://ai.google.dev/api/caching#Schema
fn adapt_to_json_schema_subset(json: &mut Value) -> Result<()> {
    if let Value::Object(obj) = json {
        const UNSUPPORTED_KEYS: [&str; 4] = ["if", "then", "else", "$ref"];

        for key in UNSUPPORTED_KEYS {
            if obj.contains_key(key) {
                return Err(anyhow::anyhow!(
                    "Schema cannot be made compatible because it contains \"{}\" ",
                    key
                ));
            }
        }

        const KEYS_TO_REMOVE: [&str; 5] = [
            "format",
            "additionalProperties",
            "exclusiveMinimum",
            "exclusiveMaximum",
            "optional",
        ];
        for key in KEYS_TO_REMOVE {
            obj.remove(key);
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
        if let Some(subschemas) = obj.get_mut("oneOf") {
            if subschemas.is_array() {
                let subschemas_clone = subschemas.clone();
                obj.remove("oneOf");
                obj.insert("anyOf".to_string(), subschemas_clone);
            }
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
}
