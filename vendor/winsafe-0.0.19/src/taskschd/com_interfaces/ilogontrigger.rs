#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::prelude::*;
use crate::vt::*;

/// [`ILogonTrigger`](crate::ILogonTrigger) interface.
#[repr(C)]
pub struct ILogonTriggerVT {
	pub ITriggerVT: ITriggerVT,
	pub get_Delay: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Delay: fn(COMPTR, PCSTR) -> HRES,
	pub get_UserId: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_UserId: fn(COMPTR, PCSTR) -> HRES,
}

com_interface! { ILogonTrigger: "72dade38-fae4-4b3e-baf4-5d009af02b1c";
	/// [`ILogonTrigger`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iLogontrigger)
	/// COM interface over [`ILogonTriggerVT`](crate::vt::ILogonTriggerVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let trigger: w::ITrigger; // initialized somewhere
	/// # let trigger = unsafe { w::ITrigger::null() };
	///
	/// let logon_trigger = trigger
	///     .QueryInterface::<w::ILogonTrigger>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl oleaut_IDispatch for ILogonTrigger {}
impl taskschd_ITrigger for ILogonTrigger {}
impl taskschd_ILogonTrigger for ILogonTrigger {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`ILogonTrigger`](crate::ILogonTrigger).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ILogonTrigger: taskschd_ITrigger {
	fn_com_bstr_get! { get_Delay: ILogonTriggerVT;
		/// [`ILogonTrigger::get_RandomDelay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ilogontrigger-get_delay)
		/// method.
	}

	fn_com_bstr_get! { get_UserId: ILogonTriggerVT;
		/// [`ILogonTrigger::get_UserId`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ilogontrigger-get_userid)
		/// method.
	}

	fn_com_bstr_set! { put_Delay: ILogonTriggerVT, delay;
		/// [`ILogonTrigger::put_RandomDelay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ilogontrigger-put_delay)
		/// method.
	}

	fn_com_bstr_set! { put_UserId: ILogonTriggerVT, user_id;
		/// [`ILogonTrigger::put_UserId`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ilogontrigger-put_userid)
		/// method.
	}
}
