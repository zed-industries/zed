use gpui2::elements::div::{div, Div};
use gpui2::style::StyleHelpers;

pub trait Stack: Sized + StyleHelpers {
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

/// Horizontally stacks elements
#[deprecated = "Use `Stack::h_stack` instead."]
pub fn h_stack<V: 'static>() -> Div<V> {
    div().flex().flex_row().items_center()
}

/// Vertically stacks elements
#[deprecated = "Use `Stack::v_stack` instead."]
pub fn v_stack<V: 'static>() -> Div<V> {
    div().flex().flex_col()
}
