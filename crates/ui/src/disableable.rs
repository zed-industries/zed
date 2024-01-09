/// A trait for elements that can be disabled. Generally used to implement disabling an element's interactivity and changing it's appearance to reflect that it is disabled.
pub trait Disableable {
    /// Sets whether the element is disabled.
    fn disabled(self, disabled: bool) -> Self;
}
