//! [OpenHarmony] OS Window Handles
//!
//! ## Background
//!
//! Applications on [OpenHarmony] use [ArkUI] for defining their UI. Applications can use an
//! [XComponent] to render using native Code (e.g. Rust) via EGL.
//! Native code will receive a callback `OnSurfaceCreatedCB(OH_NativeXComponent *component, void *window)`
//! when the `XComponent` is created. The window argument has the type [`OHNativeWindow`] / `EGLNativeWindowType`.
//! The window can then be used to create a surface with
//! `eglCreateWindowSurface(eglDisplay_, eglConfig_, window, NULL)`
//!
//! [OpenHarmony]: https://gitee.com/openharmony/docs/blob/master/en/OpenHarmony-Overview.md
//! [ArkUI]: https://gitee.com/openharmony/docs/blob/master/en/application-dev/ui/arkui-overview.md
//! [XComponent]: https://gitee.com/openharmony/docs/blob/master/en/application-dev/ui/arkts-common-components-xcomponent.md
//! [`OHNativeWindow`]: https://gitee.com/openharmony/docs/blob/master/en/application-dev/reference/apis-arkgraphics2d/_native_window.md

use core::ffi::c_void;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for OpenHarmony.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OhosDisplayHandle {}

impl OhosDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::OhosDisplayHandle;
    /// let handle = OhosDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

impl DisplayHandle<'static> {
    /// Create an OpenHarmony-based display handle.
    ///
    /// As no data is borrowed by this handle, it is completely safe to create. This function
    /// may be useful to windowing framework implementations that want to avoid unsafe code.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::ohos();
    /// do_something(handle);
    /// ```
    pub fn ohos() -> Self {
        // SAFETY: No data is borrowed.
        unsafe { Self::borrow_raw(OhosDisplayHandle::new().into()) }
    }
}

/// Raw window handle for Ohos NDK.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OhosNdkWindowHandle {
    pub native_window: NonNull<c_void>,
}

impl OhosNdkWindowHandle {
    /// Create a new handle to an [`OHNativeWindow`] on OpenHarmony.
    ///
    /// The handle will typically be created from an [`XComponent`], consult the
    /// [native `XComponent` Guidelines] for more details.
    ///
    /// [`XComponent`]: https://gitee.com/openharmony/docs/blob/master/en/application-dev/ui/arkts-common-components-xcomponent.md
    /// [native `XComponent` Guidelines]: https://gitee.com/openharmony/docs/blob/OpenHarmony-4.0-Release/en/application-dev/napi/xcomponent-guidelines.md
    /// [`OHNativeWindow`]: https://gitee.com/openharmony/docs/blob/master/en/application-dev/reference/apis-arkgraphics2d/_native_window.md
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use core::ffi::c_void;
    /// # use raw_window_handle::OhosNdkWindowHandle;
    /// # #[allow(non_camel_case_types)]
    /// # type OH_NativeXComponent = ();
    ///
    /// /// Called When the `XComponent` is created.
    /// ///
    /// /// See the [XComponent Guidelines](https://gitee.com/openharmony/docs/blob/OpenHarmony-4.0-Release/en/application-dev/napi/xcomponent-guidelines.md)
    /// /// for more details
    /// extern "C" fn on_surface_created_callback(component: *mut OH_NativeXComponent, window: *mut c_void) {
    ///     let handle = OhosNdkWindowHandle::new(NonNull::new(window).unwrap());
    /// }
    /// ```
    pub fn new(native_window: NonNull<c_void>) -> Self {
        Self { native_window }
    }
}
