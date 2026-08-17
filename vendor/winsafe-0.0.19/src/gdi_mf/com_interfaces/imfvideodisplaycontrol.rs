#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

impl gdi_mf_IMFVideoDisplayControl for IMFVideoDisplayControl {}

/// This trait is enabled with `gdi` and `mf` features, and provides methods
/// for [`IMFVideoDisplayControl`](crate::IMFVideoDisplayControl).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_mf_IMFVideoDisplayControl: mf_IMFVideoDisplayControl {
	/// [`GetCurrentImage`](https://learn.microsoft.com/en-us/windows/win32/api/evr/nf-evr-imfvideodisplaycontrol-getcurrentimage)
	/// method.
	///
	/// Returns bitmap description, DIB bytes and time stamp.
	#[must_use]
	fn GetCurrentImage(&self) -> HrResult<(BITMAPINFOHEADER, Vec<u8>, i64)> {
		let mut bih = BITMAPINFOHEADER::default();
		let mut dib_ptr = std::ptr::null_mut::<u8>();
		let mut dib_sz = u32::default();
		let mut time_stamp = i64::default();

		ok_to_hrresult(
			unsafe {
				(vt::<IMFVideoDisplayControlVT>(self).GetCurrentImage)(
					self.ptr(),
					&mut bih as *mut _ as _,
					&mut dib_ptr,
					&mut dib_sz,
					&mut time_stamp,
				)
			},
		).map(|_| {
			let dib_vec = unsafe {
				std::slice::from_raw_parts(dib_ptr, dib_sz as _)
			}.to_vec();
			let _ = unsafe { CoTaskMemFreeGuard::new(dib_ptr as _, 0) };
			(bih, dib_vec, time_stamp)
		})
	}
}
