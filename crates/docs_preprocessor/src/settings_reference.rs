//! Generates the body of `docs/src/reference/all-settings.md` from the
//! settings JSON schema (rustdocs on the settings structs) and
//! [`SettingsContent::defaults`].
//!
//! The `{#SETTINGS_REFERENCE#}` placeholder in the docs is replaced with the
//! generated markdown by the mdbook preprocessor.

use serde_json::{Map, Value};
use settings::SettingsContent;
use std::fmt::Write;

/// Custom section titles for settings whose derived title would be wrong or
/// would break existing anchor links from other docs pages.
const TITLE_OVERRIDES: &[(&str, &str)] = &[("proxy", "Network Proxy")];

/// Custom anchors for sections that are linked from other docs pages under a
/// name that differs from the derived one.
const ANCHOR_OVERRIDES: &[(&str, &str)] = &[("terminal-detect-venv", "terminal-detect")];

/// Settings whose default value is too large to inline into the document.
const DEFAULT_SKIPPED: &[(&str, &str)] = &[(
    "languages",
    "a set of per-language overrides for many languages, listed in the [default settings](https://github.com/zed-industries/zed/blob/main/assets/settings/default.json)",
)];

/// Words that should keep a specific capitalization in derived titles.
const TITLE_WORDS: &[(&str, &str)] = &[
    ("ai", "AI"),
    ("api", "API"),
    ("cli", "CLI"),
    ("dap", "DAP"),
    ("jsx", "JSX"),
    ("lsp", "LSP"),
    ("repl", "REPL"),
    ("ssh", "SSH"),
    ("ui", "UI"),
    ("url", "URL"),
    ("wsl", "WSL"),
];

pub fn generate_settings_reference() -> String {
    let schema = schemars::schema_for!(SettingsContent);
    let schema = schema.as_value().clone();
    let defs = match schema.get("$defs") {
        Some(Value::Object(defs)) => defs.clone(),
        _ => Map::new(),
    };
    let defaults = serde_json::to_value(SettingsContent::defaults())
        .expect("failed to serialize default settings");

    let generator = ReferenceGenerator { defs };
    let properties = schema
        .get("properties")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();

    let mut keys = properties.keys().collect::<Vec<_>>();
    keys.sort();

    let mut out = String::new();
    for key in keys {
        generator.render_section(
            &mut out,
            2,
            key,
            &slug(&title_for(key, Some(key))),
            &properties[key],
            defaults.get(key),
            Some(key),
        );
    }
    out
}

struct ReferenceGenerator {
    defs: Map<String, Value>,
}

impl ReferenceGenerator {
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

    fn description<'a>(&'a self, schema: &'a Value) -> Option<&'a str> {
        let mut expanded = Vec::new();
        self.expand(schema, &mut expanded);
        expanded
            .iter()
            .find_map(|node| node.get("description")?.as_str())
    }

    fn properties<'a>(&'a self, schema: &'a Value) -> Option<&'a Map<String, Value>> {
        let mut expanded = Vec::new();
        self.expand(schema, &mut expanded);
        expanded
            .iter()
            .find_map(|node| node.get("properties")?.as_object())
            .filter(|properties| !properties.is_empty())
    }

    /// The enum variants of the schema, as `(value, description)` pairs, if
    /// every non-null branch of the schema is a constant.
    fn enum_variants(&self, schema: &Value) -> Option<Vec<(String, String)>> {
        let mut expanded = Vec::new();
        self.expand(schema, &mut expanded);
        let mut variants = Vec::new();
        for node in expanded {
            if let Some(constant) = node.get("const") {
                variants.push((
                    constant.to_string(),
                    node.get("description")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                ));
            } else if let Some(values) = node.get("enum").and_then(Value::as_array) {
                for value in values {
                    variants.push((value.to_string(), String::new()));
                }
            }
        }
        (!variants.is_empty()).then_some(variants)
    }

    fn type_names(&self, schema: &Value) -> Vec<String> {
        let mut expanded = Vec::new();
        self.expand(schema, &mut expanded);
        let mut names = Vec::new();
        let mut push = |name: &str| {
            if name != "null" && !names.iter().any(|existing| existing == name) {
                names.push(name.to_string());
            }
        };
        for node in expanded {
            match node.get("type") {
                Some(Value::String(name)) => push(name),
                Some(Value::Array(entries)) => {
                    for entry in entries {
                        if let Some(name) = entry.as_str() {
                            push(name);
                        }
                    }
                }
                _ => {}
            }
        }
        names
    }

