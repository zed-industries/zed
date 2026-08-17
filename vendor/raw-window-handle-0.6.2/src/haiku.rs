use core::ffi::c_void;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for Haiku.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HaikuDisplayHandle {}

impl HaikuDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::HaikuDisplayHandle;
    /// let handle = HaikuDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

impl DisplayHandle<'static> {
    /// Create an Haiku-based display handle.
    ///
    /// As no data is borrowed by this handle, it is completely safe to create. This function
    /// may be useful to windowing framework implementations that want to avoid unsafe code.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::haiku();
    /// do_something(handle);
    /// ```
    pub fn haiku() -> Self {
        // SAFETY: No data is borrowed.
        unsafe { Self::borrow_raw(HaikuDisplayHandle::new().into()) }
    }
}

/// Raw window handle for Haiku.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HaikuWindowHandle {
    /// A pointer to a BWindow object
    pub b_window: NonNull<c_void>,
    /// A pointer to a BDirectWindow object that might be null
    pub b_direct_window: Option<NonNull<c_void>>,
}

impl HaikuWindowHandle {
    /// Create a new handle to a window.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::HaikuWindowHandle;
    /// # type BWindow = ();
    /// #
    /// let b_window: NonNull<BWindow>;
    /// # b_window = NonNull::from(&());
    /// let mut handle = HaikuWindowHandle::new(b_window.cast());
    /// // Optionally set `b_direct_window`.
    /// handle.b_direct_window = None;
    /// ```
    pub fn new(b_window: NonNull<c_void>) -> Self {
        Self {
            b_window,
            b_direct_window: None,
        }
    }
}
