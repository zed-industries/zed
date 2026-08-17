#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};
use crate::prelude::*;

impl_handle! { HUPDATERSRC;
	/// Handle to an
	/// [updateable resource](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-beginupdateresourcew).
	/// Originally just a `HANDLE`.
}

impl kernel_Hupdatersrc for HUPDATERSRC {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HUPDATERSRC`](crate::HUPDATERSRC).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hupdatersrc: Handle {
	/// [`BeginUpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-beginupdateresourcew)
	/// function.
	#[must_use]
	fn BeginUpdateResource(
		file_name: &str,
		delete_existing_resources: bool,
	) -> SysResult<EndUpdateResourceGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::BeginUpdateResourceW(
					WString::from_str(file_name).as_ptr(),
					delete_existing_resources as _,
				),
			).map(|h| EndUpdateResourceGuard::new(h))
		}
	}

	/// [`UpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-updateresourcew)
	/// function.
	fn UpdateResource(&self,
		resource_type: RtStr,
		resource_id: IdStr,
		language: LANGID,
		data: &[u8],
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::UpdateResourceW(
					self.ptr(),
					resource_type.as_ptr(),
					resource_id.as_ptr(),
					language.into(),
					vec_ptr(data) as _,
					data.len() as _,
				)
			},
		)
	}
}
