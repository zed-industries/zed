use anyhow::{Context as _, Result, bail};
use schemars::{JsonSchema, Schema, generate::SchemaSettings};
use serde_json::{Map, Value};

pub fn root_schema_for<T: JsonSchema>() -> Schema {
    SchemaSettings::draft07()
        .with(|settings| {
            settings.meta_schema = None;
            settings.inline_subschemas = true;
        })
        .into_generator()
        .root_schema_for::<T>()
}

/// Removes redundant root metadata, inlines same-document references, and makes
/// empty object inputs explicit.
pub fn normalize_tool_schema(schema: &mut Value) {
    inline_refs(schema);

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

/// Inlines same-document `$ref`s and drops the definitions they came from.
///
/// Providers disagree about references and Google rejects them outright, while
/// context servers routinely ship them: pydantic emits `$defs` for any nested
/// model or enum, and `zod-to-json-schema` emits `definitions` for reused
/// shapes. Schemas built by [`root_schema_for`] never contain references,
/// because it inlines subschemas during generation.
///
/// A schema whose references cannot all be resolved is left exactly as it
/// arrived, so a provider rejects it with a description of what is wrong
/// instead of receiving something half-rewritten.
fn inline_refs(schema: &mut Value) {
    let Some(root) = schema.as_object() else {
        return;
    };
    if !root.contains_key("$defs") && !root.contains_key("definitions") {
        return;
    }

    let mut inlined = schema.clone();
    match inline_refs_fallibly(&mut inlined) {
        Ok(()) => *schema = inlined,
        Err(error) => log::warn!("leaving tool schema references unresolved: {error:#}"),
    }
}

fn inline_refs_fallibly(schema: &mut Value) -> Result<()> {
    let Some(root) = schema.as_object_mut() else {
        return Ok(());
    };
    let defs = root.remove("$defs");
    let legacy_defs = root.remove("definitions");

    inline_refs_recursive(schema, defs.as_ref(), legacy_defs.as_ref(), &mut Vec::new())
}

fn inline_refs_recursive(
    value: &mut Value,
    defs: Option<&Value>,
    legacy_defs: Option<&Value>,
    visiting: &mut Vec<String>,
) -> Result<()> {
    match value {
        Value::Object(object) => {
            if let Some(reference) = object.get("$ref").and_then(Value::as_str) {
                // A self-referential schema, such as a tree node whose children
                // are trees, cannot be inlined to any finite depth; an empty
                // schema accepts anything, which is the closest available
                // approximation.
                if visiting.iter().any(|visited| visited == reference) {
                    *object = Map::new();
                    return Ok(());
                }

                let reference = reference.to_string();
                let (definitions_key, name) = parse_ref(&reference)?;
                let definition = match definitions_key {
                    "$defs" => defs,
                    _ => legacy_defs,
                }
                .and_then(|definitions| definitions.get(name))
                .with_context(|| format!("no {definitions_key} entry for {reference}"))?;

                let mut resolved = definition.clone();
                if let Value::Object(resolved) = &mut resolved {
                    for (key, sibling) in object.iter() {
                        if key != "$ref" {
                            resolved.insert(key.clone(), sibling.clone());
                        }
                    }
                }
                *value = resolved;

                visiting.push(reference);
                let result = inline_refs_recursive(value, defs, legacy_defs, visiting);
                visiting.pop();
                return result;
            }

            for child in object.values_mut() {
                inline_refs_recursive(child, defs, legacy_defs, visiting)?;
            }
        }
        Value::Array(items) => {
            for item in items {
                inline_refs_recursive(item, defs, legacy_defs, visiting)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn parse_ref(reference: &str) -> Result<(&'static str, &str)> {
    if let Some(name) = reference.strip_prefix("#/$defs/") {
        return Ok(("$defs", name));
    }
    if let Some(name) = reference.strip_prefix("#/definitions/") {
        return Ok(("definitions", name));
    }
    bail!("only `#/$defs/<name>` and `#/definitions/<name>` are supported, got {reference}")
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
    fn inlines_the_references_context_servers_generate() {
        // The shape pydantic produces for a tool taking a nested model with an
        // enum field, which the MCP Python SDK derives from a typed handler.
        let mut schema = json!({
            "$defs": {
                "Filter": {
                    "type": "object",
                    "properties": { "severity": { "$ref": "#/$defs/Severity" } }
                },
                "Severity": { "type": "string", "enum": ["low", "high"] }
            },
            "type": "object",
            "properties": {
                "filter": { "anyOf": [{ "$ref": "#/$defs/Filter" }, { "type": "null" }] }
            }
        });

        normalize_tool_schema(&mut schema);

        assert_eq!(
            schema,
            json!({
                "type": "object",
                "properties": {
                    "filter": {
                        "anyOf": [
                            {
                                "type": "object",
                                "properties": {
                                    "severity": { "type": "string", "enum": ["low", "high"] }
                                }
                            },
                            { "type": "null" }
                        ]
                    }
                }
            })
        );
    }

    #[test]
    fn inlines_legacy_definitions_and_keeps_sibling_keywords() {
        let mut schema = json!({
            "definitions": { "Glob": { "type": "string" } },
            "type": "object",
            "properties": {
                "glob": { "$ref": "#/definitions/Glob", "description": "a pattern" }
            }
        });

        normalize_tool_schema(&mut schema);

        assert_eq!(
            schema["properties"]["glob"],
            json!({ "type": "string", "description": "a pattern" })
        );
        assert_eq!(schema.get("definitions"), None);
    }

    #[test]
    fn replaces_a_recursive_reference_with_an_unconstrained_schema() {
        let mut schema = json!({
            "$defs": {
                "Node": {
                    "type": "object",
                    "properties": { "child": { "$ref": "#/$defs/Node" } }
                }
            },
            "type": "object",
            "properties": { "root": { "$ref": "#/$defs/Node" } }
        });

        normalize_tool_schema(&mut schema);

        assert_eq!(
            schema["properties"]["root"],
            json!({ "type": "object", "properties": { "child": {} } })
        );
    }

    #[test]
    fn leaves_a_schema_untouched_when_a_reference_cannot_be_resolved() {
        let unresolvable = json!({
            "$defs": { "Known": { "type": "string" } },
            "type": "object",
            "properties": { "value": { "$ref": "#/$defs/Missing" } }
        });
        let mut schema = unresolvable.clone();

        normalize_tool_schema(&mut schema);

        assert_eq!(schema, unresolvable);
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