    fn range_suffix(&self, schema: &Value) -> String {
        let mut expanded = Vec::new();
        self.expand(schema, &mut expanded);
        let minimum = expanded.iter().find_map(|node| node.get("minimum"));
        let maximum = expanded.iter().find_map(|node| node.get("maximum"));
        match (minimum, maximum) {
            (Some(minimum), Some(maximum)) => format!(" from `{minimum}` to `{maximum}`"),
            (Some(minimum), None) => format!(" of at least `{minimum}`"),
            (None, Some(maximum)) => format!(" of at most `{maximum}`"),
            (None, None) => String::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_section(
        &self,
        out: &mut String,
        level: usize,
        key: &str,
        anchor: &str,
        schema: &Value,
        default: Option<&Value>,
        top_level_key: Option<&str>,
    ) {
        let anchor = ANCHOR_OVERRIDES
            .iter()
            .find_map(|(from, to)| (*from == anchor).then_some(*to))
            .unwrap_or(anchor);
        let title = title_for(key, top_level_key);
        let heading = "#".repeat(level);
        let _ = writeln!(out, "{heading} {title} {{#{anchor}}}\n");

        let description = self.description(schema).unwrap_or_default();
        let mut paragraphs = description
            .split("\n\n")
            .filter(|paragraph| !paragraph.trim_start().starts_with("Default:"))
            .map(|paragraph| paragraph.trim())
            .filter(|paragraph| !paragraph.is_empty());
        if let Some(first) = paragraphs.next() {
            let _ = writeln!(out, "- Description: {}", join_lines(first));
        }
        let _ = writeln!(out, "- Setting: `{key}`");

        let skipped_default = top_level_key
            .filter(|top_level_key| *top_level_key == key)
            .and_then(|key| {
                DEFAULT_SKIPPED
                    .iter()
                    .find_map(|(skipped, text)| (*skipped == key).then_some(*text))
            });
        if let Some(text) = skipped_default {
            let _ = writeln!(out, "- Default: {text}");
        } else {
            match default {
                None | Some(Value::Null) => {
                    let _ = writeln!(out, "- Default: `null`");
                }
                Some(value @ (Value::Object(_) | Value::Array(_))) if !is_empty(value) => {
                    if level == 2 {
                        let mut wrapper = Map::new();
                        wrapper.insert(key.to_string(), value.clone());
                        let rendered =
                            serde_json::to_string_pretty(&Value::Object(wrapper)).unwrap();
                        let _ = writeln!(out, "- Default:\n\n```json [settings]\n{rendered}\n```");
                    } else {
                        let rendered = serde_json::to_string(value).unwrap();
                        let _ = writeln!(out, "- Default: `{rendered}`");
                    }
                }
                Some(value) => {
                    let _ = writeln!(out, "- Default: `{value}`");
                }
            }
        }
        let _ = writeln!(out);

        for paragraph in paragraphs {
            let _ = writeln!(out, "{paragraph}\n");
        }

        if let Some(properties) = self.properties(schema) {
            if level < 4 {
                for (child_key, child_schema) in properties {
                    let child_anchor = format!("{anchor}-{}", slug(child_key));
                    self.render_section(
                        out,
                        level + 1,
                        child_key,
                        &child_anchor,
                        child_schema,
                        default.and_then(|default| default.get(child_key)),
                        None,
                    );
                }
            }
            return;
        }

        if let Some(variants) = self.enum_variants(schema) {
            let _ = writeln!(out, "**Options**\n");
            for (value, description) in variants {
                if description.is_empty() {
                    let _ = writeln!(out, "- `{value}`");
                } else {
                    let _ = writeln!(out, "- `{value}`: {}", join_lines(&description));
                }
            }
            let other_types = self.type_names(schema);
            for name in other_types {
                if name != "string" {
                    let _ = writeln!(out, "- any `{name}` value");
                }
            }
            let _ = writeln!(out);
            return;
        }

        let types = self.type_names(schema);
        let summary = match types.as_slice() {
            [] => None,
            [single] => Some(self.type_summary(single, schema)),
            multiple => Some(
                multiple
                    .iter()
                    .map(|name| self.type_summary(name, schema))
                    .collect::<Vec<_>>()
                    .join(", or "),
            ),
        };
        if let Some(summary) = summary {
            let _ = writeln!(out, "**Options**\n\n{summary}\n");
        }
    }

    fn type_summary(&self, type_name: &str, schema: &Value) -> String {
        match type_name {
            "boolean" => String::from("`true` or `false`"),
            "integer" => format!("`integer` values{}", self.range_suffix(schema)),
            "number" => format!("`float` values{}", self.range_suffix(schema)),
            "string" => String::from("`string` values"),
            "array" => String::from("a list of values"),
            "object" => String::from("an object of key-value pairs"),
            other => format!("`{other}` values"),
        }
    }
}

fn is_empty(value: &Value) -> bool {
    match value {
        Value::Object(object) => object.is_empty(),
        Value::Array(items) => items.is_empty(),
        _ => false,
    }
}

fn join_lines(text: &str) -> String {
    text.lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

fn title_for(key: &str, top_level_key: Option<&str>) -> String {
    if let Some(top_level_key) = top_level_key {
        if let Some(title) = TITLE_OVERRIDES
            .iter()
            .find_map(|(overridden, title)| (*overridden == top_level_key).then_some(*title))
        {
            return title.to_string();
        }
    }
    key.split(['_', '.'])
        .map(|word| {
            TITLE_WORDS
                .iter()
                .find_map(|(from, to)| (*from == word).then_some(to.to_string()))
                .unwrap_or_else(|| {
                    let mut chars = word.chars();
                    match chars.next() {
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                        None => String::new(),
                    }
                })
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn slug(text: &str) -> String {
    text.chars()
        .flat_map(|character| match character {
            'A'..='Z' => Some(character.to_ascii_lowercase()),
            'a'..='z' | '0'..='9' | '-' => Some(character),
            ' ' | '_' | '.' => Some('-'),
            _ => None,
        })
        .collect()
}
