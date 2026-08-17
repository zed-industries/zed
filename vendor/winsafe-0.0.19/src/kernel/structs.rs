#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

/// [`ACL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct ACL {
	pub AclRevision: u8,
	pub Sbz1: u8,
	pub AclSize: u16,
	pub AceCount: u16,
	pub Sbz2: u16,
}

/// [`BY_HANDLE_FILE_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-by_handle_file_information)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct BY_HANDLE_FILE_INFORMATION {
	pub dwFileAttributes: co::FILE_ATTRIBUTE,
	pub ftCreationTime: FILETIME,
	pub ftLastAccessTime: FILETIME,
	pub ftLastWriteTime: FILETIME,
	pub dwVolumeSerialNumber: u32,
	pub nFileSizeHigh: u32,
	pub nFileSizeLow: u32,
	pub nNumberOfLinks: u32,
	pub nFileIndexHigh: u32,
	pub nFileIndexLow: u32,
}

/// [`CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-claim_security_attribute_fqbn_value)
/// struct.
#[repr(C)]
pub struct CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE<'a> {
	pub Version: u64,
	Name: *mut u16,

	_Name: PhantomData<&'a mut u16>,
}

impl_default!(CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE, 'a);

impl<'a> CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE<'a> {
	pub_fn_string_ptr_get_set!('a, Name, set_Name);
}

/// [`CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-claim_security_attribute_octet_string_value)
/// struct.
#[repr(C)]
pub struct CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE<'a> {
	pValue: *mut u8,
	ValueLength: u32,

	_pValue: PhantomData<&'a mut ()>,
}

impl_default!(CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE, 'a);

impl<'a> CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE<'a> {
	pub_fn_array_buf_get_set!('a, pValue, set_pValue, ValueLength, u8);
}

/// [`CLAIM_SECURITY_ATTRIBUTE_V1`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-claim_security_attribute_v1)
/// struct.
#[repr(C)]
pub struct CLAIM_SECURITY_ATTRIBUTE_V1<'a, 'b> {
	Name: *mut u16,
	ValueType: co::CLAIM_SECURITY_ATTRIBUTE_TYPE,
	Reserved: u16,
	Flags: u32,
	ValueCount: u32,
	Values: CLAIM_SECURITY_ATTRIBUTE_V1_union0<'b>,

	_Name: PhantomData<&'a mut u16>,
}

#[repr(C)]
union CLAIM_SECURITY_ATTRIBUTE_V1_union0<'a> {
	pInt64: *mut i64, // pointers because these are all arrays with ValueCount items
	pUint64: *mut u64,
	ppString: *mut *mut u16,
	pFqbn: *mut CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE<'a>,
	pOctetString: *mut CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE<'a>,
}

impl_default!(CLAIM_SECURITY_ATTRIBUTE_V1, 'a, 'b);

impl<'a, 'b> CLAIM_SECURITY_ATTRIBUTE_V1<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, Name, set_Name);

	/// Returns the low-word part of `Flags`.
	#[must_use]
	pub const fn FlagsLo(&self) -> co::CLAIM_SECURITY_ATTRIBUTE {
		unsafe { co::CLAIM_SECURITY_ATTRIBUTE::from_raw(LOWORD(self.Flags)) }
	}

	/// Sets the low-word part of `Flags`.
	pub fn set_FlagsLo(&mut self, claim: co::CLAIM_SECURITY_ATTRIBUTE) {
		self.Flags = MAKEDWORD(claim.raw(), self.FlagsHi());
	}

	/// Returns the high-word part of `Flags`.
	#[must_use]
	pub const fn FlagsHi(&self) -> u16 {
		HIWORD(self.Flags)
	}

	/// Sets the high-word part of `Flags`.
	pub fn set_FlagsHi(&mut self, flags: u16) {
		self.Flags = MAKEDWORD(self.FlagsLo().raw(), flags);
	}

	/// Returns the `Values` field.
	///
	/// # Panics
	///
	/// Panics if `ValueType` field is invalid.
	#[must_use]
	pub fn Values(&self) -> ClaimSecurityAttr {
		unsafe {
			match self.ValueType {
				co::CLAIM_SECURITY_ATTRIBUTE_TYPE::INT64 => ClaimSecurityAttr::Int64(
					std::slice::from_raw_parts(self.Values.pInt64, self.ValueCount as _),
				),
				co::CLAIM_SECURITY_ATTRIBUTE_TYPE::UINT64 => ClaimSecurityAttr::Uint64(
					std::slice::from_raw_parts(self.Values.pUint64, self.ValueCount as _),
				),
				co::CLAIM_SECURITY_ATTRIBUTE_TYPE::STRING => ClaimSecurityAttr::String(
					std::slice::from_raw_parts(self.Values.ppString, self.ValueCount as _)
						.iter()
						.map(|str_ptr| WString::from_wchars_nullt(*str_ptr).to_string())
						.collect(),
				),
				co::CLAIM_SECURITY_ATTRIBUTE_TYPE::FQBN => ClaimSecurityAttr::Fbqn(
					std::slice::from_raw_parts(self.Values.pFqbn, self.ValueCount as _),
				),
				co::CLAIM_SECURITY_ATTRIBUTE_TYPE::OCTET_STRING => ClaimSecurityAttr::OctetString(
					std::slice::from_raw_parts(self.Values.pOctetString, self.ValueCount as _),
				),
				_ => panic!("Invalid ValueType.")
			}
		}
	}
}

/// [`CLAIM_SECURITY_ATTRIBUTES_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-claim_security_attributes_information)
/// struct.
#[repr(C)]
pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION<'a, 'b> {
	pub Version: u16,
	Reserved: u16,
	AttributeCount: u32,
	pAttributeV1: *mut CLAIM_SECURITY_ATTRIBUTE_V1<'a, 'b>,
}

impl_default!(CLAIM_SECURITY_ATTRIBUTES_INFORMATION, 'a, 'b);

impl<'a, 'b> CLAIM_SECURITY_ATTRIBUTES_INFORMATION<'a, 'b> {
	/// Returns the `pAttributeV1` field.
	#[must_use]
	pub fn pAttributeV1(&self) -> &[CLAIM_SECURITY_ATTRIBUTE_V1<'a, 'b>] {
		unsafe {
			std::slice::from_raw_parts(self.pAttributeV1, self.AttributeCount as _)
		}
	}
}

/// [`CONSOLE_READCONSOLE_CONTROL`](https://learn.microsoft.com/en-us/windows/console/console-readconsole-control)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct CONSOLE_READCONSOLE_CONTROL {
	pub nLength: u32,
	pub nInitialChars: u32,
	pub dwCtrlWakeupMask: u32,
	pub dwControlKeyState: u32,
}

