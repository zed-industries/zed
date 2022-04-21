use crate::parse_json_with_comments;
use anyhow::{Context, Result};
use assets::Assets;
use collections::BTreeMap;
use gpui::{keymap::Binding, MutableAppContext};
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
    schema::{InstanceType, Schema, SchemaObject, SingleOrVec, SubschemaValidation},
    JsonSchema,
};
use serde::Deserialize;
use serde_json::{value::RawValue, Value};

#[derive(Deserialize, Default, Clone, JsonSchema)]
#[serde(transparent)]
pub struct KeymapFileContent(Vec<KeymapBlock>);

#[derive(Deserialize, Default, Clone, JsonSchema)]
pub struct KeymapBlock {
    #[serde(default)]
    context: Option<String>,
    bindings: BTreeMap<String, KeymapAction>,
}

#[derive(Deserialize, Default, Clone)]
#[serde(transparent)]
pub struct KeymapAction(Box<RawValue>);

impl JsonSchema for KeymapAction {
    fn schema_name() -> String {
        "KeymapAction".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        Schema::Bool(true)
    }
}

#[derive(Deserialize)]
struct ActionWithData(Box<str>, Box<RawValue>);

impl KeymapFileContent {
    pub fn load_defaults(cx: &mut MutableAppContext) {
        for path in ["keymaps/default.json", "keymaps/vim.json"] {
            Self::load(path, cx).unwrap();
        }
    }

    pub fn load(asset_path: &str, cx: &mut MutableAppContext) -> Result<()> {
        let content = Assets::get(asset_path).unwrap().data;
        let content_str = std::str::from_utf8(content.as_ref()).unwrap();
        Ok(parse_json_with_comments::<Self>(content_str)?.add(cx)?)
    }

    pub fn add(self, cx: &mut MutableAppContext) -> Result<()> {
        for KeymapBlock { context, bindings } in self.0 {
            cx.add_bindings(
                bindings
                    .into_iter()
                    .map(|(keystroke, action)| {
                        let action = action.0.get();

                        // This is a workaround for a limitation in serde: serde-rs/json#497
                        // We want to deserialize the action data as a `RawValue` so that we can
                        // deserialize the action itself dynamically directly from the JSON
                        // string. But `RawValue` currently does not work inside of an untagged enum.
                        let action = if action.starts_with('[') {
                            let ActionWithData(name, data) = serde_json::from_str(action)?;
                            cx.deserialize_action(&name, Some(data.get()))
                        } else {
                            let name = serde_json::from_str(action)?;
                            cx.deserialize_action(name, None)
                        }
                        .with_context(|| {
                            format!(
                            "invalid binding value for keystroke {keystroke}, context {context:?}"
                        )
                        })?;
                        Binding::load(&keystroke, action, context.as_deref())
                    })
                    .collect::<Result<Vec<_>>>()?,
            )
        }
        Ok(())
    }
}

pub fn keymap_file_json_schema(action_names: &[&'static str]) -> serde_json::Value {
    let mut root_schema = SchemaSettings::draft07()
        .with(|settings| settings.option_add_null_type = false)
        .into_generator()
        .into_root_schema_for::<KeymapFileContent>();

    let action_schema = Schema::Object(SchemaObject {
        subschemas: Some(Box::new(SubschemaValidation {
            one_of: Some(vec![
                Schema::Object(SchemaObject {
                    instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
                    enum_values: Some(
                        action_names
                            .into_iter()
                            .map(|name| Value::String(name.to_string()))
                            .collect(),
                    ),
                    ..Default::default()
                }),
                Schema::Object(SchemaObject {
                    instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Array))),
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
