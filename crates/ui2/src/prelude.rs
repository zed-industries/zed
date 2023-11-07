pub use gpui::{
    div, Component, Element, ElementId, ParentElement, SharedString, StatefulInteractive,
    StatelessInteractive, Styled, ViewContext, WindowContext,
};

pub use crate::elevation::*;
pub use crate::ButtonVariant;
pub use theme2::ActiveTheme;

use gpui::Hsla;
use strum::EnumIter;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum FileSystemStatus {
    #[default]
    None,
    Conflict,
    Deleted,
}

impl std::fmt::Display for FileSystemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "None",
                Self::Conflict => "Conflict",
                Self::Deleted => "Deleted",
            }
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum GitStatus {
    #[default]
    None,
    Created,
    Modified,
    Deleted,
    Conflict,
    Renamed,
}

impl GitStatus {
    pub fn hsla(&self, cx: &WindowContext) -> Hsla {
        match self {
            Self::None => cx.theme().styles.system.transparent,
            Self::Created => cx.theme().styles.git.created,
            Self::Modified => cx.theme().styles.git.modified,
            Self::Deleted => cx.theme().styles.git.deleted,
            Self::Conflict => cx.theme().styles.git.conflict,
            Self::Renamed => cx.theme().styles.git.renamed,
        }
    }
}

impl std::fmt::Display for GitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "None",
                Self::Created => "Created",
                Self::Modified => "Modified",
                Self::Deleted => "Deleted",
                Self::Conflict => "Conflict",
                Self::Renamed => "Renamed",
            }
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum DiagnosticStatus {
    #[default]
    None,
    Error,
    Warning,
    Info,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum IconSide {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum OrderMethod {
    #[default]
    Ascending,
    Descending,
    MostRecent,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum Shape {
    #[default]
    Circle,
    RoundedRectangle,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum DisclosureControlVisibility {
    #[default]
    OnHover,
    Always,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum DisclosureControlStyle {
    /// Shows the disclosure control only when hovered where possible.
    ///
    /// More compact, but not available everywhere.
    ChevronOnHover,
    /// Shows an icon where possible, otherwise shows a chevron.
    ///
    /// For example, in a file tree a folder or file icon is shown
    /// instead of a chevron
    Icon,
    /// Always shows a chevron.
    Chevron,
    /// Completely hides the disclosure control where possible.
    None,
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
