mod keymap_file;
mod settings_file;
mod settings_store;

use gpui::AppContext;
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

pub fn init(cx: &mut AppContext) {
    let mut store = SettingsStore::default();
    store
        .set_default_settings(default_settings().as_ref(), cx)
        .unwrap();
    cx.set_global(store);
}

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
