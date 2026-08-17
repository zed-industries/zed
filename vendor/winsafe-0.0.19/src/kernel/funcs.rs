#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, ffi_types::*, privs::*};
use crate::prelude::*;

/// [`AllocateAndInitializeSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-allocateandinitializesid)
/// function.
///
/// # Panics
///
/// Panics if `sub_authorities` has more than 8 elements.
///
/// # Examples
///
/// Create a well-known SID for the Everyone group:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let sid_everyone = w::AllocateAndInitializeSid(
///     &w::SID_IDENTIFIER_AUTHORITY::WORLD,
///     &[
///         co::RID::SECURITY_WORLD,
///     ],
/// )?;
///
/// // FreeSid() automatically called
/// # Ok::<_, co::ERROR>(())
/// ```
///
/// Create a SID for the BUILTIN\Administrators group:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let sid_builtin_administrators = w::AllocateAndInitializeSid(
///     &w::SID_IDENTIFIER_AUTHORITY::NT,
///     &[
///         co::RID::SECURITY_BUILTIN_DOMAIN,
///         co::RID::DOMAIN_ALIAS_ADMINS,
///     ],
/// )?;
///
/// // FreeSid() automatically called
/// # Ok::<_, co::ERROR>(())
/// ```
#[must_use]
pub fn AllocateAndInitializeSid(
	identifier_authority: &SID_IDENTIFIER_AUTHORITY,
	sub_authorities: &[co::RID],
) -> SysResult<FreeSidGuard>
{
	if sub_authorities.len() > 8 {
		panic!("You must specify at most 8 sub authorities.");
	}

	let mut psid = std::ptr::null_mut() as *mut SID;

	unsafe {
		bool_to_sysresult(
			ffi::AllocateAndInitializeSid(
				identifier_authority as *const _ as _,
				sub_authorities.len() as _,
				if sub_authorities.len() >= 1 { sub_authorities[0].raw() } else { 0 },
				if sub_authorities.len() >= 2 { sub_authorities[1].raw() } else { 0 },
				if sub_authorities.len() >= 3 { sub_authorities[2].raw() } else { 0 },
				if sub_authorities.len() >= 4 { sub_authorities[3].raw() } else { 0 },
				if sub_authorities.len() >= 5 { sub_authorities[4].raw() } else { 0 },
				if sub_authorities.len() >= 6 { sub_authorities[5].raw() } else { 0 },
				if sub_authorities.len() >= 7 { sub_authorities[6].raw() } else { 0 },
				if sub_authorities.len() >= 8 { sub_authorities[7].raw() } else { 0 },
				&mut psid as *mut _ as _,
			),
		).map(|_| FreeSidGuard::new(psid))
	}
}

/// [`ConvertSidToStringSid`](https://learn.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertsidtostringsidw)
/// function.
///
/// You don't need to call this function directly, because [`SID`](crate::SID)
/// implements [`Display`](std::fmt::Display) and
/// [`ToString`](std::string::ToString) traits, which call it.
#[must_use]
pub fn ConvertSidToStringSid(sid: &SID) -> SysResult<String> {
	let mut pstr = std::ptr::null_mut() as *mut u16;
	bool_to_sysresult(
		unsafe { ffi::ConvertSidToStringSidW(sid as *const _ as _, &mut pstr) },
	)?;
	let name = unsafe { WString::from_wchars_nullt(pstr) }.to_string();
	let _ = unsafe { LocalFreeGuard::new(HLOCAL::from_ptr(pstr as _)) }; // free returned pointer
	Ok(name)
}

/// [`ConvertStringSidToSid`](https://learn.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertstringsidtosidw)
/// function.
#[must_use]
pub fn ConvertStringSidToSid(str_sid: &str) -> SysResult<LocalFreeSidGuard> {
	let mut pbuf = std::ptr::null_mut() as *mut u8;
	unsafe {
		bool_to_sysresult(
			ffi::ConvertStringSidToSidW(
				WString::from_str(str_sid).as_ptr(),
				&mut pbuf,
			),
		).map(|_| LocalFreeSidGuard::new(HLOCAL::from_ptr(pbuf as _)))
	}
}

/// [`CopyFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-copyfilew)
/// function.
pub fn CopyFile(
	existing_file: &str,
	new_file: &str,
	fail_if_exists: bool,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::CopyFileW(
				WString::from_str(existing_file).as_ptr(),
				WString::from_str(new_file).as_ptr(),
				fail_if_exists as _,
			)
		},
	)
}

/// [`CopySid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-copysid)
/// function.
#[must_use]
pub fn CopySid(src: &SID) -> SysResult<SidGuard> {
	let sid_sz = GetLengthSid(&src);
	let sid_buf = HGLOBAL::GlobalAlloc(
		Some(co::GMEM::FIXED | co::GMEM::ZEROINIT),
		sid_sz as _,
	)?;

	unsafe {
		bool_to_sysresult(
			ffi::CopySid(
				sid_sz,
				sid_buf.ptr(),
				src as *const _ as _,
			),
		).map(|_| SidGuard::new(sid_buf))
	}
}

/// [`CreateDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createdirectoryw)
/// function.
pub fn CreateDirectory(
	path_name: &str,
	security_attributes: Option<&SECURITY_ATTRIBUTES>,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::CreateDirectoryW(
				WString::from_str(path_name).as_ptr(),
				security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
			)
		},
	)
}

/// [`CreateWellKnownSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createwellknownsid)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let sid = w::CreateWellKnownSid(
///     co::WELL_KNOWN_SID_TYPE::LocalSystem,
///     None,
/// )?;
/// # Ok::<_, co::ERROR>(())
/// ```
#[must_use]
pub fn CreateWellKnownSid(
	well_known_sid: co::WELL_KNOWN_SID_TYPE,
	domain_sid: Option<&SID>,
) -> SysResult<SidGuard>
{
	let mut sid_sz = u32::default();

	unsafe {
		ffi::CreateWellKnownSid( // retrieve needed buffer sizes
			well_known_sid.raw(),
			domain_sid.map_or(std::ptr::null(), |s| s as *const _ as _),
			std::ptr::null_mut(),
			&mut sid_sz,
		);
	}
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let sid_buf = HGLOBAL::GlobalAlloc(
		Some(co::GMEM::FIXED | co::GMEM::ZEROINIT),
		sid_sz as _,
	)?;

	unsafe {
		bool_to_sysresult(
			ffi::CreateWellKnownSid(
				well_known_sid.raw(),
				domain_sid.map_or(std::ptr::null(), |s| s as *const _ as _),
				sid_buf.ptr(),
				&mut sid_sz,
			),
		).map(|_| SidGuard::new(sid_buf))
	}
}

/// [`DeleteFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-deletefilew)
/// function.
pub fn DeleteFile(file_name: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { ffi::DeleteFileW(WString::from_str(file_name).as_ptr()) },
	)
}

/// [`DecryptFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-decryptfilew)
/// function.
pub fn DecryptFile(file_name: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { ffi::DecryptFileW(WString::from_str(file_name).as_ptr(), 0) },
	)
}

