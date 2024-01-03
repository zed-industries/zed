use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub enum BaseKeymap {
    #[default]
    VSCode,
    JetBrains,
    SublimeText,
    Atom,
    TextMate,
}

impl BaseKeymap {
    pub const OPTIONS: [(&'static str, Self); 5] = [
        ("VSCode (Default)", Self::VSCode),
        ("Atom", Self::Atom),
        ("JetBrains", Self::JetBrains),
        ("Sublime Text", Self::SublimeText),
        ("TextMate", Self::TextMate),
    ];

    pub fn asset_path(&self) -> Option<&'static str> {
        match self {
            BaseKeymap::JetBrains => Some("keymaps/jetbrains.json"),
            BaseKeymap::SublimeText => Some("keymaps/sublime_text.json"),
            BaseKeymap::Atom => Some("keymaps/atom.json"),
            BaseKeymap::TextMate => Some("keymaps/textmate.json"),
            BaseKeymap::VSCode => None,
        }
    }

    pub fn names() -> impl Iterator<Item = &'static str> {
        Self::OPTIONS.iter().map(|(name, _)| *name)
    }

    pub fn from_names(option: &str) -> BaseKeymap {
        Self::OPTIONS
            .iter()
            .copied()
            .find_map(|(name, value)| (name == option).then(|| value))
            .unwrap_or_default()
    }
}

impl Settings for BaseKeymap {
    const KEY: Option<&'static str> = Some("base_keymap");

    type FileContent = Option<Self>;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(user_values
            .first()
            .and_then(|v| **v)
            .unwrap_or(default_value.unwrap()))
    }
}
