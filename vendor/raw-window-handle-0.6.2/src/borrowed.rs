//! Borrowable window handles based on the ones in this crate.
//!
//! These should be 100% safe to pass around and use, no possibility of dangling or invalidity.

use core::borrow::Borrow;
use core::fmt;
use core::marker::PhantomData;

use crate::{HandleError, RawDisplayHandle, RawWindowHandle};

/// A display that acts as a wrapper around a display handle.
///
/// Objects that implement this trait should be able to return a [`DisplayHandle`] for the display
/// that they are associated with. This handle should last for the lifetime of the object, and should
/// return an error if the application is inactive.
///
/// Implementors of this trait will be windowing systems, like [`winit`] and [`sdl2`]. These windowing
/// systems should implement this trait on types that represent the top-level display server. It
/// should be implemented by tying the lifetime of the [`DisplayHandle`] to the lifetime of the
/// display object.
///
/// Users of this trait will include graphics libraries, like [`wgpu`] and [`glutin`]. These APIs
/// should be generic over a type that implements `HasDisplayHandle`, and should use the
/// [`DisplayHandle`] type to access the display handle.
///
/// Note that these requirements are not enforced on `HasDisplayHandle`, rather, they are enforced on the
/// constructors of [`DisplayHandle`]. This is because the `HasDisplayHandle` trait is safe to implement.
///
/// [`winit`]: https://crates.io/crates/winit
/// [`sdl2`]: https://crates.io/crates/sdl2
/// [`wgpu`]: https://crates.io/crates/wgpu
/// [`glutin`]: https://crates.io/crates/glutin
pub trait HasDisplayHandle {
    /// Get a handle to the display controller of the windowing system.
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError>;
}

impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for &H {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        (**self).display_handle()
    }
}

impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for &mut H {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        (**self).display_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::boxed::Box<H> {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        (**self).display_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::rc::Rc<H> {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        (**self).display_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::sync::Arc<H> {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        (**self).display_handle()
    }
}

/// The handle to the display controller of the windowing system.
///
/// This is the primary return type of the [`HasDisplayHandle`] trait. It is guaranteed to contain
/// a valid platform-specific display handle for its lifetime.
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct DisplayHandle<'a> {
    raw: RawDisplayHandle,
    _marker: PhantomData<&'a ()>,
}

impl fmt::Debug for DisplayHandle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DisplayHandle").field(&self.raw).finish()
    }
}

impl<'a> DisplayHandle<'a> {
    /// Create a `DisplayHandle` from a [`RawDisplayHandle`].
    ///
    /// # Safety
    ///
    /// Users can safely assume that non-`null`/`0` fields are valid handles, and it is up to the
    /// implementer of this trait to ensure that condition is upheld.
    ///
    /// Despite that qualification, implementors should still make a best-effort attempt to fill in all
    /// available fields. If an implementation doesn't, and a downstream user needs the field, it should
    /// try to derive the field from other fields the implementer *does* provide via whatever methods the
    /// platform provides.
    ///
    /// It is not possible to invalidate a [`DisplayHandle`] on any platform without additional unsafe code.
    pub unsafe fn borrow_raw(raw: RawDisplayHandle) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// Get the underlying raw display handle.
    pub fn as_raw(&self) -> RawDisplayHandle {
        self.raw
    }
}

impl AsRef<RawDisplayHandle> for DisplayHandle<'_> {
    fn as_ref(&self) -> &RawDisplayHandle {
        &self.raw
    }
}

impl Borrow<RawDisplayHandle> for DisplayHandle<'_> {
    fn borrow(&self) -> &RawDisplayHandle {
        &self.raw
    }
}

impl From<DisplayHandle<'_>> for RawDisplayHandle {
    fn from(handle: DisplayHandle<'_>) -> Self {
        handle.raw
    }
}

impl<'a> HasDisplayHandle for DisplayHandle<'a> {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(*self)
    }
}

