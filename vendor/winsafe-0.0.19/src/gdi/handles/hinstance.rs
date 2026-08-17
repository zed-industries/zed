#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::gdi::ffi;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl gdi_Hinstance for HINSTANCE {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HINSTANCE`](crate::HINSTANCE).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_Hinstance: user_Hinstance {
	/// [`LoadImage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HBITMAP`](crate::HBITMAP).
	#[must_use]
	fn LoadImageBitmap(&self,
		name: IdObmStr,
		sz: SIZE,
		load: co::LR,
	) -> SysResult<DeleteObjectGuard<HBITMAP>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::LoadImageW(
					self.ptr(), name.as_ptr(), 0, sz.cx, sz.cy, load.raw()),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`LoadImage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HCURSOR`](crate::HCURSOR).
	#[must_use]
	fn LoadImageCursor(&self,
		name: IdOcrStr,
		sz: SIZE,
		load: co::LR,
	) -> SysResult<DestroyCursorGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::LoadImageW(
					self.ptr(), name.as_ptr(), 2, sz.cx, sz.cy, load.raw()),
			).map(|h| DestroyCursorGuard::new(h))
		}
	}

	/// [`LoadImage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HICON`](crate::HICON).
	#[must_use]
	fn LoadImageIcon(&self,
		name: IdOicStr,
		sz: SIZE,
		load: co::LR,
	) -> SysResult<DestroyIconGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::LoadImageW(
					self.ptr(), name.as_ptr(), 1, sz.cx, sz.cy, load.raw()),
			).map(|h| DestroyIconGuard::new(h))
		}
	}
}
