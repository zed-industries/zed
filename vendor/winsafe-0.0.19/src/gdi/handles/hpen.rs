#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::gdi::ffi;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl_handle! { HPEN;
	/// Handle to a
	/// [pen](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hpen)
	/// GDI object.
}

impl GdiObject for HPEN {}
impl GdiObjectSelect for HPEN {}
impl gdi_Hpen for HPEN {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HPEN`](crate::HPEN).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_Hpen: Handle {
	/// [`CreatePen`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpen)
	/// function.
	#[must_use]
	fn CreatePen(
		style: co::PS,
		width: i32,
		color: COLORREF,
	) -> SysResult<DeleteObjectGuard<HPEN>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::CreatePen(style.raw(), width, color.into()),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreatePenIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpenindirect)
	/// function.
	#[must_use]
	fn CreatePenIndirect(lp: &mut LOGPEN) -> SysResult<DeleteObjectGuard<HPEN>> {
		unsafe {
			ptr_to_sysresult_handle(ffi::CreatePenIndirect(lp as *const _ as _))
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`GetStockObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// function.
	#[must_use]
	fn GetStockObject(sp: co::STOCK_PEN) -> SysResult<HPEN> {
		ptr_to_sysresult_handle(unsafe { ffi::GetStockObject(sp.raw()) })
	}
}
