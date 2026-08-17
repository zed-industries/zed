#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`ITaskService`](crate::ITaskService) virtual table.
#[repr(C)]
pub struct ITaskServiceVT {
	pub IDispatchVT: IDispatchVT,
	pub GetFolder: fn(COMPTR, PCSTR, *mut COMPTR) -> HRES,
	pub GetRunningTasks: fn(COMPTR, i32, *mut COMPTR) -> HRES,
	pub NewTask: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub Connect: fn(COMPTR, VARIANT, VARIANT, VARIANT, VARIANT) -> HRES,
	pub get_Connected: fn(COMPTR, *mut i16) -> HRES,
	pub get_TargetServer: fn(COMPTR, *mut PSTR) -> HRES,
	pub get_ConnectedUser: fn(COMPTR, *mut PSTR) -> HRES,
	pub get_ConnectedDomain: fn(COMPTR, *mut PSTR) -> HRES,
	pub get_HighestVersion: fn(COMPTR, *mut u32) -> HRES,
}

com_interface! { ITaskService: "2faba4c7-4da9-4013-9697-20cc3fd40f85";
	/// [`ITaskService`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-itaskservice)
	/// COM interface over [`ITaskServiceVT`](crate::vt::ITaskServiceVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let obj = w::CoCreateInstance::<w::ITaskService>(
	///     &co::CLSID::TaskScheduler,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
}

impl oleaut_IDispatch for ITaskService {}
impl taskschd_ITaskService for ITaskService {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`ITaskService`](crate::ITaskService).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ITaskService: oleaut_IDispatch {
	/// [`ITaskService::Connect`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-connect)
	/// method.
	fn Connect(&self,
		server_name: Option<&str>,
		user: Option<&str>,
		domain: Option<&str>,
		password: Option<&str>,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskServiceVT>(self).Connect)(
					self.ptr(),
					match server_name {
						Some(server_name) => VARIANT::new_bstr(server_name)?,
						None => VARIANT::default(),
					},
					match user {
						Some(user) => VARIANT::new_bstr(user)?,
						None => VARIANT::default(),
					},
					match domain {
						Some(domain) => VARIANT::new_bstr(domain)?,
						None => VARIANT::default(),
					},
					match password {
						Some(password) => VARIANT::new_bstr(password)?,
						None => VARIANT::default(),
					},
				)
			},
		)
	}

	/// [`ITaskService::get_Connected`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-get_connected)
	/// method.
	#[must_use]
	fn get_Connected(&self) -> HrResult<bool> {
		let mut connected = i16::default();
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskServiceVT>(self).get_Connected)(
					self.ptr(),
					&mut connected,
				)
			},
		).map(|_| connected != 0)
	}

	fn_com_bstr_get! { get_ConnectedDomain: ITaskServiceVT;
		/// [`ITaskService::get_ConnectedDomain`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-get_connecteddomain)
		/// method.
	}

	fn_com_bstr_get! { get_ConnectedUser: ITaskServiceVT;
		/// [`ITaskService::get_ConnectedUser`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-get_connecteduser)
		/// method.
	}

	/// [`ITaskService::get_HighestVersion`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-get_highestversion)
	/// method.
	#[must_use]
	fn get_HighestVersion(&self) -> HrResult<u32> {
		let mut ver = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskServiceVT>(self).get_HighestVersion)(
					self.ptr(),
					&mut ver,
				)
			},
		).map(|_| ver)
	}

	fn_com_bstr_get! { get_TargetServer: ITaskServiceVT;
		/// [`ITaskService::get_TargetServer`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-get_targetserver)
		/// method.
	}

	/// [`ITaskService::GetFolder`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-getfolder)
	/// method.
	#[must_use]
	fn GetFolder(&self, path: &str) -> HrResult<ITaskFolder> {
		let mut queried = unsafe { ITaskFolder::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskServiceVT>(self).GetFolder)(
					self.ptr(),
					BSTR::SysAllocString(path)?.as_ptr(),
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`ITaskService::NewTask`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itaskservice-newtask)
	/// method.
	#[must_use]
	fn NewTask(&self) -> HrResult<ITaskDefinition> {
		let mut queried = unsafe { ITaskDefinition::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskServiceVT>(self).NewTask)(
					self.ptr(),
					0,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}
}