/// [`DEV_BROADCAST_DEVICEINTERFACE`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_deviceinterface_w)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct DEV_BROADCAST_DEVICEINTERFACE {
	pub hdr: DEV_BROADCAST_HDR,
	pub dbcc_classguid: GUID,
	dbcc_name: [u16; 1],
}

impl DEV_BROADCAST_DEVICEINTERFACE {
	/// Returns the `dbcc_name` field.
	#[must_use]
	pub fn dbcc_name(&self) -> String {
		unsafe { WString::from_wchars_nullt(self.dbcc_name.as_ptr()) }
			.to_string()
	}
}

/// [`DEV_BROADCAST_HANDLE`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_handle)
/// struct.
#[repr(C)]
pub struct DEV_BROADCAST_HANDLE {
	pub hdr: DEV_BROADCAST_HDR,
	pub dbch_handle: usize,
	pub dbch_hdevnotify: usize, // HDEVNOTIFY
	pub dbch_eventguid: GUID,
	pub dbch_nameoffset: i16,
	pub dbch_data: [u8; 1],
}

/// [`DEV_BROADCAST_HDR`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_hdr)
/// struct.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DEV_BROADCAST_HDR {
	pub dbch_size: u32,
	pub dbch_devicetype: co::DBT_DEVTYP,
	dbch_reserved: u32,
}

/// [`DEV_BROADCAST_OEM`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_oem)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct DEV_BROADCAST_OEM {
	pub hdr: DEV_BROADCAST_HDR,
	pub dbco_identifier: u32,
	pub dbco_suppfunc: u32,
}

/// [`DEV_BROADCAST_PORT`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_port_w)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct DEV_BROADCAST_PORT {
	pub hdr: DEV_BROADCAST_HDR,
	dbcp_name: [u16; 1],
}

impl DEV_BROADCAST_PORT {
	/// Returns the `dbcp_name` field.
	#[must_use]
	pub fn dbcp_name(&self) -> String {
		unsafe { WString::from_wchars_nullt(self.dbcp_name.as_ptr()) }
			.to_string()
	}
}

/// [`DEV_BROADCAST_VOLUME`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_volume)
/// struct.
#[derive(Default)]
pub struct DEV_BROADCAST_VOLUME {
	pub hdr: DEV_BROADCAST_HDR,
	pub dbcv_unitmask: u32,
	pub dbcv_flags: co::DBTF,
}

/// [`DISK_SPACE_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-disk_space_information)
/// struct.
#[repr(C)]
#[derive(Default, Clone, PartialEq, Eq)]
pub struct DISK_SPACE_INFORMATION {
	pub ActualTotalAllocationUnits: u64,
	pub ActualAvailableAllocationUnits: u64,
	pub ActualPoolUnavailableAllocationUnits: u64,
	pub CallerTotalAllocationUnits: u64,
	pub CallerAvailableAllocationUnits: u64,
	pub CallerPoolUnavailableAllocationUnits: u64,
	pub UsedAllocationUnits: u64,
	pub TotalReservedAllocationUnits: u64,
	pub VolumeStorageReserveAllocationUnits: u64,
	pub AvailableCommittedAllocationUnits: u64,
	pub PoolAvailableAllocationUnits: u64,
	pub SectorsPerAllocationUnit: u32,
	pub BytesPerSector: u32,
}

/// [`FILETIME`](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-filetime)
/// struct.
///
/// Can be converted to [`SYSTEMTIME`](crate::SYSTEMTIME) with
/// [`FileTimeToSystemTime`](crate::FileTimeToSystemTime) function.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FILETIME {
	pub dwLowDateTime: u32,
	pub dwHighDateTime: u32,
}

/// [`GUID`](https://learn.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid)
/// struct.
///
/// The [`Default`](std::default::Default) implementation returns `GUID::NULL`
/// (all zeros).
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GUID {
	data1: u32,
	data2: u16,
	data3: u16,
	data4: u64,
}

impl std::fmt::Display for GUID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
			self.data1, self.data2, self.data3,
			self.data4.swap_bytes() >> 48,
			self.data4.swap_bytes() & 0x0000_ffff_ffff_ffff,
		)
	}
}
impl std::fmt::Debug for GUID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self, f)
	}
}

impl GUID {
	/// Creates a new `GUID` from a representative hex string, which can be
	/// copied straight from standard `GUID` declarations.
	///
	/// # Panics
	///
	/// Panics if the string has an invalid format.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let g = w::GUID::new("43826d1e-e718-42ee-bc55-a1e261c37bfe");
	/// ```
	#[must_use]
	pub const fn new(guid_str: &str) -> Self {
		if guid_str.len() != 36 {
			panic!("Bad number of GUID chars.");
		}

		let chs = guid_str.as_bytes();
		let p1 = Self::parse_block([chs[0], chs[1], chs[2], chs[3], chs[4],
			chs[5], chs[6], chs[7]]);
		let p2 = Self::parse_block([chs[9], chs[10], chs[11], chs[12]]);
		let p3 = Self::parse_block([chs[14], chs[15], chs[16], chs[17]]);
		let p4 = Self::parse_block([chs[19], chs[20], chs[21], chs[22]]);
		let p5 = Self::parse_block([chs[24], chs[25], chs[26], chs[27], chs[28],
			chs[29], chs[30], chs[31], chs[32], chs[33], chs[34], chs[35]]);

		Self {
			data1: p1 as _,
			data2: p2 as _,
			data3: p3 as _,
			data4: ((p4 << 48) | p5).swap_bytes(),
		}
	}

	const fn parse_block<const N: usize>(chars: [u8; N]) -> u64 {
		let mut res: u64 = 0;
		let mut idx: usize = 0;
		while idx < N {
			let ch = chars[idx];
			if !Self::valid_char(ch) {
				panic!("Bad GUID char.");
			}
			res += Self::char_to_num(ch) * 16_u64.pow((N - idx - 1) as _);
			idx += 1;
		}
		res
	}

	const fn valid_char(ch: u8) -> bool {
		(ch >= 48 && ch <= 57) // 0-9
			|| (ch >= 65 && ch <= 70) // A-F
			|| (ch >= 97 && ch <= 102) // a-f
	}

	const fn char_to_num(ch: u8) -> u64 {
		if ch >= 48 && ch <= 57 {
			ch as u64 - 48
		} else if ch >= 65 && ch <= 70 {
			ch as u64 - 65 + 10
		} else if ch >= 97 && ch <= 102 {
			ch as u64 - 97 + 10
		} else {
			panic!("Bad GUID char in conversion.");
		}
	}
}

/// [`HEAPLIST32`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/ns-tlhelp32-heaplist32)
/// struct.
#[repr(C)]
pub struct HEAPLIST32 {
	dwSize: usize,
	pub th32ProcessID: u32,
	pub th32HeapID: usize,
	pub dwFlags: co::HF32,
}

