use anyhow::Result;
use schemars::{
    JsonSchema, Schema,
    generate::SchemaSettings,
    transform::{Transform, transform_subschemas},
};
use serde_json::{Map, Value, json};

/// Indicates the format used to define the input schema for a language model tool.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LanguageModelToolSchemaFormat {
    /// A JSON schema, see https://json-schema.org
    JsonSchema,
    /// A subset of an OpenAPI 3.0 schema object supported by Google AI, see https://ai.google.dev/api/caching#Schema
    JsonSchemaSubset,
}

pub fn root_schema_for<T: JsonSchema>(format: LanguageModelToolSchemaFormat) -> Schema {
    let mut generator = match format {
        LanguageModelToolSchemaFormat::JsonSchema => SchemaSettings::draft07()
            .with(|settings| {
                settings.meta_schema = None;
                settings.inline_subschemas = true;
            })
            .into_generator(),
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
        if let Some(obj) = schema.as_object_mut() {
            // `Option<T>` produces `type: [T, "null"]`. Convert to OpenAPI 3.0's
            // `nullable: true` so nullability isn't silently dropped.
            convert_null_in_types_to_nullable(obj);

            // Any remaining multi-type array (uncommon in Rust-generated schemas)
            // is collapsed to its first entry to keep this schema subset-compatible.
            if let Some(type_field) = obj.get_mut("type")
                && let Some(types) = type_field.as_array()
                && let Some(first_type) = types.first().cloned()
            {
                *type_field = first_type;
            }

            // oneOf is not supported, use anyOf instead
            if let Some(one_of) = obj.remove("oneOf") {
                obj.insert("anyOf".to_string(), one_of);
            }
        }

        transform_subschemas(self, schema);

        if let Some(obj) = schema.as_object_mut() {
            collapse_nullable_only_any_of(obj);
        }
    }
}

/// Tries to adapt a JSON schema representation to be compatible with the specified format.
///
/// If the json cannot be made compatible with the specified format, an error is returned.
pub fn adapt_schema_to_format(
    json: &mut Value,
    format: LanguageModelToolSchemaFormat,
) -> Result<()> {
    log::trace!("Adapting schema to format {:?}: {}", format, json);

    if let Value::Object(obj) = json {
        obj.remove("$schema");
        obj.remove("title");
        obj.remove("description");
    }

    match format {
        LanguageModelToolSchemaFormat::JsonSchema => preprocess_json_schema(json),
        LanguageModelToolSchemaFormat::JsonSchemaSubset => adapt_to_json_schema_subset(json),
    }?;

    log::trace!("Adapted schema: {}", json);
    Ok(())
}

fn preprocess_json_schema(json: &mut Value) -> Result<()> {
    if let Value::Object(obj) = json
        && matches!(obj.get("type"), Some(Value::String(s)) if s == "object")
    {
        if !obj.contains_key("additionalProperties") {
            obj.insert("additionalProperties".to_string(), Value::Bool(false));
        }

        if !obj.contains_key("properties") {
            obj.insert("properties".to_string(), Value::Object(Default::default()));
        }
    }
    Ok(())
}

