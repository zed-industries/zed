//! Contains the [`VimSettings`] and [`VimModeSetting`] settings used to configure Vim mode.
//!
//! These are in their own crate as we want other crates to be able to enable or
//! disable Vim mode without having to depend on the `vim` crate in its
//! entirety.

use std::sync::Arc;

use anyhow::Result;
use collections::HashMap;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

/// Initializes the `vim_settings` crate.
pub fn init(cx: &mut AppContext) {
    VimModeSetting::register(cx);
    VimSettings::register(cx);
}

/// Whether or not to enable Vim mode.
///
/// Default: false
pub struct VimModeSetting(pub bool);

impl Settings for VimModeSetting {
    const KEY: Option<&'static str> = Some("vim_mode");

    type FileContent = Option<bool>;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        Ok(Self(
            sources
                .user
                .or(sources.server)
                .copied()
                .flatten()
                .unwrap_or(sources.default.ok_or_else(Self::missing_default)?),
        ))
    }
}

/// Controls when to use system clipboard.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UseSystemClipboard {
    /// Don't use system clipboard.
    Never,
    /// Use system clipboard.
    Always,
    /// Use system clipboard for yank operations.
    OnYank,
}

#[derive(Deserialize)]
pub struct VimSettings {
    pub toggle_relative_line_numbers: bool,
    pub use_system_clipboard: UseSystemClipboard,
    pub use_multiline_find: bool,
    pub use_smartcase_find: bool,
    pub enable_vim_sneak: bool,
    pub custom_digraphs: HashMap<String, Arc<str>>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct VimSettingsContent {
    pub toggle_relative_line_numbers: Option<bool>,
    pub use_system_clipboard: Option<UseSystemClipboard>,
    pub use_multiline_find: Option<bool>,
    pub use_smartcase_find: Option<bool>,
    pub enable_vim_sneak: Option<bool>,
    pub custom_digraphs: Option<HashMap<String, Arc<str>>>,
}

impl Settings for VimSettings {
    const KEY: Option<&'static str> = Some("vim");

    type FileContent = VimSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}