impl_default_with_size!(HEAPLIST32, dwSize);

/// [`LANGID`](https://learn.microsoft.com/en-us/windows/win32/intl/language-identifiers)
/// language identifier.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LANGID(u16);

impl_intunderlying!(LANGID, u16);

impl std::fmt::Display for LANGID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(self, f)
	}
}

impl LANGID {
	/// [`LANGID`](crate::LANGID) composed of
	/// [`LANG::NEUTRAL`](crate::co::LANG::NEUTRAL) and
	/// [`SUBLANG::SYS_DEFAULT`](crate::co::SUBLANG::SYS_DEFAULT).
	pub const SYSTEM_DEFAULT: Self = Self::new(co::LANG::NEUTRAL, co::SUBLANG::SYS_DEFAULT);

	/// [`LANGID`](crate::LANGID) composed of
	/// [`LANG::NEUTRAL`](crate::co::LANG::NEUTRAL) and
	/// [`SUBLANG::DEFAULT`](crate::co::SUBLANG::DEFAULT).
	pub const USER_DEFAULT: Self = Self::new(co::LANG::NEUTRAL, co::SUBLANG::DEFAULT);

	/// Creates a new `LANGID`. Originally
	/// [`MAKELANGID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-makelangid)
	/// macro.
	#[must_use]
	pub const fn new(lang: co::LANG, sublang: co::SUBLANG) -> Self {
		Self((sublang.raw() << 10) | lang.raw())
	}

	/// Returns the primary language ID. Originally
	/// [`PRIMARYLANGID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-primarylangid)
	/// macro.
	#[must_use]
	pub const fn primary_lang_id(self) -> co::LANG {
		unsafe { co::LANG::from_raw(self.0 & 0x3ff) }
	}

	/// Returns the sublanguage ID. Originally
	/// [`SUBLANGID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-sublangid)
	/// macro.
	#[must_use]
	pub const fn sub_lang_id(self) -> co::SUBLANG {
		unsafe { co::SUBLANG::from_raw(self.0 >> 10) }
	}
}

/// [`LCID`](https://learn.microsoft.com/en-us/windows/win32/intl/locale-identifiers)
/// locale identifier.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LCID(u32);

impl_intunderlying!(LCID, u32);

impl LCID {
	/// [`LCID`](crate::LCID) composed of
	/// [`LANGID::SYSTEM_DEFAULT`](crate::LANGID::SYSTEM_DEFAULT) and
	/// [`SORT::DEFAULT`](crate::co::SORT::DEFAULT).
	pub const SYSTEM_DEFAULT: Self = Self::new(LANGID::SYSTEM_DEFAULT, co::SORT::DEFAULT);

	/// [`LCID`](crate::LCID) composed of
	/// [`LANGID::USER_DEFAULT`](crate::LANGID::USER_DEFAULT) and
	/// [`SORT::DEFAULT`](crate::co::SORT::DEFAULT).
	pub const USER_DEFAULT: Self = Self::new(LANGID::USER_DEFAULT, co::SORT::DEFAULT);

	/// Creates a new `LCID`. Originally
	/// [`MAKELCID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-makelcid)
	/// macro.
	#[must_use]
	pub const fn new(lang_id: LANGID, sort_id: co::SORT) -> Self {
		Self(((sort_id.raw() as u32) << 16) | lang_id.raw() as u32)
	}

	/// Returns the language identifier. Originally
	/// [`LANGIDFROMLCID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-langidfromlcid)
	/// macro.
	#[must_use]
	pub const fn lang_id(self) -> LANGID {
		unsafe { LANGID::from_raw(self.raw() as _) }
	}

	/// Returns the sort ID. Originally
	/// [`SORTIDFROMLCID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-sortidfromlcid)
	/// macro.
	#[must_use]
	pub const fn sort_id(self) -> co::SORT {
		unsafe { co::SORT::from_raw(((self.raw() >> 16) & 0xf) as _) }
	}
}

/// [`LUID`](https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-luid)
/// identifier.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LUID {
	LowPart: u32,
	HighPart: i32,
}

impl std::fmt::Display for LUID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "LUID lo: {:#04x}, hi: {:#04x}", self.low_part(), self.high_part())
	}
}

impl LUID {
	pub const SYSTEM: Self = Self::new(0x3e7, 0x0);
	pub const ANONYMOUS_LOGON: Self = Self::new(0x3e6, 0x0);
	pub const LOCALSERVICE: Self = Self::new(0x3e5, 0x0);
	pub const NETWORKSERVICE: Self = Self::new(0x3e4, 0x0);
	pub const IUSER: Self = Self::new(0x3e3, 0x0);
	pub const PROTECTED_TO_SYSTEM: Self = Self::new(0x3e2, 0x0);

	/// Creates a new `LUID`.
	#[must_use]
	pub const fn new(low_part: u32, high_part: i32) -> Self {
		Self { LowPart: low_part, HighPart: high_part }
	}

	/// Returns the low part.
	#[must_use]
	pub const fn low_part(&self) -> u32 {
		self.LowPart
	}

	/// Returns the high part.
	#[must_use]
	pub const fn high_part(&self) -> i32 {
		self.HighPart
	}
}

/// [`LUID_AND_ATTRIBUTES`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-luid_and_attributes)
/// struct.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LUID_AND_ATTRIBUTES {
	pub Luid: LUID,
	pub Attributes: co::SE_PRIV_ATTR,
}

impl LUID_AND_ATTRIBUTES {
	/// Constructs a new `LUID_AND_ATTRIBUTES`.
	#[must_use]
	pub const fn new(luid: LUID, attrs: co::SE_PRIV_ATTR) -> Self {
		Self { Luid: luid, Attributes: attrs }
	}
}

/// [`MODULEENTRY32`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/ns-tlhelp32-moduleentry32w)
/// struct.
#[repr(C)]
pub struct MODULEENTRY32 {
	dwSize: u32,
	th32ModuleID: u32,
	pub th32ProcessID: u32,
	pub GlblcntUsage: u32,
	pub ProccntUsage: u32,
	pub modBaseAddr: *mut std::ffi::c_void,
	pub modBaseSize: u32,
	pub hModule: HINSTANCE,
	szModule: [u16; MAX_MODULE_NAME32 + 1],
	szExePath: [u16; MAX_PATH],
}

impl_default_with_size!(MODULEENTRY32, dwSize);

impl MODULEENTRY32 {
	pub_fn_string_arr_get_set!(szModule, set_szModule);
	pub_fn_string_arr_get_set!(szExePath, set_szExePath);
}

/// [`MEMORYSTATUSEX`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-memorystatusex)
/// struct.
#[repr(C)]
pub struct MEMORYSTATUSEX {
	dwLength: u32,
	pub dwMemoryLoad: u32,
	pub ullTotalPhys: u64,
	pub ullAvailPhys: u64,
	pub ullTotalPageFile: u64,
	pub ullAvailPageFile: u64,
	pub ullTotalVirtual: u64,
	pub ullAvailVirtual: u64,
	pub ullAvailExtendedVirtual: u64,
}