/// [`EncryptFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-encryptfilew)
/// function.
pub fn EncryptFile(file_name: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { ffi::EncryptFileW(WString::from_str(file_name).as_ptr()) },
	)
}

/// [`EncryptionDisable`](https://learn.microsoft.com/en-us/windows/win32/api/winefs/nf-winefs-encryptiondisable)
/// function.
pub fn EncryptionDisable(dir_path: &str, disable: bool) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			ffi::EncryptionDisable(
				WString::from_str(dir_path).as_ptr(),
				disable as _,
			)
		},
	)
}

/// [`EqualDomainSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-equaldomainsid)
/// function.
#[must_use]
pub fn EqualDomainSid(sid1: &SID, sid2: &SID) -> SysResult<bool> {
	let mut is_equal: BOOL = 0;
	bool_to_sysresult(
		unsafe {
			ffi::EqualDomainSid(
				sid1 as *const _ as _,
				sid2 as *const _ as _,
				&mut is_equal,
			)
		},
	).map(|_| is_equal != 0)
}

/// [`EqualPrefixSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-equalprefixsid)
/// function.
#[must_use]
pub fn EqualPrefixSid(sid1: &SID, sid2: &SID) -> SysResult<bool> {
	match unsafe {
		ffi::EqualPrefixSid(sid1 as *const _ as _, sid2 as *const _ as _)
	} {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`EqualSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-equalsid)
/// function.
#[must_use]
pub fn EqualSid(sid1: &SID, sid2: &SID) -> SysResult<bool> {
	match unsafe {
		ffi::EqualSid(sid1 as *const _ as _, sid2 as *const _ as _)
	} {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`ExitProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitprocess)
/// function.
pub fn ExitProcess(exit_code: u32) {
	unsafe { ffi::ExitProcess(exit_code) }
}

/// [`ExitThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread)
/// function.
pub fn ExitThread(exit_code: u32) {
	unsafe { ffi::ExitThread(exit_code) }
}

/// [`ExpandEnvironmentStrings`](https://learn.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-expandenvironmentstringsw)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let expanded = w::ExpandEnvironmentStrings(
///     "Os %OS%, home %HOMEPATH% and temp %TEMP%",
/// )?;
///
/// println!("{}", expanded);
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[must_use]
pub fn ExpandEnvironmentStrings(src: &str) -> SysResult<String> {
	let wsrc = WString::from_str(src);
	let len = unsafe {
		ffi::ExpandEnvironmentStringsW(
			wsrc.as_ptr(),
			std::ptr::null_mut(),
			0,
		)
	};

	let mut buf = WString::new_alloc_buf(len as _);
	bool_to_sysresult(
		unsafe {
			ffi::ExpandEnvironmentStringsW(
				wsrc.as_ptr(),
				buf.as_mut_ptr(),
				len,
			)
		} as _,
	).map(|_| buf.to_string())
}

/// [`FileTimeToSystemTime`](https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-filetimetosystemtime)
/// function.
pub fn FileTimeToSystemTime(
	file_time: &FILETIME,
	system_time: &mut SYSTEMTIME,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::FileTimeToSystemTime(
				file_time as *const _ as _,
				system_time as *mut _ as _,
			)
		},
	)
}

/// [`FlushProcessWriteBuffers`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushprocesswritebuffers)
/// function.
pub fn FlushProcessWriteBuffers() {
	unsafe { ffi::FlushProcessWriteBuffers() }
}

/// [`FormatMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
/// function.
///
/// You don't need to call this function: all error types implement the
/// [`FormattedError`](crate::prelude::FormattedError) trait which will
/// automatically call `FormatMessage`.
#[must_use]
pub unsafe fn FormatMessage(
	flags: co::FORMAT_MESSAGE,
	source: Option<*mut std::ffi::c_void>,
	message_id: u32,
	lang_id: LANGID,
	args: Option<&[*mut std::ffi::c_void]>,
) -> SysResult<String>
{
	let mut ptr_buf = std::ptr::null_mut() as *mut u16;

	let nchars = match ffi::FormatMessageW(
		flags.raw(),
		source.unwrap_or(std::ptr::null_mut()),
		message_id,
		u16::from(lang_id) as _,
		&mut ptr_buf as *mut *mut _ as _, // pass pointer to pointer
		0,
		args.map_or(std::ptr::null_mut(), |arr| arr.as_ptr() as _),
	) as _ {
		0 => Err(GetLastError()),
		nchars => Ok(nchars),
	}?;

	let final_wstr = WString::from_wchars_count(ptr_buf, nchars as _);
	let _ = LocalFreeGuard::new(HLOCAL::from_ptr(ptr_buf as _)); // free returned pointer
	let final_str = final_wstr.to_string();
	Ok(final_str)
}

/// [`GetBinaryType`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getbinarytypew)
/// function.
#[must_use]
pub fn GetBinaryType(application_name: &str) -> SysResult<co::SCS> {
	let mut binary_type = co::SCS::default();
	bool_to_sysresult(
		unsafe {
			ffi::GetBinaryTypeW(
				WString::from_str(application_name).as_ptr(),
				binary_type.as_mut(),
			)
		},
	).map(|_| binary_type)
}

/// [`GetCommandLine`](https://learn.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-getcommandlinew)
/// function.
///
/// For an example, see [`CommandLineToArgv`](crate::CommandLineToArgv).
#[must_use]
pub fn GetCommandLine() -> String {
	unsafe { WString::from_wchars_nullt(ffi::GetCommandLineW()) }
		.to_string()
}

/// [`GetComputerName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcomputernamew)
/// function.
#[must_use]
pub fn GetComputerName() -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_COMPUTERNAME_LENGTH + 1);
	let mut sz = buf.buf_len() as u32;

	bool_to_sysresult(
		unsafe { ffi::GetComputerNameW(buf.as_mut_ptr(), &mut sz) },
	).map(|_| buf.to_string())
}

/// [`GetCurrentDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcurrentdirectory)
/// function.
#[must_use]
pub fn GetCurrentDirectory() -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
	bool_to_sysresult(
		unsafe {
			ffi::GetCurrentDirectoryW(
				buf.buf_len() as _,
				buf.as_mut_ptr(),
			)
		} as _,
	).map(|_| buf.to_string())
}

/// [`GetCurrentProcessId`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid)
/// function.
#[must_use]
pub fn GetCurrentProcessId() -> u32 {
	unsafe { ffi::GetCurrentProcessId() }
}

/// [`GetCurrentThreadId`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid)
/// function.
#[must_use]
pub fn GetCurrentThreadId() -> u32 {
	unsafe { ffi::GetCurrentThreadId() }
}

/// [`GetDriveType`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getdrivetypew)
/// function.
#[must_use]
pub fn GetDriveType(root_path_name: Option<&str>) -> co::DRIVE {
	unsafe {
		co::DRIVE::from_raw(
			ffi::GetDriveTypeW(WString::from_opt_str(root_path_name).as_ptr()),
		)
	}
}

