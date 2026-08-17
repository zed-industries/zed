#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};
use crate::prelude::*;

impl_handle! { HTHREAD;
	/// Handle to a
	/// [thread](https://learn.microsoft.com/en-us/windows/win32/procthread/processes-and-threads).
	/// Originally just a `HANDLE`.
}

impl kernel_Hthread for HTHREAD {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HTHREAD`](crate::HTHREAD).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hthread: Handle {
	/// [`CreateThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)
	/// function.
	///
	/// Returns the thread handle and its ID.
	#[must_use]
	fn CreateThread(
		thread_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		stack_size: usize,
		start_addr: *mut std::ffi::c_void,
		parameter: *mut std::ffi::c_void,
		flags: co::THREAD_CREATE,
	) -> SysResult<(CloseHandleGuard<HTHREAD>, u32)>
	{
		let mut thread_id = u32::default();
		unsafe {
			ptr_to_sysresult_handle(
				ffi::CreateThread(
					thread_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
					stack_size,
					start_addr,
					parameter,
					flags.raw(),
					&mut thread_id,
				)
			).map(|h| (CloseHandleGuard::new(h), thread_id))
		}
	}

	/// [`GetCurrentThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread)
	/// function.
	#[must_use]
	fn GetCurrentThread() -> HTHREAD {
		HTHREAD(unsafe { ffi::GetCurrentThread() })
	}

	/// [`GetExitCodeThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread)
	/// function.
	#[must_use]
	fn GetExitCodeThread(&self) -> SysResult<u32> {
		let mut exit_code = u32::default();
		bool_to_sysresult(
			unsafe { ffi::GetExitCodeThread(self.ptr(), &mut exit_code) },
		).map(|_| exit_code)
	}

	/// [`GetProcessIdOfThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessidofthread)
	/// function.
	#[must_use]
	fn GetProcessIdOfThread(&self) -> SysResult<u32> {
		match unsafe { ffi::GetProcessIdOfThread(self.ptr()) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetThreadId`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadid)
	/// function.
	#[must_use]
	fn GetThreadId(&self) -> SysResult<u32> {
		match unsafe { ffi::GetThreadId(self.ptr()) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetThreadTimes`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadtimes)
	/// function.
	fn GetThreadTimes(&self,
		creation: &mut FILETIME,
		exit: &mut FILETIME,
		kernel: &mut FILETIME,
		user: &mut FILETIME,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::GetThreadTimes(
					self.ptr(),
					creation as *mut _ as _,
					exit as *mut _ as _,
					kernel as *mut _ as _,
					user as *mut _ as _,
				)
			},
		)
	}

	/// [`OpenThreadToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthreadtoken)
	/// function.
	#[must_use]
	fn OpenThreadToken(&self,
		desired_access: co::TOKEN,
		open_as_self: bool,
	) -> SysResult<CloseHandleGuard<HACCESSTOKEN>>
	{
		let mut handle = HACCESSTOKEN::NULL;
		unsafe {
			bool_to_sysresult(
				ffi::OpenThreadToken(
					self.ptr(),
					desired_access.raw(),
					open_as_self as _,
					handle.as_mut(),
				),
			).map(|_| CloseHandleGuard::new(handle))
		}
	}

	/// [`ResumeThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread)
	/// function.
	fn ResumeThread(&self) -> SysResult<u32> {
		minus1_as_error(unsafe { ffi::ResumeThread(self.ptr()) })
	}

	/// [`SetThreadIdealProcessor`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessor)
	/// function.
	///
	/// Returns the previous ideal processor.
	fn SetThreadIdealProcessor(&self, ideal_processor: u32) -> SysResult<u32> {
		minus1_as_error(
			unsafe { ffi::SetThreadIdealProcessor(self.ptr(), ideal_processor) },
		)
	}

	/// [`SetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex)
	/// function.
	///
	/// Returns the previous ideal processor.
	fn SetThreadIdealProcessorEx(&self,
		ideal_processor: PROCESSOR_NUMBER,
	) -> SysResult<PROCESSOR_NUMBER>
	{
		let mut prev = PROCESSOR_NUMBER::default();
		bool_to_sysresult(
			unsafe {
				ffi::SetThreadIdealProcessorEx(
					self.ptr(),
					&ideal_processor as *const _ as _,
					&mut prev as *mut _ as _,
				)
			},
		).map(|_| prev)
	}

	/// [`SetThreadPriorityBoost`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriorityboost)
	/// function.
	fn SetThreadPriorityBoost(&self,
		disable_priority_boost: bool,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::SetThreadPriorityBoost(
					self.ptr(),
					disable_priority_boost as _,
				)
			},
		)
	}

	/// [`SuspendThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-suspendthread)
	/// function.
	fn SuspendThread(&self) -> SysResult<u32> {
		minus1_as_error(unsafe { ffi::SuspendThread(self.ptr()) })
	}

	/// [`TerminateThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminatethread)
	/// function.
	fn TerminateThread(&self, exit_code: u32) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::TerminateThread(self.ptr(), exit_code) },
		)
	}
}