impl_default_with_size!(MEMORYSTATUSEX, dwLength);

/// [`OSVERSIONINFOEX`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-osversioninfoexw)
/// struct.
#[repr(C)]
pub struct OSVERSIONINFOEX {
	dwOSVersionInfoSize: u32,
	pub dwMajorVersion: u32,
	pub dwMinorVersion: u32,
	pub dwBuildNumber: u32,
	pub dwPlatformId: co::VER_PLATFORM,
	szCSDVersion: [u16; 128],
	pub wServicePackMajor: u16,
	pub wServicePackMinor: u16,
	pub wSuiteMask: co::VER_SUITE,
	pub wProductType: co::VER_NT,
	wReserved: u8,
}

impl_default_with_size!(OSVERSIONINFOEX, dwOSVersionInfoSize);

impl OSVERSIONINFOEX {
	pub_fn_string_arr_get_set!(szCSDVersion, set_szCSDVersion);
}

/// [`OVERLAPPED`](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-overlapped)
/// struct.
#[repr(C)]
pub struct OVERLAPPED {
	pub Internal: usize,
	pub InternalHigh: usize,
	pub Pointer: usize,
	pub hEvent: HEVENT,
}

impl_default!(OVERLAPPED);

/// [`POWERBROADCAST_SETTING`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-powerbroadcast_setting)
/// struct.
#[repr(C)]
pub struct POWERBROADCAST_SETTING {
	pub PowerSetting: co::POWER_SETTING,
	pub DataLength: u32,
	Data: [u8; 1],
}

impl VariableSized for POWERBROADCAST_SETTING {}

impl POWERBROADCAST_SETTING {
	/// Returns the `Data` field according to `PowerSetting` identifier.
	///
	/// # Panics
	///
	/// Panics if `PowerSetting` identifier is invalid.
	///
	/// # Safety
	///
	/// Make sure the struct contains the correct size and data described by the
	/// `PowerSetting` identifier.
	#[must_use]
	pub unsafe fn data(&self) -> PowerSetting {
		match self.PowerSetting {
			co::POWER_SETTING::ACDC_POWER_SOURCE => PowerSetting::AcDcPowerSource(
				co::SYSTEM_POWER_CONDITION::from_raw(
					std::slice::from_raw_parts(self.Data.as_ptr() as *const _, 1)[0],
				),
			),
			co::POWER_SETTING::BATTERY_PERCENTAGE_REMAINING => PowerSetting::BatteryPercentageRemaining(
				std::slice::from_raw_parts(self.Data.as_ptr() as *const u32, 1)[0] as _
			),
			co::POWER_SETTING::CONSOLE_DISPLAY_STATE => PowerSetting::ConsoleDisplayState(
				co::MONITOR_DISPLAY_STATE::from_raw(
					std::slice::from_raw_parts(self.Data.as_ptr() as *const _, 1)[0],
				),
			),
			co::POWER_SETTING::GLOBAL_USER_PRESENCE => PowerSetting::GlobalUserPresence(
				co::USER_ACTIVITY_PRESENCE::from_raw(
					std::slice::from_raw_parts(self.Data.as_ptr() as *const _, 1)[0],
				),
			),
			co::POWER_SETTING::IDLE_BACKGROUND_TASK => PowerSetting::IdleBackgroundTask,
			co::POWER_SETTING::MONITOR_POWER_ON => PowerSetting::MonitorPowerOn(
				match std::slice::from_raw_parts(self.Data.as_ptr() as *const u32, 1)[0] {
					0 => false,
					_ => true,
				},
			),
			co::POWER_SETTING::POWER_SAVING_STATUS => PowerSetting::PowerSavingStatus(
				match std::slice::from_raw_parts(self.Data.as_ptr() as *const u32, 1)[0] {
					0 => false,
					_ => true,
				},
			),
			co::POWER_SETTING::POWERSCHEME_PERSONALITY => PowerSetting::PowerSchemePersonality(
				std::slice::from_raw_parts(self.Data.as_ptr() as *const co::POWER_SAVINGS, 1)[0],
			),
			co::POWER_SETTING::SESSION_DISPLAY_STATUS => PowerSetting::SessionDisplayStatus(
				co::MONITOR_DISPLAY_STATE::from_raw(
					std::slice::from_raw_parts(self.Data.as_ptr() as *const _, 1)[0],
				),
			),
			co::POWER_SETTING::SESSION_USER_PRESENCE => PowerSetting::SessionUserPresence(
				co::USER_ACTIVITY_PRESENCE::from_raw(
					std::slice::from_raw_parts(self.Data.as_ptr() as *const _, 1)[0],
				),
			),
			co::POWER_SETTING::LIDSWITCH_STATE_CHANGE => PowerSetting::LidSwitchStateChange(
				match std::slice::from_raw_parts(self.Data.as_ptr() as *const u8, 1)[0] {
					0 => PowerSettingLid::Closed,
					_ => PowerSettingLid::Opened,
				},
			),
			co::POWER_SETTING::SYSTEM_AWAYMODE => PowerSetting::SystemAwayMode(
				match std::slice::from_raw_parts(self.Data.as_ptr() as *const u8, 1)[0] {
					0 => PowerSettingAwayMode::Exiting,
					_ => PowerSettingAwayMode::Entering,
				},
			),
			_ => panic!("Invalid co::POWER_SETTING."),
		}
	}
}

/// [`PROCESS_HEAP_ENTRY`](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-process_heap_entry)
/// struct.
#[repr(C)]
pub struct PROCESS_HEAP_ENTRY {
	pub lpData: *mut std::ffi::c_void,
	pub cbData: u32,
	pub cbOverhead: u8,
	pub iRegionIndex: u8,
	pub wFlags: co::PROCESS_HEAP,
	union0: PROCESS_HEAP_ENTRY_union0,
}

#[repr(C)]
union PROCESS_HEAP_ENTRY_union0 {
	Block: PROCESS_HEAP_ENTRY_Block,
	Region: PROCESS_HEAP_ENTRY_Region,
}

/// [`PROCESS_HEAP_ENTRY`](crate::PROCESS_HEAP_ENTRY) `Block`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_HEAP_ENTRY_Block {
	pub hMem: *mut std::ffi::c_void,
	dwReserved: [u32; 3],
}

/// [`PROCESS_HEAP_ENTRY`](crate::PROCESS_HEAP_ENTRY) `Region`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_HEAP_ENTRY_Region {
	pub dwCommittedSize: u32,
	pub dwUnCommittedSize: u32,
	pub lpFirstBlock: *mut std::ffi::c_void,
	pub lpLastBlock: *mut std::ffi::c_void,
}

