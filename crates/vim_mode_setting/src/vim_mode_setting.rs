//! Contains the [`ModalEditing`] setting for modal editing modes (Vim/Helix).
//!
//! This is in its own crate to allow other crates to check modal editing mode
//! without depending on the `vim` crate in its entirety.

use settings::ModalEditingContent;
use settings::{RegisterSetting, Settings, SettingsContent};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, RegisterSetting)]
pub enum ModalEditing {
    /// Standard editing mode
    #[default]
    None,
    /// Vim-style modal editing
    Vim,
    /// Helix-style modal editing
    Helix,
}

impl From<ModalEditingContent> for ModalEditing {
    fn from(value: ModalEditingContent) -> Self {
        match value {
            ModalEditingContent::None => Self::None,
            ModalEditingContent::Vim => Self::Vim,
            ModalEditingContent::Helix => Self::Helix,
        }
    }
}

impl From<ModalEditing> for ModalEditingContent {
    fn from(value: ModalEditing) -> Self {
        match value {
            ModalEditing::None => Self::None,
            ModalEditing::Vim => Self::Vim,
            ModalEditing::Helix => Self::Helix,
        }
    }
}

impl Display for ModalEditing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModalEditing::None => write!(f, "None"),
            ModalEditing::Vim => write!(f, "Vim"),
            ModalEditing::Helix => write!(f, "Helix"),
        }
    }
}

impl Settings for ModalEditing {
    fn from_settings(content: &SettingsContent) -> Self {
        if let Some(modal_editing) = content.modal_editing {
            return modal_editing.into();
        }

        // Legacy fallback: check old boolean fields
        {
            if content.helix_mode.unwrap_or(false) {
                return Self::Helix;
            }
            if content.vim_mode.unwrap_or(false) {
                return Self::Vim;
            }
        }

        Self::None
    }
}

impl ModalEditing {
    pub fn is_enabled(&self) -> bool {
        *self != ModalEditing::None
    }

    pub fn is_vim(&self) -> bool {
        *self == ModalEditing::Vim
    }

    pub fn is_helix(&self) -> bool {
        *self == ModalEditing::Helix
    }
}

#[deprecated(note = "Use ModalEditing instead")]
#[derive(RegisterSetting)]
pub struct VimModeSetting(pub bool);

#[allow(deprecated)]
impl Settings for VimModeSetting {
    fn from_settings(content: &SettingsContent) -> Self {
        let modal = ModalEditing::from_settings(content);
        Self(modal == ModalEditing::Vim || modal == ModalEditing::Helix)
    }
}

#[deprecated(note = "Use ModalEditing instead")]
#[derive(RegisterSetting)]
pub struct HelixModeSetting(pub bool);

#[allow(deprecated)]
impl Settings for HelixModeSetting {
    fn from_settings(content: &SettingsContent) -> Self {
        let modal = ModalEditing::from_settings(content);
        Self(modal == ModalEditing::Helix)
    }
}
