use anyhow::{Context, Result};
use assets::Assets;
use collections::BTreeMap;
use gpui::{keymap::Binding, MutableAppContext};
use serde::Deserialize;
use serde_json::value::RawValue;

#[derive(Deserialize, Default, Clone)]
#[serde(transparent)]
pub struct KeymapFile(BTreeMap<String, ActionsByKeystroke>);

type ActionsByKeystroke = BTreeMap<String, Box<RawValue>>;

#[derive(Deserialize)]
struct ActionWithData<'a>(#[serde(borrow)] &'a str, #[serde(borrow)] &'a RawValue);

impl KeymapFile {
    pub fn load_defaults(cx: &mut MutableAppContext) {
        for path in ["keymaps/default.json", "keymaps/vim.json"] {
            Self::load(path, cx).unwrap();
        }
    }

    pub fn load(asset_path: &str, cx: &mut MutableAppContext) -> Result<()> {
        let content = Assets::get(asset_path).unwrap().data;
        let content_str = std::str::from_utf8(content.as_ref()).unwrap();
        Ok(serde_json::from_str::<Self>(content_str)?.add(cx)?)
    }

    pub fn add(self, cx: &mut MutableAppContext) -> Result<()> {
        for (context, actions) in self.0 {
            let context = if context == "*" { None } else { Some(context) };
            cx.add_bindings(
                actions
                    .into_iter()
                    .map(|(keystroke, action)| {
                        let action = action.get();

                        // This is a workaround for a limitation in serde: serde-rs/json#497
                        // We want to deserialize the action data as a `RawValue` so that we can
                        // deserialize the action itself dynamically directly from the JSON
                        // string. But `RawValue` currently does not work inside of an untagged enum.
                        let action = if action.starts_with('[') {
                            let ActionWithData(name, data) = serde_json::from_str(action)?;
                            cx.deserialize_action(name, Some(data.get()))
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
