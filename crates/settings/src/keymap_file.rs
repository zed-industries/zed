use std::rc::Rc;

use crate::{settings_store::parse_json_with_comments, SettingsAssets};
use anyhow::{anyhow, Result};
use collections::{BTreeMap, HashMap};
use gpui::{Action, AppContext, KeyBinding, KeyBindingContextPredicate, NoAction, SharedString};
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
    schema::{ArrayValidation, InstanceType, Schema, SchemaObject, SubschemaValidation},
    JsonSchema,
};
use serde::Deserialize;
use serde_json::Value;
use std::fmt::Write;
use util::asset_str;

#[derive(Debug, Deserialize, Default, Clone, JsonSchema)]
#[serde(transparent)]
pub struct KeymapFile(Vec<KeymapSection>);

#[derive(Debug, Deserialize, Default, Clone, JsonSchema)]
pub struct KeymapSection {
    #[serde(default)]
    context: String,
    #[serde(default)]
    use_key_equivalents: bool,
    bindings: BTreeMap<String, KeymapAction>,
}

impl KeymapSection {
    pub fn bindings(&self) -> &BTreeMap<String, KeymapAction> {
        &self.bindings
    }
}

/// Keymap action as a JSON value, since it can either be null for no action, or the name of the
/// action, or an array of the name of the action and the action input data.
///
/// Unlike the other deserializable types here, this doc-comment will not be included in the
/// generated JSON schema, as it manually defines its `JsonSchema` impl. The actual schema used for
/// it is automatically generated in `KeymapFile::generate_json_schema`.
#[derive(Debug, Deserialize, Default, Clone)]
#[serde(transparent)]
pub struct KeymapAction(Value);

impl std::fmt::Display for KeymapAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
                let strings: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "{}", strings.join(", "))
            }
            _ => write!(f, "{}", self.0),
        }
    }
}

impl JsonSchema for KeymapAction {
    /// This is used when generating the JSON schema for the `KeymapAction` type, so that it can
    /// reference the keymap action schema.
    fn schema_name() -> String {
        "KeymapAction".into()
    }

    /// This schema will be replaced with the full action schema in
    /// `KeymapFile::generate_json_schema`.
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        Schema::Bool(true)
    }
}

impl KeymapFile {
    pub fn load_asset(asset_path: &str, cx: &mut AppContext) -> Result<()> {
        let content = asset_str::<SettingsAssets>(asset_path);

        Self::parse(content.as_ref())?.add_to_cx(cx)
    }

    pub fn parse(content: &str) -> Result<Self> {
        if content.is_empty() {
            return Ok(Self::default());
        }
        parse_json_with_comments::<Self>(content)
    }

    pub fn add_to_cx(&self, cx: &mut AppContext) -> Result<()> {
        let key_equivalents = crate::key_equivalents::get_key_equivalents(&cx.keyboard_layout());

        // Accumulate errors in order to support partial load of user keymap in the presence of
        // errors in context and binding parsing.
        let mut errors = Vec::new();
        let mut success_count = 0;
        let mut failure_count = 0;

        for KeymapSection {
            context,
            use_key_equivalents,
            bindings,
        } in self.0.iter()
        {
            let context_predicate: Option<Rc<KeyBindingContextPredicate>> = if context.is_empty() {
                None
            } else {
                match KeyBindingContextPredicate::parse(context) {
                    Ok(context_predicate) => Some(context_predicate.into()),
                    Err(err) => {
                        failure_count += bindings.len();
                        errors.push((context, format!("Context parse error: {}", err)));
                        continue;
                    }
                }
            };

            let key_equivalents = if *use_key_equivalents {
                key_equivalents.as_ref()
            } else {
                None
            };

            let mut section_errors = String::new();
            let bindings = bindings
                .iter()
                .map(|(keystrokes, action)| {
                    (
                        keystrokes,
                        Self::load_keybinding(
                            keystrokes,
                            action,
                            context_predicate.clone(),
                            key_equivalents,
                            cx,
                        ),
                    )
                })
                .flat_map(|(keystrokes, result)| {
                    result
                        .inspect_err(|err| {
                            failure_count += 1;
                            write!(section_errors, "\n\n  Binding \"{keystrokes}\": {err}")
                                .unwrap();
                        })
                        .ok()
                })
                .collect::<Vec<_>>();
            if !section_errors.is_empty() {
                errors.push((context, section_errors))
            }

            success_count += bindings.len();
            cx.bind_keys(bindings);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            let mut error_message = format!(
                "Encountered errors in keymap file. \
                Loaded {success_count} bindings, and failed to load {failure_count} bindings.\n\n"
            );
            for (context, section_errors) in errors {
                if context.is_empty() {
                    write!(
                        error_message,
                        "In keymap section without context predicate:"
                    )
                    .unwrap()
                } else {
                    write!(
                        error_message,
                        "In keymap section with context \"{context}\":"
                    )
                    .unwrap()
                }
                write!(error_message, "{section_errors}").unwrap();
            }
            Err(anyhow!(error_message))
        }
    }

