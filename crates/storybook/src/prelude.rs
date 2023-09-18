#[derive(Default, PartialEq)]
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
pub enum InteractionState {
    #[default]
    Enabled,
    Hovered,
    Active,
    Focused,
    Dragged,
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
