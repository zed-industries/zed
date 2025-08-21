//! Contains the [`VimModeSetting`] and [`HelixModeSetting`] used to enable/disable Vim and Helix modes.
//!
//! This is in its own crate as we want other crates to be able to enable or
//! disable Vim/Helix modes without having to depend on the `vim` crate in its
//! entirety.

use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

/// Initializes the `vim_mode_setting` crate.
pub fn init(cx: &mut App) {
    EditorModeSetting::register(cx);
}

/// Whether or not to enable Vim mode.
///
/// Default: `EditMode::Default`
pub struct EditorModeSetting(pub EditorMode);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
pub enum EditorMode {
    Vim,
    VimInsert,
    Helix,
    #[default]
    Default,
}

impl Settings for EditorModeSetting {
    const KEY: Option<&'static str> = Some("editor_mode");

    type FileContent = Option<EditorMode>;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        Ok(Self(
            sources
                .user
                .or(sources.server)
                .copied()
                .flatten()
                .unwrap_or(sources.default.ok_or_else(Self::missing_default)?),
        ))
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {
        // TODO: could possibly check if any of the `vim.<foo>` keys are set?
    }
}
