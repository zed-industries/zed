#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dwm::ffi;
use crate::ole::privs::*;
use crate::prelude::*;

impl dwm_Hwnd for HWND {}

/// This trait is enabled with the `dwm` feature, and provides methods for
/// [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dwm_Hwnd: uxtheme_Hwnd {
	/// [`DwmExtendFrameIntoClientArea`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmextendframeintoclientarea)
	/// function.
	fn DwmExtendFrameIntoClientArea(&self,
		margins_inset: &MARGINS,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				ffi::DwmExtendFrameIntoClientArea(
					self.ptr(),
					margins_inset as *const _ as _,
				)
			},
		)
	}

	/// [`DwmInvalidateIconicBitmaps`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwminvalidateiconicbitmaps)
	/// function.
	fn DwmInvalidateIconicBitmaps(&self) -> HrResult<()> {
		ok_to_hrresult(
			unsafe { ffi::DwmInvalidateIconicBitmaps(self.ptr()) },
		)
	}

	/// [`DwmSetIconicLivePreviewBitmap`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmseticoniclivepreviewbitmap)
	/// function.
	fn DwmSetIconicLivePreviewBitmap(&self,
		hbmp: HBITMAP,
		pt_client: Option<POINT>,
		sit_flags: Option<co::DWM_SIT>,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				ffi::DwmSetIconicLivePreviewBitmap(
					self.ptr(),
					hbmp.ptr(),
					pt_client.map_or(std::ptr::null(), |pt| &pt as *const _ as _),
					sit_flags.unwrap_or_default().raw(),
				)
			},
		)
	}

	/// [`DwmSetIconicThumbnail`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmseticonicthumbnail)
	/// function.
	fn DwmSetIconicThumbnail(&self,
		hbmp: HBITMAP,
		sit_flags: Option<co::DWM_SIT>,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				ffi::DwmSetIconicThumbnail(
					self.ptr(),
					hbmp.ptr(),
					sit_flags.unwrap_or_default().raw(),
				)
			},
		)
	}
}
