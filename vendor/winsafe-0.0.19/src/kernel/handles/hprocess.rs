#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, ffi_types::*, privs::*};
use crate::prelude::*;

impl_handle! { HPROCESS;
	/// Handle to a
	/// [process](https://learn.microsoft.com/en-us/windows/win32/procthread/processes-and-threads).
	/// Originally just a `HANDLE`.
}

impl kernel_Hprocess for HPROCESS {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HPROCESS`](crate::HPROCESS).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hprocess: Handle {
	/// [`CheckRemoteDebuggerPresent`](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-checkremotedebuggerpresent)
	/// function.
	#[must_use]
	fn CheckRemoteDebuggerPresent(&self) -> SysResult<bool> {
		let mut present: BOOL = 0;
		bool_to_sysresult(
			unsafe { ffi::CheckRemoteDebuggerPresent(self.ptr(), &mut present) },
		).map(|_| present != 0)
	}

	/// [`CreateProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)
	/// function.
	#[must_use]
	fn CreateProcess(
		application_name: Option<&str>,
		command_line: Option<&str>,
		process_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		thread_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		inherit_handles: bool,
		creation_flags: co::CREATE,
		environment: Option<Vec<(&str, &str)>>,
		current_dir: Option<&str>,
		si: &mut STARTUPINFO,
	) -> SysResult<CloseHandlePiGuard>
	{
		let mut buf_cmd_line = WString::from_opt_str(command_line);
		let mut pi = PROCESS_INFORMATION::default();

		unsafe {
			bool_to_sysresult(
				ffi::CreateProcessW(
					WString::from_opt_str(application_name).as_ptr(),
					buf_cmd_line.as_mut_ptr(),
					process_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
					thread_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
					inherit_handles as _,
					creation_flags.raw(),
					environment.map_or(std::ptr::null_mut(), |environment| {
						WString::from_str_vec(
							&environment.iter()
								.map(|(name, val)| format!("{}={}", name, val))
								.collect::<Vec<_>>()
						).as_ptr() as _
					}),
					WString::from_opt_str(current_dir).as_ptr(),
					si as *mut _ as _,
					&mut pi as *mut _ as _,
				),
			).map(|_| CloseHandlePiGuard::new(pi))
		}
	}

	/// [`FlushInstructionCache`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushinstructioncache)
	/// function.
	fn FlushInstructionCache(&self,
		base_address: *mut std::ffi::c_void,
		size: usize,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe { ffi::FlushInstructionCache(self.ptr(), base_address, size) },
		)
	}

	/// [`GetCurrentProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)
	/// function.
	#[must_use]
	fn GetCurrentProcess() -> HPROCESS {
		HPROCESS(unsafe { ffi::GetCurrentProcess() })
	}

	/// [`GetExitCodeProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess)
	/// function.
	#[must_use]
	fn GetExitCodeProcess(&self) -> SysResult<u32> {
		let mut exit_code = u32::default();
		bool_to_sysresult(
			unsafe { ffi::GetExitCodeProcess(self.ptr(), &mut exit_code) },
		).map(|_| exit_code)
	}

	/// [`GetGuiResources`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getguiresources)
	/// function.
	#[must_use]
	fn GetGuiResources(&self, flags: co::GR) -> SysResult<u32> {
		match unsafe { ffi::GetGuiResources(self.ptr(), flags.raw()) } {
			0 => Err(GetLastError()),
			count => Ok(count),
		}
	}

	/// [`GetPriorityClass`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getpriorityclass)
	/// function.
	#[must_use]
	fn GetPriorityClass(&self) -> SysResult<co::PRIORITY_CLASS> {
		match unsafe { ffi::GetPriorityClass(self.ptr()) } {
			0 => Err(GetLastError()),
			pc => Ok(unsafe { co::PRIORITY_CLASS::from_raw(pc) }),
		}
	}

	/// [`GetProcessHandleCount`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesshandlecount)
	/// function.
	#[must_use]
	fn GetProcessHandleCount(&self) -> SysResult<u32> {
		let mut count = u32::default();
		bool_to_sysresult(
			unsafe { ffi::GetProcessHandleCount(self.ptr(), &mut count) },
		).map(|_| count)
	}

	/// [`GetProcessId`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessid)
	/// function.
	#[must_use]
	fn GetProcessId(&self) -> SysResult<u32> {
		match unsafe { ffi::GetProcessId(self.ptr()) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetProcessTimes`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes)
	/// function.
	fn GetProcessTimes(&self,
		creation: &mut FILETIME,
		exit: &mut FILETIME,
		kernel: &mut FILETIME,
		user: &mut FILETIME,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::GetProcessTimes(
					self.ptr(),
					creation as *mut _ as _,
					exit as *mut _ as _,
					kernel as *mut _ as _,
					user as *mut _ as _,
				)
			},
		)
	}

	/// [`IsProcessCritical`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-isprocesscritical)
	/// function.
	#[must_use]
	fn IsProcessCritical(&self) -> SysResult<bool> {
		let mut critical: BOOL = 0;
		bool_to_sysresult(
			unsafe { ffi::IsProcessCritical(self.ptr(), &mut critical) },
		).map(|_| critical != 0)
	}

	/// [`IsWow64Process`](https://learn.microsoft.com/en-us/windows/win32/api/wow64apiset/nf-wow64apiset-iswow64process)
	/// function.
	#[must_use]
	fn IsWow64Process(&self) -> SysResult<bool> {
		let mut wow64: BOOL = 0;
		match unsafe { ffi::IsWow64Process(self.ptr(), &mut wow64) } {
			0 => Err(GetLastError()),
			_ => Ok(wow64 != 0),
		}
	}

	/// [`OpenProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)
	/// function.
	///
	/// This method will return
	/// [`ERROR::INVALID_PARAMETER`](crate::co::ERROR::INVALID_PARAMETER) if you
	/// try to open a system process.
	#[must_use]
	fn OpenProcess(
		desired_access: co::PROCESS,
		inherit_handle: bool,
		process_id: u32,
	) -> SysResult<CloseHandleGuard<HPROCESS>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::OpenProcess(
					desired_access.raw(),
					inherit_handle as _,
					process_id,
				),
			).map(|h| CloseHandleGuard::new(h))
		}
	}

	/// [`OpenProcessToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let htoken = w::HPROCESS::GetCurrentProcess()
	///     .OpenProcessToken(co::TOKEN::ADJUST_PRIVILEGES | co::TOKEN::QUERY)?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn OpenProcessToken(&self,
		desired_access: co::TOKEN,
	) -> SysResult<CloseHandleGuard<HACCESSTOKEN>>
	{
		let mut handle = HACCESSTOKEN::NULL;
		unsafe {
			bool_to_sysresult(
				ffi::OpenProcessToken(
					self.ptr(),
					desired_access.raw(),
					handle.as_mut(),
				),
			).map(|_| CloseHandleGuard::new(handle))
		}
	}

	/// [`QueryFullProcessImageName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-queryfullprocessimagenamew)
	/// function.
	#[must_use]
	fn QueryFullProcessImageName(&self,
		flags: co::PROCESS_NAME,
	) -> SysResult<String>
	{
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
		let mut sz = buf.buf_len() as u32;

		bool_to_sysresult(
			unsafe {
				ffi::QueryFullProcessImageNameW(
					self.ptr(),
					flags.raw(),
					buf.as_mut_ptr(),
					&mut sz,
				)
			},
		).map(|_| buf.to_string())
	}

	/// [`QueryProcessAffinityUpdateMode`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-queryprocessaffinityupdatemode)
	/// function.
	#[must_use]
	fn QueryProcessAffinityUpdateMode(&self) -> SysResult<co::PROCESS_AFFINITY> {
		let mut affinity = co::PROCESS_AFFINITY::default();
		bool_to_sysresult(
			unsafe {
				ffi::QueryProcessAffinityUpdateMode(
					self.ptr(),
					affinity.as_mut(),
				)
			},
		).map(|_| affinity)
	}

	/// [`SetPriorityClass`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass)
	/// function.
	fn SetPriorityClass(&self,
		prority_class: co::PRIORITY_CLASS,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe { ffi::SetPriorityClass(self.ptr(), prority_class.raw()) },
		)
	}

	/// [`SetProcessAffinityUpdateMode`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessaffinityupdatemode)
	/// function.
	fn SetProcessAffinityUpdateMode(&self,
		flags: co::PROCESS_AFFINITY,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe { ffi::SetProcessAffinityUpdateMode(self.ptr(), flags.raw()) },
		)
	}

	/// [`SetProcessPriorityBoost`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocesspriorityboost)
	/// function.
	fn SetProcessPriorityBoost(&self,
		disable_priority_boost: bool,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::SetProcessPriorityBoost(
					self.ptr(),
					disable_priority_boost as _,
				)
			},
		)
	}

	/// [`TerminateProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess)
	/// function.
	fn TerminateProcess(&self, exit_code: u32) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::TerminateProcess(self.ptr(), exit_code) })
	}

	/// [`WaitForSingleObject`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)
	/// function.
	fn WaitForSingleObject(&self,
		milliseconds: Option<u32>,
	) -> SysResult<co::WAIT>
	{
		unsafe { HEVENT::from_ptr(self.ptr()) }
			.WaitForSingleObject(milliseconds)
	}
}
