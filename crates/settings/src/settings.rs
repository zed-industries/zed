mod editable_setting_control;
mod keymap_file;
mod settings_file;
mod settings_store;

use gpui::AppContext;
use rust_embed::RustEmbed;
use std::{borrow::Cow, str};
use util::asset_str;

pub use editable_setting_control::*;
pub use keymap_file::KeymapFile;
pub use settings_file::*;
pub use settings_store::{
    Settings, SettingsJsonSchemaParams, SettingsLocation, SettingsSources, SettingsStore,
};

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

pub fn init_font_fallbacks(cx: &mut AppContext) {
    FontFamilies::register(cx);
}

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FontFamiliesContent {
    ui_font_family: Option<Vec<String>>,
    buffer_font_family: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct FontFamilies {
    pub ui_font_family: Vec<String>,
    pub buffer_font_family: Vec<String>,
}

impl Settings for FontFamilies {
    const KEY: Option<&'static str> = None;

    type FileContent = FontFamiliesContent;

    fn load(
        sources: crate::SettingsSources<Self::FileContent>,
        _: &mut AppContext,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            ui_font_family: sources
                .user
                .and_then(|fallbacks| fallbacks.ui_font_family.clone())
                .or_else(|| sources.default.ui_font_family.clone())
                .unwrap_or_default(),
            buffer_font_family: sources
                .user
                .and_then(|fallbacks| fallbacks.buffer_font_family.clone())
                .or_else(|| sources.default.buffer_font_family.clone())
                .unwrap_or_default(),
        })
    }
}
