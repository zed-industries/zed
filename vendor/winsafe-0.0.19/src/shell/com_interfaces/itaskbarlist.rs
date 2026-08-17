#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`ITaskbarList`](crate::ITaskbarList) virtual table.
#[repr(C)]
pub struct ITaskbarListVT {
	pub IUnknownVT: IUnknownVT,
	pub HrInit: fn(COMPTR) -> HRES,
	pub AddTab: fn(COMPTR, HANDLE) -> HRES,
	pub DeleteTab: fn(COMPTR, HANDLE) -> HRES,
	pub ActivateTab: fn(COMPTR, HANDLE) -> HRES,
	pub SetActiveAlt: fn(COMPTR, HANDLE) -> HRES,
}

com_interface! { ITaskbarList: "56fdf342-fd6d-11d0-958a-006097c9a090";
	/// [`ITaskbarList`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
	/// COM interface over [`ITaskbarListVT`](crate::vt::ITaskbarListVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let obj = w::CoCreateInstance::<w::ITaskbarList>(
	///     &co::CLSID::TaskbarList,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
}

impl shell_ITaskbarList for ITaskbarList {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`ITaskbarList`](crate::ITaskbarList).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_ITaskbarList: ole_IUnknown {
	/// [`ITaskbarList::ActivateTab`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-activatetab)
	/// method.
	fn ActivateTab(&self, hwnd: &HWND) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskbarListVT>(self).ActivateTab)(self.ptr(), hwnd.ptr())
			},
		)
	}

	/// [`ITaskbarList::AddTab`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-addtab)
	/// method.
	fn AddTab(&self, hwnd: &HWND) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskbarListVT>(self).AddTab)(self.ptr(), hwnd.ptr())
			},
		)
	}

	/// [`ITaskbarList::DeleteTab`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-deletetab)
	/// method.
	fn DeleteTab(&self, hwnd: &HWND) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskbarListVT>(self).DeleteTab)(self.ptr(), hwnd.ptr())
			},
		)
	}

	fn_com_noparm! { HrInit: ITaskbarListVT;
		/// [`ITaskbarList::HrInit`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-hrinit)
		/// method.
	}

	/// [`ITaskbarList::SetActiveAlt`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
	/// method.
	fn SetActiveAlt(&self, hwnd: &HWND) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskbarListVT>(self).SetActiveAlt)(self.ptr(), hwnd.ptr())
			},
		)
	}
}
