#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`ITaskbarList4`](crate::ITaskbarList4) virtual table.
#[repr(C)]
pub struct ITaskbarList4VT {
	pub ITaskbarList3VT: ITaskbarList3VT,
	pub SetTabProperties: fn(COMPTR, HANDLE, u32) -> HRES,
}

com_interface! { ITaskbarList4: "c43dc798-95d1-4bea-9030-bb99e2983a1a";
	/// [`ITaskbarList4`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist4)
	/// COM interface over [`ITaskbarList4VT`](crate::vt::ITaskbarList4VT).
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
	/// let obj = w::CoCreateInstance::<w::ITaskbarList4>(
	///     &co::CLSID::TaskbarList,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
}

impl shell_ITaskbarList for ITaskbarList4 {}
impl shell_ITaskbarList2 for ITaskbarList4 {}
impl shell_ITaskbarList3 for ITaskbarList4 {}
impl shell_ITaskbarList4 for ITaskbarList4 {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`ITaskbarList4`](crate::ITaskbarList4).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_ITaskbarList4: shell_ITaskbarList3 {
	/// [`ITaskbarList4::SetTabProperties`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist4-settabproperties)
	/// method.
	fn SetTabProperties(&self,
		hwnd_tab: &HWND,
		stp_flags: co::STPFLAG,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskbarList4VT>(self).SetTabProperties)(
					self.ptr(),
					hwnd_tab.ptr(),
					stp_flags.raw(),
				)
			},
		)
	}
}
