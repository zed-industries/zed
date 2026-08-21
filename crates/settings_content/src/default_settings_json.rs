//! Generates `assets/settings/default.json` from [`UserSettingsContent::defaults`].
//!
//! The Rust settings structs are the single source of truth for Zed's default
//! settings. The JSONC file shipped in the assets is a generated artifact:
//! values come from [`UserSettingsContent::defaults`], and the `//` comments
//! come from the rustdoc of the corresponding fields (via the schemars-emitted
//! `description` properties in the JSON schema).
//!
//! Regenerate the file with `script/generate-default-settings`. A test in this
//! module fails whenever the checked-in file drifts from the generated output.

use anyhow::{Context as _, Result, anyhow};
use serde_json::{Map, Value};

use crate::{SettingsContent, UserSettingsContent};

const SCHEMA_KEY: &str = "$schema";
const SCHEMA_URL: &str = "zed://schemas/settings";
const PRINT_WIDTH: usize = 120;
const INDENT: &str = "  ";

/// Keys that are serialized as absent (their Rust default is `None`), but that
/// the generated file should still show with an explicit `null`, to document
/// that the setting exists and defaults to null.
const EXPLICIT_NULL_PATHS: &[&[&str]] = &[
    &["buffer_font_fallbacks"],
    &["ui_font_fallbacks"],
    &["agent_ui_font_family"],
    &["agent_ui_font_size"],
    &["agent_buffer_font_family"],
    &["markdown_preview_font_size"],
    &["markdown_preview_font_family"],
    &["markdown_preview_code_font_family"],
    &["max_tabs"],
    &["audio", "experimental.output_audio_device"],
    &["audio", "experimental.input_audio_device"],
    &["minimap", "current_line_highlight"],
    &["project_panel", "scrollbar", "show"],
    &["outline_panel", "scrollbar", "show"],
    &["diagnostics", "inline", "max_severity"],
    &["edit_predictions", "copilot", "enterprise_uri"],
    &["edit_predictions", "copilot", "proxy"],
    &["edit_predictions", "copilot", "proxy_no_verify"],
    &["terminal", "scrollbar", "show"],
    &["node", "path"],
    &["node", "npm_path"],
];

/// The release-channel and platform override sections at the end of the file.
/// They contain sparse overlays, so keys whose serialized value matches an
/// empty [`SettingsContent`] are pruned from them.
const OVERRIDE_SECTIONS: &[&str] = &[
    "preview", "nightly", "stable", "dev", "linux", "macos", "windows",
];

/// Generates the full JSONC text of `assets/settings/default.json`.
pub fn generate_default_settings_json() -> Result<String> {
    let schema = schemars::schema_for!(UserSettingsContent);
    let schema = schema.as_value().clone();
    let defs = match schema.get("$defs") {
        Some(Value::Object(defs)) => defs.clone(),
        _ => Map::new(),
    };

    let mut value =
        serde_json::to_value(UserSettingsContent::defaults()).context("serializing defaults")?;
    let root = value
        .as_object_mut()
        .ok_or_else(|| anyhow!("defaults did not serialize to an object"))?;
    prune_override_sections(root)?;
    insert_explicit_nulls(root)?;

    let emitter = Emitter { defs };
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(INDENT);
    out.push_str(&format!("{:?}: {:?},\n", SCHEMA_KEY, SCHEMA_URL));
    emitter.emit_object_body(root, &schema, 1, true, &mut out);
    out.push_str("}\n");
    Ok(out)
}

fn prune_override_sections(root: &mut Map<String, Value>) -> Result<()> {
    let empty = serde_json::to_value(SettingsContent::default())
        .context("serializing empty SettingsContent")?;
    let empty = empty
        .as_object()
        .ok_or_else(|| anyhow!("empty SettingsContent did not serialize to an object"))?;
    for section in OVERRIDE_SECTIONS {
        if let Some(Value::Object(overrides)) = root.get_mut(*section) {
            overrides.retain(|key, value| empty.get(key) != Some(value));
        }
    }
    Ok(())
}

fn insert_explicit_nulls(root: &mut Map<String, Value>) -> Result<()> {
    for path in EXPLICIT_NULL_PATHS {
        let (leaf, parents) = path
            .split_last()
            .ok_or_else(|| anyhow!("empty explicit null path"))?;
        let mut object = &mut *root;
        for segment in parents {
            object = object
                .get_mut(*segment)
                .and_then(Value::as_object_mut)
                .ok_or_else(|| {
                    anyhow!(
                        "explicit null path not found in defaults: {}",
                        path.join("/")
                    )
                })?;
        }
        if object.insert(leaf.to_string(), Value::Null).is_some() {
            return Err(anyhow!(
                "explicit null path already has a value: {}",
                path.join("/")
            ));
        }
    }
    Ok(())
}

