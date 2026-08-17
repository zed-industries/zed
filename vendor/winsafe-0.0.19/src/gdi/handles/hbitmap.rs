#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::gdi::ffi;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl GdiObject for HBITMAP {}
impl GdiObjectSelect for HBITMAP {}
impl gdi_Hbitmap for HBITMAP {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HBITMAP`](crate::HBITMAP).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_Hbitmap: Handle {
	/// [`CreateBitmap`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createbitmap)
	/// function.
	#[must_use]
	fn CreateBitmap(
		sz: SIZE,
		num_planes: u32,
		bit_count: u32,
		bits: *mut u8,
	) -> SysResult<DeleteObjectGuard<HBITMAP>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::CreateBitmap(sz.cx, sz.cy, num_planes, bit_count, bits as _),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`GetObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hbmp: w::HBITMAP; // initialized somewhere
	/// # let hbmp = w::HBITMAP::NULL;
	///
	/// let mut bitmap = w::BITMAP::default();
	/// hbmp.GetObject(&mut bitmap)?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn GetObject(&self, pv: &mut BITMAP) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::GetObjectW(
					self.ptr(),
					std::mem::size_of::<BITMAP>() as _,
					pv as *mut _ as _,
				)
			},
		)
	}
}
