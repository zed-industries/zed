#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::ffi;

impl user_Hinstance for HINSTANCE {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HINSTANCE`](crate::HINSTANCE).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hinstance: kernel_Hinstance {
	/// [`CreateDialogParam`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdialogparamw)
	/// function.
	///
	/// # Safety
	///
	/// To create a dialog, you must provide a dialog procedure.
	unsafe fn CreateDialogParam(&self,
		resource_id: IdStr,
		hwnd_parent: Option<&HWND>,
		dialog_proc: DLGPROC,
		init_param: Option<isize>,
	) -> SysResult<HWND>
	{
		ptr_to_sysresult_handle(
			unsafe {
				ffi::CreateDialogParamW(
					self.ptr(),
					resource_id.as_ptr(),
					hwnd_parent.map_or(std::ptr::null_mut(), |h| h.ptr()),
					dialog_proc as _,
					init_param.unwrap_or_default(),
				)
			},
		)
	}

	/// [`DialogBoxIndirectParam`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dialogboxindirectparamw)
	/// function.
	///
	/// # Safety
	///
	/// To create a dialog, you must provide a dialog procedure.
	unsafe fn DialogBoxIndirectParam(&self,
		dialog_template: &DLGTEMPLATE,
		hwnd_parent: Option<&HWND>,
		dialog_proc: DLGPROC,
		init_param: Option<isize>,
	) -> SysResult<isize>
	{
		match unsafe {
			ffi::DialogBoxIndirectParamW(
				self.ptr(),
				dialog_template as *const _ as _,
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.ptr()),
				dialog_proc as _,
				init_param.unwrap_or_default(),
			)
		} {
			-1 => Err(GetLastError()),
			res => Ok(res), // assumes hWndParent as valid, so no check for zero
		}
	}


	/// [`DialogBoxParam`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dialogboxparamw)
	/// function.
	///
	/// # Safety
	///
	/// To create a dialog, you must provide a dialog procedure.
	unsafe fn DialogBoxParam(&self,
		resource_id: IdStr,
		hwnd_parent: Option<&HWND>,
		dialog_proc: DLGPROC,
		init_param: Option<isize>,
	) -> SysResult<isize>
	{
		match unsafe {
			ffi::DialogBoxParamW(
				self.ptr(),
				resource_id.as_ptr(),
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.ptr()),
				dialog_proc as _,
				init_param.unwrap_or_default(),
			)
		} {
			-1 => Err(GetLastError()),
			res => Ok(res), // assumes hWndParent as valid, so no check for zero
		}
	}

	/// [`GetClassInfoEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclassinfoexw)
	/// function.
	///
	/// # Examples
	///
	/// Retrieving information of a window class created in our application:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let mut wcx = w::WNDCLASSEX::default();
	/// w::HINSTANCE::GetModuleHandle(None)?
	///     .GetClassInfoEx("SOME_CLASS_NAME", &mut wcx)?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn GetClassInfoEx(&self,
		class_name: &str,
		wcx: &mut WNDCLASSEX,
	) -> SysResult<ATOM>
	{
		match unsafe {
			ffi::GetClassInfoExW(
				self.ptr(),
				WString::from_str(class_name).as_ptr(),
				wcx as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			atom => Ok(unsafe { ATOM::from_raw(atom as _) }),
		}
	}

	/// [`LoadAccelerators`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadacceleratorsw)
	/// function.
	#[must_use]
	fn LoadAccelerators(&self, table_name: IdStr) -> SysResult<HACCEL> {
		ptr_to_sysresult_handle(
			unsafe { ffi::LoadAcceleratorsW(self.ptr(), table_name.as_ptr()) },
		)
	}

	/// [`LoadCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
	/// function.
	///
	/// # Examples
	///
	/// Loading a system cursor:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let sys_cursor = w::HINSTANCE::NULL
	///     .LoadCursor(w::IdIdcStr::Idc(co::IDC::ARROW))?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn LoadCursor(&self,
		resource_id: IdIdcStr,
	) -> SysResult<DestroyCursorGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::LoadCursorW(self.ptr(), resource_id.as_ptr()),
			).map(|h| DestroyCursorGuard::new(h))
		}
	}

	/// [`LoadIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadiconw)
	/// function.
	///
	/// # Examples
	///
	/// Loading a system icon:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let sys_icon = w::HINSTANCE::NULL
	///     .LoadIcon(w::IdIdiStr::Idi(co::IDI::INFORMATION))?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn LoadIcon(&self, icon_id: IdIdiStr) -> SysResult<DestroyIconGuard> {
		unsafe {
			ptr_to_sysresult_handle(ffi::LoadIconW(self.ptr(), icon_id.as_ptr()))
				.map(|h| DestroyIconGuard::new(h))
		}
	}

	/// [`LoadMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadmenuw)
	/// function.
	#[must_use]
	fn LoadMenu(&self, resource_id: IdStr) -> SysResult<HMENU> {
		ptr_to_sysresult_handle(
			unsafe { ffi::LoadMenuW(self.ptr(), resource_id.as_ptr()) },
		)
	}

	/// [`LoadString`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadstringw)
	/// function.
	#[must_use]
	fn LoadString(&self, id: u16) -> SysResult<String> {
		let mut pdata: *const u16 = std::ptr::null_mut();
		match unsafe {
			ffi::LoadStringW(
				self.ptr(),
				id as _,
				&mut pdata as *mut _ as  _, 0,
			)
		} {
			0 => Err(GetLastError()),
			len => Ok(WString::from_wchars_count(pdata, len as _).to_string())
		}
	}
}
