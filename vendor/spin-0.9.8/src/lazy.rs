//! Synchronization primitives for lazy evaluation.
//!
//! Implementation adapted from the `SyncLazy` type of the standard library. See:
//! <https://doc.rust-lang.org/std/lazy/struct.SyncLazy.html>

use crate::{once::Once, RelaxStrategy, Spin};
use core::{cell::Cell, fmt, ops::Deref};

/// A value which is initialized on the first access.
///
/// This type is a thread-safe `Lazy`, and can be used in statics.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use spin::Lazy;
///
/// static HASHMAP: Lazy<HashMap<i32, String>> = Lazy::new(|| {
///     println!("initializing");
///     let mut m = HashMap::new();
///     m.insert(13, "Spica".to_string());
///     m.insert(74, "Hoyten".to_string());
///     m
/// });
///
/// fn main() {
///     println!("ready");
///     std::thread::spawn(|| {
///         println!("{:?}", HASHMAP.get(&13));
///     }).join().unwrap();
///     println!("{:?}", HASHMAP.get(&74));
///
///     // Prints:
///     //   ready
///     //   initializing
///     //   Some("Spica")
///     //   Some("Hoyten")
/// }
/// ```
pub struct Lazy<T, F = fn() -> T, R = Spin> {
    cell: Once<T, R>,
    init: Cell<Option<F>>,
}

impl<T: fmt::Debug, F, R> fmt::Debug for Lazy<T, F, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lazy")
            .field("cell", &self.cell)
            .field("init", &"..")
            .finish()
    }
}

// We never create a `&F` from a `&Lazy<T, F>` so it is fine
// to not impl `Sync` for `F`
// we do create a `&mut Option<F>` in `force`, but this is
// properly synchronized, so it only happens once
// so it also does not contribute to this impl.
unsafe impl<T, F: Send> Sync for Lazy<T, F> where Once<T>: Sync {}
// auto-derived `Send` impl is OK.

impl<T, F, R> Lazy<T, F, R> {
    /// Creates a new lazy value with the given initializing
    /// function.
    pub const fn new(f: F) -> Self {
        Self {
            cell: Once::new(),
            init: Cell::new(Some(f)),
        }
    }
    /// Retrieves a mutable pointer to the inner data.
    ///
    /// This is especially useful when interfacing with low level code or FFI where the caller
    /// explicitly knows that it has exclusive access to the inner data. Note that reading from
    /// this pointer is UB until initialized or directly written to.
    pub fn as_mut_ptr(&self) -> *mut T {
        self.cell.as_mut_ptr()
    }
}

impl<T, F: FnOnce() -> T, R: RelaxStrategy> Lazy<T, F, R> {
    /// Forces the evaluation of this lazy value and
    /// returns a reference to result. This is equivalent
    /// to the `Deref` impl, but is explicit.
    ///
    /// # Examples
    ///
    /// ```
    /// use spin::Lazy;
    ///
    /// let lazy = Lazy::new(|| 92);
    ///
    /// assert_eq!(Lazy::force(&lazy), &92);
    /// assert_eq!(&*lazy, &92);
    /// ```
    pub fn force(this: &Self) -> &T {
        this.cell.call_once(|| match this.init.take() {
            Some(f) => f(),
            None => panic!("Lazy instance has previously been poisoned"),
        })
    }
}

impl<T, F: FnOnce() -> T, R: RelaxStrategy> Deref for Lazy<T, F, R> {
    type Target = T;

    fn deref(&self) -> &T {
        Self::force(self)
    }
}

impl<T: Default, R> Default for Lazy<T, fn() -> T, R> {
    /// Creates a new lazy value using `Default` as the initializing function.
    fn default() -> Self {
        Self::new(T::default)
    }
}
