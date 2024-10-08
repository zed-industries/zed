use gpui::DefiniteLength;

/// A trait for elements that can have a fixed with. Enables the use of the `width` and `full_width` methods.
pub trait FixedWidth {
    /// Sets the width of the element.
    fn width(self, width: DefiniteLength) -> Self;

    /// Sets the element's width to the full width of its container.
    fn full_width(self) -> Self;
}
