use crate::{settings_store::parse_json_with_comments, SettingsAssets};
use anyhow::{anyhow, Context, Result};
use collections::{BTreeMap, HashMap};
use gpui::{Action, AppContext, KeyBinding, SharedString};
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
    schema::{ArrayValidation, InstanceType, Metadata, Schema, SchemaObject, SubschemaValidation},
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

        fn add_deprecation_notice(schema_object: &mut SchemaObject, new_name: &SharedString) {
            schema_object.extensions.insert(
                // deprecationMessage is not part of the JSON Schema spec,
                // but json-language-server recognizes it.
                "deprecationMessage".to_owned(),
                format!("Deprecated, use {new_name}").into(),
            );
        }

        let empty_object: SchemaObject = SchemaObject {
            instance_type: set(InstanceType::Object),
            ..Default::default()
        };

        let mut keymap_action_alternatives = Vec::new();
        for (name, action_schema) in action_schemas.iter() {
            let schema = if let Some(Schema::Object(schema)) = action_schema {
                Some(schema.clone())
            } else {
                None
            };

            // If the type has a description, also apply it to the value. Ideally it would be
            // removed and applied to the overall array, but `json-language-server` does not show
            // these descriptions.
            let description = schema.as_ref().and_then(|schema| {
                schema
                    .metadata
                    .as_ref()
                    .and_then(|metadata| metadata.description.as_ref())
            });
            let mut matches_action_name = SchemaObject {
                const_value: Some(Value::String(name.to_string())),
                ..Default::default()
            };
            if let Some(description) = description {
                matches_action_name.metadata = set(Metadata {
                    description: Some(description.clone()),
                    ..Default::default()
                });
            }

            // Add an alternative for plain action names.
            let deprecation = deprecations.get(name);
            let mut plain_action = SchemaObject {
                instance_type: set(InstanceType::String),
                const_value: Some(Value::String(name.to_string())),
                ..Default::default()
            };
            if let Some(new_name) = deprecation {
                add_deprecation_notice(&mut plain_action, new_name);
            }
            keymap_action_alternatives.push(plain_action.into());

            // When all fields are skipped or an empty struct is added with impl_actions! /
            // impl_actions_as! an empty struct is produced. The action should be invoked without
            // data in this case.
            if let Some(schema) = schema {
                if schema != empty_object {
                    let mut action_with_data = SchemaObject {
                        instance_type: set(InstanceType::Array),
                        array: Some(
                            ArrayValidation {
                                items: set(vec![matches_action_name.into(), schema.into()]),
                                min_items: Some(2),
                                max_items: Some(2),
                                ..Default::default()
                            }
                            .into(),
                        ),
                        ..Default::default()
                    };
                    if let Some(new_name) = deprecation {
                        add_deprecation_notice(&mut action_with_data, new_name);
                    }
                    keymap_action_alternatives.push(action_with_data.into());
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
