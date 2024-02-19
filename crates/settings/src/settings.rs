mod keymap_file;
mod settings_file;
mod settings_store;

use rust_embed::RustEmbed;
use std::{borrow::Cow, str};
use util::asset_str;

pub use keymap_file::KeymapFile;
pub use settings_file::*;
pub use settings_store::{Settings, SettingsJsonSchemaParams, SettingsStore};

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "settings/*"]
#[include = "keymaps/*"]
#[exclude = "*.DS_Store"]
pub struct SettingsAssets;

pub fn default_settings() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/default.json")
}

pub fn default_keymap() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("keymaps/default.json")
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

pub fn initial_runnables_content() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/initial_runnables.json")
}
