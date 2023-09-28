use gpui2::elements::div::Div;

use crate::prelude::*;

pub trait Stack: StyleHelpers {
    /// Horizontally stacks elements.
    fn h_stack(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// Vertically stacks elements.
    fn v_stack(self) -> Self {
        self.flex().flex_col()
    }
}

impl<V> Stack for Div<V> {}

/// Horizontally stacks elements.
///
/// Sets `flex()`, `flex_row()`, `items_center()`
pub fn h_stack<V: 'static>() -> Div<V> {
    div().h_stack()
}

/// Vertically stacks elements.
///
/// Sets `flex()`, `flex_col()`
pub fn v_stack<V: 'static>() -> Div<V> {
    div().v_stack()
}
