pub use crate::base::boolean_t;

#[derive(Default)]
pub struct ScreenCaptureAccess;

impl ScreenCaptureAccess {
    /// If current app not in list, will open window.
    /// Return the same result as preflight.
    #[inline]
    pub fn request(&self) -> bool {
        unsafe { CGRequestScreenCaptureAccess() == 1 }
    }

    /// Return `true` if has access
    #[inline]
    pub fn preflight(&self) -> bool {
        unsafe { CGPreflightScreenCaptureAccess() == 1 }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
extern "C" {
    // Screen Capture Access
    fn CGRequestScreenCaptureAccess() -> boolean_t;
    fn CGPreflightScreenCaptureAccess() -> boolean_t;
}
