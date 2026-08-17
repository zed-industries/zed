use core::ffi::c_void;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for the Web.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebDisplayHandle {}

impl WebDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::WebDisplayHandle;
    /// let handle = WebDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

impl DisplayHandle<'static> {
    /// Create a Web-based display handle.
    ///
    /// As no data is borrowed by this handle, it is completely safe to create. This function
    /// may be useful to windowing framework implementations that want to avoid unsafe code.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::web();
    /// do_something(handle);
    /// ```
    pub fn web() -> Self {
        // SAFETY: No data is borrowed.
        unsafe { Self::borrow_raw(WebDisplayHandle::new().into()) }
    }
}

/// Raw window handle for the Web.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebWindowHandle {
    /// An ID value inserted into the [data attributes] of the canvas element as '`raw-handle`'.
    ///
    /// When accessing from JS, the attribute will automatically be called `rawHandle`.
    ///
    /// Each canvas created by the windowing system should be assigned their own unique ID.
    ///
    /// [data attributes]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/data-*
    pub id: u32,
}

impl WebWindowHandle {
    /// Create a new handle to a canvas element.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::WebWindowHandle;
    /// #
    /// let id: u32 = 0; // canvas.rawHandle;
    /// let handle = WebWindowHandle::new(id);
    /// ```
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

/// Raw window handle for a Web canvas registered via [`wasm-bindgen`].
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebCanvasWindowHandle {
    /// A pointer to the [`JsValue`] of an [`HtmlCanvasElement`].
    ///
    /// Note: This uses [`c_void`] to avoid depending on `wasm-bindgen`
    /// directly.
    ///
    /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    //
    // SAFETY: Not using `JsValue` is sound because `wasm-bindgen` guarantees
    // that there's only one version of itself in any given binary, and hence
    // we can't have a type-confusion where e.g. one library used `JsValue`
    // from `v0.2` of `wasm-bindgen`, and another used `JsValue` from `v1.0`;
    // the binary will simply fail to compile!
    //
    // Reference: TODO
    pub obj: NonNull<c_void>,
}

impl WebCanvasWindowHandle {
    /// Create a new handle from a pointer to [`HtmlCanvasElement`].
    ///
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::WebCanvasWindowHandle;
    /// # type HtmlCanvasElement = ();
    /// # type JsValue = ();
    /// let canvas: &HtmlCanvasElement;
    /// # canvas = &();
    /// let value: &JsValue = &canvas; // Deref to `JsValue`
    /// let obj: NonNull<c_void> = NonNull::from(value).cast();
    /// let mut handle = WebCanvasWindowHandle::new(obj);
    /// ```
    pub fn new(obj: NonNull<c_void>) -> Self {
        Self { obj }
    }
}

#[cfg(all(target_family = "wasm", feature = "wasm-bindgen-0-2"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(target_family = "wasm", feature = "wasm-bindgen-0-2")))
)]
/// These implementations are only available when `wasm-bindgen-0-2` is enabled.
impl WebCanvasWindowHandle {
    /// Create a new `WebCanvasWindowHandle` from a [`wasm_bindgen::JsValue`].
    ///
    /// The `JsValue` should refer to a `HtmlCanvasElement`, and the lifetime
    /// of the value should be at least as long as the lifetime of this.
    pub fn from_wasm_bindgen_0_2(js_value: &wasm_bindgen::JsValue) -> Self {
        Self::new(NonNull::from(js_value).cast())
    }

    /// Convert to the underlying [`wasm_bindgen::JsValue`].
    ///
    /// # Safety
    ///
    /// The inner pointer must be valid. This is ensured if this handle was
    /// borrowed from [`WindowHandle`][crate::WindowHandle].
    pub unsafe fn as_wasm_bindgen_0_2(&self) -> &wasm_bindgen::JsValue {
        unsafe { self.obj.cast().as_ref() }
    }
}

/// Raw window handle for a Web offscreen canvas registered via
/// [`wasm-bindgen`].
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebOffscreenCanvasWindowHandle {
    /// A pointer to the [`JsValue`] of an [`OffscreenCanvas`].
    ///
    /// Note: This uses [`c_void`] to avoid depending on `wasm-bindgen`
    /// directly.
    ///
    /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    //
    // SAFETY: See WebCanvasWindowHandle.
    pub obj: NonNull<c_void>,
}

impl WebOffscreenCanvasWindowHandle {
    /// Create a new handle from a pointer to an [`OffscreenCanvas`].
    ///
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::WebOffscreenCanvasWindowHandle;
    /// # type OffscreenCanvas = ();
    /// # type JsValue = ();
    /// let canvas: &OffscreenCanvas;
    /// # canvas = &();
    /// let value: &JsValue = &canvas; // Deref to `JsValue`
    /// let obj: NonNull<c_void> = NonNull::from(value).cast();
    /// let mut handle = WebOffscreenCanvasWindowHandle::new(obj);
    /// ```
    pub fn new(obj: NonNull<c_void>) -> Self {
        Self { obj }
    }
}

#[cfg(all(target_family = "wasm", feature = "wasm-bindgen-0-2"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(target_family = "wasm", feature = "wasm-bindgen-0-2")))
)]
/// These implementations are only available when `wasm-bindgen-0-2` is enabled.
impl WebOffscreenCanvasWindowHandle {
    /// Create a new `WebOffscreenCanvasWindowHandle` from a
    /// [`wasm_bindgen::JsValue`].
    ///
    /// The `JsValue` should refer to a `HtmlCanvasElement`, and the lifetime
    /// of the value should be at least as long as the lifetime of this.
    pub fn from_wasm_bindgen_0_2(js_value: &wasm_bindgen::JsValue) -> Self {
        Self::new(NonNull::from(js_value).cast())
    }

    /// Convert to the underlying [`wasm_bindgen::JsValue`].
    ///
    /// # Safety
    ///
    /// The inner pointer must be valid. This is ensured if this handle was
    /// borrowed from [`WindowHandle`][crate::WindowHandle].
    pub unsafe fn as_wasm_bindgen_0_2(&self) -> &wasm_bindgen::JsValue {
        unsafe { self.obj.cast().as_ref() }
    }
}
