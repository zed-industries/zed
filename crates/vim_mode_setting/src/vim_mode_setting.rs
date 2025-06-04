//! Contains the [`VimModeSetting`] used to enable/disable Vim mode.
//!
//! This is in its own crate as we want other crates to be able to enable or
//! disable Vim mode without having to depend on the `vim` crate in its
//! entirety.

use anyhow::Result;
use gpui::App;
use settings::{Settings, SettingsSources};

/// Initializes the `vim_mode_setting` crate.
pub fn init(cx: &mut App) {
    VimModeSetting::register(cx);
}

/// Whether or not to enable Vim mode.
///
/// Default: false
pub struct VimModeSetting(pub bool);

impl Settings for VimModeSetting {
    const KEY: Option<&'static str> = Some("vim_mode");

    type FileContent = Option<bool>;

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
