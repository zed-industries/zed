/// Whether an element is able to be toggled.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Toggleable {
    Toggleable(ToggleState),
    NotToggleable,
}

/// The current state of a [`Toggleable`] element.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum ToggleState {
    Toggled,
    NotToggled,
}

impl ToggleState {
    /// Returns whether an entry is toggled.
    pub fn is_toggled(&self) -> bool {
        match self {
            ToggleState::Toggled => true,
            ToggleState::NotToggled => false,
        }
    }
}

impl From<bool> for ToggleState {
    fn from(toggled: bool) -> Self {
        match toggled {
            true => Self::Toggled,
            false => Self::NotToggled,
        }
    }
}

impl From<ToggleState> for bool {
    fn from(value: ToggleState) -> Self {
        value.is_toggled()
    }
}
