#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::prelude::*;
use crate::vt::*;

/// [`ITaskDefinition`](crate::ITaskDefinition) virtual table.
#[repr(C)]
pub struct ITaskDefinitionVT {
	pub IDispatchVT: IDispatchVT,
	pub get_RegistrationInfo: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_RegistrationInfo: fn(COMPTR, COMPTR) -> HRES,
	pub get_Triggers: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_Triggers: fn(COMPTR, COMPTR) -> HRES,
	pub get_Settings: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_Settings: fn(COMPTR, COMPTR) -> HRES,
	pub get_Data: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Data: fn(COMPTR, PCSTR) -> HRES,
	pub get_Principal: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_Principal: fn(COMPTR, COMPTR) -> HRES,
	pub get_Actions: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_Actions: fn(COMPTR, COMPTR) -> HRES,
	pub get_XmlText: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_XmlText: fn(COMPTR, PCSTR) -> HRES,
}

com_interface! { ITaskDefinition: "f5bc8fc5-536d-4f77-b852-fbc1356fdeb6";
	/// [`ITaskDefinition`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-itaskdefinition)
	/// COM interface over [`ITaskDefinitionVT`](crate::vt::ITaskDefinitionVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for ITaskDefinition {}
impl taskschd_ITaskDefinition for ITaskDefinition {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`ITaskDefinition`](crate::ITaskDefinition).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ITaskDefinition: oleaut_IDispatch {
	fn_com_interface_get! { get_Actions: ITaskDefinitionVT, IActionCollection;
		/// [`ITaskDefinition::get_Actions`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-get_actions)
		/// method.
	}

	fn_com_bstr_get! { get_Data: ITaskDefinitionVT;
		/// [`ITaskDefinition::get_Data`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-get_data)
		/// method.
	}

	fn_com_interface_get! { get_RegistrationInfo: ITaskDefinitionVT, IRegistrationInfo;
		/// [`ITaskDefinition::get_RegistrationInfo`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-get_registrationinfo)
		/// method.
	}

	fn_com_interface_get! { get_Triggers: ITaskDefinitionVT, ITriggerCollection;
		/// [`ITaskDefinition::get_Triggers`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-get_triggers)
		/// method.
	}

	fn_com_bstr_get! { get_XmlText: ITaskDefinitionVT;
		/// [`ITaskDefinition::get_XmlText`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-get_xmltext)
		/// method.
	}

	fn_com_bstr_set! { put_Data: ITaskDefinitionVT, data;
		/// [`ITaskDefinition::put_Data`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-put_data)
		/// method.
	}

	fn_com_bstr_set! { put_XmlText: ITaskDefinitionVT, xml;
		/// [`ITaskDefinition::put_XmlText`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskdefinition-put_xmltext)
		/// method.
	}
}
