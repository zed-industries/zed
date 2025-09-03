//! Contains the [`VimModeSetting`] and [`HelixModeSetting`] used to enable/disable Vim and Helix modes.
//!
//! This is in its own crate as we want other crates to be able to enable or
//! disable Vim/Helix modes without having to depend on the `vim` crate in its
//! entirety.

use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};

/// Initializes the `vim_mode_setting` crate.
pub fn init(cx: &mut App) {
    VimModeSetting::register(cx);
    HelixModeSetting::register(cx);
}

pub struct VimModeSetting(pub bool);

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    SettingsUi,
    SettingsKey,
    JsonSchema,
)]
#[settings_key(None)]
pub struct VimModeSettingContent {
    /// Whether or not to enable Vim mode.
    ///
    /// Default: false
    pub vim_mode: Option<bool>,
}

impl Settings for VimModeSetting {
    type FileContent = VimModeSettingContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        Ok(Self(
            sources
                .user
                .and_then(|mode| mode.vim_mode)
                .or(sources.server.and_then(|mode| mode.vim_mode))
                .or(sources.default.vim_mode)
                .ok_or_else(Self::missing_default)?,
        ))
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {
        // TODO: could possibly check if any of the `vim.<foo>` keys are set?
    }
}

#[derive(Debug)]
pub struct HelixModeSetting(pub bool);

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    SettingsUi,
    SettingsKey,
    JsonSchema,
)]
#[settings_key(None)]
pub struct HelixModeSettingContent {
    /// Whether or not to enable Helix mode.
    ///
    /// Default: false
    pub helix_mode: Option<bool>,
}

impl Settings for HelixModeSetting {
    type FileContent = HelixModeSettingContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        Ok(Self(
            sources
                .user
                .and_then(|mode| mode.helix_mode)
                .or(sources.server.and_then(|mode| mode.helix_mode))
                .or(sources.default.helix_mode)
                .ok_or_else(Self::missing_default)?,
        ))
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {
        // TODO: could possibly check if any of the `helix.<foo>` keys are set?
    }
}