fn adapt_to_json_schema_subset(json: &mut Value) -> Result<()> {
    if let Value::Object(obj) = json {
        const UNSUPPORTED_KEYS: [&str; 4] = ["if", "then", "else", "$ref"];

        for key in UNSUPPORTED_KEYS {
            anyhow::ensure!(
                !obj.contains_key(key),
                "Schema cannot be made compatible because it contains \"{key}\""
            );
        }

        const KEYS_TO_REMOVE: [(&str, fn(&Value) -> bool); 6] = [
            ("format", |value| value.is_string()),
            ("additionalProperties", |_| true),
            ("propertyNames", |_| true),
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

        convert_null_in_types_to_nullable(obj);
        convert_types_to_any_of_defs(obj);

        // After the conversions above, `type` should only ever still be an array
        // if a malformed input had a single-element type array (e.g. `["string"]`).
        // Collapse it to a single value so downstream consumers see a scalar.
        if let Some(type_value) = obj.get_mut("type")
            && let Some(types) = type_value.as_array()
            && let Some(first_type) = types.first().cloned()
        {
            *type_value = first_type;
        }

        if matches!(obj.get("description"), Some(Value::String(_)))
            && !obj.contains_key("type")
            && !(obj.contains_key("anyOf")
                || obj.contains_key("oneOf")
                || obj.contains_key("allOf"))
        {
            obj.insert("type".to_string(), Value::String("string".to_string()));
        }

        if let Some(subschemas) = obj.get_mut("oneOf")
            && subschemas.is_array()
        {
            let subschemas_clone = subschemas.clone();
            obj.remove("oneOf");
            push_any_of_constraint(obj, subschemas_clone);
        }

        for (_, value) in obj.iter_mut() {
            if let Value::Object(_) | Value::Array(_) = value {
                adapt_to_json_schema_subset(value)?;
            }
        }

        // Children may have been rewritten from `{"type": "null"}` into
        // `{"nullable": true}`. Fold those into the parent so the result matches
        // OpenAPI 3.0's convention of `nullable: true` as a sibling of `type`.
        collapse_nullable_only_any_of(obj);
    } else if let Value::Array(arr) = json {
        for item in arr.iter_mut() {
            adapt_to_json_schema_subset(item)?;
        }
    }
    Ok(())
}

fn convert_null_in_types_to_nullable(obj: &mut Map<String, Value>) {
    let mut nullable_found_in_type = false;

    if let Some(type_entry) = obj.get_mut("type") {
        if let Some(types) = type_entry.as_array_mut() {
            let mut had_null_type = false;
            types.retain(|t| {
                if t.as_str() == Some("null") {
                    had_null_type = true;
                    false
                } else {
                    true
                }
            });

            if had_null_type {
                nullable_found_in_type = true;
                if types.len() == 1 {
                    *type_entry = types.remove(0);
                } else if types.is_empty() {
                    obj.remove("type");
                }
            }
        } else if let Some(type_str) = type_entry.as_str() {
            if type_str == "null" {
                nullable_found_in_type = true;
                obj.remove("type");
            }
        }
    }
    if nullable_found_in_type {
        obj.insert("nullable".to_string(), Value::Bool(true));
    }
}

fn convert_types_to_any_of_defs(obj: &mut Map<String, Value>) {
    let is_multi_type = obj
        .get("type")
        .and_then(|v| v.as_array())
        .is_some_and(|types| types.len() > 1);
    if !is_multi_type {
        return;
    }

    let Some(Value::Array(types)) = obj.remove("type") else {
        return;
    };
    let any_of_schemas = types.into_iter().map(|t| json!({"type": t})).collect();
    push_any_of_constraint(obj, Value::Array(any_of_schemas));
}

fn push_any_of_constraint(obj: &mut Map<String, Value>, any_of_schemas: Value) {
    if let Some(existing_any_of) = obj.remove("anyOf") {
        let mut all_of = match obj.remove("allOf") {
            Some(Value::Array(arr)) => arr,
            _ => Vec::new(),
        };
        // Always preserve the pre-existing `anyOf` — earlier this push was
        // skipped when `allOf` was non-empty, which silently dropped it.
        all_of.push(json!({"anyOf": existing_any_of}));
        all_of.push(json!({"anyOf": any_of_schemas}));
        obj.insert("allOf".to_string(), Value::Array(all_of));
    } else if let Some(all_of) = obj.get_mut("allOf").and_then(|v| v.as_array_mut()) {
        all_of.push(json!({"anyOf": any_of_schemas}));
    } else {
        obj.insert("anyOf".to_string(), any_of_schemas);
    }
}

/// Folds `{nullable: true}`-only entries out of an `anyOf` array and onto the
/// parent object. This matches OpenAPI 3.0 semantics, where nullability is
/// expressed as a sibling of `type` rather than a separate variant.
fn collapse_nullable_only_any_of(obj: &mut Map<String, Value>) {
    let Some(Value::Array(mut any_of)) = obj.remove("anyOf") else {
        return;
    };

    let mut found_nullable_only = false;
    any_of.retain(|entry| {
        let is_nullable_only = entry
            .as_object()
            .is_some_and(|m| m.len() == 1 && matches!(m.get("nullable"), Some(Value::Bool(true))));
        if is_nullable_only {
            found_nullable_only = true;
            false
        } else {
            true
        }
    });

    if !found_nullable_only {
        obj.insert("anyOf".to_string(), Value::Array(any_of));
        return;
    }

    obj.insert("nullable".to_string(), Value::Bool(true));

    if any_of.is_empty() {
        return;
    }

    // If a single variant remains and its keys don't collide with the parent's
    // existing keys, inline it. `anyOf` with a single entry is equivalent to
    // just that entry, and inlining produces the canonical OpenAPI form
    // (e.g. `{type: "string", nullable: true}`).
    if any_of.len() == 1
        && let Value::Object(entry_obj) = &any_of[0]
        && entry_obj.keys().all(|k| !obj.contains_key(k))
    {
        let entry = any_of.remove(0);
        if let Value::Object(entry_obj) = entry {
            for (k, v) in entry_obj {
                obj.insert(k, v);
            }
        }
        return;
    }

    obj.insert("anyOf".to_string(), Value::Array(any_of));
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn obj(value: Value) -> Map<String, Value> {
        match value {
            Value::Object(map) => map,
            other => panic!("expected JSON object, got {other}"),
        }
    }

    #[test]
    fn test_convert_null_in_types_to_nullable() {
        // ["string", "null"] -> "string", nullable: true
        let mut o = obj(json!({"type": ["string", "null"]}));
        convert_null_in_types_to_nullable(&mut o);
        assert_eq!(o, obj(json!({"type": "string", "nullable": true})));

        // "null" -> nullable: true
        let mut o = obj(json!({"type": "null"}));
        convert_null_in_types_to_nullable(&mut o);
        assert_eq!(o, obj(json!({"nullable": true})));

        // ["string", "number", "null"] -> ["string", "number"], nullable: true (anyOf handled elsewhere)
        let mut o = obj(json!({"type": ["string", "number", "null"]}));
        convert_null_in_types_to_nullable(&mut o);
        assert_eq!(
            o,
            obj(json!({"type": ["string", "number"], "nullable": true}))
        );

        // "string" (no change, not nullable)
        let mut o = obj(json!({"type": "string"}));
        convert_null_in_types_to_nullable(&mut o);
        assert_eq!(o, obj(json!({"type": "string"})));

        // ["string", "number"] (no change, not nullable)
        let mut o = obj(json!({"type": ["string", "number"]}));
        convert_null_in_types_to_nullable(&mut o);
        assert_eq!(o, obj(json!({"type": ["string", "number"]})));

        // object with other properties, ["boolean", "null"]
        let mut o = obj(json!({
            "description": "A test field",
            "type": ["boolean", "null"]
        }));
        convert_null_in_types_to_nullable(&mut o);
        assert_eq!(
            o,
            obj(json!({
                "description": "A test field",
                "type": "boolean",
                "nullable": true
            }))
        );
    }

    #[test]
    fn test_convert_types_to_any_of_defs() {
        // ["string", "number"] -> anyOf with string and number
        let mut o = obj(json!({"type": ["string", "number"]}));
        convert_types_to_any_of_defs(&mut o);
        assert_eq!(
            o,
            obj(json!({
                "anyOf": [
                    {"type": "string"},
                    {"type": "number"}
                ]
            }))
        );

        // "string" (no change)
        let mut o = obj(json!({"type": "string"}));
        convert_types_to_any_of_defs(&mut o);
        assert_eq!(o, obj(json!({"type": "string"})));

        // single-element array (no change, fallback in caller collapses it)
        let mut o = obj(json!({"type": ["string"]}));
        convert_types_to_any_of_defs(&mut o);
        assert_eq!(o, obj(json!({"type": ["string"]})));

        // object with other properties, ["string", "number"]
        let mut o = obj(json!({
            "description": "A test field",
            "type": ["string", "number"]
        }));
        convert_types_to_any_of_defs(&mut o);
        assert_eq!(
            o,
            obj(json!({
                "description": "A test field",
                "anyOf": [
                    {"type": "string"},
                    {"type": "number"}
                ]
            }))
        );

        // anyOf already present (no change)
        let mut o = obj(json!({
            "anyOf": [
                {"type": "string"},
                {"type": "number"}
            ]
        }));
        convert_types_to_any_of_defs(&mut o);
        assert_eq!(
            o,
            obj(json!({
                "anyOf": [
                    {"type": "string"},
                    {"type": "number"}
                ]
            }))
        );

        // both type array and anyOf present
        let mut o = obj(json!({
            "type": ["string", "number"],
            "anyOf": [
                {"format": "email"}
            ]
        }));
        convert_types_to_any_of_defs(&mut o);
        assert_eq!(
            o,
            obj(json!({
                "allOf": [
                    {
                        "anyOf": [
                            {"format": "email"}
                        ]
                    },
                    {
                        "anyOf": [
                            {"type": "string"},
                            {"type": "number"}
                        ]
                    }
                ]
            }))
        );

        // type array + anyOf + pre-existing allOf: pre-existing anyOf must not
        // be silently dropped just because allOf is non-empty.
        let mut o = obj(json!({
            "type": ["string", "number"],
            "anyOf": [{"minLength": 5}],
            "allOf": [{"maxLength": 100}]
        }));
        convert_types_to_any_of_defs(&mut o);
        assert_eq!(
            o,
            obj(json!({
                "allOf": [
                    {"maxLength": 100},
                    {"anyOf": [{"minLength": 5}]},
                    {"anyOf": [{"type": "string"}, {"type": "number"}]}
                ]
            }))
        );
    }

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

        let mut json = json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            },
            "additionalProperties": { "type": "string" },
            "propertyNames": { "pattern": "^[A-Za-z]+$" }
        });

        adapt_to_json_schema_subset(&mut json).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" }
                }
            })
        );
    }

    #[test]
    fn test_transform_null_in_any_of() {
        let mut json = json!({
            "anyOf": [
                { "type": "string" },
                { "type": "null" }
            ]
        });

        adapt_to_json_schema_subset(&mut json).unwrap();

        // Should collapse to the canonical OpenAPI 3.0 form: `nullable: true`
        // as a sibling of `type`, rather than a separate anyOf variant.
        assert_eq!(
            json,
            json!({
                "type": "string",
                "nullable": true
            })
        );
    }

    #[test]
    fn test_transform_null_in_any_of_with_multiple_non_null_variants() {
        let mut json = json!({
            "anyOf": [
                { "type": "string" },
                { "type": "number" },
                { "type": "null" }
            ]
        });

        adapt_to_json_schema_subset(&mut json).unwrap();

        // When more than one non-null variant remains, keep the anyOf and just
        // hoist `nullable: true` onto the parent.
        assert_eq!(
            json,
            json!({
                "nullable": true,
                "anyOf": [
                    { "type": "string" },
                    { "type": "number" }
                ]
            })
        );
    }

    #[test]
    fn test_transform_conflicting_any_of_sources() {
        let mut json = json!({
            "type": ["string", "number"],
            "anyOf": [
                { "minLength": 5 }
            ],
            "oneOf": [
                { "pattern": "^a" },
                { "pattern": "^b" }
            ]
        });

        adapt_to_json_schema_subset(&mut json).unwrap();

        assert_eq!(
            json,
            json!({
                "allOf": [
                    {
                        "anyOf": [
                            { "minLength": 5 },
                        ]
                    },
                    {
                        "anyOf": [
                            {"type": "string"},
                            {"type": "number"}
                        ]
                    },
                    {
                        "anyOf": [
                            { "pattern": "^a" },
                            { "pattern": "^b" }
                        ]
                    }
                ]
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
                        "type": "string",
                        "nullable": true
                    }
                }
            })
        );
    }

    #[test]
    fn test_transform_array_type_to_single_type() {
        let mut json = json!({
            "type": "object",
            "properties": {
                "projectSlugOrId": {
                    "type": ["string", "number"],
                    "description": "Project slug or numeric ID"
                },
                "optionalName": {
                    "type": ["string", "null"],
                    "description": "An optional name"
                }
            }
        });

        adapt_to_json_schema_subset(&mut json).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "object",
                "properties": {
                    "projectSlugOrId": {
                        "anyOf": [
                            {"type": "string"},
                            {"type": "number"}
                        ],
                        "description": "Project slug or numeric ID"
                    },
                    "optionalName": {
                        "type": "string",
                        "description": "An optional name",
                        "nullable": true
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
