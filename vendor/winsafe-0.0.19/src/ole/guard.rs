use crate::co;
use crate::ole::ffi;
use crate::prelude::*;

/// RAII implementation which automatically calls
/// [`CoLockObjectExternal`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-colockobjectexternal)
/// to unlock the COM pointer.
pub struct CoLockObjectExternalGuard<'a, T>
	where T: ole_IUnknown,
{
	com_obj: &'a T,
}

impl<'a, T> Drop for CoLockObjectExternalGuard<'a, T>
	where T: ole_IUnknown,
{
	fn drop(&mut self) {
		unsafe {
			ffi::CoLockObjectExternal(self.com_obj.ptr(), 0, 1); // ignore errors
		}
	}
}

impl<'a, T> CoLockObjectExternalGuard<'a, T>
	where T: ole_IUnknown,
{
	/// Constructs the guard by keeping the reference to the COM pointer.
	///
	/// # Safety
	///
	/// Be sure the COM pointer has been locked with a previous call to
	/// [`CoLockObjectExternal`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-colockobjectexternal).
	#[must_use]
	pub const unsafe fn new(com_obj: &'a T) -> Self {
		Self { com_obj }
	}
}

//------------------------------------------------------------------------------

/// RAII implementation which automatically calls
/// [`CoTaskMemFree`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemfree)
/// when the object goes out of scope.
pub struct CoTaskMemFreeGuard {
	pmem: *mut std::ffi::c_void,
	sz: usize,
}

impl Drop for CoTaskMemFreeGuard {
	fn drop(&mut self) {
		if !self.pmem.is_null() {
			unsafe { ffi::CoTaskMemFree(self.pmem); }
		}
	}
}

impl CoTaskMemFreeGuard {
	/// Constructs the guard.
	///
	/// # Safety
	///
	/// Be sure the pointer must be freed with
	/// [`CoTaskMemFree`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemfree)
	/// at the end of the scope, and the size is correct.
	#[must_use]
	pub const unsafe fn new(pmem: *mut std::ffi::c_void, sz: usize) -> Self {
		Self { pmem, sz }
	}

	/// Ejects the underlying memory pointer and size, leaving null and zero in
	/// their places.
	///
	/// Since the internal memory pointer will be invalidated, the destructor
	/// will not run. It's your responsibility to run it, otherwise you'll cause
	/// a memory leak.
	#[must_use]
	pub fn leak(&mut self) -> (*mut std::ffi::c_void, usize) {
		(
			std::mem::replace(&mut self.pmem, std::ptr::null_mut()),
			std::mem::replace(&mut self.sz, 0),
		)
	}

	pub_fn_mem_block!();
}

//------------------------------------------------------------------------------

/// RAII implementation which automatically calls
/// [`CoUninitialize`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize)
/// when the object goes out of scope.
pub struct CoUninitializeGuard {
	hr: co::HRESULT,
}

impl Drop for CoUninitializeGuard {
	fn drop(&mut self) {
		unsafe { ffi::CoUninitialize(); }
	}
}

impl CoUninitializeGuard {
	/// Constructs the guard by taking ownership of the code.
	///
	/// # Safety
	///
	/// Be sure you need to call
	/// [`CoUninitialize`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(hr: co::HRESULT) -> Self {
		Self { hr }
	}

	/// Returns the informational success code returned by
	/// [`CoInitializeEx`](crate::CoInitializeEx).
	#[must_use]
	pub const fn hr(&self) -> co::HRESULT {
		self.hr
	}
}
