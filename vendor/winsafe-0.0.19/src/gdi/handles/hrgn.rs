#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::gdi::ffi;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl GdiObject for HRGN {}
impl GdiObjectSelect for HRGN {}
impl gdi_Hrgn for HRGN {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HRGN`](crate::HRGN).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_Hrgn: Handle {
	/// [`CreateRectRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createrectrgn)
	/// function.
	#[must_use]
	fn CreateRectRgn(bounds: RECT) -> SysResult<DeleteObjectGuard<HRGN>> {
		unsafe {
			ptr_to_sysresult_handle(
				ffi::CreateRectRgn(
					bounds.left, bounds.top, bounds.right, bounds.bottom),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateRectRgnIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createrectrgnindirect)
	/// function.
	#[must_use]
	fn CreateRectRgnIndirect(rc: RECT) -> SysResult<DeleteObjectGuard<HRGN>> {
		unsafe {
			ptr_to_sysresult_handle(
				ffi::CreateRectRgnIndirect(&rc as *const _ as _),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateRoundRectRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createroundrectrgn)
	/// function.
	#[must_use]
	fn CreateRoundRectRgn(
		bounds: RECT,
		size: SIZE,
	) -> SysResult<DeleteObjectGuard<HRGN>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::CreateRoundRectRgn(
					bounds.left, bounds.top, bounds.right, bounds.top,
					size.cx, size.cy,
				),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`OffsetClipRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-offsetcliprgn)
	/// function.
	fn OffsetClipRgn(&self, x: i32, y: i32) -> SysResult<co::REGION> {
		match unsafe { ffi::OffsetClipRgn(self.ptr(), x, y) } {
			0 => Err(GetLastError()),
			ret => Ok(unsafe { co::REGION::from_raw(ret) }),
		}
	}

	/// [`OffsetRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-offsetrgn)
	/// function.
	fn OffsetRgn(&self, x: i32, y: i32) -> SysResult<co::REGION> {
		match unsafe { ffi::OffsetRgn(self.ptr(), x, y) } {
			0 => Err(GetLastError()),
			ret => Ok(unsafe { co::REGION::from_raw(ret) }),
		}
	}

	/// [`PtInRegion`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ptinregion)
	/// function.
	#[must_use]
	fn PtInRegion(&self, x: i32, y: i32) -> bool {
		unsafe { ffi::PtInRegion(self.ptr(), x, y) != 0 }
	}

	/// [`RectInRegion`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectinregion)
	/// function.
	#[must_use]
	fn RectInRegion(&self, rc: &RECT) -> bool {
		unsafe { ffi::RectInRegion(self.ptr(), rc as *const _ as _) != 0 }
	}
}