impl_default!(PROCESS_HEAP_ENTRY);

impl PROCESS_HEAP_ENTRY {
	/// Retrieves the `Block` union field.
	#[must_use]
	pub fn Block(&self) -> Option<&PROCESS_HEAP_ENTRY_Block> {
		if self.wFlags.has(co::PROCESS_HEAP::ENTRY_MOVEABLE) {
			Some(unsafe { &self.union0.Block })
		} else {
			None
		}
	}

	/// Retrieves the `Region` union field.
	#[must_use]
	pub fn Region(&self) -> Option<&PROCESS_HEAP_ENTRY_Region> {
		if self.wFlags.has(co::PROCESS_HEAP::REGION) {
			Some(unsafe { &self.union0.Region })
		} else {
			None
		}
	}
}

/// [`PROCESS_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information)
/// struct.
#[repr(C)]
pub struct PROCESS_INFORMATION {
	pub hProcess: HPROCESS,
	pub hThread: HTHREAD,
	pub dwProcessId: u32,
	pub dwThreadId: u32,
}

impl_default!(PROCESS_INFORMATION);

/// [`PROCESSENTRY32`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/ns-tlhelp32-processentry32w)
/// struct.
#[repr(C)]
pub struct PROCESSENTRY32 {
	dwSize: u32,
	cntUsage: u32,
	pub th32ProcessID: u32,
	th32DefaultHeapID: u64,
	th32ModuleID: u32,
	pub cntThreads: u32,
	pub th32ParentProcessID: u32,
	pub pcPriClassBase: i32,
	dwFlags: u32,
	szExeFile: [u16; MAX_PATH],
}

impl_default_with_size!(PROCESSENTRY32, dwSize);

impl PROCESSENTRY32 {
	pub_fn_string_arr_get_set!(szExeFile, set_szExeFile);
}

/// [`PROCESSOR_NUMBER`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-processor_number)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PROCESSOR_NUMBER {
	pub Group: u16,
	pub Number: u8,
	Reserved: u8,
}

/// [`SECURITY_ATTRIBUTES`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa379560(v=vs.85))
/// struct.
#[repr(C)]
pub struct SECURITY_ATTRIBUTES<'a> {
	nLength: u32,
	lpSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
	bInheritHandle: i32,

	_lpSecurityDescriptor: PhantomData<&'a mut SECURITY_DESCRIPTOR>,
}

impl_default_with_size!(SECURITY_ATTRIBUTES, nLength, 'a);

impl<'a> SECURITY_ATTRIBUTES<'a> {
	pub_fn_ptr_get_set!('a, lpSecurityDescriptor, set_lpSecurityDescriptor, SECURITY_DESCRIPTOR);
	pub_fn_bool_get_set!(bInheritHandle, set_bInheritHandle);
}

/// [`SECURITY_DESCRIPTOR`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_descriptor)
/// struct.
#[repr(C)]
pub struct SECURITY_DESCRIPTOR {
	pub Revision: u8,
	pub Sbz1: u8,
	pub Control: co::SE,
	pub Owner: *mut std::ffi::c_void,
	pub Group: *mut std::ffi::c_void,
	pub Sacl: *mut ACL,
	pub Dacl: *mut ACL,
}

impl Default for SECURITY_DESCRIPTOR {
	fn default() -> Self {
		InitializeSecurityDescriptor().unwrap()
	}
}

/// [`SERVICE_TIMECHANGE_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_timechange_info)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct SERVICE_TIMECHANGE_INFO {
	liNewTime: i64,
	liOldTime: i64,
}

impl SERVICE_TIMECHANGE_INFO {
	/// Returns the `liNewTime` field.
	#[must_use]
	pub const fn liNewTime(&self) -> FILETIME {
		FILETIME {
			dwLowDateTime: LODWORD(self.liNewTime as _),
			dwHighDateTime: HIDWORD(self.liNewTime as _),
		}
	}

	/// Returns the `liOldTime` field.
	#[must_use]
	pub const fn liOldTime(&self) -> FILETIME {
		FILETIME {
			dwLowDateTime: LODWORD(self.liOldTime as _),
			dwHighDateTime: HIDWORD(self.liOldTime as _),
		}
	}

	/// Sets the `liNewTime` field.
	pub fn set_liNewTime(&mut self, ft: FILETIME) {
		self.liNewTime = MAKEQWORD(ft.dwLowDateTime, ft.dwHighDateTime) as _;
	}

	/// Sets the `liOldTime` field.
	pub fn set_liOldTime(&mut self, ft: FILETIME) {
		self.liOldTime = MAKEQWORD(ft.dwLowDateTime, ft.dwHighDateTime) as _;
	}
}

/// [`SERVICE_STATUS`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_status)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct SERVICE_STATUS {
	pub dwServiceType: co::SERVICE_TYPE,
	pub dwCurrentState: co::SERVICE_STATE,
	pub dwControlsAccepted: co::SERVICE_ACCEPT,
	pub dwWin32ExitCode: u32,
	pub dwServiceSpecificExitCode: u32,
	pub dwCheckPoint: u32,
	pub dwWaitPoint: u32,
}

/// [`SID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)
/// struct.
///
/// Note that you cannot directly instantiate this
/// [`VariableSized`](crate::prelude::VariableSized) struct, because the
/// `SubAuthority` field is dynamically allocated.
///
/// Possible ways:
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid) as [`FreeSidGuard`](crate::guard::FreeSidGuard);
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid) as [`LocalFreeSidGuard`](crate::guard::LocalFreeSidGuard);
/// * [`CopySid`](crate::CopySid) as [`SidGuard`](crate::guard::SidGuard);
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid) as [`SidGuard`](crate::guard::SidGuard);
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid) as [`SidGuard`](crate::guard::SidGuard);
/// * [`LookupAccountName`](crate::LookupAccountName) as [`SidGuard`](crate::guard::SidGuard).
#[repr(C)]
pub struct SID {
	pub Revision: u8,
	pub(in crate::kernel) SubAuthorityCount: u8,
	pub IdentifierAuthority: SID_IDENTIFIER_AUTHORITY,
	SubAuthority: [co::RID; 1],
}

impl VariableSized for SID {}

impl std::fmt::Display for SID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match ConvertSidToStringSid(self) {
			Ok(name) => write!(f, "{}", name),
			Err(err) => write!(f, "{}", err),
		}
	}
}

impl SID {
	/// Returns the `SubAuthorityCount` field.
	#[must_use]
	pub fn SubAuthorityCount(&self) -> u8 {
		self.SubAuthority().len() as _
	}

	/// Returns the `SubAuthority` field.
	#[must_use]
	pub fn SubAuthority(&self) -> &[co::RID] {
		unsafe {
			std::slice::from_raw_parts(
				self.SubAuthority.as_ptr(), self.SubAuthorityCount as _)
		}
	}
}

