#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::{ffi, privs::*};
use crate::prelude::*;

impl_handle! { HSERVICE;
	/// Handle to a
	/// [service](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openservicew).
	/// Originally `SC_HANDLE`.
}

impl kernel_Hservice for HSERVICE {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HSERVICE`](crate::HSERVICE).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hservice: Handle {
	/// [`DeleteService`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-deleteservice)
	/// function.
	fn DeleteService(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::DeleteService(self.ptr()) })
	}
}
