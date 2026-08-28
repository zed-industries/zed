use schemars::{JsonSchema, Schema, generate::SchemaSettings};
use serde_json::Value;

pub fn root_schema_for<T: JsonSchema>() -> Schema {
    SchemaSettings::draft07()
        .with(|settings| {
            settings.meta_schema = None;
            settings.inline_subschemas = true;
        })
        .into_generator()
        .root_schema_for::<T>()
}

/// Removes redundant root metadata and makes empty object inputs explicit.
pub fn normalize_tool_schema(schema: &mut Value) {
    let Value::Object(object) = schema else {
        return;
    };

    object.remove("$schema");
    object.remove("title");
    object.remove("description");

    if object.get("type").and_then(Value::as_str) == Some("object") {
        object
            .entry("properties")
            .or_insert_with(|| Value::Object(Default::default()));
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn normalizes_tool_schema_without_changing_validation_keywords() {
        let mut schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "Search input",
            "description": "Searches files",
            "type": "object",
            "additionalProperties": true,
            "if": { "required": ["query"] },
            "then": { "required": ["limit"] }
        });

        normalize_tool_schema(&mut schema);

        assert_eq!(
            schema,
            json!({
                "type": "object",
                "properties": {},
                "additionalProperties": true,
                "if": { "required": ["query"] },
                "then": { "required": ["limit"] }
            })
        );
    }

    #[test]
    fn normalization_is_idempotent() {
        let mut schema = json!({
            "title": "Empty input",
            "type": "object"
        });

        normalize_tool_schema(&mut schema);
        let normalized = schema.clone();
        normalize_tool_schema(&mut schema);

        assert_eq!(schema, normalized);
    }

    #[test]
    fn normalization_leaves_non_object_schemas_unchanged_except_for_metadata() {
        let mut schema = json!({
            "title": "Text input",
            "description": "Accepts text",
            "type": "string"
        });

        normalize_tool_schema(&mut schema);

        assert_eq!(schema, json!({ "type": "string" }));
    }
}