struct Emitter {
    defs: Map<String, Value>,
}

impl Emitter {
    /// Collects the schema node and, transitively, every subschema reachable
    /// through `$ref`, `anyOf`, `oneOf`, and `allOf`.
    fn expand<'a>(&'a self, schema: &'a Value, out: &mut Vec<&'a Value>) {
        if out.len() > 32 {
            return;
        }
        if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
            if let Some(name) = reference.strip_prefix("#/$defs/") {
                if let Some(definition) = self.defs.get(name) {
                    self.expand(definition, out);
                }
            }
        }
        out.push(schema);
        for combinator in ["anyOf", "oneOf", "allOf"] {
            if let Some(subschemas) = schema.get(combinator).and_then(Value::as_array) {
                for subschema in subschemas {
                    self.expand(subschema, out);
                }
            }
        }
    }

    fn property_schema<'a>(&'a self, schema: &'a Value, key: &str) -> Option<&'a Value> {
        let mut expanded = Vec::new();
        self.expand(schema, &mut expanded);
        expanded
            .iter()
            .find_map(|node| node.get("properties")?.get(key))
    }

    fn additional_properties_schema<'a>(&'a self, schema: &'a Value) -> Option<&'a Value> {
        let mut expanded = Vec::new();
        self.expand(schema, &mut expanded);
        expanded.iter().find_map(|node| {
            let additional = node.get("additionalProperties")?;
            additional.is_object().then_some(additional)
        })
    }

    fn item_schema<'a>(&'a self, schema: &'a Value) -> Option<&'a Value> {
        let mut expanded = Vec::new();
        self.expand(schema, &mut expanded);
        expanded
            .iter()
            .find_map(|node| node.get("items").filter(|items| items.is_object()))
    }

    /// Whether this object schema describes a map (no fixed `properties`).
    fn is_map(&self, schema: &Value) -> bool {
        let mut expanded = Vec::new();
        self.expand(schema, &mut expanded);
        !expanded.iter().any(|node| {
            node.get("properties")
                .and_then(Value::as_object)
                .is_some_and(|properties| !properties.is_empty())
        })
    }

    /// The order in which to emit the keys of an object: schema property
    /// order first, then any remaining keys in serialization order.
    fn ordered_keys(&self, object: &Map<String, Value>, schema: Option<&Value>) -> Vec<String> {
        let mut keys = Vec::with_capacity(object.len());
        if let Some(schema) = schema {
            if self.is_map(schema) {
                keys.extend(object.keys().cloned());
                keys.sort();
                return keys;
            }
            let mut expanded = Vec::new();
            self.expand(schema, &mut expanded);
            for node in expanded {
                if let Some(properties) = node.get("properties").and_then(Value::as_object) {
                    for key in properties.keys() {
                        if object.contains_key(key) && !keys.contains(key) {
                            keys.push(key.clone());
                        }
                    }
                }
            }
        }
        for key in object.keys() {
            if !keys.contains(key) {
                keys.push(key.clone());
            }
        }
        keys
    }

    fn description<'a>(&'a self, schema: &'a Value) -> Option<&'a str> {
        let mut expanded = Vec::new();
        self.expand(schema, &mut expanded);
        expanded
            .iter()
            .find_map(|node| node.get("description")?.as_str())
    }

    fn emit_object_body(
        &self,
        object: &Map<String, Value>,
        schema: &Value,
        depth: usize,
        comments: bool,
        out: &mut String,
    ) {
        let comments = comments && !self.is_map(schema);
        let indent = INDENT.repeat(depth);
        for key in self.ordered_keys(object, Some(schema)) {
            let value = &object[&key];
            let property = self
                .property_schema(schema, &key)
                .or_else(|| self.additional_properties_schema(schema));
            if comments {
                if let Some(description) = property.and_then(|property| self.description(property))
                {
                    let first_paragraph = description.split("\n\n").next().unwrap_or_default();
                    for line in first_paragraph.lines() {
                        out.push_str(&indent);
                        out.push_str("//");
                        if !line.is_empty() {
                            out.push(' ');
                        }
                        out.push_str(line);
                        out.push('\n');
                    }
                }
            }
            out.push_str(&indent);
            let rendered_key = format!("{:?}: ", key);
            out.push_str(&rendered_key);
            self.emit_value(
                value,
                property,
                depth,
                indent.len() + rendered_key.len(),
                comments,
                out,
            );
            out.push_str(",\n");
        }
    }

    fn emit_value(
        &self,
        value: &Value,
        schema: Option<&Value>,
        depth: usize,
        line_prefix_len: usize,
        comments: bool,
        out: &mut String,
    ) {
        match value {
            Value::Object(object) if !object.is_empty() => {
                if !comments {
                    if let Some(inline) = self.render_inline(value) {
                        if line_prefix_len + inline.len() < PRINT_WIDTH {
                            out.push_str(&inline);
                            return;
                        }
                    }
                }
                out.push_str("{\n");
                match schema {
                    Some(schema) => self.emit_object_body(object, schema, depth + 1, comments, out),
                    None => self.emit_object_body(object, &Value::Null, depth + 1, false, out),
                }
                out.push_str(&INDENT.repeat(depth));
                out.push('}');
            }
            Value::Array(items) if !items.is_empty() => {
                if let Some(inline) = self.render_inline(value) {
                    if line_prefix_len + inline.len() < PRINT_WIDTH {
                        out.push_str(&inline);
                        return;
                    }
                }
                let item_schema = schema.and_then(|schema| self.item_schema(schema));
                let indent = INDENT.repeat(depth + 1);
                out.push_str("[\n");
                for item in items {
                    out.push_str(&indent);
                    self.emit_value(item, item_schema, depth + 1, indent.len(), false, out);
                    out.push_str(",\n");
                }
                out.push_str(&INDENT.repeat(depth));
                out.push(']');
            }
            _ => out.push_str(&render_scalar(value)),
        }
    }

    /// Renders a value on a single line, or `None` if it contains nothing that
    /// would benefit (empty containers and scalars always render inline).
    fn render_inline(&self, value: &Value) -> Option<String> {
        match value {
            Value::Object(object) => {
                if object.is_empty() {
                    return Some(String::from("{}"));
                }
                let mut parts = Vec::with_capacity(object.len());
                for (key, value) in object {
                    parts.push(format!("{:?}: {}", key, self.render_inline(value)?));
                }
                Some(format!("{{ {} }}", parts.join(", ")))
            }
            Value::Array(items) => {
                if items.is_empty() {
                    return Some(String::from("[]"));
                }
                let items = items
                    .iter()
                    .map(|item| self.render_inline(item))
                    .collect::<Option<Vec<_>>>()?;
                Some(format!("[{}]", items.join(", ")))
            }
            _ => {
                let rendered = render_scalar(value);
                (!rendered.contains('\n')).then_some(rendered)
            }
        }
    }
}

