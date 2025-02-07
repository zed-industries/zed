/// A trait for elements that can be toggled.
///
/// Implement this for elements that are visually distinct
/// when in two opposing states, like checkboxes or switches.
pub trait Toggleable {
    /// Sets whether the element is selected.
    fn toggle_state(self, selected: bool) -> Self;
}

/// Represents the selection status of an element.
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ToggleState {
    /// The element is not selected.
    #[default]
    Unselected,
    /// The selection state of the element is indeterminate.
    Indeterminate,
    /// The element is selected.
    Selected,
}

impl ToggleState {
    /// Returns the inverse of the current selection status.
    ///
    /// Indeterminate states become selected if inverted.
    pub fn inverse(&self) -> Self {
        match self {
            Self::Unselected | Self::Indeterminate => Self::Selected,
            Self::Selected => Self::Unselected,
        }
    }

    /// Creates a `ToggleState` from the given `any_checked` and `all_checked` flags.
    pub fn from_any_and_all(any_checked: bool, all_checked: bool) -> Self {
        match (any_checked, all_checked) {
            (true, true) => Self::Selected,
            (false, false) => Self::Unselected,
            _ => Self::Indeterminate,
        }
    }

    /// Returns whether this toggle state is selected
    pub fn selected(&self) -> bool {
        match self {
            ToggleState::Indeterminate | ToggleState::Unselected => false,
            ToggleState::Selected => true,
        }
    }
}

impl From<bool> for ToggleState {
    fn from(selected: bool) -> Self {
        if selected {
            Self::Selected
        } else {
            Self::Unselected
        }
    }
}

impl From<Option<bool>> for ToggleState {
    fn from(selected: Option<bool>) -> Self {
        match selected {
            Some(true) => Self::Selected,
            Some(false) => Self::Unselected,
            None => Self::Indeterminate,
        }
    }
}
