use gpui2::elements::div::{div, Div};
use gpui2::style::StyleHelpers;

/// Horizontally stacks elements
pub fn h_stack<V: 'static>() -> Div<V> {
    div().flex().flex_row()
}

/// Vertically stacks elements
pub fn v_stack<V: 'static>() -> Div<V> {
    div().flex().flex_col()
}