/// [`SID_AND_ATTRIBUTES`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_and_attributes)
/// struct.
#[repr(C)]
#[derive(Clone)]
pub struct SID_AND_ATTRIBUTES<'a> {
	Sid: *mut SID,
	pub Attributes: u32,

	_Sid: PhantomData<&'a mut SID>,
}

impl_default!(SID_AND_ATTRIBUTES, 'a);

impl<'a> SID_AND_ATTRIBUTES<'a> {
	pub_fn_ptr_get_set!('a, Sid, set_Sid, SID);
}

/// [`SID_AND_ATTRIBUTES_HASH`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_and_attributes_hash)
/// struct.
#[repr(C)]
pub struct SID_AND_ATTRIBUTES_HASH<'a> {
	SidCount: u32,
	SidAttr: *mut SID_AND_ATTRIBUTES<'a>,
	pub Hash: [usize; SID_HASH_SIZE],
}

impl_default!(SID_AND_ATTRIBUTES_HASH, 'a);

impl<'a> SID_AND_ATTRIBUTES_HASH<'a> {
	pub_fn_array_buf_get_set!('a, SidAttr, set_SidAttr, SidCount, SID_AND_ATTRIBUTES);
}

/// [`SID_IDENTIFIER_AUTHORITY`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_identifier_authority)
/// struct.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SID_IDENTIFIER_AUTHORITY {
	pub Value: [u8; 6],
}

impl std::fmt::Display for SID_IDENTIFIER_AUTHORITY {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(&self.Value, f) // delegate to array Debug
	}
}

macro_rules! predef_sid_ident_au {
	($name:ident, $val:expr) => {
		/// Predefined `SID_IDENTIFIER_AUTHORITY`. Originally has `SECURITY`
		/// prefix and `AUTHORITY` suffix.
		pub const $name: Self = Self { Value: $val };
	};
}

impl SID_IDENTIFIER_AUTHORITY {
	predef_sid_ident_au!(NULL, [0, 0, 0, 0, 0, 0]);
	predef_sid_ident_au!(WORLD, [0, 0, 0, 0, 0, 1]);
	predef_sid_ident_au!(LOCAL, [0, 0, 0, 0, 0, 2]);
	predef_sid_ident_au!(CREATOR, [0, 0, 0, 0, 0, 3]);
	predef_sid_ident_au!(NON_UNIQUE, [0, 0, 0, 0, 0, 4]);
	predef_sid_ident_au!(RESOURCE_MANAGER, [0, 0, 0, 0, 0, 9]);
	predef_sid_ident_au!(NT, [0, 0, 0, 0, 0, 5]);
	predef_sid_ident_au!(APP_PACKAGE, [0, 0, 0, 0, 0, 15]);
	predef_sid_ident_au!(MANDATORY_LABEL, [0, 0, 0, 0, 0, 16]);
	predef_sid_ident_au!(SCOPED_POLICY_ID, [0, 0, 0, 0, 0, 17]);
	predef_sid_ident_au!(AUTHENTICATION, [0, 0, 0, 0, 0, 18]);
	predef_sid_ident_au!(PROCESS_TRUST, [0, 0, 0, 0, 0, 19]);
}

/// [`STARTUPINFO`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfow)
/// struct.
#[repr(C)]
pub struct STARTUPINFO<'a, 'b> {
	cb: u32,
	lpReserved: *mut u16,
	lpDesktop: *mut u16,
	lpTitle: *mut u16,
	pub dwX: u32,
	pub dwY: u32,
	pub dwXSize: u32,
	pub dwYSize: u32,
	pub dwXCountChars: u32,
	pub dwYCountChars: u32,
	pub dwFillAttribute: u32,
	pub dwFlags: co::STARTF,
	wShowWindow: u16, // co::SW, should be 32-bit
	cbReserved2: u16,
	lpReserved2: *mut u8,
	pub hStdInput: HPIPE,
	pub hStdOutput: HPIPE,
	pub hStdError: HPIPE,

	_lpDesktop: PhantomData<&'a mut u16>,
	_lpTitle: PhantomData<&'b mut u16>,
}

impl_default_with_size!(STARTUPINFO, cb, 'a, 'b);

impl<'a, 'b> STARTUPINFO<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, lpDesktop, set_lpDesktop);
	pub_fn_string_ptr_get_set!('a, lpTitle, set_lpTitle);

	/// Returns the `wShowWindow` field.
	#[must_use]
	pub const fn wShowWindow(&self) -> co::SW {
		unsafe { co::SW::from_raw(self.wShowWindow as _) }
	}

	/// Sets the `wShowWindow` field.
	pub fn set_wShowWindow(&mut self, val: co::SW) {
		self.wShowWindow = val.raw() as _;
	}
}

/// [`SYSTEM_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-system_info)
/// struct.
#[repr(C)]
pub struct SYSTEM_INFO {
	pub wProcessorArchitecture: co::PROCESSOR_ARCHITECTURE,
	wReserved: u16,
	pub dwPageSize: u32,
	pub lpMinimumApplicationAddress: *mut std::ffi::c_void,
	pub lpMaximumApplicationAddress: *mut std::ffi::c_void,
	pub dwActiveProcessorMask: usize,
	pub dwNumberOfProcessors: u32,
	pub dwProcessorType: co::PROCESSOR,
	pub dwAllocationGranularity: u32,
	pub wProcessorLevel: u16,
	pub wProcessorRevision: u16,
}

impl_default!(SYSTEM_INFO);

/// [`SYSTEMTIME`](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-systemtime)
/// struct.
///
/// Can be converted to [`FILETIME`](crate::FILETIME) with
/// [`SystemTimeToFileTime`](crate::SystemTimeToFileTime) function.
#[repr(C)]
#[derive(Default, Clone, PartialEq, Eq)]
pub struct SYSTEMTIME {
	pub wYear: u16,
	pub wMonth: u16,
	pub wDayOfWeek: u16,
	pub wDay: u16,
	pub wHour: u16,
	pub wMinute: u16,
	pub wSecond: u16,
	pub wMilliseconds: u16,
}

/// [`THREADENTRY32`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/ns-tlhelp32-threadentry32)
/// struct.
#[repr(C)]
pub struct THREADENTRY32 {
	dwSize: u32,
	cntUsage: u32,
	pub th32ThreadID: u32,
	pub th32OwnerProcessID: u32,
	pub tpBasePri: i32,
	tpDeltaPri: i32,
	dwFlags: u32,
}

impl_default_with_size!(THREADENTRY32, dwSize);

/// [`TIME_ZONE_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/ns-timezoneapi-time_zone_information)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct TIME_ZONE_INFORMATION {
	pub bias: i32,
	standardName: [u16; 32],
	pub standardDate: SYSTEMTIME,
	pub standardBias: i32,
	daylightName: [u16; 32],
	pub daylightDate: SYSTEMTIME,
	pub daylightBias: i32,
}

