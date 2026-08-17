#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::{ffi, guard::*, privs::*};
use crate::prelude::*;

impl_handle! { HSC;
	/// Handle to a
	/// [Service Control Manager](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openscmanagerw).
	/// Originally `SC_HANDLE`.
}

impl kernel_Hsc for HSC {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HSC`](crate::HSC).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hsc: Handle {
	/// [`CreateService`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-createservicew)
	/// function.
	#[must_use]
	fn CreateService(&self,
		service_name: &str,
		display_name: Option<&str>,
		desired_access: co::SERVICE,
		service_type: co::SERVICE_TYPE,
		start_type: co::SERVICE_START,
		error_control: co::SERVICE_ERROR,
		binary_path_name: Option<&str>,
		load_order_group: Option<&[impl AsRef<str>]>,
		tag_id: Option<&mut u32>,
		dependencies: Option<&[impl AsRef<str>]>,
		service_start_name: Option<&str>,
		password: Option<&str>,
	) -> SysResult<CloseServiceHandleSvcGuard>
	{
		let binary_path_name_quoted = binary_path_name.map(|s| {
			if s.starts_with('"') { s.to_owned() }
			else { format!("\"{}\"", s) }
		});

		unsafe {
			ptr_to_sysresult_handle(
				ffi::CreateServiceW(
					self.ptr(),
					WString::from_str(service_name).as_ptr(),
					WString::from_opt_str(display_name).as_ptr(),
					desired_access.raw(),
					service_type.raw(),
					start_type.raw(),
					error_control.raw(),
					WString::from_opt_str(binary_path_name_quoted).as_ptr(),
					load_order_group.map_or(std::ptr::null_mut(), |v| WString::from_str_vec(v).as_ptr()),
					tag_id.map_or(std::ptr::null_mut(), |n| n),
					dependencies.map_or(std::ptr::null_mut(), |v| WString::from_str_vec(v).as_ptr()),
					WString::from_opt_str(service_start_name).as_ptr(),
					WString::from_opt_str(password).as_ptr(),
				)
			).map(|h| CloseServiceHandleSvcGuard::new(h))
		}
	}

	/// [`OpenSCManager`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openscmanagerw)
	/// function.
	#[must_use]
	fn OpenSCManager(
		machine_name: Option<&str>,
		desired_access: co::SC_MANAGER,
	) -> SysResult<CloseServiceHandleGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::OpenSCManagerW(
					WString::from_opt_str(machine_name).as_ptr(),
					std::ptr::null(),
					desired_access.raw(),
				),
			).map(|h| CloseServiceHandleGuard::new(h))
		}
	}

	/// [`OpenService`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openservicew)
	/// function.
	#[must_use]
	fn OpenService(&self,
		service_name: &str,
		desired_access: co::SERVICE,
	) -> SysResult<CloseServiceHandleSvcGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::OpenServiceW(
					self.ptr(),
					WString::from_str(service_name).as_ptr(),
					desired_access.raw(),
				)
			).map(|h| CloseServiceHandleSvcGuard::new(h))
		}
	}
}
