use gpui::{div, Component, ParentElement};

use crate::{Icon, IconColor, IconElement, IconSize};

/// Whether the entry is toggleable, and if so, whether it is currently toggled.
///
/// To make an element toggleable, simply add a `Toggle::Toggled(_)` and handle it's cases.
///
/// You can check if an element is toggleable with `.is_toggleable()`
///
/// Possible values:
/// - `Toggle::NotToggleable` - The entry is not toggleable
/// - `Toggle::Toggled(true)` - The entry is toggleable and toggled
/// - `Toggle::Toggled(false)` - The entry is toggleable and not toggled
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Toggle {
    NotToggleable,
    Toggled(bool),
}

impl Toggle {
    /// Returns true if the entry is toggled (or is not toggleable.)
    ///
    /// As element that isn't toggleable is always "expanded" or "enabled"
    /// returning true in that case makes sense.
    pub fn is_toggled(&self) -> bool {
        match self {
            Self::Toggled(false) => false,
            _ => true,
        }
    }

    pub fn is_toggleable(&self) -> bool {
        match self {
            Self::Toggled(_) => true,
            _ => false,
        }
    }
}

impl From<bool> for Toggle {
    fn from(toggled: bool) -> Self {
        Toggle::Toggled(toggled)
    }
}

pub fn disclosure_control<V: 'static>(toggle: Toggle) -> impl Component<V> {
    match (toggle.is_toggleable(), toggle.is_toggled()) {
        (false, _) => div(),
        (_, true) => div().child(
            IconElement::new(Icon::ChevronDown)
                .color(IconColor::Muted)
                .size(IconSize::Small),
        ),
        (_, false) => div().child(
            IconElement::new(Icon::ChevronRight)
                .color(IconColor::Muted)
                .size(IconSize::Small),
        ),
    }
}
