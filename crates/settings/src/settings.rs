mod editable_setting_control;
mod json_schema;
mod keymap_file;
mod settings_file;
mod settings_store;

use gpui::AppContext;
use rust_embed::RustEmbed;
use std::{borrow::Cow, str};
use util::asset_str;

pub use editable_setting_control::*;
pub use json_schema::*;
pub use keymap_file::KeymapFile;
pub use settings_file::*;
pub use settings_store::{Settings, SettingsLocation, SettingsSources, SettingsStore};

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "settings/*"]
#[include = "keymaps/*"]
#[exclude = "*.DS_Store"]
pub struct SettingsAssets;

pub fn init(cx: &mut AppContext) {
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

pub fn vim_keymap() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("keymaps/vim.json")
}

pub fn initial_user_settings_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_user_settings.json")
}

pub fn initial_local_settings_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_local_settings.json")
}

pub fn initial_keymap_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("keymaps/initial.json")
}

pub fn initial_tasks_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_tasks.json")
}
