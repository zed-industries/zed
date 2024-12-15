use std::sync::Arc;

use anyhow::Result;
use collections::HashMap;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_derive::Serialize;
use settings::{Settings, SettingsSources};

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
    pub custom_digraphs: HashMap<String, Arc<str>>,
    pub highlight_on_yank_duration: u64,
    pub mode_indicator: ModeIndicatorSettings,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct VimSettingsContent {
    pub toggle_relative_line_numbers: Option<bool>,
    pub use_system_clipboard: Option<UseSystemClipboard>,
    pub use_multiline_find: Option<bool>,
    pub use_smartcase_find: Option<bool>,
    pub custom_digraphs: Option<HashMap<String, Arc<str>>>,
    pub highlight_on_yank_duration: Option<u64>,
    pub mode_indicator: Option<ModeIndicatorSettings>,
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModeIndicatorLocation {
    /// Show the vim mode indicator on the left side of the status bar.
    Left,
    /// Show the vim mode indicator on the right side of the status bar.
    #[default]
    Right,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ModeIndicatorSettings {
    pub location: Option<ModeIndicatorLocation>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ModeIndicatorSettingsContent {
    /// Whether to show the vim mode indicator to the left or to the right in the status bar.
    pub location: Option<Option<ModeIndicatorLocation>>,
}

impl Settings for VimSettings {
    const KEY: Option<&'static str> = Some("vim");

    type FileContent = VimSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}

impl Settings for ModeIndicatorSettings {
    const KEY: Option<&'static str> = Some("mode_indicator");

    type FileContent = ModeIndicatorSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}
