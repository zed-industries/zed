#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::{ffi, proc};

impl_handle! { HDC;
	/// Handle to a
	/// [device context](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdc).
}

impl user_Hdc for HDC {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HDC`](crate::HDC).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hdc: Handle {
	/// [`DrawFocusRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawfocusrect)
	/// function.
	fn DrawFocusRect(&self, rect: &RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::DrawFocusRect(self.ptr(), rect as *const _ as _) },
		)
	}

	/// [`DrawText`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawtextw)
	/// function.
	fn DrawText(&self,
		text: &str,
		bounds: &RECT,
		format: co::DT,
	) -> SysResult<i32>
	{
		let wtext = WString::from_str(text);
		match unsafe {
			ffi::DrawTextW(
				self.ptr(),
				wtext.as_ptr(),
				wtext.str_len() as _,
				bounds as *const _ as _,
				format.raw(),
			)
		} {
			0 => Err(GetLastError()),
			i => Ok(i),
		}
	}

	/// [`DrawTextExW`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawtextexw)
	/// function.
	fn DrawTextEx(&self,
		text: &str,
		bounds: &RECT,
		format: co::DT,
		dtp: Option<&DRAWTEXTPARAMS>,
	) -> SysResult<i32>
	{
		let wtext = WString::from_str(text);
		match unsafe {
			ffi::DrawTextExW(
				self.ptr(),
				wtext.as_ptr(),
				wtext.str_len() as _,
				bounds as *const _ as _,
				format.raw(),
				dtp.map_or(std::ptr::null(), |p| p as *const _ as _),
			)
		} {
			0 => Err(GetLastError()),
			i => Ok(i),
		}
	}

	/// [`EnumDisplayMonitors`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaymonitors)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hdc: w::HDC; // initialized somewhere
	/// # let hdc = w::HDC::NULL;
	///
	/// hdc.EnumDisplayMonitors(
	///     None,
	///     |hmon: w::HMONITOR, hdc: w::HDC, rc: &w::RECT| -> bool {
	///         println!("HMONITOR: {}, ", hmon);
	///         true
	///     },
	/// )?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn EnumDisplayMonitors<F>(&self,
		rc_clip: Option<RECT>,
		func: F,
	) -> SysResult<()>
		where F: FnMut(HMONITOR, HDC, &RECT) -> bool,
	{
		bool_to_sysresult(
			unsafe {
				ffi::EnumDisplayMonitors(
					self.ptr(),
					rc_clip.map_or(std::ptr::null_mut(), |rc| &rc as *const _ as _),
					proc::hdc_enum_display_monitors::<F> as _,
					&func as *const _ as _,
				)
			},
		)
	}

	/// [`InvertRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invertrect)
	/// function.
	fn InvertRect(&self, rc: &RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::InvertRect(self.ptr(), rc as *const _ as _) },
		)
	}

	/// [`PaintDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-paintdesktop)
	/// function.
	fn PaintDesktop(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::PaintDesktop(self.ptr()) })
	}

	/// [`WindowFromDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfromdc)
	/// function.
	#[must_use]
	fn WindowFromDC(&self) -> Option<HWND> {
		ptr_to_option_handle(unsafe { ffi::WindowFromDC(self.ptr()) })
	}
}
