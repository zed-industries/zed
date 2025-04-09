mod editable_setting_control;
mod json_schema;
mod key_equivalents;
mod keymap_file;
mod settings_file;
mod settings_store;

use anyhow::Result;
use gpui::App;
use rust_embed::RustEmbed;
use std::{borrow::Cow, collections::BTreeMap, fmt, str};
use util::asset_str;

pub use editable_setting_control::*;
pub use json_schema::*;
pub use key_equivalents::*;
pub use keymap_file::{
    KeyBindingValidator, KeyBindingValidatorRegistration, KeymapFile, KeymapFileLoadResult,
};
pub use settings_file::*;
pub use settings_store::{
    InvalidSettingsError, LocalSettingsKind, Settings, SettingsLocation, SettingsSources,
    SettingsStore, TaskKind, parse_json_with_comments,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct WorktreeId(usize);

impl From<WorktreeId> for usize {
    fn from(value: WorktreeId) -> Self {
        value.0
    }
}

impl WorktreeId {
    pub fn from_usize(handle_id: usize) -> Self {
        Self(handle_id)
    }

    pub fn from_proto(id: u64) -> Self {
        Self(id as usize)
    }

    pub fn to_proto(&self) -> u64 {
        self.0 as u64
    }

    pub fn to_usize(&self) -> usize {
        self.0
    }
}

impl fmt::Display for WorktreeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "settings/*"]
#[include = "keymaps/*"]
#[exclude = "*.DS_Store"]
pub struct SettingsAssets;

pub fn init(cx: &mut App) {
    let mut settings = SettingsStore::new(cx);
    settings
        .set_default_settings(&default_settings(), cx)
        .unwrap();
    cx.set_global(settings);
}

pub fn default_settings() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/default.json")
}

#[cfg(target_os = "macos")]
pub const DEFAULT_KEYMAP_PATH: &str = "keymaps/default-macos.json";

#[cfg(not(target_os = "macos"))]
pub const DEFAULT_KEYMAP_PATH: &str = "keymaps/default-linux.json";

pub fn default_keymap() -> Cow<'static, str> {
    asset_str::<SettingsAssets>(DEFAULT_KEYMAP_PATH)
}

pub const VIM_KEYMAP_PATH: &str = "keymaps/vim.json";

pub fn vim_keymap() -> Cow<'static, str> {
    asset_str::<SettingsAssets>(VIM_KEYMAP_PATH)
}

pub fn initial_user_settings_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_user_settings.json")
}

pub fn initial_server_settings_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_server_settings.json")
}

pub fn initial_project_settings_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_local_settings.json")
}

pub fn initial_keymap_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("keymaps/initial.json")
}

pub fn initial_tasks_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_tasks.json")
}

pub fn initial_debug_tasks_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_debug_tasks.json")
}

fn read_vscode_settings(content: &str) -> Result<BTreeMap<String, String>> {
    let nested: serde_json::Value = settings_store::parse_json_with_comments(content)?;
    fn helper(
        flattened: &mut BTreeMap<String, String>,
        prefix: &mut Vec<String>,
        current: serde_json::Map<String, serde_json::Value>,
    ) -> Result<()> {
        for (k, v) in current {
            if let Ok(map) = serde_json::from_value(v.clone()) {
                prefix.push(k);
                helper(flattened, prefix, map)?;
                prefix.pop();
            } else {
                let existing =
                    flattened.insert(format!("{}.{}", prefix.join("."), k), v.to_string());
                debug_assert!(existing.is_none());
            }
        }
        Ok(())
    }
    let mut prefix = Vec::new();
    let mut flattened = Default::default();
    helper(&mut flattened, &mut prefix, serde_json::from_value(nested)?)?;
    Ok(flattened)
}

#[cfg(test)]
mod tests {
    use collections::BTreeMap;

    use super::*;

    #[test]
    fn test_flatten_vscode_settings() {
        let config = r#"{
            "a": { "b": 1, "c.d": 2, "e.f": {"g.h.i": 3} }
            }"#;
        let expected: BTreeMap<String, String> =
            [("a.b", "1"), ("a.c.d", "2"), ("a.e.f.g.h.i", "3")]
                .iter()
                .map(|&(k, v)| (k.to_owned(), v.to_owned()))
                .collect();
        assert_eq!(expected, read_vscode_settings(config).unwrap());
        assert!(read_vscode_settings("not_a_map").is_err());
    }
}
