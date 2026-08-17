#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};
use crate::prelude::*;

impl_handle! { HEVENT;
	/// Handle to a named or unnamed
	/// [event](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventw)
	/// object. Originally just a `HANDLE`.
}

impl kernel_Hevent for HEVENT {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HEVENT`](crate::HEVENT).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hevent: Handle {
	/// [`CreateEvent`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventw)
	/// function.
	#[must_use]
	fn CreateEvent(
		security_attributes: Option<&mut SECURITY_ATTRIBUTES>,
		manual_reset: bool,
		initial_state: bool,
		name: Option<&str>,
	) -> SysResult<CloseHandleGuard<HEVENT>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::CreateEventW(
					security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
					manual_reset as _,
					initial_state as _,
					WString::from_opt_str(name).as_ptr(),
				)
			).map(|h| CloseHandleGuard::new(h))
		}
	}

	/// [`CreateEventEx`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventexw)
	/// method.
	#[must_use]
	fn CreateEventEx(
		security_attributes: Option<&mut SECURITY_ATTRIBUTES>,
		name: Option<&str>,
		flags: co::CREATE_EVENT,
		desired_access: co::EVENT_RIGHTS,
	) -> SysResult<CloseHandleGuard<HEVENT>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::CreateEventExW(
					security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
					WString::from_opt_str(name).as_ptr(),
					flags.raw(),
					desired_access.raw(),
				)
			).map(|h| CloseHandleGuard::new(h))
		}
	}

	/// [`OpenEvent`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-openeventw)
	/// function.
	#[must_use]
	fn OpenEvent(&self,
		desired_access: co::EVENT_RIGHTS,
		inherit_handle: bool,
		name: &str,
	) -> SysResult<CloseHandleGuard<HEVENT>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::OpenEventW(
					desired_access.raw(),
					inherit_handle as _,
					WString::from_str(name).as_ptr(),
				)
			).map(|h| CloseHandleGuard::new(h))
		}
	}

	/// [`PulseEvent`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-pulseevent)
	/// function.
	fn PulseEvent(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::PulseEvent(self.ptr()) })
	}

	/// [`ResetEvent`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-resetevent)
	/// function.
	fn ResetEvent(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::ResetEvent(self.ptr()) })
	}

	/// [`SetEvent`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-setevent)
	/// function.
	fn SetEvent(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::SetEvent(self.ptr()) })
	}

	/// [`WaitForSingleObject`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)
	/// function.
	fn WaitForSingleObject(&self,
		milliseconds: Option<u32>,
	) -> SysResult<co::WAIT>
	{
		match unsafe {
			co::WAIT::from_raw(
				ffi::WaitForSingleObject(
					self.ptr(),
					milliseconds.unwrap_or(INFINITE),
				),
			)
		} {
			co::WAIT::FAILED => Err(GetLastError()),
			wait => Ok(wait),
		}
	}
}