/// [`GetDiskFreeSpaceEx`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getdiskfreespaceexw)
/// function.
pub fn GetDiskFreeSpaceEx(
	directory_name: Option<&str>,
	free_bytes_available_to_caller: Option<&mut u64>,
	total_number_of_bytes: Option<&mut u64>,
	total_number_of_free_bytes: Option<&mut u64>,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::GetDiskFreeSpaceExW(
				WString::from_opt_str(directory_name).as_ptr(),
				free_bytes_available_to_caller.map_or(std::ptr::null_mut(), |n| n),
				total_number_of_bytes.map_or(std::ptr::null_mut(), |n| n),
				total_number_of_free_bytes.map_or(std::ptr::null_mut(), |n| n),
			)
		},
	)
}

/// [`GetDiskSpaceInformation`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getdiskspaceinformationw)
/// function.
pub fn GetDiskSpaceInformation(
	root_path: &str,
	disk_space_info: &mut DISK_SPACE_INFORMATION,
) -> SysResult<()>
{
	match unsafe {
		co::ERROR::from_raw(
			ffi::GetDiskSpaceInformationW(
				WString::from_str(root_path).as_ptr(),
				disk_space_info as *mut _ as _,
			),
		)
	} {
		co::ERROR::SUCCESS
			| co::ERROR::MORE_DATA => Ok(()),
		err => Err(err),
	}
}

/// [`GetEnvironmentStrings`](https://learn.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-getenvironmentstringsw)
/// function.
///
/// Returns the parsed strings, and automatically frees the retrieved
/// environment block with
/// [`FreeEnvironmentStrings`](https://learn.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-freeenvironmentstringsw).
///
/// # Examples
///
/// Retrieving and printing the key/value pairs of all environment strings:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let env_vars = w::GetEnvironmentStrings()?;
/// for (k, v) in env_vars.iter() {
///     println!("{} = {}", k, v);
/// }
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[must_use]
pub fn GetEnvironmentStrings() -> SysResult<HashMap<String, String>> {
	ptr_to_sysresult(unsafe { ffi::GetEnvironmentStringsW() } as _)
		.map(|ptr| {
			let vec_env_strs = parse_multi_z_str(ptr as *mut _ as _);
			unsafe { ffi::FreeEnvironmentStringsW(ptr); }
			vec_env_strs.iter()
				.map(|env_str| {
					let mut pair = env_str.split("="); // assumes correctly formatted pairs
					let key = pair.next().unwrap();
					let val = pair.next().unwrap();
					(key.to_owned(), val.to_owned())
				})
				.collect()
		})
}

/// [`GetFirmwareType`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getfirmwaretype)
/// function.
#[must_use]
pub fn GetFirmwareType() -> SysResult<co::FIRMWARE_TYPE> {
	let mut ft = u32::default();
	bool_to_sysresult(unsafe { ffi::GetFirmwareType(&mut ft) })
		.map(|_| unsafe { co::FIRMWARE_TYPE::from_raw(ft) })
}

/// [`GetLargePageMinimum`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getlargepageminimum)
/// function.
#[must_use]
pub fn GetLargePageMinimum() -> usize {
	unsafe { ffi::GetLargePageMinimum() }
}

/// [`GetLastError`](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
/// function.
///
/// This function is automatically called every time a
/// [`SysResult`](crate::SysResult) evaluates to `Err`, so it's unlikely that
/// you ever need to call it.
#[must_use]
pub fn GetLastError() -> co::ERROR {
	unsafe { co::ERROR::from_raw(ffi::GetLastError()) }
}

/// [`GetLengthSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getlengthsid)
/// function.
#[must_use]
pub fn GetLengthSid(sid: &SID) -> u32 {
	unsafe { ffi::GetLengthSid(sid as *const _ as _) }
}

/// [`GetLogicalDrives`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getlogicaldrives)
/// function.
#[must_use]
pub fn GetLogicalDrives() -> u32 {
	unsafe { ffi::GetLogicalDrives() }
}

/// [`GetLogicalDriveStrings`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getlogicaldrivestringsw)
/// function.
#[must_use]
pub fn GetLogicalDriveStrings() -> SysResult<Vec<String>> {
	let len = match unsafe {
		ffi::GetLogicalDriveStringsW(0, std::ptr::null_mut())
	} {
		0 => Err(GetLastError()),
		len => Ok(len),
	}?;

	let mut buf = WString::new_alloc_buf(len as usize + 1); // room for terminating null

	bool_to_sysresult(
		unsafe { ffi::GetLogicalDriveStringsW(len, buf.as_mut_ptr()) } as _,
	).map(|_| parse_multi_z_str(buf.as_ptr()))
}

/// [`GetFileAttributes`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileattributesw)
/// function.
///
/// # Examples
///
/// Checking whether a file or folder exists:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let file_exists = w::GetFileAttributes("C:\\Temp\\test.txt").is_ok();
/// ```
///
/// Retrieving various information about a file or folder path:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let flags = w::GetFileAttributes("C:\\Temp\\test.txt")?;
///
/// let is_compressed = flags.has(co::FILE_ATTRIBUTE::COMPRESSED);
/// let is_directory  = flags.has(co::FILE_ATTRIBUTE::DIRECTORY);
/// let is_encrypted  = flags.has(co::FILE_ATTRIBUTE::ENCRYPTED);
/// let is_hidden     = flags.has(co::FILE_ATTRIBUTE::HIDDEN);
/// let is_temporary  = flags.has(co::FILE_ATTRIBUTE::TEMPORARY);
/// # Ok::<_, co::ERROR>(())
/// ```
#[must_use]
pub fn GetFileAttributes(file_name: &str) -> SysResult<co::FILE_ATTRIBUTE> {
	const INVALID: u32 = INVALID_FILE_ATTRIBUTES as u32;
	match unsafe {
		ffi::GetFileAttributesW(WString::from_str(file_name).as_ptr())
	} {
		INVALID => Err(GetLastError()),
		flags => Ok(unsafe { co::FILE_ATTRIBUTE::from_raw(flags) }),
	}
}

/// [`GetLocalTime`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getlocaltime)
/// function.
///
/// This function retrieves local time; for UTC time use
/// [`GetSystemTime`](crate::GetSystemTime).
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let mut st = w::SYSTEMTIME::default();
/// w::GetLocalTime(&mut st);
/// ```
pub fn GetLocalTime(st: &mut SYSTEMTIME) {
	unsafe { ffi::GetLocalTime(st as *mut _ as _) }
}

/// [`GetNativeSystemInfo`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getnativesysteminfo)
/// function.
pub fn GetNativeSystemInfo(si: &mut SYSTEM_INFO) {
	unsafe { ffi::GetNativeSystemInfo(si as *mut _ as _) }
}

