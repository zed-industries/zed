#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};
use crate::prelude::*;

impl_handle! { HGLOBAL;
	/// Handle to a
	/// [global memory block](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalalloc).
	/// Originally just a `HANDLE`.
	///
	/// The allocated memory block is accessible through the
	/// [`GlobalLock`](crate::prelude::kernel_Hglobal::GlobalLock) method.
}

impl kernel_Hglobal for HGLOBAL {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HGLOBAL`](crate::HGLOBAL).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hglobal: Handle {
	/// [`GlobalAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalalloc)
	/// function.
	#[must_use]
	fn GlobalAlloc(
		flags: Option<co::GMEM>,
		num_bytes: usize,
	) -> SysResult<GlobalFreeGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::GlobalAlloc(flags.unwrap_or_default().raw(), num_bytes),
			).map(|h| GlobalFreeGuard::new(h))
		}
	}

	/// [`GlobalFlags`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalflags)
	/// function.
	#[must_use]
	fn GlobalFlags(&self) -> SysResult<co::GMEM> {
		match unsafe { ffi::GlobalFlags(self.ptr()) } {
			GMEM_INVALID_HANDLE => Err(GetLastError()),
			flags => Ok(unsafe { co::GMEM::from_raw(flags) }),
		}
	}

	/// [`GlobalLock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globallock)
	/// function.
	///
	/// Calls [`GlobalSize`](crate::prelude::kernel_Hglobal::GlobalSize) to
	/// retrieve the size of the memory block.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hglobal = w::HGLOBAL::GlobalAlloc(
	///     Some(co::GMEM::FIXED | co::GMEM::ZEROINIT),
	///     120,
	/// )?;
	///
	/// let mut block = hglobal.GlobalLock()?;
	///
	/// block.as_mut_slice()[0] = 40;
	///
	/// // GlobalUnlock() called automatically
	///
	/// // GlobalFree() called automatically
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn GlobalLock(&self) -> SysResult<GlobalUnlockGuard<'_, Self>> {
		let mem_sz = self.GlobalSize()?;
		unsafe {
			ptr_to_sysresult(ffi::GlobalLock(self.ptr()))
				.map(|ptr| GlobalUnlockGuard::new(self, ptr, mem_sz),
			)
		}
	}

	/// [`GlobalReAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalrealloc)
	/// function.
	///
	/// Originally this method returns the handle to the reallocated memory
	/// object; here the original handle is automatically updated.
	fn GlobalReAlloc(&mut self,
		num_bytes: usize,
		flags: Option<co::GMEM>,
	) -> SysResult<()>
	{
		ptr_to_sysresult_handle(
			unsafe {
				ffi::GlobalReAlloc(
					self.ptr(),
					num_bytes,
					flags.unwrap_or_default().raw(),
				)
			},
		).map(|h| { *self = h; })
	}

	/// [`GlobalSize`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalsize)
	/// function.
	#[must_use]
	fn GlobalSize(&self) -> SysResult<usize> {
		match unsafe { ffi::GlobalSize(self.ptr()) } {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}
}
