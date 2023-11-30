use gpui::DefiniteLength;

/// A trait for elements that have a fixed with.
pub trait FixedWidth {
    /// Sets the width of the element.
    fn width(self, width: DefiniteLength) -> Self;

    /// Sets the element's width to the full width of its container.
    fn full_width(self) -> Self;
}