/// [`GetPrivateProfileSection`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprivateprofilesectionw)
/// function.
///
/// # Examples
///
/// Reading all key/value pairs of a section from an INI file:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let pairs = w::GetPrivateProfileSection(
///     "MySection",
///     "C:\\Temp\\foo.ini",
/// )?;
///
/// for (key, val) in pairs.iter() {
///     println!("{} = {}", key, val);
/// }
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[must_use]
pub fn GetPrivateProfileSection(
	section_name: &str,
	file_name: &str,
) -> SysResult<Vec<(String, String)>> {
	let mut buf_sz = SSO_LEN; // start with no string heap allocation
	loop {
		let mut buf = WString::new_alloc_buf(buf_sz);
		let returned_chars = unsafe { // char count without terminating null
			ffi::GetPrivateProfileSectionW(
				WString::from_str(section_name).as_ptr(),
				buf.as_mut_ptr(),
				buf.buf_len() as _,
				WString::from_str(file_name).as_ptr(),
			)
		} + 1 + 1; // plus terminating null count, plus weird extra count

		if GetLastError() == co::ERROR::FILE_NOT_FOUND {
			return Err(co::ERROR::FILE_NOT_FOUND);
		} else if (returned_chars as usize) < buf_sz { // to break, must have at least 1 char gap
			return Ok(
				parse_multi_z_str(buf.as_ptr())
					.iter()
					.map(|line| match line.split_once('=') {
						Some((key, val)) => (key.to_owned(), val.to_owned()),
						None => (String::default(), String::default()),
					})
					.collect(),
			);
		}

		buf_sz *= 2; // double the buffer size to try again
	}
}

/// [`GetPrivateProfileSectionNames`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprivateprofilesectionnamesw)
/// function.
///
/// # Examples
///
/// Reading all section names from an INI file:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let sections = w::GetPrivateProfileSectionNames(
///     Some("C:\\Temp\\foo.ini"),
/// )?;
///
/// for section in sections.iter() {
///     println!("{}", section);
/// }
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[must_use]
pub fn GetPrivateProfileSectionNames(
	file_name: Option<&str>,
) -> SysResult<Vec<String>>
{
	let mut buf_sz = SSO_LEN; // start with no string heap allocation
	loop {
		let mut buf = WString::new_alloc_buf(buf_sz);
		let returned_chars = unsafe { // char count without terminating null
			ffi::GetPrivateProfileSectionNamesW(
				buf.as_mut_ptr(),
				buf.buf_len() as _,
				WString::from_opt_str(file_name).as_ptr(),
			)
		} + 1 + 1; // plus terminating null count, plus weird extra count

		if GetLastError() == co::ERROR::FILE_NOT_FOUND {
			return Err(co::ERROR::FILE_NOT_FOUND);
		} else if (returned_chars as usize) < buf_sz { // to break, must have at least 1 char gap
			return Ok(parse_multi_z_str(buf.as_ptr()));
		}

		buf_sz *= 2; // double the buffer size to try again
	}
}

/// [`GetPrivateProfileString`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprivateprofilestringw)
/// function.
///
/// The related writing function is
/// [`WritePrivateProfileString`](crate::WritePrivateProfileString).
///
/// # Examples
///
/// Reading from an INI file:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let val = w::GetPrivateProfileString(
///     Some("MySection"),
///     Some("MyKey"),
///     None,
///     "C:\\Temp\\foo.ini",
/// )?;
///
/// println!("{}", val);
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[must_use]
pub fn GetPrivateProfileString(
	section_name: Option<&str>,
	key_name: Option<&str>,
	default_val: Option<&str>,
	file_name: &str,
) -> SysResult<String>
{
	let mut buf_sz = SSO_LEN; // start with no string heap allocation
	loop {
		let mut buf = WString::new_alloc_buf(buf_sz);
		let returned_chars = unsafe { // char count without terminating null
			ffi::GetPrivateProfileStringW(
				WString::from_opt_str(section_name).as_ptr(),
				WString::from_opt_str(key_name).as_ptr(),
				WString::from_opt_str(default_val).as_ptr(),
				buf.as_mut_ptr(),
				buf.buf_len() as _,
				WString::from_str(file_name).as_ptr(),
			)
		} + 1; // plus terminating null count

		if GetLastError() == co::ERROR::FILE_NOT_FOUND {
			return Err(co::ERROR::FILE_NOT_FOUND);
		} else if (returned_chars as usize) < buf_sz { // to break, must have at least 1 char gap
			return Ok(buf.to_string());
		}

		buf_sz *= 2; // double the buffer size to try again
	}
}

/// [`GetSidLengthRequired`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getsidlengthrequired)
/// function.
#[must_use]
pub fn GetSidLengthRequired(sub_authority_count: u8) -> u32 {
	unsafe { ffi::GetSidLengthRequired(sub_authority_count) }
}

/// [`GetStartupInfo`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getstartupinfow)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let mut si = w::STARTUPINFO::default();
/// w::GetStartupInfo(&mut si);
/// ```
pub fn GetStartupInfo(si: &mut STARTUPINFO) {
	unsafe { ffi::GetStartupInfoW(si as *mut _ as _) }
}

/// [`GetSystemDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemdirectoryw)
/// function.
#[must_use]
pub fn GetSystemDirectory() -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
	bool_to_sysresult(
		unsafe {
			ffi::GetSystemDirectoryW(buf.as_mut_ptr(), buf.buf_len() as _)
		} as _,
	).map(|_| buf.to_string())
}

/// [`GetSystemFileCacheSize`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getsystemfilecachesize)
/// function.
///
/// Returns minimum and maximum size of file cache (in bytes), and enabled cache
/// limit flags, respectively.
#[must_use]
pub fn GetSystemFileCacheSize() -> SysResult<(usize, usize, co::FILE_CACHE)> {
	let (mut min, mut max) = (usize::default(), usize::default());
	let mut flags = co::FILE_CACHE::default();
	bool_to_sysresult(
		unsafe {
			ffi::GetSystemFileCacheSize(&mut min, &mut max, flags.as_mut())
		},
	).map(|_| (min, max, flags))
}

/// [`GetSystemInfo`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsysteminfo)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let mut si = w::SYSTEM_INFO::default();
/// w::GetSystemInfo(&mut si);
/// ```
pub fn GetSystemInfo(si: &mut SYSTEM_INFO) {
	unsafe { ffi::GetSystemInfo(si as *mut _ as _) }
}

/// [`GetSystemTime`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtime)
/// function.
///
/// This function retrieves UTC time; for local time use
/// [`GetLocalTime`](crate::GetLocalTime).
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let mut st = w::SYSTEMTIME::default();
/// w::GetSystemTime(&mut st);
/// ```
pub fn GetSystemTime(st: &mut SYSTEMTIME) {
	unsafe { ffi::GetSystemTime(st as *mut _ as _) }
}

/// [`GetSystemTimeAsFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeasfiletime)
/// function.
pub fn GetSystemTimeAsFileTime(ft: &mut FILETIME) {
	unsafe { ffi::GetSystemTimeAsFileTime(ft as *mut _ as _) }
}

/// [`GetSystemTimePreciseAsFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimepreciseasfiletime)
/// function.
pub fn GetSystemTimePreciseAsFileTime(ft: &mut FILETIME) {
	unsafe { ffi::GetSystemTimePreciseAsFileTime(ft as *mut _ as _) }
}

/// [`GetSystemTimes`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemtimes)
/// function.
pub fn GetSystemTimes(
	idle_time: &mut FILETIME,
	kernel_time: &mut FILETIME,
	user_time: &mut FILETIME,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::GetSystemTimes(
				idle_time as *mut _ as _,
				kernel_time as *mut _ as _,
				user_time as *mut _ as _,
			)
		},
	)
}

