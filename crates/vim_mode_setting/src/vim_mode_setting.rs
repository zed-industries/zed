//! Contains the [`ModalEditing`] setting used to select a modal editing mode.
//!
//! This is in its own crate so other crates can inspect the modal editing mode
//! without depending on the `vim` crate in its entirety.

use gpui::App;
use settings::{ModalEditingContent, RegisterSetting, Settings, SettingsContent};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, RegisterSetting)]
pub enum ModalEditing {
    #[default]
    None,
    Vim,
    Helix,
}

impl Settings for ModalEditing {
    fn from_settings(content: &SettingsContent) -> Self {
        content.modal_editing.unwrap().into()
    }
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

impl ModalEditing {
    pub fn is_enabled(self) -> bool {
        self != Self::None
    }

    pub fn is_vim(self) -> bool {
        self == Self::Vim
    }

    pub fn is_helix(self) -> bool {
        self == Self::Helix
    }

    pub fn is_enabled_in(cx: &App) -> bool {
        Self::try_get(cx).is_some_and(|mode| mode.is_enabled())
    }
}
