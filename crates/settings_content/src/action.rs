use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result};

use collections::HashMap;
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings_macros::MergeFrom;

/// The name of a registered GPUI action, serialized as a plain JSON string, for
/// example, "editor::Cancel"` or `"workspace::CloseActiveItem"`.
///
/// This newtype exists so that settings fields like `command_aliases`, or the
/// keymap file bindings, can request JSON-schema auto completion over the set
/// of actions known at runtime.
#[derive(Serialize, Deserialize, Default, MergeFrom, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct ActionName(String);

/// Small helper function to populate the schema's `deprecationMessage` field with the
/// provided deprecation message.
fn add_deprecation(schema: &mut Schema, message: String) {
    schema.insert("deprecationMessage".into(), Value::String(message));
}

/// Small helper function to populate the schema's `description` field with the
/// provided description.
fn add_description(schema: &mut Schema, description: &str) {
    schema.insert("description".into(), Value::String(description.to_string()));
}

impl ActionName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Build the JSON schema to be used for `$defs/ActionName`, basically an
    /// `anyOf` of all of the available actions with per-action documentation
    /// and deprecation metadata attached.
    pub fn build_schema<'a>(
        action_names: impl IntoIterator<Item = &'a str>,
        action_documentation: &HashMap<&str, &str>,
        deprecations: &HashMap<&str, &str>,
        deprecation_messages: &HashMap<&str, &str>,
    ) -> Schema {
        let mut alternatives = Vec::new();

        for action_name in action_names {
            let mut entry = json_schema!({
                "type": "string",
                "const": action_name
            });

            if let Some(message) = deprecation_messages.get(action_name) {
                add_deprecation(&mut entry, message.to_string());
            } else if let Some(new_name) = deprecations.get(action_name) {
                add_deprecation(&mut entry, format!("Deprecated, use {new_name}"));
            }

            if let Some(description) = action_documentation.get(action_name) {
                add_description(&mut entry, description);
            }

            alternatives.push(entry);
        }

        json_schema!({ "anyOf": alternatives })
    }
}

impl Display for ActionName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{}", self.0)
    }
}

impl AsRef<str> for ActionName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl JsonSchema for ActionName {
    /// The name under which this type should be stored in a generator's `$defs`
    /// map when schemars encounters it during schema generation.
    /// Keeping it stable as `"ActionName"` lets consumers reference it by
    /// `#/$defs/ActionName` and lets [`util::schemars::replace_subschema`] look
    /// it up at runtime to swap in the real schema.
    fn schema_name() -> Cow<'static, str> {
        "ActionName".into()
    }

    /// Returns `true` as a placeholder.
    ///
    /// The real schema, an `anyOf` of every registered action name with action
    /// documentation and deprecation metadata, cannot be produced here because
    /// `JsonSchema::json_schema` receives no runtime context. It is instead
    /// built by call sites that do have access to the GPUI action registry
    /// using [`ActionName::build_schema`].
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!(true)
    }
}

/// A GPUI action together with its input data, serialized as a two-element JSON
/// array of the form `["namespace::Name", { ... }]`, for example,
/// `["pane::ActivateItem", { "index": 0 }]`.
#[derive(Deserialize, Default)]
#[serde(transparent)]
pub struct ActionWithArguments(pub Value);

impl JsonSchema for ActionWithArguments {
    /// The name under which this type should be stored in a generator's `$defs`
    /// map when schemars encounters it during schema generation.
    /// Keeping it stable as `"ActionWithArguments"` lets consumers reference it
    /// by `#/$defs/ActionWithArguments` and lets
    /// [`util::schemars::replace_subschema`] look it up at runtime to swap in
    /// the real schema.
    fn schema_name() -> Cow<'static, str> {
        "ActionWithArguments".into()
    }

    /// Returns `true` as a placeholder.
    ///
    /// The real schema, an `anyOf` of every registered action name that
    /// supports arguments, with action documentation and deprecation metadata,
    /// cannot be produced here because `JsonSchema::json_schema` receives no
    /// runtime context. At the time of writing, it is instead built by
    /// [`KeymapFile::generate_json_schema`], where all of the runtime
    /// information is available.
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_schema_produces_anyof_of_consts_per_name() {
        let mut action_documentation = HashMap::default();
        let mut deprecations = HashMap::default();
        let mut deprecation_messages = HashMap::default();
        action_documentation.insert("editor::Cancel", "Cancel the current operation.");
        deprecations.insert("workspace::CloseCurrentItem", "workspace::CloseActiveItem");
        deprecation_messages.insert("editor::Explode", "DO NOT USE!");

        let schema = ActionName::build_schema(
            [
                "editor::Cancel",
                "editor::Explode",
                "workspace::CloseCurrentItem",
                "workspace::CloseActiveItem",
            ],
            &action_documentation,
            &deprecations,
            &deprecation_messages,
        );

        let value = schema.to_value();
        let values = value
            .pointer("/anyOf")
            .and_then(|v| v.as_array())
            .expect("anyOf should be present");
        assert_eq!(values.len(), 4);

        let (name, schema_type, description) = (
            values[0].get("const").and_then(Value::as_str),
            values[0].get("type").and_then(Value::as_str),
            values[0].get("description").and_then(Value::as_str),
        );
        assert_eq!(name, Some("editor::Cancel"));
        assert_eq!(schema_type, Some("string"));
        assert_eq!(description, Some("Cancel the current operation."));

        let (name, schema_type, message) = (
            values[1].get("const").and_then(Value::as_str),
            values[1].get("type").and_then(Value::as_str),
            values[1].get("deprecationMessage").and_then(Value::as_str),
        );
        assert_eq!(name, Some("editor::Explode"));
        assert_eq!(schema_type, Some("string"));
        assert_eq!(message, Some("DO NOT USE!"));

        let (name, schema_type, message) = (
            values[2].get("const").and_then(Value::as_str),
            values[2].get("type").and_then(Value::as_str),
            values[2].get("deprecationMessage").and_then(Value::as_str),
        );
        assert_eq!(name, Some("workspace::CloseCurrentItem"));
        assert_eq!(schema_type, Some("string"));
        assert_eq!(message, Some("Deprecated, use workspace::CloseActiveItem"));

        let (name, schema_type) = (
            values[3].get("const").and_then(Value::as_str),
            values[3].get("type").and_then(Value::as_str),
        );
        assert_eq!(name, Some("workspace::CloseActiveItem"));
        assert_eq!(schema_type, Some("string"));
    }
}
