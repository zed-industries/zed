use gpui::{ClickEvent, CursorStyle, WindowContext};

/// A trait for elements that can be clicked. Enables the use of the `on_click` method.
pub trait Clickable {
    /// Sets the click handler that will fire whenever the element is clicked.
    fn on_click(self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self;
    /// Sets the cursor style when hovering over the element.
    fn cursor_style(self, cursor_style: CursorStyle) -> Self;
}
