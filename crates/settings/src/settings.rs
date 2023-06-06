mod keymap_file;
mod settings_file;
mod settings_store;

use gpui::AssetSource;
pub use keymap_file::{keymap_file_json_schema, KeymapFileContent};
pub use settings_file::*;
pub use settings_store::{Setting, SettingsJsonSchemaParams, SettingsStore};
use std::{borrow::Cow, str};

pub const DEFAULT_SETTINGS_ASSET_PATH: &str = "settings/default.json";
const INITIAL_USER_SETTINGS_ASSET_PATH: &str = "settings/initial_user_settings.json";
const INITIAL_LOCAL_SETTINGS_ASSET_PATH: &str = "settings/initial_local_settings.json";

pub fn default_settings() -> Cow<'static, str> {
    asset_str(&assets::Assets, DEFAULT_SETTINGS_ASSET_PATH)
}

pub fn initial_user_settings_content(assets: &dyn AssetSource) -> Cow<'_, str> {
    asset_str(assets, INITIAL_USER_SETTINGS_ASSET_PATH)
}

pub fn initial_local_settings_content(assets: &dyn AssetSource) -> Cow<'_, str> {
    asset_str(assets, INITIAL_LOCAL_SETTINGS_ASSET_PATH)
}

fn asset_str<'a>(assets: &'a dyn AssetSource, path: &str) -> Cow<'a, str> {
    match assets.load(path).unwrap() {
        Cow::Borrowed(s) => Cow::Borrowed(str::from_utf8(s).unwrap()),
        Cow::Owned(s) => Cow::Owned(String::from_utf8(s).unwrap()),
    }
}
