#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::ole::privs::*;
use crate::oleaut::ffi;
use crate::prelude::*;

/// [`OleLoadPicture`](https://learn.microsoft.com/en-us/windows/win32/api/olectl/nf-olectl-oleloadpicture)
/// function.
///
/// # Examples
///
/// Parsing an image from raw data:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let stream: w::IStream; // initialized somewhere
/// # let stream = unsafe { w::IStream::null() };
///
/// let picture = w::OleLoadPicture(&stream, None, true)?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[must_use]
pub fn OleLoadPicture(
	stream: &impl ole_IStream,
	size: Option<u32>,
	keep_original_format: bool,
) -> HrResult<IPicture>
{
	let mut queried = unsafe { IPicture::null() };
	ok_to_hrresult(
		unsafe {
			ffi::OleLoadPicture(
				stream.ptr() as _,
				size.unwrap_or_default() as _,
				!keep_original_format as _, // note: reversed
				&IPicture::IID as *const _ as _,
				queried.as_mut(),
			)
		},
	).map(|_| queried)
}

/// [`OleLoadPicturePath`](https://learn.microsoft.com/en-us/windows/win32/api/olectl/nf-olectl-oleloadpicturepath)
/// function.
///
/// The picture must be in BMP (bitmap), JPEG, WMF (metafile), ICO (icon), or
/// GIF format.
#[must_use]
pub fn OleLoadPicturePath(
	path: &str,
	transparent_color: Option<COLORREF>,
) -> HrResult<IPicture>
{
	let mut queried = unsafe { IPicture::null() };
	ok_to_hrresult(
		unsafe {
			ffi::OleLoadPicturePath(
				WString::from_str(path).as_ptr(),
				std::ptr::null_mut(),
				0,
				transparent_color.map_or(0, |c| c.into()),
				&IPicture::IID as *const _ as _,
				queried.as_mut(),
			)
		},
	).map(|_| queried)
}

/// [`PSGetNameFromPropertyKey`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-psgetnamefrompropertykey)
/// function.
#[must_use]
pub fn PSGetNameFromPropertyKey(prop_key: &PROPERTYKEY) -> HrResult<String> {
	let mut pstr = std::ptr::null_mut::<u16>();
	ok_to_hrresult(
		unsafe {
			ffi::PSGetNameFromPropertyKey(
				prop_key as *const _ as _,
				&mut pstr,
			)
		},
	).map(|_| {
		let name = unsafe { WString::from_wchars_nullt(pstr) };
		let _ = unsafe { CoTaskMemFreeGuard::new(pstr as _, 0) };
		name.to_string()
	})
}

/// [`SystemTimeToVariantTime`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-systemtimetovarianttime)
/// function. The inverse operation is performed by
/// [`VariantTimeToSystemTime`](crate::VariantTimeToSystemTime).
///
/// Note that this function resolves the time to one second; milliseconds are
/// ignored.
#[must_use]
pub fn SystemTimeToVariantTime(st: &SYSTEMTIME) -> SysResult<f64> {
	let mut double = f64::default();
	match unsafe {
		ffi::SystemTimeToVariantTime(st as *const _ as _, &mut double)
	} {
		0 => Err(co::ERROR::INVALID_PARAMETER),
		_ => Ok(double),
	}
}

/// [`VariantTimeToSystemTime`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-varianttimetosystemtime)
/// function. The inverse operation is performed by
/// [`SystemTimeToVariantTime`](SystemTimeToVariantTime).
#[must_use]
pub fn VariantTimeToSystemTime(
	var_time: f64,
	st: &mut SYSTEMTIME,
) -> SysResult<()>
{
	match unsafe {
		ffi::VariantTimeToSystemTime(var_time, st as *mut _ as _)
	} {
		0 => Err(co::ERROR::INVALID_PARAMETER),
		_ => Ok(()),
	}
}
