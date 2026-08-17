#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::uxtheme::ffi;

impl_handle! { HTHEME;
	/// Handle to a
	/// [theme](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/).
}

impl uxtheme_Htheme for HTHEME {}

/// This trait is enabled with the `uxtheme` feature, and provides methods for
/// [`HTHEME`](crate::HTHEME).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait uxtheme_Htheme: Handle {
	/// [`DrawThemeBackground`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-drawthemebackground)
	/// function.
	fn DrawThemeBackground(&self,
		hdc: &HDC,
		part_state: co::VS,
		rc: RECT,
		rc_clip: RECT,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				ffi::DrawThemeBackground(
					self.ptr(),
					hdc.ptr(),
					part_state.part,
					part_state.state,
					&rc as *const _ as _,
					&rc_clip as *const _ as _,
				)
			},
		)
	}

	/// [`GetThemeAppProperties`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemeappproperties)
	/// function.
	#[must_use]
	fn GetThemeAppProperties() -> co::STAP {
		unsafe { co::STAP::from_raw(ffi::GetThemeAppProperties()) }
	}

	/// [`GetThemeBackgroundContentRect`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundcontentrect)
	/// function.
	#[must_use]
	fn GetThemeBackgroundContentRect(&self,
		hdc: &HDC,
		part_state: co::VS,
		bounds: RECT,
	) -> HrResult<RECT>
	{
		let mut rc_content = RECT::default();

		ok_to_hrresult(
			unsafe {
				ffi::GetThemeBackgroundContentRect(
					self.ptr(),
					hdc.ptr(),
					part_state.part,
					part_state.state,
					&bounds as *const _ as _,
					&mut rc_content as *mut _ as _,
				)
			},
		).map(|_| rc_content)
	}

	/// [`GetThemeBackgroundExtent`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundextent)
	/// function.
	#[must_use]
	fn GetThemeBackgroundExtent(&self,
		hdc: &HDC,
		part_state: co::VS,
		rc_content: RECT,
	) -> HrResult<RECT>
	{
		let mut rc_extent = RECT::default();

		ok_to_hrresult(
			unsafe {
				ffi::GetThemeBackgroundExtent(
					self.ptr(),
					hdc.ptr(),
					part_state.part,
					part_state.state,
					&rc_content as *const _ as _,
					&mut rc_extent as *mut _ as _,
				)
			},
		 ).map(|_| rc_extent)
	}

	/// [`GetThemeBackgroundRegion`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundregion)
	/// function.
	#[must_use]
	fn GetThemeBackgroundRegion(&self,
		hdc: &HDC,
		part_state: co::VS,
		rc: RECT,
	) -> HrResult<DeleteObjectGuard<HRGN>>
	{
		let mut hrgn = HRGN::NULL;
		unsafe {
			ok_to_hrresult(
				ffi::GetThemeBackgroundRegion(
					self.ptr(),
					hdc.ptr(),
					part_state.part,
					part_state.state,
					&rc as *const _ as _,
					hrgn.as_mut(),
				),
			).map(|_| DeleteObjectGuard::new(hrgn))
		}
	}

	/// [`GetThemeColor`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemecolor)
	/// function.
	#[must_use]
	fn GetThemeColor(&self,
		part_state: co::VS,
		prop: co::TMT,
	) -> HrResult<COLORREF>
	{
		let mut color = COLORREF::default();
		ok_to_hrresult(
			unsafe {
				ffi::GetThemeColor(
					self.ptr(),
					part_state.part,
					part_state.state,
					prop.raw(),
					color.as_mut(),
				)
			},
		).map(|_| color)
	}

	/// [`GetThemeMargins`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthememargins)
	/// function.
	#[must_use]
	fn GetThemeMargins(&self,
		hdc_fonts: Option<&HDC>,
		part_state: co::VS,
		prop: co::TMT,
		draw_dest: Option<&RECT>,
	) -> HrResult<MARGINS>
	{
		let mut margins = MARGINS::default();
		ok_to_hrresult(
			unsafe {
				ffi::GetThemeMargins(
					self.ptr(),
					hdc_fonts.map_or(std::ptr::null_mut(), |h| h.ptr()),
					part_state.part(),
					part_state.state(),
					prop.raw(),
					draw_dest.map_or(std::ptr::null(), |p| p as *const _ as _),
					&mut margins as *mut _ as _,
				)
			},
		).map(|_| margins)
	}

	/// [`GetThemeMetric`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthememetric)
	/// function.
	#[must_use]
	fn GetThemeMetric(&self,
		hdc_fonts: Option<&HDC>,
		part_state: co::VS,
		prop: co::TMT,
	) -> HrResult<i32>
	{
		let mut val = i32::default();
		ok_to_hrresult(
			unsafe {
				ffi::GetThemeMetric(
					self.ptr(),
					hdc_fonts.map_or(std::ptr::null_mut(), |h| h.ptr()),
					part_state.part(),
					part_state.state(),
					prop.raw(),
					&mut val,
				)
			},
		).map(|_| val)
	}

	/// [`GetThemePartSize`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemepartsize)
	/// function.
	#[must_use]
	fn GetThemePartSize(&self,
		hdc_fonts: Option<&HDC>,
		part_state: co::VS,
		draw_dest: Option<&RECT>,
		esize: co::THEMESIZE,
	) -> HrResult<SIZE>
	{
		let mut sz = SIZE::default();
		ok_to_hrresult(
			unsafe {
				ffi::GetThemePartSize(
					self.ptr(),
					hdc_fonts.map_or(std::ptr::null_mut(), |h| h.ptr()),
					part_state.part(),
					part_state.state(),
					draw_dest.map_or(std::ptr::null(), |p| p as *const _ as _),
					esize.raw(),
					&mut sz as *mut _ as _,
				)
			}
		).map(|_| sz)
	}

	/// [`GetThemePosition`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemeposition)
	/// function.
	#[must_use]
	fn GetThemePosition(&self,
		part_state: co::VS,
		prop: co::TMT,
	) -> HrResult<POINT>
	{
		let mut pt = POINT::default();
		ok_to_hrresult(
			unsafe {
				ffi::GetThemePosition(
					self.ptr(),
					part_state.part(),
					part_state.state(),
					prop.raw(),
					&mut pt as *mut _ as _,
				)
			},
		).map(|_| pt)
	}

	/// [`GetThemePropertyOrigin`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemepropertyorigin)
	/// function.
	#[must_use]
	fn GetThemePropertyOrigin(&self,
		part_state: co::VS,
		prop: co::TMT,
	) -> HrResult<co::PROPERTYORIGIN>
	{
		let mut origin = co::PROPERTYORIGIN::default();
		ok_to_hrresult(
			unsafe {
				ffi::GetThemePropertyOrigin(
					self.ptr(),
					part_state.part(),
					part_state.state(),
					prop.raw(),
					origin.as_mut(),
				)
			},
		).map(|_| origin)
	}

	/// [`GetThemeRect`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemerect)
	/// function.
	#[must_use]
	fn GetThemeRect(&self,
		part_state: co::VS,
		prop: co::TMT,
	) -> HrResult<RECT>
	{
		let mut rc = RECT::default();
		ok_to_hrresult(
			unsafe {
				ffi::GetThemeRect(
					self.ptr(),
					part_state.part(),
					part_state.state(),
					prop.raw(),
					&mut rc as *mut _ as _,
				)
			},
		).map(|_| rc)
	}

	/// [`IsThemeBackgroundPartiallyTransparent`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemebackgroundpartiallytransparent)
	/// function.
	#[must_use]
	fn IsThemeBackgroundPartiallyTransparent(&self,
		part_state: co::VS,
	) -> bool
	{
		unsafe {
			ffi::IsThemeBackgroundPartiallyTransparent(
				self.ptr(), part_state.part, part_state.state) != 0
		}
	}

	/// [`IsThemePartDefined`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemepartdefined)
	/// function.
	#[must_use]
	fn IsThemePartDefined(&self, part_state: co::VS) -> bool {
		unsafe {
			ffi::IsThemePartDefined(
				self.ptr(), part_state.part, part_state.state) != 0
		}
	}
}
