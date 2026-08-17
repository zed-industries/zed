#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IAction`](crate::IAction) virtual table.
#[repr(C)]
pub struct IActionVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Id: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Id: fn(COMPTR, PCSTR) -> HRES,
	pub get_Type: fn(COMPTR, *mut u32) -> HRES,
}

com_interface! { IAction: "bae54997-48b1-4cbe-9965-d6be263ebea4";
	/// [`IAction`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iaction)
	/// COM interface over [`IActionVT`](crate::vt::IActionVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for IAction {}
impl taskschd_IAction for IAction {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IAction`](crate::IAction).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IAction: oleaut_IDispatch {
	fn_com_bstr_get! { get_Id: IActionVT;
		/// [`IAction::get_Id`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iaction-get_id)
		/// method.
	}

	/// [`IAction::get_Type`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iaction-get_type)
	/// method.
	#[must_use]
	fn get_Type(&self) -> HrResult<co::TASK_ACTION_TYPE> {
		let mut at = co::TASK_ACTION_TYPE::default();
		ok_to_hrresult(
			unsafe { (vt::<IActionVT>(self).get_Type)(self.ptr(), at.as_mut()) },
		).map(|_| at)
	}

	fn_com_bstr_set! { put_Id: IActionVT, id;
		/// [`IAction::put_Id`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iaction-put_id)
		/// method.
	}
}
