use once_cell::sync::OnceCell;

/// Lazily initialized static variable.
///
/// Used in generated code.
///
/// Currently a wrapper around `once_cell`s `OnceCell`.
pub struct Lazy<T> {
    once_cell: OnceCell<T>,
}

impl<T> Lazy<T> {
    /// Uninitialized state.
    pub const fn new() -> Lazy<T> {
        Lazy {
            once_cell: OnceCell::new(),
        }
    }

    /// Lazily initialize the value.
    pub fn get(&self, f: impl FnOnce() -> T) -> &T {
        self.once_cell.get_or_init(f)
    }
}
