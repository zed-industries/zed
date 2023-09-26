use gpui2::elements::div::{div, Div};
use gpui2::style::StyleHelpers;

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
pub fn h_stack<V: 'static>() -> Div<V> {
    div().h_stack()
}

/// Vertically stacks elements.
pub fn v_stack<V: 'static>() -> Div<V> {
    div().v_stack()
}
