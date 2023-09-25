use gpui2::{hsla, Hsla};
use strum::EnumIter;

#[derive(Default)]
pub struct SystemColor {
    pub transparent: Hsla,
}

impl SystemColor {
    pub fn new() -> SystemColor {
        SystemColor {
            transparent: hsla(0.0, 0.0, 0.0, 0.0),
        }
    }
    pub fn color(&self) -> Hsla {
        self.transparent
    }
}

#[derive(Default, PartialEq)]
pub enum FileSystemStatus {
    #[default]
    None,
    Conflict,
    Deleted,
}
#[derive(Default, PartialEq)]
pub enum GitStatus {
    #[default]
    None,
    Created,
    Modified,
    Deleted,
    Conflict,
    Renamed,
}
#[derive(Default, PartialEq)]
pub enum DiagnosticStatus {
    #[default]
    None,
    Error,
    Warning,
    Info,
}
#[derive(Default, PartialEq)]
pub enum IconSide {
    #[default]
    Left,
    Right,
}

#[derive(Default, PartialEq)]
pub enum OrderMethod {
    #[default]
    Ascending,
    Descending,
    MostRecent,
}

#[derive(Default, Copy, Clone, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Ghost,
    Filled,
}

#[derive(Default, PartialEq)]
pub enum InputVariant {
    #[default]
    Ghost,
    Filled,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum Shape {
    #[default]
    Circle,
    RoundedRectangle,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum DisclosureControlVisibility {
    #[default]
    OnHover,
    Always,
}

#[derive(Default, PartialEq, Copy, Clone, EnumIter)]
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

    pub fn as_str(&self) -> &'static str {
        match self {
            InteractionState::Enabled => "Enabled",
            InteractionState::Hovered => "Hovered",
            InteractionState::Active => "Active",
            InteractionState::Focused => "Focused",
            InteractionState::Disabled => "Disabled",
        }
    }
}

#[derive(Default, PartialEq)]
pub enum SelectedState {
    #[default]
    Unselected,
    PartiallySelected,
    Selected,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ToggleState {
    Toggled,
    NotToggled,
}
