use crate::{settings_store::parse_json_with_comments, SettingsAssets};
use anyhow::{anyhow, Context, Result};
use collections::BTreeMap;
use gpui::{Action, AppContext, KeyBinding, SharedString};
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
    schema::{InstanceType, Schema, SchemaObject, SingleOrVec, SubschemaValidation},
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

impl ToString for KeymapAction {
    fn to_string(&self) -> String {
        match &self.0 {
            Value::String(s) => s.clone(),
            Value::Array(arr) => arr
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            _ => self.0.to_string(),
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
        for KeymapBlock { context, bindings } in self.0 {
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
                    .map(|action| KeyBinding::load(&keystroke, action, context.as_deref()))
                })
                .collect::<Result<Vec<_>>>()?;

            cx.bind_keys(bindings);
        }
        Ok(())
    }

    pub fn generate_json_schema(action_names: &[SharedString]) -> serde_json::Value {
        let mut root_schema = SchemaSettings::draft07()
            .with(|settings| settings.option_add_null_type = false)
            .into_generator()
            .into_root_schema_for::<KeymapFile>();

        let action_schema = Schema::Object(SchemaObject {
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(vec![
                    Schema::Object(SchemaObject {
                        instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
                        enum_values: Some(
                            action_names
                                .iter()
                                .map(|name| Value::String(name.to_string()))
                                .collect(),
                        ),
                        ..Default::default()
                    }),
                    Schema::Object(SchemaObject {
                        instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Array))),
                        ..Default::default()
                    }),
                    Schema::Object(SchemaObject {
                        instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Null))),
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        });

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
