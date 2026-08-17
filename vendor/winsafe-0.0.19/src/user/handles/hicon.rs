#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::ffi;

impl_handle! { HICON;
	/// Handle to an
	/// [icon](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hicon).
}

impl user_Hicon for HICON {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HICON`](crate::HICON).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hicon: Handle {
	/// [`CopyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-copyicon)
	/// function.
	#[must_use]
	fn CopyIcon(&self) -> SysResult<DestroyIconGuard> {
		unsafe {
			ptr_to_sysresult_handle(ffi::CopyIcon(self.ptr()))
				.map(|h| DestroyIconGuard::new(h))
		}
	}
}
