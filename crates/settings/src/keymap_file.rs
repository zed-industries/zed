use crate::{settings_store::parse_json_with_comments, SettingsAssets};
use anyhow::{anyhow, Context, Result};
use collections::{BTreeMap, HashMap};
use gpui::{Action, AppContext, KeyBinding, NoAction, SharedString};
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
    schema::{ArrayValidation, InstanceType, Schema, SchemaObject, SubschemaValidation},
    JsonSchema,
};
use serde::Deserialize;
use serde_json::Value;
use util::{asset_str, ResultExt};

#[derive(Debug, Deserialize, Default, Clone, JsonSchema)]
#[serde(transparent)]
pub struct KeymapFile(Vec<KeymapBlock>);

#[derive(Debug, Deserialize, Default, Clone, JsonSchema)]
pub struct KeymapBlock {
    #[serde(default)]
    context: Option<String>,
    #[serde(default)]
    use_key_equivalents: Option<bool>,
    bindings: BTreeMap<String, KeymapAction>,
}

impl KeymapBlock {
    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }

    pub fn bindings(&self) -> &BTreeMap<String, KeymapAction> {
        &self.bindings
    }
}

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
    fn schema_name() -> String {
        "KeymapAction".into()
    }

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

    pub fn add_to_cx(self, cx: &mut AppContext) -> Result<()> {
        let key_equivalents = crate::key_equivalents::get_key_equivalents(&cx.keyboard_layout());

        for KeymapBlock {
            context,
            use_key_equivalents,
            bindings,
        } in self.0
        {
            let bindings = bindings
                .into_iter()
                .filter_map(|(keystroke, action)| {
                    let action = action.0;

                    // This is a workaround for a limitation in serde: serde-rs/json#497
                    // We want to deserialize the action data as a `RawValue` so that we can
                    // deserialize the action itself dynamically directly from the JSON
                    // string. But `RawValue` currently does not work inside of an untagged enum.
                    match action {
                        Value::Array(items) => {
                            let Ok([name, data]): Result<[serde_json::Value; 2], _> =
                                items.try_into()
                            else {
                                return Some(Err(anyhow!("Expected array of length 2")));
                            };
                            let serde_json::Value::String(name) = name else {
                                return Some(Err(anyhow!(
                                    "Expected first item in array to be a string."
                                )));
                            };
                            cx.build_action(&name, Some(data))
                        }
                        Value::String(name) => cx.build_action(&name, None),
                        Value::Null => Ok(no_action()),
                        _ => {
                            return Some(Err(anyhow!("Expected two-element array, got {action:?}")))
                        }
                    }
                    .with_context(|| {
                        format!(
                            "invalid binding value for keystroke {keystroke}, context {context:?}"
                        )
                    })
                    .log_err()
                    .map(|action| {
                        KeyBinding::load(
                            &keystroke,
                            action,
                            context.as_deref(),
                            if use_key_equivalents.unwrap_or_default() {
                                key_equivalents.as_ref()
                            } else {
                                None
                            },
                        )
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            cx.bind_keys(bindings);
        }
        Ok(())
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

        let mut root_schema = generator.into_root_schema_for::<KeymapFile>();
        root_schema
            .definitions
            .insert("KeymapAction".to_owned(), action_schema);

        // This and other json schemas can be viewed via `debug: open language server logs` ->
        // `json-language-server` -> `Server Info`.
        serde_json::to_value(root_schema).unwrap()
    }

    pub fn blocks(&self) -> &[KeymapBlock] {
        &self.0
    }
}

fn no_action() -> Box<dyn gpui::Action> {
    gpui::NoAction.boxed_clone()
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
