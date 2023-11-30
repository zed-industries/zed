/// A trait for elements that can be disabled.
pub trait Disableable {
    /// Sets whether the element is disabled.
    fn disabled(self, disabled: bool) -> Self;
}
