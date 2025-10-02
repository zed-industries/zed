//! Contains the [`VimModeSetting`] and [`HelixModeSetting`] used to enable/disable Vim and Helix modes.
//!
//! This is in its own crate as we want other crates to be able to enable or
//! disable Vim/Helix modes without having to depend on the `vim` crate in its
//! entirety.

use gpui::App;
use settings::{Settings, SettingsContent};

/// Initializes the `vim_mode_setting` crate.
pub fn init(cx: &mut App) {
    VimModeSetting::register(cx);
    HelixModeSetting::register(cx);
}

pub struct VimModeSetting(pub bool);

impl Settings for VimModeSetting {
    fn from_settings(content: &SettingsContent, _cx: &mut App) -> Self {
        Self(content.vim_mode.unwrap())
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _content: &mut SettingsContent) {
        // TODO: could possibly check if any of the `vim.<foo>` keys are set?
    }
}

pub struct HelixModeSetting(pub bool);

impl Settings for HelixModeSetting {
    fn from_settings(content: &SettingsContent, _cx: &mut App) -> Self {
        Self(content.helix_mode.unwrap())
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut SettingsContent) {}
}
