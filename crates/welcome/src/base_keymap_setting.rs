use std::fmt::{Display, Formatter};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

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
    None,
}

impl Display for BaseKeymap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseKeymap::VSCode => write!(f, "VSCode"),
            BaseKeymap::JetBrains => write!(f, "JetBrains"),
            BaseKeymap::SublimeText => write!(f, "Sublime Text"),
            BaseKeymap::Atom => write!(f, "Atom"),
            BaseKeymap::TextMate => write!(f, "TextMate"),
            BaseKeymap::None => write!(f, "None"),
        }
    }
}

impl BaseKeymap {
    #[cfg(target_os = "macos")]
    pub const OPTIONS: [(&'static str, Self); 5] = [
        ("VSCode (Default)", Self::VSCode),
        ("Atom", Self::Atom),
        ("JetBrains", Self::JetBrains),
        ("Sublime Text", Self::SublimeText),
        ("TextMate", Self::TextMate),
    ];

    #[cfg(not(target_os = "macos"))]
    pub const OPTIONS: [(&'static str, Self); 4] = [
        ("VSCode (Default)", Self::VSCode),
        ("Atom", Self::Atom),
        ("JetBrains", Self::JetBrains),
        ("Sublime Text", Self::SublimeText),
    ];

    pub fn asset_path(&self) -> Option<&'static str> {
        #[cfg(target_os = "macos")]
        match self {
            BaseKeymap::JetBrains => Some("keymaps/macos/jetbrains.json"),
            BaseKeymap::SublimeText => Some("keymaps/macos/sublime_text.json"),
            BaseKeymap::Atom => Some("keymaps/macos/atom.json"),
            BaseKeymap::TextMate => Some("keymaps/macos/textmate.json"),
            BaseKeymap::VSCode => None,
            BaseKeymap::None => None,
        }

        #[cfg(not(target_os = "macos"))]
        match self {
            BaseKeymap::JetBrains => Some("keymaps/linux/jetbrains.json"),
            BaseKeymap::SublimeText => Some("keymaps/linux/sublime_text.json"),
            BaseKeymap::Atom => Some("keymaps/linux/atom.json"),
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
    const KEY: Option<&'static str> = Some("base_keymap");

    type FileContent = Option<Self>;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        if let Some(Some(user_value)) = sources.user.copied() {
            return Ok(user_value);
        }
        sources.default.ok_or_else(Self::missing_default)
    }
}