/// [`GetTempFileName`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-gettempfilenamew)
/// function.
#[must_use]
pub fn GetTempFileName(
	path_name: &str,
	prefix: &str,
	unique: u32,
) -> SysResult<String>
{
	let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
	bool_to_sysresult(
		unsafe {
			ffi::GetTempFileNameW(
				WString::from_str(path_name).as_ptr(),
				WString::from_str(prefix).as_ptr(),
				unique,
				buf.as_mut_ptr(),
			)
		} as _,
	).map(|_| buf.to_string())
}

/// [`GetTempPath`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-gettemppathw)
/// function.
#[must_use]
pub fn GetTempPath() -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
	bool_to_sysresult(
		unsafe { ffi::GetTempPathW(buf.buf_len() as _, buf.as_mut_ptr()) } as _,
	).map(|_| buf.to_string())
}

/// [`GetTickCount64`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-gettickcount64)
/// function.
#[must_use]
pub fn GetTickCount64() -> u64 {
	unsafe { ffi::GetTickCount64() }
}

/// [`GetUserName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getusernamew)
/// function.
#[must_use]
pub fn GetUserName() -> SysResult<String> {
	let mut name_sz = u32::default();

	unsafe { ffi::GetUserNameW(std::ptr::null_mut(), &mut name_sz); }
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut name_buf = WString::new_alloc_buf(name_sz as _);

	bool_to_sysresult(
		unsafe { ffi::GetUserNameW(name_buf.as_mut_ptr(), &mut name_sz) },
	).map(|_| name_buf.to_string())
}

/// [`GetVolumeInformation`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getvolumeinformationw)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let mut name = String::default();
/// let mut serial_no = u32::default();
/// let mut max_comp_len = u32::default();
/// let mut sys_flags = co::FILE_VOL::default();
/// let mut sys_name = String::default();
///
/// w::GetVolumeInformation(
///     Some("C:\\"),
///     Some(&mut name),
///     Some(&mut serial_no),
///     Some(&mut max_comp_len),
///     Some(&mut sys_flags),
///     Some(&mut sys_name),
/// )?;
///
/// println!("Name: {}", name);
/// println!("Serial no: {:#010x}", serial_no);
/// println!("Max comp len: {}", max_comp_len);
/// println!("Sys flags: {:?}", sys_flags);
/// println!("Sys name: {}", sys_name);
/// # Ok::<_, co::ERROR>(())
/// ```
pub fn GetVolumeInformation(
	root_path_name: Option<&str>,
	name: Option<&mut String>,
	serial_number: Option<&mut u32>,
	max_component_len: Option<&mut u32>,
	file_system_flags: Option<&mut co::FILE_VOL>,
	file_system_name: Option<&mut String>,
) -> SysResult<()>
{
	let mut name_buf = match name {
		None => (WString::default(), 0),
		Some(_) => (WString::new_alloc_buf(MAX_PATH + 1), MAX_PATH + 1),
	};
	let mut sys_name_buf = match file_system_name {
		None => (WString::default(), 0),
		Some(_) => (WString::new_alloc_buf(MAX_PATH + 1), MAX_PATH + 1),
	};

	bool_to_sysresult(
		unsafe {
			ffi::GetVolumeInformationW(
				WString::from_opt_str(root_path_name).as_ptr(),
				match name {
					Some(_) => name_buf.0.as_mut_ptr(),
					None => std::ptr::null_mut(),
				},
				name_buf.1 as u32,
				serial_number.map_or(std::ptr::null_mut(), |n| n),
				max_component_len.map_or(std::ptr::null_mut(), |m| m),
				file_system_flags.map_or(std::ptr::null_mut(), |f| f.as_mut()),
				match file_system_name {
					Some(_) => sys_name_buf.0.as_mut_ptr(),
					None => std::ptr::null_mut(),
				},
				sys_name_buf.1 as u32,
			)
		},
	).map(|_| {
		if let Some(name) = name {
			*name = name_buf.0.to_string();
		}
		if let Some(sys_name) = file_system_name {
			*sys_name = sys_name_buf.0.to_string();
		}
	})
}

/// [`GetVolumePathName`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getvolumepathnamew)
/// function.
#[must_use]
pub fn GetVolumePathName(file_name: &str) -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
	bool_to_sysresult(
		unsafe {
			ffi::GetVolumePathNameW(
				WString::from_str(file_name).as_ptr(),
				buf.as_mut_ptr(),
				buf.buf_len() as _,
			)
		} as _,
	).map(|_| buf.to_string())
}

/// [`GetWindowsAccountDomainSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getwindowsaccountdomainsid)
/// function.
#[must_use]
pub fn GetWindowsAccountDomainSid(sid: &SID) -> SysResult<SidGuard> {
	let mut ad_sid_sz = u32::default();

	unsafe {
		ffi::GetWindowsAccountDomainSid(
			sid as *const _ as _,
			std::ptr::null_mut(),
			&mut ad_sid_sz,
		)
	};
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let ad_sid_buf = HGLOBAL::GlobalAlloc(
		Some(co::GMEM::FIXED | co::GMEM::ZEROINIT),
		ad_sid_sz as _,
	)?;

	unsafe {
		bool_to_sysresult(
			ffi::GetWindowsAccountDomainSid(
				sid as *const _ as _,
				ad_sid_buf.ptr(),
				&mut ad_sid_sz,
			),
		).map(|_| SidGuard::new(ad_sid_buf))
	}
}

/// [`GlobalMemoryStatusEx`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-globalmemorystatusex)
/// function.
pub fn GlobalMemoryStatusEx(msx: &mut MEMORYSTATUSEX) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { ffi::GlobalMemoryStatusEx(msx as *mut _ as _) },
	)
}

/// [`HIBYTE`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632656(v=vs.85))
/// macro.
#[must_use]
pub const fn HIBYTE(v: u16) -> u8 {
	(v >> 8 & 0xff) as _
}

/// Returns the high-order `u32` of an `u64`.
#[must_use]
pub const fn HIDWORD(v: u64) -> u32 {
	(v >> 32 & 0xffff_ffff) as _
}

/// [`HIWORD`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632657(v=vs.85))
/// macro.
#[must_use]
pub const fn HIWORD(v: u32) -> u16 {
	(v >> 16 & 0xffff) as _
}

/// [`InitializeSecurityDescriptor`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-initializesecuritydescriptor)
/// function.
///
/// You don't need to call this function directly, because
/// [`SECURITY_DESCRIPTOR`](crate::SECURITY_DESCRIPTOR) implements the
/// [`Default`](std::default::Default) trait, which calls it.
#[must_use]
pub fn InitializeSecurityDescriptor() -> SysResult<SECURITY_DESCRIPTOR> {
	let mut sd = unsafe { std::mem::zeroed::<SECURITY_DESCRIPTOR>() };
	bool_to_sysresult(
		unsafe {
			ffi::InitializeSecurityDescriptor(
				&mut sd as *mut _ as _,
				SECURITY_DESCRIPTOR_REVISION,
			)
		},
	).map(|_| sd)
}