fn render_scalar(value: &Value) -> String {
    match value {
        Value::Number(number) => {
            if let Some(float) = number.as_f64() {
                if number.as_u64().is_none()
                    && number.as_i64().is_none()
                    && float.is_finite()
                    && float.fract() == 0.0
                    && float.abs() < 9e15
                {
                    return format!("{}", float as i64);
                }
            }
            number.to_string()
        }
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use settings_json::parse_json_with_comments;

    #[test]
    fn generated_default_settings_json_is_up_to_date() {
        let generated = generate_default_settings_json().unwrap();
        let checked_in = include_str!("../../../assets/settings/default.json");
        if generated == checked_in {
            return;
        }
        let diff_line = generated
            .lines()
            .zip(checked_in.lines())
            .position(|(generated, checked_in)| generated != checked_in)
            .map(|index| {
                format!(
                    "first difference at line {}:\n  generated:  {}\n  checked in: {}",
                    index + 1,
                    generated.lines().nth(index).unwrap_or(""),
                    checked_in.lines().nth(index).unwrap_or("")
                )
            })
            .unwrap_or_else(|| String::from("files differ in length"));
        panic!(
            "assets/settings/default.json is out of date; run script/generate-default-settings to regenerate it.\n{diff_line}"
        );
    }

    #[test]
    fn generated_default_settings_json_round_trips() {
        let generated = generate_default_settings_json().unwrap();
        let reparsed: UserSettingsContent = parse_json_with_comments(&generated).unwrap();
        assert_eq!(reparsed, UserSettingsContent::defaults());
    }
}
