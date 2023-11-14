use gpui::{div, Node};

use crate::StyledExt;

/// Horizontally stacks elements.
///
/// Sets `flex()`, `flex_row()`, `items_center()`
pub fn h_stack<V: 'static>() -> Node<V> {
    div().h_flex()
}

/// Vertically stacks elements.
///
/// Sets `flex()`, `flex_col()`
pub fn v_stack<V: 'static>() -> Node<V> {
    div().v_flex()
}