/// [`InitiateSystemShutdown`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-initiatesystemshutdownw)
/// function.
pub fn InitiateSystemShutdown(
	machine_name: Option<&str>,
	message: Option<&str>,
	timeout: u32,
	force_apps_closed: bool,
	reboot_after_shutdown: bool,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::InitiateSystemShutdownW(
				WString::from_opt_str(machine_name).as_ptr(),
				WString::from_opt_str(message).as_ptr(),
				timeout,
				force_apps_closed as _,
				reboot_after_shutdown as _,
			)
		},
	)
}

/// [`InitiateSystemShutdownEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-initiatesystemshutdownexw)
/// function.
pub fn InitiateSystemShutdownEx(
	machine_name: Option<&str>,
	message: Option<&str>,
	timeout: u32,
	force_apps_closed: bool,
	reboot_after_shutdown: bool,
	reason: Option<co::SHTDN_REASON>,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::InitiateSystemShutdownExW(
				WString::from_opt_str(machine_name).as_ptr(),
				WString::from_opt_str(message).as_ptr(),
				timeout,
				force_apps_closed as _,
				reboot_after_shutdown as _,
				reason.unwrap_or_default().raw(),
			)
		},
	)
}

/// [`IsDebuggerPresent`](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-isdebuggerpresent)
/// function.
#[must_use]
pub fn IsDebuggerPresent() -> bool {
	unsafe { ffi::IsDebuggerPresent() != 0 }
}

/// [`IsNativeVhdBoot`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-isnativevhdboot)
/// function.
#[must_use]
pub fn IsNativeVhdBoot() -> SysResult<bool> {
	let mut is_native: BOOL = 0;
	bool_to_sysresult(unsafe { ffi::IsNativeVhdBoot(&mut is_native) })
		.map(|_| is_native != 0)
}

/// [`IsValidSecurityDescriptor`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-isvalidsecuritydescriptor)
/// function.
#[must_use]
pub fn IsValidSecurityDescriptor(sd: &SECURITY_DESCRIPTOR) -> bool {
	unsafe { ffi::IsValidSecurityDescriptor(sd as *const _ as _) != 0 }
}

