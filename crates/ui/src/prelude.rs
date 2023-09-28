pub use gpui2::elements::div::{div, ScrollState};
pub use gpui2::style::{StyleHelpers, Styleable};
pub use gpui2::{Element, IntoElement, ParentElement, ViewContext};

pub use crate::{ButtonVariant, InputVariant};

use gpui2::{hsla, rgb, Hsla};
use strum::EnumIter;

#[derive(Default)]
pub struct SystemColor {
    pub transparent: Hsla,
    pub mac_os_traffic_light_red: Hsla,
    pub mac_os_traffic_light_yellow: Hsla,
    pub mac_os_traffic_light_green: Hsla,
}

impl SystemColor {
    pub fn new() -> SystemColor {
        SystemColor {
            transparent: hsla(0.0, 0.0, 0.0, 0.0),
            mac_os_traffic_light_red: rgb::<Hsla>(0xEC695E),
            mac_os_traffic_light_yellow: rgb::<Hsla>(0xF4BF4F),
            mac_os_traffic_light_green: rgb::<Hsla>(0x62C554),
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

#[derive(Default, PartialEq)]
pub enum SelectedState {
    #[default]
    Unselected,
    PartiallySelected,
    Selected,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ToggleState {
    #[default]
    NotToggeable,
    Toggled,
    NotToggled,
}