    fn load_keybinding(
        keystrokes: &str,
        action: &KeymapAction,
        context: Option<Rc<KeyBindingContextPredicate>>,
        key_equivalents: Option<&HashMap<char, char>>,
        cx: &mut AppContext,
    ) -> Result<KeyBinding> {
        let action = match &action.0 {
            Value::Array(items) => {
                if items.len() != 2 {
                    return Err(anyhow!(
                        "Expected array for an action with input data to have a length of 2."
                    ));
                }
                let serde_json::Value::String(ref name) = items[0] else {
                    return Err(anyhow!(
                        "Expected first item to be the name of an action that takes input data. \
                        Instead got {}",
                        items[0]
                    ));
                };
                cx.build_action(&name, Some(items[1].clone()))
            }
            Value::String(name) => cx.build_action(&name, None),
            Value::Null => Ok(NoAction.boxed_clone()),
            _ => Err(anyhow!("Expected two-element array, got {action:?}")),
        };

        KeyBinding::load(keystrokes, action?, context, key_equivalents)
    }

    pub fn generate_json_schema_for_registered_actions(cx: &mut AppContext) -> Value {
        let mut generator = SchemaSettings::draft07()
            .with(|settings| settings.option_add_null_type = false)
            .into_generator();

        let action_schemas = cx.action_schemas(&mut generator);
        let deprecations = cx.action_deprecations();
        KeymapFile::generate_json_schema(generator, action_schemas, deprecations)
    }