/// [`IsValidSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-isvalidsid)
/// function.
#[must_use]
pub fn IsValidSid(sid: &SID) -> SysResult<bool> {
	match unsafe { ffi::IsValidSid(sid as *const _ as _) } {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`IsWellKnownSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-iswellknownsid)
/// function.
#[must_use]
pub fn IsWellKnownSid(
	sid: &SID,
	well_known_sid: co::WELL_KNOWN_SID_TYPE,
) -> bool
{
	unsafe {
		ffi::IsWellKnownSid(sid as *const _ as _, well_known_sid.raw()) != 0
	}
}

/// [`IsWindows10OrGreater`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows10orgreater)
/// function.
#[must_use]
pub fn IsWindows10OrGreater() -> SysResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WINTHRESHOLD.raw()) as _,
		LOBYTE(co::WIN32::WINNT_WINTHRESHOLD.raw()) as _,
		0,
	)
}

/// [`IsWindows7OrGreater`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows7orgreater)
/// function.
#[must_use]
pub fn IsWindows7OrGreater() -> SysResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WIN7.raw()) as _,
		LOBYTE(co::WIN32::WINNT_WIN7.raw()) as _,
		0,
	)
}

/// [`IsWindows8OrGreater`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows8orgreater)
/// function.
#[must_use]
pub fn IsWindows8OrGreater() -> SysResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WIN8.raw()) as _,
		LOBYTE(co::WIN32::WINNT_WIN8.raw()) as _,
		0,
	)
}

/// [`IsWindows8Point1OrGreater`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows8point1orgreater)
/// function.
#[must_use]
pub fn IsWindows8Point1OrGreater() -> SysResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WINBLUE.raw()) as _,
		LOBYTE(co::WIN32::WINNT_WINBLUE.raw()) as _,
		0,
	)
}

/// [`IsWindowsServer`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindowsserver)
/// function.
#[must_use]
pub fn IsWindowsServer() -> SysResult<bool> {
	let mut osvi = OSVERSIONINFOEX::default();
	osvi.wProductType = co::VER_NT::WORKSTATION;
	let cond_mask = VerSetConditionMask(
		0, co::VER_MASK::PRODUCT_TYPE, co::VER_COND::EQUAL);
	VerifyVersionInfo(&mut osvi, co::VER_MASK::PRODUCT_TYPE, cond_mask)
		.map(|b| !b) // not workstation
}

/// [`IsWindowsVersionOrGreater`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindowsversionorgreater)
/// function.
#[must_use]
pub fn IsWindowsVersionOrGreater(
	major_version: u16,
	minor_version: u16,
	service_pack_major: u16,
) -> SysResult<bool>
{
	let mut osvi = OSVERSIONINFOEX::default();
	let cond_mask = VerSetConditionMask(
		VerSetConditionMask(
			VerSetConditionMask(0, co::VER_MASK::MAJORVERSION, co::VER_COND::GREATER_EQUAL),
			co::VER_MASK::MINORVERSION, co::VER_COND::GREATER_EQUAL,
		),
		co::VER_MASK::SERVICEPACKMAJOR, co::VER_COND::GREATER_EQUAL
	);

	osvi.dwMajorVersion = major_version as _;
	osvi.dwMinorVersion = minor_version as _;
	osvi.wServicePackMajor = service_pack_major;

	VerifyVersionInfo(
		&mut osvi,
		co::VER_MASK::MAJORVERSION | co::VER_MASK::MINORVERSION | co::VER_MASK::SERVICEPACKMAJOR,
		cond_mask,
	)
}

/// [`IsWindowsVistaOrGreater`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindowsvistaorgreater)
/// function.
#[must_use]
pub fn IsWindowsVistaOrGreater() -> SysResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_VISTA.raw()) as _,
		LOBYTE(co::WIN32::WINNT_VISTA.raw()) as _,
		0,
	)
}

/// [`LOBYTE`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632658(v=vs.85))
/// macro.
#[must_use]
pub const fn LOBYTE(v: u16) -> u8 {
	(v & 0xff) as _
}

/// Returns the low-order `u32` of an `u64`.
#[must_use]
pub const fn LODWORD(v: u64) -> u32 {
	(v & 0xffff_ffff) as _
}

/// [`LookupAccountName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupaccountnamew)
/// function.
///
/// Returns account's domain name, `SID` and type, respectively.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let user_name = w::GetUserName()?;
/// let (domain_name, sid, kind) = w::LookupAccountName(None, &user_name)?;
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[must_use]
pub fn LookupAccountName(
	system_name: Option<&str>,
	account_name: &str,
) -> SysResult<(String, SidGuard, co::SID_NAME_USE)>
{
	let mut sid_sz = u32::default();
	let mut domain_sz = u32::default();
	let mut sid_name_use = co::SID_NAME_USE::default();

	unsafe {
		ffi::LookupAccountNameW( // retrieve needed buffer sizes
			WString::from_opt_str(system_name).as_ptr(),
			WString::from_str(account_name).as_ptr(),
			std::ptr::null_mut(),
			&mut sid_sz,
			std::ptr::null_mut(),
			&mut domain_sz,
			sid_name_use.as_mut(),
		);
	}
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let sid_buf = HGLOBAL::GlobalAlloc(
		Some(co::GMEM::FIXED | co::GMEM::ZEROINIT),
		sid_sz as _,
	)?;
	let mut domain_buf = WString::new_alloc_buf(domain_sz as _);

	unsafe {
		bool_to_sysresult(
			ffi::LookupAccountNameW(
				WString::from_opt_str(system_name).as_ptr(),
				WString::from_str(account_name).as_ptr(),
				sid_buf.ptr(),
				&mut sid_sz,
				domain_buf.as_mut_ptr(),
				&mut domain_sz,
				sid_name_use.as_mut(),
			),
		).map(|_| (domain_buf.to_string(), SidGuard::new(sid_buf), sid_name_use))
	}
}

/// [`LookupAccountSid`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupaccountsidw)
/// function.
///
/// Returns account name, domain name and type, respectively.
#[must_use]
pub fn LookupAccountSid(
	system_name: Option<&str>,
	sid: &SID,
) -> SysResult<(String, String, co::SID_NAME_USE)>
{
	let mut account_sz = u32::default();
	let mut domain_sz = u32::default();
	let mut sid_name_use = co::SID_NAME_USE::default();

	unsafe {
		ffi::LookupAccountSidW( // retrieve needed buffer sizes
			WString::from_opt_str(system_name).as_ptr(),
			sid as *const _ as _,
			std::ptr::null_mut(),
			&mut account_sz,
			std::ptr::null_mut(),
			&mut domain_sz,
			sid_name_use.as_mut(),
		);
	}
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut account_buf = WString::new_alloc_buf(account_sz as _);
	let mut domain_buf = WString::new_alloc_buf(domain_sz as _);

	bool_to_sysresult(
		unsafe {
			ffi::LookupAccountSidW(
				WString::from_opt_str(system_name).as_ptr(),
				sid as *const _ as _,
				account_buf.as_mut_ptr(),
				&mut account_sz,
				domain_buf.as_mut_ptr(),
				&mut domain_sz,
				sid_name_use.as_mut(),
			)
		},
	).map(|_| (account_buf.to_string(), domain_buf.to_string(), sid_name_use))
}

/// [`LookupPrivilegeName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegenamew)
/// function.
#[must_use]
pub fn LookupPrivilegeName(
	system_name: Option<&str>,
	luid: LUID,
) -> SysResult<co::SE_PRIV>
{
	let mut cch_name = u32::default();

	bool_to_sysresult(
		unsafe {
			ffi::LookupPrivilegeNameW(
				WString::from_opt_str(system_name).as_ptr(),
				&luid as *const _ as _,
				std::ptr::null_mut(),
				&mut cch_name,
			)
		},
	)?;

	let mut buf = WString::new_alloc_buf(cch_name as _);

	bool_to_sysresult(
		unsafe {
			ffi::LookupPrivilegeNameW(
				WString::from_opt_str(system_name).as_ptr(),
				&luid as *const _ as _,
				buf.as_mut_ptr(),
				&mut cch_name,
			)
		},
	).map(|_| co::SE_PRIV::try_from(buf.to_string().as_str()))?
}

/// [`LookupPrivilegeValue`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let htoken = w::HPROCESS::GetCurrentProcess()
///     .OpenProcessToken(co::TOKEN::ADJUST_PRIVILEGES | co::TOKEN::QUERY)?;
///
/// let luid = w::LookupPrivilegeValue(None, co::SE_PRIV::SHUTDOWN_NAME)?;
/// # Ok::<_, co::ERROR>(())
/// ```
#[must_use]
pub fn LookupPrivilegeValue(
	system_name: Option<&str>,
	name: co::SE_PRIV,
) -> SysResult<LUID>
{
	let mut luid = LUID::new(0, 0);
	bool_to_sysresult(
		unsafe {
			ffi::LookupPrivilegeValueW(
				WString::from_opt_str(system_name).as_ptr(),
				WString::from(name).as_ptr(),
				&mut luid as *mut _ as _,
			)
		},
	).map(|_| luid)
}

/// [`LOWORD`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632659(v=vs.85))
/// macro.
#[must_use]
pub const fn LOWORD(v: u32) -> u16 {
	(v & 0xffff) as _
}

/// Function that implements
/// [`MAKELONG`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632660(v=vs.85)),
/// [`MAKEWPARAM`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makewparam),
/// and
/// [`MAKELPARAM`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makelparam)
/// macros.
#[must_use]
pub const fn MAKEDWORD(lo: u16, hi: u16) -> u32 {
	((lo as u32 & 0xffff) | ((hi as u32 & 0xffff) << 16)) as _
}

/// Similar to [`MAKEDWORD`](crate::MAKEDWORD), but for `u64`.
#[must_use]
pub const fn MAKEQWORD(lo: u32, hi: u32) -> u64 {
	((lo as u64 & 0xffff_ffff) | ((hi as u64 & 0xffff_ffff) << 32)) as _
}

/// [`MAKEWORD`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632663(v=vs.85))
/// macro.
#[must_use]
pub const fn MAKEWORD(lo: u8, hi: u8) -> u16 {
	(lo as u16 & 0xff) | ((hi as u16 & 0xff) << 8) as u16
}

/// [`MoveFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefilew)
/// function.
pub fn MoveFile(existing_file: &str, new_file: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			ffi::MoveFileW(
				WString::from_str(existing_file).as_ptr(),
				WString::from_str(new_file).as_ptr(),
			)
		},
	)
}

/// [`MulDiv`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-muldiv)
/// function.
#[must_use]
pub fn MulDiv(number: i32, numerator: i32, denominator: i32) -> i32 {
	unsafe { ffi::MulDiv(number, numerator, denominator) }
}

/// [`MultiByteToWideChar`](https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-multibytetowidechar)
/// function.
///
/// If `multi_byte_str` doesn't have a terminating null, the resulting
/// `Vec<u16>` also won't include one.
#[must_use]
pub fn MultiByteToWideChar(
	code_page: co::CP,
	flags: co::MBC,
	multi_byte_str: &[u8],
) -> SysResult<Vec<u16>>
{
	let num_bytes = match unsafe {
		ffi::MultiByteToWideChar(
			code_page.raw() as _,
			flags.raw(),
			vec_ptr(multi_byte_str),
			multi_byte_str.len() as _,
			std::ptr::null_mut(),
			0,
		)
	} {
		0 => Err(GetLastError()),
		num_bytes => Ok(num_bytes),
	}?;

	let mut buf = vec![0u16; num_bytes as _];

	bool_to_sysresult(
		unsafe {
			ffi::MultiByteToWideChar(
				code_page.raw() as _,
				flags.raw(),
				vec_ptr(multi_byte_str),
				multi_byte_str.len() as _,
				buf.as_mut_ptr(),
				num_bytes as _,
			)
		},
	).map(|_| buf)
}

/// [`OutputDebugString`](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringw)
/// function.
pub fn OutputDebugString(output_string: &str) {
	unsafe {
		ffi::OutputDebugStringW(WString::from_str(output_string).as_ptr())
	}
}

