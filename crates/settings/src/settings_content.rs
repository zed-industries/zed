mod language;
mod theme;
pub use language::*;
pub use theme::*;

use std::env;

use collections::HashMap;
use gpui::{App, SharedString};
use release_channel::ReleaseChannel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ActiveSettingsProfileName, Settings};

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SettingsContent {
    #[serde(flatten)]
    pub project: ProjectSettingsContent,

    pub base_keymap: Option<BaseKeymapContent>,

    pub auto_update: Option<bool>,

    pub title_bar: Option<TitleBarSettingsContent>,
}

impl SettingsContent {
    pub fn languages_mut(&mut self) -> &mut HashMap<SharedString, LanguageSettingsContent> {
        &mut self.project.all_languages.languages.0
    }
}

// todo!() what should this be?
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ServerSettingsContent {
    #[serde(flatten)]
    pub project: ProjectSettingsContent,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub(crate) struct UserSettingsContent {
    #[serde(flatten)]
    pub(crate) content: SettingsContent,

    pub(crate) dev: Option<SettingsContent>,
    pub(crate) nightly: Option<SettingsContent>,
    pub(crate) preview: Option<SettingsContent>,
    pub(crate) stable: Option<SettingsContent>,

    pub(crate) macos: Option<SettingsContent>,
    pub(crate) windows: Option<SettingsContent>,
    pub(crate) linux: Option<SettingsContent>,

    #[serde(default)]
    pub(crate) profiles: HashMap<String, SettingsContent>,
}

pub struct ExtensionsSettingsContent {
    pub(crate) all_languages: AllLanguageSettingsContent,
}

impl UserSettingsContent {
    pub(crate) fn for_release_channel(&self) -> Option<&SettingsContent> {
        match *release_channel::RELEASE_CHANNEL {
            ReleaseChannel::Dev => self.dev.as_ref(),
            ReleaseChannel::Nightly => self.nightly.as_ref(),
            ReleaseChannel::Preview => self.preview.as_ref(),
            ReleaseChannel::Stable => self.stable.as_ref(),
        }
    }

    pub(crate) fn for_os(&self) -> Option<&SettingsContent> {
        match env::consts::OS {
            "macos" => self.macos.as_ref(),
            "linux" => self.linux.as_ref(),
            "windows" => self.windows.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn for_profile(&self, cx: &App) -> Option<&SettingsContent> {
        let Some(active_profile) = cx.try_global::<ActiveSettingsProfileName>() else {
            return None;
        };
        self.profiles.get(&active_profile.0)
    }
}

/// Base key bindings scheme. Base keymaps can be overridden with user keymaps.
///
/// Default: VSCode
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub enum BaseKeymapContent {
    #[default]
    VSCode,
    JetBrains,
    SublimeText,
    Atom,
    TextMate,
    Emacs,
    Cursor,
    None,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ProjectSettingsContent {
    #[serde(flatten)]
    pub(crate) all_languages: AllLanguageSettingsContent,
}

#[derive(Copy, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct TitleBarSettingsContent {
    /// Controls when the title bar is visible: "always" | "never" | "hide_in_full_screen".
    ///
    /// Default: "always"
    pub show: Option<TitleBarVisibilityContent>,
    /// Whether to show the branch icon beside branch switcher in the title bar.
    ///
    /// Default: false
    pub show_branch_icon: Option<bool>,
    /// Whether to show onboarding banners in the title bar.
    ///
    /// Default: true
    pub show_onboarding_banner: Option<bool>,
    /// Whether to show user avatar in the title bar.
    ///
    /// Default: true
    pub show_user_picture: Option<bool>,
    /// Whether to show the branch name button in the titlebar.
    ///
    /// Default: true
    pub show_branch_name: Option<bool>,
    /// Whether to show the project host and name in the titlebar.
    ///
    /// Default: true
    pub show_project_items: Option<bool>,
    /// Whether to show the sign in button in the title bar.
    ///
    /// Default: true
    pub show_sign_in: Option<bool>,
    /// Whether to show the menus in the title bar.
    ///
    /// Default: false
    pub show_menus: Option<bool>,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TitleBarVisibilityContent {
    Always,
    Never,
    HideInFullScreen,
}
