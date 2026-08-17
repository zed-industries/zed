#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::ffi;

impl user_Hprocess for HPROCESS {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HPROCESS`](crate::HPROCESS).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hprocess: kernel_Hprocess {
	/// [`SetUserObjectInformation`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setuserobjectinformationw)
	/// function.
	///
	/// # Safety
	///
	/// The `pv_info` type varies according to `index`. If you set it wrong,
	/// you're likely to cause a buffer overrun.
	unsafe fn SetUserObjectInformation<T>(&self,
		index: co::UOI,
		pv_info: &mut T,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			ffi::SetUserObjectInformationW(
				self.ptr(),
				index.raw(),
				pv_info as *mut _ as _,
				std::mem::size_of::<T>() as _,
			),
		)
	}
}