/// [`QueryPerformanceCounter`](https://learn.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let freq = w::QueryPerformanceFrequency()?;
/// let t0 = w::QueryPerformanceCounter()?;
///
/// // perform some operation...
///
/// let duration_ms =
///     ((w::QueryPerformanceCounter()? - t0) as f64 / freq as f64) * 1000.0;
///
/// println!("Operation lasted {:.2} ms", duration_ms);
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[must_use]
pub fn QueryPerformanceCounter() -> SysResult<i64> {
	let mut perf_count = i64::default();
	bool_to_sysresult(unsafe { ffi::QueryPerformanceCounter(&mut perf_count) })
		.map(|_| perf_count)
}

/// [`QueryPerformanceFrequency`](https://learn.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter)
/// function.
///
/// Usually used with
/// [`QueryPerformanceCounter`](crate::QueryPerformanceCounter).
#[must_use]
pub fn QueryPerformanceFrequency() -> SysResult<i64> {
	let mut freq = i64::default();
	bool_to_sysresult(unsafe { ffi::QueryPerformanceFrequency(&mut freq) })
		.map(|_| freq)
}

/// [`ReplaceFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-replacefilew)
/// function.
pub fn ReplaceFile(
	replaced: &str,
	replacement: &str,
	backup: Option<&str>,
	flags: co::REPLACEFILE,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::ReplaceFileW(
				WString::from_str(replaced).as_ptr(),
				WString::from_str(replacement).as_ptr(),
				WString::from_opt_str(backup).as_ptr(),
				flags.raw(),
				std::ptr::null_mut(),
				std::ptr::null_mut(),
			)
		},
	)
}

/// [`SetCurrentDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setcurrentdirectory)
/// function.
pub fn SetCurrentDirectory(path_name: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			ffi::SetCurrentDirectoryW(WString::from_str(path_name).as_ptr())
		},
	)
}

/// [`SetFileAttributes`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfileattributesw)
/// function.
pub fn SetFileAttributes(
	file_name: &str,
	attributes: co::FILE_ATTRIBUTE,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::SetFileAttributesW(
				WString::from_str(file_name).as_ptr(),
				attributes.raw(),
			)
		},
	)
}

/// [`SetLastError`](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
/// function.
pub fn SetLastError(err_code: co::ERROR) {
	unsafe { ffi::SetLastError(err_code.raw()) }
}

/// [`SetThreadStackGuarantee`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadstackguarantee)
/// function.
///
/// Returns the size of the previous stack.
pub fn SetThreadStackGuarantee(stack_size_in_bytes: u32) -> SysResult<u32> {
	let mut sz = stack_size_in_bytes;
	bool_to_sysresult(unsafe { ffi::SetThreadStackGuarantee(&mut sz) })
		.map(|_| sz)
}

/// [`Sleep`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleep)
/// function.
pub fn Sleep(milliseconds: u32) {
	unsafe { ffi::Sleep(milliseconds) }
}

/// [`SwitchToThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-switchtothread)
/// function.
pub fn SwitchToThread() -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::SwitchToThread() })
}

/// [`SystemTimeToFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetofiletime)
/// function.
pub fn SystemTimeToFileTime(
	st: &SYSTEMTIME,
	ft: &mut FILETIME,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::SystemTimeToFileTime(st as *const _ as _, ft as *mut _ as _)
		},
	)
}

/// [`SystemTimeToTzSpecificLocalTime`](https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetotzspecificlocaltime)
/// function.
pub fn SystemTimeToTzSpecificLocalTime(
	time_zone: Option<&TIME_ZONE_INFORMATION>,
	universal_time: &SYSTEMTIME,
	local_time: &mut SYSTEMTIME,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::SystemTimeToTzSpecificLocalTime(
				time_zone.map_or(std::ptr::null(), |lp| lp as *const _ as _),
				universal_time as *const _ as _,
				local_time as *mut _ as _,
			)
		},
	)
}

/// [`VerifyVersionInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-verifyversioninfow)
/// function.
#[must_use]
pub fn VerifyVersionInfo(
	osvix: &mut OSVERSIONINFOEX,
	type_mask: co::VER_MASK,
	condition_mask: u64,
) -> SysResult<bool>
{
	match unsafe {
		ffi::VerifyVersionInfoW(
			osvix as *mut _ as _,
			type_mask.raw(),
			condition_mask,
		)
	} {
		0 => match GetLastError() {
			co::ERROR::OLD_WIN_VERSION => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`VerSetConditionMask`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-versetconditionmask)
/// function.
#[must_use]
pub fn VerSetConditionMask(
	condition_mask: u64,
	type_mask: co::VER_MASK,
	condition: co::VER_COND,
) -> u64
{
	unsafe {
		ffi::VerSetConditionMask(condition_mask, type_mask.raw(), condition.raw())
	}
}

/// [`WideCharToMultiByte`](https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-widechartomultibyte)
/// function.
///
/// If `wide_char_str` doesn't have a terminating null, the resulting `Vec<u8>`
/// also won't include one.
#[must_use]
pub fn WideCharToMultiByte(
	code_page: co::CP,
	flags: co::WC,
	wide_char_str: &[u16],
	default_char: Option<u8>,
	used_default_char: Option<&mut bool>,
) -> SysResult<Vec<u8>>
{
	let mut default_char_buf = default_char.unwrap_or_default();

	let num_bytes = match unsafe {
		ffi::WideCharToMultiByte(
			code_page.raw() as _,
			flags.raw(),
			vec_ptr(wide_char_str),
			wide_char_str.len() as _,
			std::ptr::null_mut(),
			0,
			&mut default_char_buf,
			std::ptr::null_mut(),
		)
	} {
		0 => Err(GetLastError()),
		num_bytes => Ok(num_bytes),
	}?;

	let mut u8_buf = vec![0u8; num_bytes as _];
	let mut bool_buf: BOOL = 0;

	bool_to_sysresult(
		unsafe {
			ffi::WideCharToMultiByte(
				code_page.raw() as _,
				flags.raw(),
				vec_ptr(wide_char_str),
				wide_char_str.len() as _,
				u8_buf.as_mut_ptr() as _,
				num_bytes as _,
				&mut default_char_buf,
				&mut bool_buf,
			)
		},
	).map(|_| {
		if let Some(used_default_char) = used_default_char {
			*used_default_char = bool_buf != 0;
		}
		u8_buf
	})
}

/// [`WritePrivateProfileString`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-writeprivateprofilestringw)
/// function.
///
/// The related reading function is
/// [`GetPrivateProfileString`](crate::GetPrivateProfileString).
///
/// # Examples
///
/// Writing value into an INI file:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// w::WritePrivateProfileString(
///     "MySection",
///     Some("MyKey"),
///     Some("new value"),
///     "C:\\Temp\\foo.ini",
/// )?;
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
pub fn WritePrivateProfileString(
	section_name: &str,
	key_name: Option<&str>,
	new_val: Option<&str>,
	file_name: &str,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::WritePrivateProfileStringW(
				WString::from_str(section_name).as_ptr(),
				WString::from_opt_str(key_name).as_ptr(),
				WString::from_opt_str(new_val).as_ptr(),
				WString::from_str(file_name).as_ptr(),
			)
		},
	)
}
