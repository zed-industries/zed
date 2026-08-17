#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};
use crate::prelude::*;

impl_handle! { HPIPE;
	/// Handle to an
	/// [anonymous pipe](https://learn.microsoft.com/en-us/windows/win32/ipc/anonymous-pipes).
	/// Originally just a `HANDLE`.
}

impl kernel_Hpipe for HPIPE {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HPIPE`](crate::HPIPE).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hpipe: Handle {
	/// [`CreatePipe`](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)
	/// function.
	///
	/// Returns handles to the read and write pipes.
	#[must_use]
	fn CreatePipe(
		attrs: Option<&mut SECURITY_ATTRIBUTES>,
		size: u32,
	) -> SysResult<(CloseHandleGuard<HPIPE>, CloseHandleGuard<HPIPE>)>
	{
		let (mut hread, mut hwrite) = (HPIPE::NULL, HPIPE::NULL);
		unsafe {
			bool_to_sysresult(
				ffi::CreatePipe(
					hread.as_mut(),
					hwrite.as_mut(),
					attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
					size,
				),
			).map(|_| (CloseHandleGuard::new(hread), CloseHandleGuard::new(hwrite)))
		}
	}

	/// [`ReadFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)
	/// function.
	///
	/// Returns the number of bytes read.
	fn ReadFile(&self,
		buffer: &mut [u8],
		overlapped: Option<&mut OVERLAPPED>,
	) -> SysResult<u32>
	{
		unsafe { HFILE::from_ptr(self.ptr()) }
			.ReadFile(buffer, overlapped)
	}

	/// [`WriteFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)
	/// function.
	fn WriteFile(&self,
		data: &[u8],
		overlapped: Option<&mut OVERLAPPED>,
	) -> SysResult<u32>
	{
		unsafe { HFILE::from_ptr(self.ptr()) }
			.WriteFile(data, overlapped)
	}
}
