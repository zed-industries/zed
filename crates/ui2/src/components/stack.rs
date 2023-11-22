use gpui::{div, Div};

use crate::StyledExt;

/// Horizontally stacks elements.
///
/// Sets `flex()`, `flex_row()`, `items_center()`
pub fn h_stack() -> Div {
    div().h_flex()
}

/// Vertically stacks elements.
///
/// Sets `flex()`, `flex_col()`
pub fn v_stack() -> Div {
    div().v_flex()
}
