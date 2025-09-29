use std::fmt::{Display, Formatter};

use crate::{
    self as settings,
    settings_content::{BaseKeymapContent, SettingsContent},
};
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, VsCodeSettings};

/// Base key bindings scheme. Base keymaps can be overridden with user keymaps.
///
/// Default: VSCode
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub enum BaseKeymap {
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

impl From<BaseKeymapContent> for BaseKeymap {
    fn from(value: BaseKeymapContent) -> Self {
        match value {
            BaseKeymapContent::VSCode => Self::VSCode,
            BaseKeymapContent::JetBrains => Self::JetBrains,
            BaseKeymapContent::SublimeText => Self::SublimeText,
            BaseKeymapContent::Atom => Self::Atom,
            BaseKeymapContent::TextMate => Self::TextMate,
            BaseKeymapContent::Emacs => Self::Emacs,
            BaseKeymapContent::Cursor => Self::Cursor,
            BaseKeymapContent::None => Self::None,
        }
    }
}
impl Into<BaseKeymapContent> for BaseKeymap {
    fn into(self) -> BaseKeymapContent {
        match self {
            BaseKeymap::VSCode => BaseKeymapContent::VSCode,
            BaseKeymap::JetBrains => BaseKeymapContent::JetBrains,
            BaseKeymap::SublimeText => BaseKeymapContent::SublimeText,
            BaseKeymap::Atom => BaseKeymapContent::Atom,
            BaseKeymap::TextMate => BaseKeymapContent::TextMate,
            BaseKeymap::Emacs => BaseKeymapContent::Emacs,
            BaseKeymap::Cursor => BaseKeymapContent::Cursor,
            BaseKeymap::None => BaseKeymapContent::None,
        }
    }
}

impl Display for BaseKeymap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseKeymap::VSCode => write!(f, "VSCode"),
            BaseKeymap::JetBrains => write!(f, "JetBrains"),
            BaseKeymap::SublimeText => write!(f, "Sublime Text"),
            BaseKeymap::Atom => write!(f, "Atom"),
            BaseKeymap::TextMate => write!(f, "TextMate"),
            BaseKeymap::Emacs => write!(f, "Emacs (beta)"),
            BaseKeymap::Cursor => write!(f, "Cursor (beta)"),
            BaseKeymap::None => write!(f, "None"),
        }
    }
}

impl BaseKeymap {
    #[cfg(target_os = "macos")]
    pub const OPTIONS: [(&'static str, Self); 7] = [
        ("VSCode (Default)", Self::VSCode),
        ("Atom", Self::Atom),
        ("JetBrains", Self::JetBrains),
        ("Sublime Text", Self::SublimeText),
        ("Emacs (beta)", Self::Emacs),
        ("TextMate", Self::TextMate),
        ("Cursor", Self::Cursor),
    ];

    #[cfg(not(target_os = "macos"))]
    pub const OPTIONS: [(&'static str, Self); 6] = [
        ("VSCode (Default)", Self::VSCode),
        ("Atom", Self::Atom),
        ("JetBrains", Self::JetBrains),
        ("Sublime Text", Self::SublimeText),
        ("Emacs (beta)", Self::Emacs),
        ("Cursor", Self::Cursor),
    ];

    pub fn asset_path(&self) -> Option<&'static str> {
        #[cfg(target_os = "macos")]
        match self {
            BaseKeymap::JetBrains => Some("keymaps/macos/jetbrains.json"),
            BaseKeymap::SublimeText => Some("keymaps/macos/sublime_text.json"),
            BaseKeymap::Atom => Some("keymaps/macos/atom.json"),
            BaseKeymap::TextMate => Some("keymaps/macos/textmate.json"),
            BaseKeymap::Emacs => Some("keymaps/macos/emacs.json"),
            BaseKeymap::Cursor => Some("keymaps/macos/cursor.json"),
            BaseKeymap::VSCode => None,
            BaseKeymap::None => None,
        }

        #[cfg(not(target_os = "macos"))]
        match self {
            BaseKeymap::JetBrains => Some("keymaps/linux/jetbrains.json"),
            BaseKeymap::SublimeText => Some("keymaps/linux/sublime_text.json"),
            BaseKeymap::Atom => Some("keymaps/linux/atom.json"),
            BaseKeymap::Emacs => Some("keymaps/linux/emacs.json"),
            BaseKeymap::Cursor => Some("keymaps/linux/cursor.json"),
            BaseKeymap::TextMate => None,
            BaseKeymap::VSCode => None,
            BaseKeymap::None => None,
        }
    }

    pub fn names() -> impl Iterator<Item = &'static str> {
        Self::OPTIONS.iter().map(|(name, _)| *name)
    }

    pub fn from_names(option: &str) -> BaseKeymap {
        Self::OPTIONS
            .iter()
            .copied()
            .find_map(|(name, value)| (name == option).then_some(value))
            .unwrap_or_default()
    }
}

impl Settings for BaseKeymap {
    fn from_settings(s: &crate::settings_content::SettingsContent, _cx: &mut App) -> Self {
        s.base_keymap.unwrap().into()
    }

    fn import_from_vscode(_vscode: &VsCodeSettings, current: &mut SettingsContent) {
        current.base_keymap = Some(BaseKeymapContent::VSCode);
    }
}
