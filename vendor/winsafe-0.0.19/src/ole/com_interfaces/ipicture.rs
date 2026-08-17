#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IPicture`](crate::IPicture) virtual table.
#[repr(C)]
pub struct IPictureVT {
	pub IUnknownVT: IUnknownVT,
	pub get_Handle: fn(COMPTR, *mut u32) -> HRES,
	pub get_hPal: fn(COMPTR, *mut u32) -> HRES,
	pub get_Type: fn(COMPTR, *mut i16) -> HRES,
	pub get_Width: fn(COMPTR, *mut i32) -> HRES,
	pub get_Height: fn(COMPTR, *mut i32) -> HRES,
	pub Render: fn(COMPTR, HANDLE, i32, i32, i32, i32, i32, i32, i32, i32, PCVOID) -> HRES,
	pub set_hPal: fn(COMPTR, u32) -> HRES,
	pub get_CurDC: fn(COMPTR, *mut HANDLE) -> HRES,
	pub SelectPicture: fn(COMPTR, HANDLE, *mut HANDLE, *mut HANDLE) -> HRES,
	pub get_KeepOriginalFormat: fn(COMPTR, *mut BOOL) -> HRES,
	pub put_KeepOriginalFormat: fn(COMPTR, BOOL) -> HRES,
	pub PictureChanged: fn(COMPTR) -> HRES,
	pub SaveAsFile: fn(COMPTR, *mut COMPTR, BOOL, *mut i32) -> HRES,
	pub get_Attributes: fn(COMPTR, *mut u32) -> HRES,
}

com_interface! { IPicture: "7bf80980-bf32-101a-8bbb-00aa00300cab";
	/// [`IPicture`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nn-ocidl-ipicture)
	/// COM interface over [`IPictureVT`](crate::vt::IPictureVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IPicture for IPicture {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IPicture`](crate::IPicture).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IPicture: ole_IUnknown {
	/// [`IPicture::get_CurDC`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_curdc)
	/// method.
	#[must_use]
	fn get_CurDC(&self) -> HrResult<HDC> {
		let mut hdc = HDC::NULL;
		ok_to_hrresult(
			unsafe {
				(vt::<IPictureVT>(self).get_CurDC)(self.ptr(), hdc.as_mut())
			},
		).map(|_| hdc)
	}

	/// [`IPicture::get_Height`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_height)
	/// method.
	///
	/// **Note:** Returns a value in HIMETRIC units. To convert it to pixels,
	/// use
	/// [`HDC::HiMetricToPixel`](crate::prelude::gdi_Hdc::HiMetricToPixel).
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{self as w, prelude::*};
	///
	/// let pic: w::IPicture; // initialized somewhere
	/// # let pic = unsafe { w::IPicture::null() };
	///
	/// let hdc = w::HWND::NULL.GetDC()?;
	///
	/// let (_, height) = hdc.HiMetricToPixel(0, pic.get_Height()?);
	/// println!("Height: {} px", height);
	/// # Ok::<_, Box<dyn std::error::Error>>(())
	/// ```
	#[must_use]
	fn get_Height(&self) -> HrResult<i32> {
		let mut h = i32::default();
		ok_to_hrresult(
			unsafe { (vt::<IPictureVT>(self).get_Height)(self.ptr(), &mut h) },
		).map(|_| h)
	}

	/// [`IPicture::get_Type`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_type)
	/// method.
	#[must_use]
	fn get_Type(&self) -> HrResult<co::PICTYPE> {
		let mut ty = i16::default();
		ok_to_hrresult(
			unsafe { (vt::<IPictureVT>(self).get_Type)(self.ptr(), &mut ty) },
		).map(|_| unsafe { co::PICTYPE::from_raw(ty) })
	}

	/// [`IPicture::get_Width`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-get_width)
	/// method.
	///
	/// **Note:** Returns a value in HIMETRIC units. To convert it to pixels,
	/// use
	/// [`HDC::HiMetricToPixel`](crate::prelude::gdi_Hdc::HiMetricToPixel).
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{self as w, prelude::*};
	///
	/// let pic: w::IPicture; // initialized somewhere
	/// # let pic = unsafe { w::IPicture::null() };
	///
	/// let hdc = w::HWND::NULL.GetDC()?;
	///
	/// let (width, _) = hdc.HiMetricToPixel(pic.get_Width()?, 0);
	/// println!("Width: {} px", width);
	/// # Ok::<_, Box<dyn std::error::Error>>(())
	/// ```
	#[must_use]
	fn get_Width(&self) -> HrResult<i32> {
		let mut w = i32::default();
		ok_to_hrresult(
			unsafe { (vt::<IPictureVT>(self).get_Width)(self.ptr(), &mut w) },
		).map(|_| w)
	}

	fn_com_noparm! { PictureChanged: IPictureVT;
		/// [`IPicture::PictureChanged`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-picturechanged)
		/// method.
	}

	/// [`IPicture::put_KeepOriginalFormat`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-put_keeporiginalformat)
	/// method.
	fn put_KeepOriginalFormat(&self, keep: bool) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IPictureVT>(self).put_KeepOriginalFormat)(
					self.ptr(),
					keep as _,
				)
			},
		)
	}

	/// [`IPicture::Render`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-render)
	/// method.
	fn Render(&self,
		hdc: &HDC,
		dest_pt: POINT,
		dest_sz: SIZE,
		src_offset: Option<POINT>,
		src_extent: SIZE,
		metafile_bounds: Option<&RECT>,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IPictureVT>(self).Render)(
					self.ptr(),
					hdc.ptr(),
					dest_pt.x, dest_pt.y,
					dest_sz.cx, dest_sz.cy,
					src_offset.map_or(0, |off| off.x),
					src_offset.map_or(0, |off| off.y),
					src_extent.cx, src_extent.cy,
					metafile_bounds.map_or(std::ptr::null_mut(), |rc| rc as *const _ as _),
				)
			},
		)
	}

	/// [`IPicture::SelectPicture`](https://learn.microsoft.com/en-us/windows/win32/api/ocidl/nf-ocidl-ipicture-selectpicture)
	/// method.
	fn SelectPicture(&self, hdc: &HDC) -> HrResult<(HDC, HBITMAP)> {
		let mut hdc_out = HDC::NULL;
		let mut hbmp = HBITMAP::NULL;

		ok_to_hrresult(
			unsafe {
				(vt::<IPictureVT>(self).SelectPicture)(
					self.ptr(),
					hdc.ptr(),
					hdc_out.as_mut(),
					hbmp.as_mut(),
				)
			},
		).map(|_| (hdc_out, hbmp))
	}
}
