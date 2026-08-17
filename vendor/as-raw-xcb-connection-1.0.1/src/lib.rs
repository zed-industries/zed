//! This crate provides the trait [`AsRawXcbConnection`].
//!
//! The idea is to facilitate interoperability in the ecosystem. The problem is as follows:
//!
//! There are multiple crates that wrap the libxcb C API to provide a "connection" type. There are
//! also multiple crates wrapping various C libraries that need a pointer to `xcb_connection_t`
//! to work correctly.
//!
//! Without this library, API consumers must pick one Rust library that wraps libxcb and only
//! accept this type in its public API. Worse, one must also pick a specific version of the crate
//! and would then only work with that type.
//!
//! The trait [`AsRawXcbConnection`] breaks this connection. All libraries that wrap libxcb can
//! implement this trait. This makes one independent from specific versions of API consumer crates.

#![allow(non_camel_case_types)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::ptr::NonNull;

/// XCB connection
///
/// This type represents `xcb_connection_t` in C. It is only ever referenced via a pointer.
pub enum xcb_connection_t {}

/// A trait to extract a raw `xcb_connection_t` from an object.
///
/// # Safety
///
/// This trait is unsafe. Implementations must provide a valid connection pointer that can be used
/// with libxcb C functions. This pointer must be valid for as long as the object on which this
/// trait is implemented. This means that the connection cannot be deallocated while the object is
/// still in use.
pub unsafe trait AsRawXcbConnection {
    /// Get a raw xcb connection pointer from this object.
    fn as_raw_xcb_connection(&self) -> *mut xcb_connection_t;
}

// Implementations for reference types

unsafe impl<T: AsRawXcbConnection + ?Sized> AsRawXcbConnection for &T {
    fn as_raw_xcb_connection(&self) -> *mut xcb_connection_t {
        (**self).as_raw_xcb_connection()
    }
}

unsafe impl<T: AsRawXcbConnection + ?Sized> AsRawXcbConnection for &mut T {
    fn as_raw_xcb_connection(&self) -> *mut xcb_connection_t {
        (**self).as_raw_xcb_connection()
    }
}

#[cfg(feature = "alloc")]
unsafe impl<T: AsRawXcbConnection + ?Sized> AsRawXcbConnection for alloc::boxed::Box<T> {
    fn as_raw_xcb_connection(&self) -> *mut xcb_connection_t {
        (**self).as_raw_xcb_connection()
    }
}

#[cfg(feature = "alloc")]
unsafe impl<T: AsRawXcbConnection + ?Sized> AsRawXcbConnection for alloc::rc::Rc<T> {
    fn as_raw_xcb_connection(&self) -> *mut xcb_connection_t {
        (**self).as_raw_xcb_connection()
    }
}

#[cfg(feature = "alloc")]
unsafe impl<T: AsRawXcbConnection + ?Sized> AsRawXcbConnection for alloc::sync::Arc<T> {
    fn as_raw_xcb_connection(&self) -> *mut xcb_connection_t {
        (**self).as_raw_xcb_connection()
    }
}

#[cfg(feature = "alloc")]
unsafe impl<T: AsRawXcbConnection + alloc::borrow::ToOwned + ?Sized> AsRawXcbConnection
    for alloc::borrow::Cow<'_, T>
{
    fn as_raw_xcb_connection(&self) -> *mut xcb_connection_t {
        (**self).as_raw_xcb_connection()
    }
}

/// An assertion that this pointer is valid for as long as the underlying connection.
///
/// This type provides an escape hatch for users who want to use a raw pointer to `xcb_connection_t`
/// but still want to use the safety guarantees of this crate. By constructing an instance of this
/// type, users can assert that the pointer is valid for as long as the underlying connection.
pub struct ValidConnection(NonNull<xcb_connection_t>);

impl ValidConnection {
    /// Create a new `ValidConnection` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be valid for as long as the underlying connection.
    pub unsafe fn new(ptr: *mut xcb_connection_t) -> Self {
        // SAFETY: Valid pointer implies non-null pointer.
        Self(NonNull::new_unchecked(ptr))
    }
}

unsafe impl AsRawXcbConnection for ValidConnection {
    fn as_raw_xcb_connection(&self) -> *mut xcb_connection_t {
        self.0.as_ptr()
    }
}
