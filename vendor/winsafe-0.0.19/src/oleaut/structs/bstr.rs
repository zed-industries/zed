#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::oleaut::ffi;

/// A
/// [string data type](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr)
/// used with COM automation.
///
/// Automatically calls
/// [`SysFreeString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysfreestring)
/// when the object goes out of scope.
#[repr(transparent)]
pub struct BSTR(*mut u16);

impl Drop for BSTR {
	fn drop(&mut self) {
		if !self.0.is_null() {
			unsafe { ffi::SysFreeString(self.0); }
		}
	}
}

impl Default for BSTR {
	fn default() -> Self {
		Self(std::ptr::null_mut())
	}
}

impl From<BSTR> for WString {
	fn from(value: BSTR) -> WString {
		unsafe { WString::from_wchars_nullt(value.as_ptr()) }
	}
}

impl std::fmt::Display for BSTR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(
			unsafe { &WString::from_wchars_nullt(self.as_ptr()) }, f)
	}
}
impl std::fmt::Debug for BSTR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "BSTR \"{}\"", self)
	}
}

impl BSTR {
	/// [`SysAllocString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysallocstring)
	/// function.
	#[must_use]
	pub fn SysAllocString(s: &str) -> HrResult<Self> {
		let str_obj = WString::from_str(s);
		let ptr = unsafe { ffi::SysAllocString(str_obj.as_ptr()) };
		if ptr.is_null() {
			Err(co::HRESULT::E_OUTOFMEMORY)
		} else {
			Ok(Self(ptr))
		}
	}

	/// [`SysReAllocString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysreallocstring)
	/// function.
	///
	/// The underlying pointer is automatically updated.
	pub fn SysReAllocString(&mut self, s: &str) -> HrResult<()> {
		let str_obj = WString::from_str(s);
		let ptr = unsafe { ffi::SysReAllocString(self.0, str_obj.as_ptr()) };
		if ptr.is_null() {
			Err(co::HRESULT::E_OUTOFMEMORY)
		} else {
			self.0 = ptr;
			Ok(())
		}
	}

	/// [`SysStringLen`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysstringlen)
	/// function.
	#[must_use]
	pub fn SysStringLen(&self) -> u32 {
		unsafe { ffi::SysStringLen(self.0) }
	}

	/// Creates a new `BSTR` by wrapping a pointer.
	///
	/// # Safety
	///
	/// Be sure the pointer has the correct type and isn't owned by anyone else,
	/// otherwise you may cause memory access violations.
	#[must_use]
	pub const unsafe fn from_ptr(p: *mut u16) -> Self {
		Self(p)
	}

	/// Returns the underlying
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// pointer to the null-terminated wide string.
	#[must_use]
	pub const fn as_ptr(&self) -> *mut u16 {
		self.0
	}

	/// Returns the underlying
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// memory block as a null-terminated `u16` slice.
	#[must_use]
	pub fn as_slice(&self) -> &[u16] {
		unsafe {
			std::slice::from_raw_parts(self.0, self.SysStringLen() as usize + 1)
		}
	}

	/// Ejects the underlying
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// pointer leaving a null pointer in its place, so that
	/// [`SysFreeString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysfreestring)
	/// won't be called.
	///
	/// Be sure to free the pointer, otherwise, as the name of this method
	/// implies, you will cause a memory leak.
	#[must_use]
	pub fn leak(&mut self) -> *mut u16 {
		std::mem::replace(&mut self.0, std::ptr::null_mut())
	}
}
