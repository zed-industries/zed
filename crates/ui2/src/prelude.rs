pub use gpui::{
    div, Element, ElementId, InteractiveElement, ParentElement, RenderOnce, SharedString, Styled,
    ViewContext, WindowContext,
};

pub use crate::StyledExt;
pub use crate::{ButtonVariant, Color};
pub use theme::ActiveTheme;

use strum::EnumIter;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum IconSide {
    #[default]
    Left,
    Right,
}

#[derive(Default, PartialEq, Copy, Clone, EnumIter, strum::Display)]
pub enum InteractionState {
    /// An element that is enabled and not hovered, active, focused, or disabled.
    ///
    /// This is often referred to as the "default" state.
    #[default]
    Enabled,
    /// An element that is hovered.
    Hovered,
    /// An element has an active mouse down or touch start event on it.
    Active,
    /// An element that is focused using the keyboard.
    Focused,
    /// An element that is disabled.
    Disabled,
    /// A toggleable element that is selected, like the active button in a
    /// button toggle group.
    Selected,
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