impl TIME_ZONE_INFORMATION {
	pub_fn_string_arr_get_set!(standardName, set_standardName);
	pub_fn_string_arr_get_set!(daylightName, set_daylightName);
}

/// [`TOKEN_ACCESS_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_access_information)
/// struct.
#[repr(C)]
pub struct TOKEN_ACCESS_INFORMATION<'a, 'b, 'c, 'd, 'e, 'f> {
	SidHash: *mut SID_AND_ATTRIBUTES_HASH<'a>,
	RestrictedSidHash: *mut SID_AND_ATTRIBUTES_HASH<'b>,
	Privileges: *mut TOKEN_PRIVILEGES,
	pub AuthenticationId: LUID,
	pub TokenType: LUID,
	pub ImpersonationLevel: co::SECURITY_IMPERSONATION,
	pub MandatoryPolicy: TOKEN_MANDATORY_POLICY,
	Flags: u32,
	pub AppContainerNumber: u32,
	PackageSid: *mut SID,
	CapabilitiesHash: *mut SID_AND_ATTRIBUTES_HASH<'e>,
	TrustLevelSid: *mut SID,
	SecurityAttributes: *mut std::ffi::c_void,

	_Privileges: PhantomData<&'c mut TOKEN_PRIVILEGES>,
	_PackageSid: PhantomData<&'d mut SID>,
	_TrustLevelSid: PhantomData<&'f mut SID>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f> TOKEN_ACCESS_INFORMATION<'a, 'b, 'c, 'd, 'e, 'f> {
	pub_fn_ptr_get_set!('a, SidHash, set_SidHash, SID_AND_ATTRIBUTES_HASH<'a>);
	pub_fn_ptr_get_set!('b, RestrictedSidHash, set_RestrictedSidHash, SID_AND_ATTRIBUTES_HASH<'b>);
	pub_fn_ptr_get_set!('c, Privileges, set_Privileges, TOKEN_PRIVILEGES);
	pub_fn_ptr_get_set!('d, PackageSid, set_PackageSid, SID);
	pub_fn_ptr_get_set!('e, CapabilitiesHash, set_CapabilitiesHash, SID_AND_ATTRIBUTES_HASH<'e>);
	pub_fn_ptr_get_set!('f, TrustLevelSid, set_TrustLevelSid, SID);
}

impl_default!(TOKEN_ACCESS_INFORMATION, 'a, 'b, 'c, 'd, 'e, 'f);

/// [`TOKEN_APPCONTAINER_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_appcontainer_information)
/// struct.
#[repr(C)]
pub struct TOKEN_APPCONTAINER_INFORMATION<'a> {
	TokenAppContainer: *mut SID,

	_TokenAppContainer: PhantomData<&'a mut SID>,
}

impl_default!(TOKEN_APPCONTAINER_INFORMATION, 'a);

impl<'a> TOKEN_APPCONTAINER_INFORMATION<'a> {
	pub_fn_ptr_get_set!('a, TokenAppContainer, set_TokenAppContainer, SID);
}

/// [`TOKEN_DEFAULT_DACL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_default_dacl)
/// struct.
#[repr(C)]
pub struct TOKEN_DEFAULT_DACL<'a> {
	DefaultDacl: *mut ACL,

	_DefaultDacl: PhantomData<&'a mut ACL>,
}

impl_default!(TOKEN_DEFAULT_DACL, 'a);

impl<'a> TOKEN_DEFAULT_DACL<'a> {
	pub_fn_ptr_get_set!('a, DefaultDacl, set_DefaultDacl, ACL);
}

/// [`TOKEN_ELEVATION`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_elevation)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct TOKEN_ELEVATION {
	TokenIsElevated: u32,
}

impl TOKEN_ELEVATION {
	pub_fn_bool_get_set!(TokenIsElevated, set_TokenIsElevated);
}

/// [`TOKEN_GROUPS`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups)
/// struct.
#[repr(C)]
pub struct TOKEN_GROUPS<'a> {
	pub(in crate::kernel) GroupCount: u32,
	Groups: [SID_AND_ATTRIBUTES<'a>; 1],
}

impl<'a> VariableSized for TOKEN_GROUPS<'a> {}

impl<'a> TOKEN_GROUPS<'a> {
	/// Returns a dynamically allocated
	/// [`TokenGroupsGuard`](crate::guard::TokenGroupsGuard).
	#[must_use]
	pub fn new(
		groups: &'a [SID_AND_ATTRIBUTES<'a>],
	) -> SysResult<TokenGroupsGuard<'a>>
	{
		TokenGroupsGuard::new(groups)
	}

	/// Returns a constant slice over the `Groups` entries.
	#[must_use]
	pub const fn Groups(&self) -> &[SID_AND_ATTRIBUTES<'a>] {
		unsafe {
			std::slice::from_raw_parts(
				self.Groups.as_ptr(),
				self.GroupCount as _,
			)
		}
	}

	/// Returns a mutable slice over the `Groups` entries.
	#[must_use]
	pub fn Groups_mut(&mut self) -> &mut [SID_AND_ATTRIBUTES<'a>] {
		unsafe {
			std::slice::from_raw_parts_mut(
				self.Groups.as_mut_ptr(),
				self.GroupCount as _,
			)
		}
	}
}

/// [`TOKEN_GROUPS_AND_PRIVILEGES`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups_and_privileges)
/// struct.
#[repr(C)]
pub struct TOKEN_GROUPS_AND_PRIVILEGES<'a, 'b, 'c> {
	pub SidCount: u32,
	pub SidLength: u32,
	Sids: *mut SID_AND_ATTRIBUTES<'a>,
	pub RestrictedSidCount: u32,
	pub RestrictedSidLength: u32,
	RestrictedSids: *mut SID_AND_ATTRIBUTES<'b>,
	pub PrivilegeCount: u32,
	pub PrivilegeLength: u32,
	Privileges: *mut LUID_AND_ATTRIBUTES,
	pub AuthenticationId: LUID,

	_Privileges: PhantomData<&'c LUID_AND_ATTRIBUTES>,
}

impl_default!(TOKEN_GROUPS_AND_PRIVILEGES, 'a, 'b, 'c);

impl<'a, 'b, 'c> TOKEN_GROUPS_AND_PRIVILEGES<'a, 'b, 'c> {
	pub_fn_ptr_get_set!('a, Sids, set_Sids, SID_AND_ATTRIBUTES<'a>);
	pub_fn_ptr_get_set!('b, RestrictedSids, set_RestrictedSids, SID_AND_ATTRIBUTES<'b>);
	pub_fn_ptr_get_set!('c, Privileges, set_Privileges, LUID_AND_ATTRIBUTES);
}

/// [`TOKEN_LINKED_TOKEN`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_linked_token)
/// struct.
#[repr(C)]
pub struct TOKEN_LINKED_TOKEN {
	pub LinkedToken: HACCESSTOKEN,
}

impl_default!(TOKEN_LINKED_TOKEN);

/// [`TOKEN_MANDATORY_LABEL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_mandatory_label)
/// struct.
#[repr(C)]
pub struct TOKEN_MANDATORY_LABEL<'a> {
	pub Label: SID_AND_ATTRIBUTES<'a>,
}

impl_default!(TOKEN_MANDATORY_LABEL, 'a);

/// [`TOKEN_MANDATORY_POLICY`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_mandatory_policy)
/// struct.
#[repr(C)]
pub struct TOKEN_MANDATORY_POLICY {
	pub Policy: co::TOKEN_MANDATORY_POLICY,
}

impl_default!(TOKEN_MANDATORY_POLICY);

/// [`TOKEN_ORIGIN`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_origin)
/// struct.
#[repr(C)]
pub struct TOKEN_ORIGIN {
	pub OriginatingLogonSession: LUID,
}

impl_default!(TOKEN_ORIGIN);

/// [`TOKEN_OWNER`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_owner)
/// struct.
#[repr(C)]
pub struct TOKEN_OWNER<'a> {
	Owner: *mut SID,

	_Owner: PhantomData<&'a mut SID>,
}

impl_default!(TOKEN_OWNER, 'a);

impl<'a> TOKEN_OWNER<'a> {
	pub_fn_ptr_get_set!('a, Owner, set_Owner, SID);
}

/// [`TOKEN_PRIMARY_GROUP`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_primary_group)
/// struct.
#[repr(C)]
pub struct TOKEN_PRIMARY_GROUP<'a> {
	PrimaryGroup: *mut SID,

	_Owner: PhantomData<&'a mut SID>,
}

impl_default!(TOKEN_PRIMARY_GROUP, 'a);

impl<'a> TOKEN_PRIMARY_GROUP<'a> {
	pub_fn_ptr_get_set!('a, PrimaryGroup, set_PrimaryGroup, SID);
}

/// [`TOKEN_PRIVILEGES`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)
/// struct.
#[repr(C)]
pub struct TOKEN_PRIVILEGES {
	pub(in crate::kernel) PrivilegeCount: u32,
	Privileges: [LUID_AND_ATTRIBUTES; 1],
}

impl VariableSized for TOKEN_PRIVILEGES {}

impl TOKEN_PRIVILEGES {
	/// Returns a dynamically allocated
	/// [`TokenPrivilegesGuard`](crate::guard::TokenPrivilegesGuard).
	#[must_use]
	pub fn new(
		privileges: &[LUID_AND_ATTRIBUTES],
	) -> SysResult<TokenPrivilegesGuard>
	{
		TokenPrivilegesGuard::new(privileges)
	}

	/// Returns a constant slice over the `Privileges` entries.
	#[must_use]
	pub const fn Privileges(&self) -> &[LUID_AND_ATTRIBUTES] {
		unsafe {
			std::slice::from_raw_parts(
				self.Privileges.as_ptr(),
				self.PrivilegeCount as _,
			)
		}
	}

	/// Returns a mutable slice over the `Privileges` entries.
	#[must_use]
	pub fn Privileges_mut(&mut self) -> &mut [LUID_AND_ATTRIBUTES] {
		unsafe {
			std::slice::from_raw_parts_mut(
				self.Privileges.as_mut_ptr(),
				self.PrivilegeCount as _,
			)
		}
	}
}

/// [`TOKEN_SOURCE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_source)
/// struct.
#[repr(C)]
#[derive(PartialEq, Eq)]
pub struct TOKEN_SOURCE {
	pub SourceName: [i8; TOKEN_SOURCE_LENGTH],
	pub SourceIdentifier: LUID,
}

impl_default!(TOKEN_SOURCE);

/// [`TOKEN_STATISTICS`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_statistics)
/// struct.
#[repr(C)]
pub struct TOKEN_STATISTICS {
	pub TokenId: LUID,
	pub AuthenticationId: LUID,
	pub ExpirationTime: i64,
	pub TokenType: co::TOKEN_TYPE,
	pub ImpersonationLevel: co::SECURITY_IMPERSONATION,
	pub DynamicCharged: u32,
	pub DynamicAvailable: u32,
	pub GroupCount: u32,
	pub PrivilegeCount: u32,
	pub ModifiedId: LUID,
}

impl_default!(TOKEN_STATISTICS);

/// [`TOKEN_USER`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_user)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct TOKEN_USER<'a> {
	pub User: SID_AND_ATTRIBUTES<'a>,
}

/// [`VALENT`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/ns-winreg-valentw)
/// struct.
#[repr(C)]
#[derive(Clone)]
pub struct VALENT {
	pub ve_valuename: *mut u16,
	pub ve_valuelen: u32,
	pub ve_valueptr: usize,
	pub ve_type: co::REG,
}

impl_default!(VALENT);

impl VALENT {
	/// Returns a projection over `src`, delimited by `ve_valueptr` and
	/// `ve_valuelen` fields.
	pub unsafe fn buf_projection<'a>(&'a self, src: &'a [u8]) -> &'a [u8] {
		let proj_idx = self.ve_valueptr - src.as_ptr() as usize;
		let proj_past_idx = proj_idx + self.ve_valuelen as usize;
		&src[proj_idx..proj_past_idx]
	}
}

/// [`WIN32_FIND_DATA`](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-win32_find_dataw)
/// struct.
#[repr(C)]
pub struct WIN32_FIND_DATA {
	pub dwFileAttributes: co::FILE_ATTRIBUTE,
	pub ftCreationTime: FILETIME,
	pub ftLastAccessTime: FILETIME,
	pub tLastWriteTime: FILETIME,
	nFileSizeHigh: u32,
	nFileSizeLow: u32,
	dwReserved0: u32,
	dwReserved1: u32,
	cFileName: [u16; MAX_PATH],
	cAlternateFileName: [u16; 14],
}

impl_default!(WIN32_FIND_DATA);

impl WIN32_FIND_DATA {
	pub_fn_string_arr_get_set!(cFileName, set_cFileName);
	pub_fn_string_arr_get_set!(cAlternateFileName, set_cAlternateFileName);

	/// Returns the nFileSizeHigh and nFileSizeLow fields.
	#[must_use]
	pub const fn nFileSize(&self) -> u64 {
		MAKEQWORD(self.nFileSizeLow, self.nFileSizeHigh)
	}
}

/// [`WTSSESSION_NOTIFICATION`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wtssession_notification)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct WTSSESSION_NOTIFICATION {
	pub cbSize: u32,
	pub dwSessionId: u32,
}