    fn generate_json_schema(
        generator: SchemaGenerator,
        action_schemas: Vec<(SharedString, Option<Schema>)>,
        deprecations: &HashMap<SharedString, SharedString>,
    ) -> serde_json::Value {
        fn set<I, O>(input: I) -> Option<O>
        where
            I: Into<O>,
        {
            Some(input.into())
        }

        fn add_deprecation(schema_object: &mut SchemaObject, message: String) {
            schema_object.extensions.insert(
                // deprecationMessage is not part of the JSON Schema spec,
                // but json-language-server recognizes it.
                "deprecationMessage".to_owned(),
                Value::String(message),
            );
        }

        fn add_deprecation_preferred_name(schema_object: &mut SchemaObject, new_name: &str) {
            add_deprecation(schema_object, format!("Deprecated, use {new_name}"));
        }

        fn add_description(schema_object: &mut SchemaObject, description: String) {
            schema_object
                .metadata
                .get_or_insert(Default::default())
                .description = Some(description);
        }

        let empty_object: SchemaObject = SchemaObject {
            instance_type: set(InstanceType::Object),
            ..Default::default()
        };

        // This is a workaround for a json-language-server issue where it matches the first
        // alternative that matches the value's shape and uses that for documentation.
        //
        // In the case of the array validations, it would even provide an error saying that the name
        // must match the name of the first alternative.
        let mut plain_action = SchemaObject {
            instance_type: set(InstanceType::String),
            const_value: Some(Value::String("".to_owned())),
            ..Default::default()
        };
        let no_action_message = "No action named this.";
        add_description(&mut plain_action, no_action_message.to_owned());
        add_deprecation(&mut plain_action, no_action_message.to_owned());
        let mut matches_action_name = SchemaObject {
            const_value: Some(Value::String("".to_owned())),
            ..Default::default()
        };
        let no_action_message = "No action named this that takes input.";
        add_description(&mut matches_action_name, no_action_message.to_owned());
        add_deprecation(&mut matches_action_name, no_action_message.to_owned());
        let action_with_input = SchemaObject {
            instance_type: set(InstanceType::Array),
            array: set(ArrayValidation {
                items: set(vec![
                    matches_action_name.into(),
                    // Accept any value, as we want this to be the preferred match when there is a
                    // typo in the name.
                    Schema::Bool(true),
                ]),
                min_items: Some(2),
                max_items: Some(2),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut keymap_action_alternatives = vec![plain_action.into(), action_with_input.into()];

        for (name, action_schema) in action_schemas.iter() {
            let schema = if let Some(Schema::Object(schema)) = action_schema {
                Some(schema.clone())
            } else {
                None
            };

            let description = schema.as_ref().and_then(|schema| {
                schema
                    .metadata
                    .as_ref()
                    .and_then(|metadata| metadata.description.clone())
            });

            let deprecation = if name == NoAction.name() {
                Some("null")
            } else {
                deprecations.get(name).map(|new_name| new_name.as_ref())
            };

            // Add an alternative for plain action names.
            let mut plain_action = SchemaObject {
                instance_type: set(InstanceType::String),
                const_value: Some(Value::String(name.to_string())),
                ..Default::default()
            };
            if let Some(new_name) = deprecation {
                add_deprecation_preferred_name(&mut plain_action, new_name);
            }
            if let Some(description) = description.clone() {
                add_description(&mut plain_action, description);
            }
            keymap_action_alternatives.push(plain_action.into());

            // Add an alternative for actions with data specified as a [name, data] array.
            //
            // When a struct with no deserializable fields is added with impl_actions! /
            // impl_actions_as! an empty object schema is produced. The action should be invoked
            // without data in this case.
            if let Some(schema) = schema {
                if schema != empty_object {
                    let mut matches_action_name = SchemaObject {
                        const_value: Some(Value::String(name.to_string())),
                        ..Default::default()
                    };
                    if let Some(description) = description.clone() {
                        add_description(&mut matches_action_name, description.to_string());
                    }
                    if let Some(new_name) = deprecation {
                        add_deprecation_preferred_name(&mut matches_action_name, new_name);
                    }
                    let action_with_input = SchemaObject {
                        instance_type: set(InstanceType::Array),
                        array: set(ArrayValidation {
                            items: set(vec![matches_action_name.into(), schema.into()]),
                            min_items: Some(2),
                            max_items: Some(2),
                            ..Default::default()
                        }),
                        ..Default::default()
                    };
                    keymap_action_alternatives.push(action_with_input.into());
                }
            }
        }

        // Placing null first causes json-language-server to default assuming actions should be
        // null, so place it last.
        keymap_action_alternatives.push(
            SchemaObject {
                instance_type: set(InstanceType::Null),
                ..Default::default()
            }
            .into(),
        );

        let action_schema = SchemaObject {
            subschemas: set(SubschemaValidation {
                one_of: Some(keymap_action_alternatives),
                ..Default::default()
            }),
            ..Default::default()
        }
        .into();

        // The `KeymapSection` schema will reference the `KeymapAction` schema by name, so replacing
        // the definition of `KeymapAction` results in the full action schema being used.
        let mut root_schema = generator.into_root_schema_for::<KeymapFile>();
        root_schema
            .definitions
            .insert(KeymapAction::schema_name(), action_schema);

        // This and other json schemas can be viewed via `debug: open language server logs` ->
        // `json-language-server` -> `Server Info`.
        serde_json::to_value(root_schema).unwrap()
    }

    pub fn sections(&self) -> &[KeymapSection] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::KeymapFile;

    #[test]
    fn can_deserialize_keymap_with_trailing_comma() {
        let json = indoc::indoc! {"[
              // Standard macOS bindings
              {
                \"bindings\": {
                  \"up\": \"menu::SelectPrev\",
                },
              },
            ]
                  "

        };
        KeymapFile::parse(json).unwrap();
    }
}
