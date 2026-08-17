#![allow(non_snake_case)]

use crate::decl::*;
use crate::gdi::ffi;
use crate::kernel::privs::*;

/// [`GdiFlush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gdiflush)
/// function.
pub fn GdiFlush() -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::GdiFlush() })
}

/// [`GdiGetBatchLimit`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gdigetbatchlimit)
/// function.
#[must_use]
pub fn GdiGetBatchLimit() -> u32 {
	unsafe { ffi::GdiGetBatchLimit() }
}

/// [`GdiSetBatchLimit`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gdisetbatchlimit)
/// function.
pub fn GdiSetBatchLimit(limit: u32) -> SysResult<u32> {
	match unsafe { ffi::GdiSetBatchLimit(limit) } {
		0 => Err(GetLastError()),
		n => Ok(n),
	}
}
