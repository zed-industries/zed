#![allow(non_snake_case)]

use std::{fmt, hash};

/// A native
/// [handle](https://learn.microsoft.com/en-us/windows/win32/sysinfo/handles-and-objects),
/// implemented by all handle types.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait Handle: Sized
	+ PartialEq + Eq + Send + hash::Hash
	+ fmt::Debug + fmt::Display
	+ fmt::LowerHex + fmt::UpperHex
{
	/// The null, uninitialized handle; equals to `0`.
	const NULL: Self;

	/// The invalid handle; equals to `-1`.
	///
	/// Operations upon this handle will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	const INVALID: Self;

	/// Creates a new handle object by wrapping a pointer.
	///
	/// This method can be used as an escape hatch to interoperate with other
	/// libraries.
	///
	/// # Safety
	///
	/// Be sure the pointer has the correct type and isn't owned by anyone else,
	/// otherwise you may cause memory access violations.
	#[must_use]
	unsafe fn from_ptr(p: *mut std::ffi::c_void) -> Self;

	/// Returns a raw copy of the underlying handle pointer.
	///
	/// # Safety
	///
	/// As the name implies, `raw_copy` returns a raw copy of the handle, so
	/// closing one of the copies won't close the others. This means a handle
	/// can be used after it has been closed, what can lead to errors and
	/// undefined behavior. Even worse: sometimes Windows reuses handle values,
	/// so you can call a method on a completely different handle type, what can
	/// be catastrophic.
	///
	/// However, in some cases the Windows API *demands* a copy of the handle –
	/// `raw_copy` is an escape hatch to fill this gap.
	#[must_use]
	unsafe fn raw_copy(&self) -> Self {
		Self::from_ptr(self.ptr())
	}

	/// Returns a mutable reference to the underlying raw pointer.
	///
	/// This method can be used as an escape hatch to interoperate with other
	/// libraries.
	///
	/// # Safety
	///
	/// This method exposes the raw pointer used by raw Windows calls. It's an
	/// opaque pointer to an internal Windows structure, and no dereferencings
	/// should be attempted.
	#[must_use]
	unsafe fn as_mut(&mut self) -> &mut *mut std::ffi::c_void;

	/// Returns the underlying raw pointer.
	///
	/// This method exposes the raw pointer used by raw Windows calls. It's an
	/// opaque pointer to an internal Windows structure, and no dereferencings
	/// should be attempted.
	///
	/// This method can be used as an escape hatch to interoperate with other
	/// libraries.
	#[must_use]
	fn ptr(&self) -> *mut std::ffi::c_void;

	/// Returns `None` if the handle is null or invalid, otherwise returns
	/// `Some(&self)`.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hfile = w::HFILE::NULL;
	///
	/// match hfile.as_opt() {
	///     Some(hfile) => println!("Never prints"),
	///     None => println!("The handle is null"),
	/// }
	/// ```
	#[must_use]
	fn as_opt(&self) -> Option<&Self> {
		if *self == Self::NULL || *self == Self::INVALID {
			None
		} else {
			Some(self)
		}
	}
}
