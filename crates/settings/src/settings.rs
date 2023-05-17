mod font_size;
mod keymap_file;
mod settings_file;
mod settings_store;

use std::{borrow::Cow, str};

pub use font_size::{adjust_font_size_delta, font_size_for_setting};
use gpui::AssetSource;
pub use keymap_file::{keymap_file_json_schema, KeymapFileContent};
pub use settings_file::*;
pub use settings_store::{Setting, SettingsJsonSchemaParams, SettingsStore};

pub const DEFAULT_SETTINGS_ASSET_PATH: &str = "settings/default.json";
pub const INITIAL_USER_SETTINGS_ASSET_PATH: &str = "settings/initial_user_settings.json";

pub fn initial_user_settings_content(assets: &'static impl AssetSource) -> Cow<'static, str> {
    match assets.load(INITIAL_USER_SETTINGS_ASSET_PATH).unwrap() {
        Cow::Borrowed(s) => Cow::Borrowed(str::from_utf8(s).unwrap()),
        Cow::Owned(s) => Cow::Owned(String::from_utf8(s).unwrap()),
    }
}