/// A handle to a window.
///
/// Objects that implement this trait should be able to return a [`WindowHandle`] for the window
/// that they are associated with. This handle should last for the lifetime of the object, and should
/// return an error if the application is inactive.
///
/// Implementors of this trait will be windowing systems, like [`winit`] and [`sdl2`]. These windowing
/// systems should implement this trait on types that represent windows.
///
/// Users of this trait will include graphics libraries, like [`wgpu`] and [`glutin`]. These APIs
/// should be generic over a type that implements `HasWindowHandle`, and should use the
/// [`WindowHandle`] type to access the window handle. The window handle should be acquired and held
/// while the window is being used, in order to ensure that the window is not deleted while it is in
/// use.
///
/// [`winit`]: https://crates.io/crates/winit
/// [`sdl2`]: https://crates.io/crates/sdl2
/// [`wgpu`]: https://crates.io/crates/wgpu
/// [`glutin`]: https://crates.io/crates/glutin
pub trait HasWindowHandle {
    /// Get a handle to the window.
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError>;
}

impl<H: HasWindowHandle + ?Sized> HasWindowHandle for &H {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        (**self).window_handle()
    }
}

impl<H: HasWindowHandle + ?Sized> HasWindowHandle for &mut H {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        (**self).window_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<H: HasWindowHandle + ?Sized> HasWindowHandle for alloc::boxed::Box<H> {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        (**self).window_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<H: HasWindowHandle + ?Sized> HasWindowHandle for alloc::rc::Rc<H> {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        (**self).window_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<H: HasWindowHandle + ?Sized> HasWindowHandle for alloc::sync::Arc<H> {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        (**self).window_handle()
    }
}

/// The handle to a window.
///
/// This is the primary return type of the [`HasWindowHandle`] trait. All *pointers* within this type
/// are guaranteed to be valid and not dangling for the lifetime of the handle. This excludes window IDs
/// like XIDs and the window ID for web platforms. See the documentation on the [`HasWindowHandle`]
/// trait for more information about these safety requirements.
///
/// This handle is guaranteed to be safe and valid.
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct WindowHandle<'a> {
    raw: RawWindowHandle,
    _marker: PhantomData<&'a ()>,
}

impl fmt::Debug for WindowHandle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("WindowHandle").field(&self.raw).finish()
    }
}

impl<'a> WindowHandle<'a> {
    /// Borrow a `WindowHandle` from a [`RawWindowHandle`].
    ///
    /// # Safety
    ///
    /// Users can safely assume that non-`null`/`0` fields are valid handles, and it is up to the
    /// implementer of this trait to ensure that condition is upheld.
    ///
    /// Despite that qualification, implementers should still make a best-effort attempt to fill in all
    /// available fields. If an implementation doesn't, and a downstream user needs the field, it should
    /// try to derive the field from other fields the implementer *does* provide via whatever methods the
    /// platform provides.
    ///
    /// Note that this guarantee only applies to *pointers*, and not any window ID types in the handle.
    /// This includes Window IDs (XIDs) from X11 and the window ID for web platforms. There is no way for
    /// Rust to enforce any kind of invariant on these types, since:
    ///
    /// - For all three listed platforms, it is possible for safe code in the same process to delete
    ///   the window.
    /// - For X11, it is possible for code in a different process to delete the window. In fact, it is
    ///   possible for code on a different *machine* to delete the window.
    ///
    /// It is *also* possible for the window to be replaced with another, valid-but-different window. User
    /// code should be aware of this possibility, and should be ready to soundly handle the possible error
    /// conditions that can arise from this.
    pub unsafe fn borrow_raw(raw: RawWindowHandle) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// Get the underlying raw window handle.
    pub fn as_raw(&self) -> RawWindowHandle {
        self.raw.clone()
    }
}

impl AsRef<RawWindowHandle> for WindowHandle<'_> {
    fn as_ref(&self) -> &RawWindowHandle {
        &self.raw
    }
}

impl Borrow<RawWindowHandle> for WindowHandle<'_> {
    fn borrow(&self) -> &RawWindowHandle {
        &self.raw
    }
}

impl From<WindowHandle<'_>> for RawWindowHandle {
    fn from(handle: WindowHandle<'_>) -> Self {
        handle.raw
    }
}

impl HasWindowHandle for WindowHandle<'_> {
    fn window_handle(&self) -> Result<Self, HandleError> {
        Ok(*self)
    }
}
