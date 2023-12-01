use gpui::{ClickEvent, IntoListener};

/// A trait for elements that can be clicked.
pub trait Clickable {
    /// Sets the click handler that will fire whenever the element is clicked.
    fn on_click(self, handler: impl IntoListener<ClickEvent>) -> Self;
}
