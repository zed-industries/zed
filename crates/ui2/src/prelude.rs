pub use gpui::{
    div, Component, Element, ElementId, InteractiveElement, ParentElement, SharedString, Styled,
    ViewContext, WindowContext,
};

pub use crate::StyledExt;
pub use crate::{ButtonVariant, TextColor};
pub use theme2::ActiveTheme;

use strum::EnumIter;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum IconSide {
    #[default]
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum OverflowStyle {
    Hidden,
    Wrap,
}

#[derive(Default, PartialEq, Copy, Clone, EnumIter, strum::Display)]
pub enum InteractionState {
    #[default]
    Enabled,
    Hovered,
    Active,
    Focused,
    Disabled,
}

impl InteractionState {
    pub fn if_enabled(&self, enabled: bool) -> Self {
        if enabled {
            *self
        } else {
            InteractionState::Disabled
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Selection {
    #[default]
    Unselected,
    Indeterminate,
    Selected,
}

impl Selection {
    pub fn inverse(&self) -> Self {
        match self {
            Self::Unselected | Self::Indeterminate => Self::Selected,
            Self::Selected => Self::Unselected,
        }
    }
}
