#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};
use crate::prelude::*;

impl_handle! { HEVENTLOG;
	/// Handle to an
	/// [event log](https://learn.microsoft.com/en-us/windows/win32/eventlog/event-logging).
	/// Originally just a `HANDLE`.
}

impl kernel_Heventlog for HEVENTLOG {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HEVENTLOG`](crate::HEVENTLOG).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Heventlog: Handle {
	/// [`RegisterEventSource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-registereventsourcew)
	/// function.
	#[must_use]
	fn RegisterEventSource(
		unc_server_name: Option<&str>,
		source_name: &str,
	) -> SysResult<DeregisterEventSourceGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::RegisterEventSourceW(
					WString::from_opt_str(unc_server_name).as_ptr(),
					WString::from_str(source_name).as_ptr(),
				),
			).map(|h| DeregisterEventSourceGuard::new(h))
		}
	}

	/// [`ReportEvent`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-reporteventw)
	/// function.
	fn ReportEvent(&self,
		event_type: co::EVENTLOG,
		category: u16,
		event_id: u32,
		user_sid: Option<&SID>,
		strings: Option<&[impl AsRef<str>]>,
		raw_data: Option<&[u8]>,
	) -> SysResult<()>
	{
		let (_wstrs, pwstrs) = create_wstr_ptr_vecs(strings);
		bool_to_sysresult(
			unsafe {
				ffi::ReportEventW(
					self.ptr(),
					event_type.raw(),
					category,
					event_id,
					user_sid.map_or(std::ptr::null(), |s| s as *const _ as _),
					strings.map_or(0, |ss| ss.len() as _),
					raw_data.map_or(0, |d| d.len() as _),
					vec_ptr(&pwstrs),
					raw_data.map_or(std::ptr::null(), |d| vec_ptr(d) as _),
				)
			},
		)
	}
}
