//! Contains the [`VimModeSetting`] and [`HelixModeSetting`] used to enable/disable Vim and Helix modes.
//!
//! This is in its own crate as we want other crates to be able to enable or
//! disable Vim/Helix modes without having to depend on the `vim` crate in its
//! entirety.

use settings::{RegisterSetting, Settings, SettingsContent};

#[derive(RegisterSetting)]
pub struct VimModeSetting(pub bool);

impl Settings for VimModeSetting {
    fn from_settings(content: &SettingsContent) -> Self {
        Self(content.vim_mode.unwrap())
    }
}

#[derive(RegisterSetting)]
pub struct HelixModeSetting(pub bool);

impl Settings for HelixModeSetting {
    fn from_settings(content: &SettingsContent) -> Self {
        Self(content.helix_mode.unwrap())
    }
}
