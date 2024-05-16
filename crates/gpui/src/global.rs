use crate::AppContext;

/// A marker trait for types that can be stored in GPUI's global state.
///
/// This trait exists to provide type-safe access to globals by restricting
/// the scope from which they can be accessed. For instance, the actual type
/// that implements [`Global`] can be private, with public accessor functions
/// that enforce correct usage.
///
/// Implement this on types you want to store in the context as a global.
pub trait Global: 'static {
    // This trait is intentionally left empty, by virtue of being a marker trait.
}

/// A trait for reading a global value from the context.
pub trait ReadGlobal<'a, Output = &'a Self> {
    /// Returns the global instance of the implementing type.
    ///
    /// Panics if a global for that type has not been assigned.
    fn global(cx: &'a AppContext) -> Output;
}

impl<'a, T: Global> ReadGlobal<'a> for T {
    fn global(cx: &'a AppContext) -> &'a Self {
        cx.global::<T>()
    }
}
