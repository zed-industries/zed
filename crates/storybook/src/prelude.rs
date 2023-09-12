#[derive(PartialEq)]
pub enum ButtonVariant {
    Ghost,
    Filled,
}

#[derive(PartialEq)]
pub enum Shape {
    Circle,
    Squircle,
}

#[derive(PartialEq)]
pub enum UIState {
    Default,
    Hovered,
    Active,
    Focused,
    Disabled,
}

#[derive(PartialEq)]
pub enum UIToggleState {
    Default,
    Enabled,
}
