//! Contains the [`VimModeSetting`] and [`HelixModeSetting`] used to enable/disable Vim and Helix modes.
//!
//! This is in its own crate as we want other crates to be able to enable or
//! disable Vim/Helix modes without having to depend on the `vim` crate in its
//! entirety.

use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use settings::{Settings, SettingsSources};
use std::fmt::Display;

/// Initializes the `vim_mode_setting` crate.
pub fn init(cx: &mut App) {
    EditorModeSetting::register(cx);
}

/// Whether or not to enable Vim mode.
///
/// Default: `EditMode::Default`
pub struct EditorModeSetting(pub EditorMode);

#[derive(Copy, Clone, Debug, PartialEq, Eq, JsonSchema, Default)]
pub enum EditorMode {
    #[default]
    Default,
    Vim(ModalMode),
    Helix(ModalMode),
}

impl<'de> Deserialize<'de> for EditorMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "default" => Ok(EditorMode::Default),
            "vim" => Ok(EditorMode::Vim(ModalMode::Normal)),
            "vim_normal" => Ok(EditorMode::Vim(ModalMode::Normal)),
            "vim_insert" => Ok(EditorMode::Vim(ModalMode::Insert)),
            "vim_replace" => Ok(EditorMode::Vim(ModalMode::Replace)),
            "vim_visual" => Ok(EditorMode::Vim(ModalMode::Visual)),
            "vim_visual_line" => Ok(EditorMode::Vim(ModalMode::VisualLine)),
            "vim_visual_block" => Ok(EditorMode::Vim(ModalMode::VisualBlock)),
            "helix_experimental" => Ok(EditorMode::Helix(ModalMode::HelixNormal)),
            _ => Err(D::Error::custom(format!("Unknown editor mode: {}", s))),
        }
    }
}

impl Serialize for EditorMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            EditorMode::Default => "default",
            EditorMode::Vim(ModalMode::Normal) => "vim",
            EditorMode::Vim(ModalMode::Insert) => "vim_insert",
            EditorMode::Vim(ModalMode::Replace) => "vim_replace",
            EditorMode::Vim(ModalMode::Visual) => "vim_visual",
            EditorMode::Vim(ModalMode::VisualLine) => "vim_visual_line",
            EditorMode::Vim(ModalMode::VisualBlock) => "vim_visual_block",
            EditorMode::Helix(ModalMode::Normal) => "helix_experimental",
            _ => return Err(serde::ser::Error::custom("unsupported editor mode variant")),
        };
        serializer.serialize_str(s)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum ModalMode {
    Normal,
    Insert,
    Replace,
    Visual,
    VisualLine,
    VisualBlock,
    HelixNormal,
}

impl Display for ModalMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModalMode::Normal => write!(f, "NORMAL"),
            ModalMode::Insert => write!(f, "INSERT"),
            ModalMode::Replace => write!(f, "REPLACE"),
            ModalMode::Visual => write!(f, "VISUAL"),
            ModalMode::VisualLine => write!(f, "VISUAL LINE"),
            ModalMode::VisualBlock => write!(f, "VISUAL BLOCK"),
            ModalMode::HelixNormal => write!(f, "HELIX NORMAL"),
        }
    }
}

impl ModalMode {
    pub fn is_visual(&self) -> bool {
        match self {
            Self::Visual | Self::VisualLine | Self::VisualBlock => true,
            Self::Normal | Self::Insert | Self::Replace | Self::HelixNormal => false,
        }
    }
}

impl Default for ModalMode {
    fn default() -> Self {
        Self::Normal
    }
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

impl EditorMode {
    pub fn is_modal(&self) -> bool {
        matches!(self, EditorMode::Vim(_) | EditorMode::Helix(_))
    }
}
