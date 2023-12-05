use gpui::{ClickEvent, Listener};

/// A trait for elements that can be clicked.
pub trait Clickable {
    /// Sets the click handler that will fire whenever the element is clicked.
    fn on_click(self, handler: Listener<ClickEvent>) -> Self;
}
