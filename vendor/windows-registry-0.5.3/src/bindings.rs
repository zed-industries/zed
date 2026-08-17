#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_link::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> BOOL);
windows_link::link!("ktmw32.dll" "system" fn CommitTransaction(transactionhandle : HANDLE) -> BOOL);
windows_link::link!("ktmw32.dll" "system" fn CreateTransaction(lptransactionattributes : *mut SECURITY_ATTRIBUTES, uow : *mut GUID, createoptions : u32, isolationlevel : u32, isolationflags : u32, timeout : u32, description : PCWSTR) -> HANDLE);
windows_link::link!("kernel32.dll" "system" fn GetProcessHeap() -> HANDLE);
windows_link::link!("kernel32.dll" "system" fn HeapAlloc(hheap : HANDLE, dwflags : HEAP_FLAGS, dwbytes : usize) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn HeapFree(hheap : HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> BOOL);
windows_link::link!("advapi32.dll" "system" fn RegCloseKey(hkey : HKEY) -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyExW(hkey : HKEY, lpsubkey : PCWSTR, reserved : u32, lpclass : PCWSTR, dwoptions : REG_OPEN_CREATE_OPTIONS, samdesired : REG_SAM_FLAGS, lpsecurityattributes : *const SECURITY_ATTRIBUTES, phkresult : *mut HKEY, lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION) -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyTransactedW(hkey : HKEY, lpsubkey : PCWSTR, reserved : u32, lpclass : PCWSTR, dwoptions : REG_OPEN_CREATE_OPTIONS, samdesired : REG_SAM_FLAGS, lpsecurityattributes : *const SECURITY_ATTRIBUTES, phkresult : *mut HKEY, lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION, htransaction : HANDLE, pextendedparemeter : *const core::ffi::c_void) -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegDeleteTreeW(hkey : HKEY, lpsubkey : PCWSTR) -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegDeleteValueW(hkey : HKEY, lpvaluename : PCWSTR) -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegEnumKeyExW(hkey : HKEY, dwindex : u32, lpname : PWSTR, lpcchname : *mut u32, lpreserved : *const u32, lpclass : PWSTR, lpcchclass : *mut u32, lpftlastwritetime : *mut FILETIME) -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegEnumValueW(hkey : HKEY, dwindex : u32, lpvaluename : PWSTR, lpcchvaluename : *mut u32, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyExW(hkey : HKEY, lpsubkey : PCWSTR, uloptions : u32, samdesired : REG_SAM_FLAGS, phkresult : *mut HKEY) -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyTransactedW(hkey : HKEY, lpsubkey : PCWSTR, uloptions : u32, samdesired : REG_SAM_FLAGS, phkresult : *mut HKEY, htransaction : HANDLE, pextendedparemeter : *const core::ffi::c_void) -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegQueryInfoKeyW(hkey : HKEY, lpclass : PWSTR, lpcchclass : *mut u32, lpreserved : *const u32, lpcsubkeys : *mut u32, lpcbmaxsubkeylen : *mut u32, lpcbmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcbmaxvaluenamelen : *mut u32, lpcbmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut FILETIME) -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegQueryValueExW(hkey : HKEY, lpvaluename : PCWSTR, lpreserved : *const u32, lptype : *mut REG_VALUE_TYPE, lpdata : *mut u8, lpcbdata : *mut u32) -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegRenameKey(hkey : HKEY, lpsubkeyname : PCWSTR, lpnewkeyname : PCWSTR) -> WIN32_ERROR);
windows_link::link!("advapi32.dll" "system" fn RegSetValueExW(hkey : HKEY, lpvaluename : PCWSTR, reserved : u32, dwtype : REG_VALUE_TYPE, lpdata : *const u8, cbdata : u32) -> WIN32_ERROR);
pub type BOOL = i32;
pub const ERROR_INVALID_DATA: WIN32_ERROR = 13u32;
pub const ERROR_NO_MORE_ITEMS: WIN32_ERROR = 259u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }
}
pub type HANDLE = *mut core::ffi::c_void;
pub type HEAP_FLAGS = u32;
pub type HKEY = *mut core::ffi::c_void;
pub const HKEY_CLASSES_ROOT: HKEY = -2147483648i32 as _;
pub const HKEY_CURRENT_CONFIG: HKEY = -2147483643i32 as _;
pub const HKEY_CURRENT_USER: HKEY = -2147483647i32 as _;
pub const HKEY_LOCAL_MACHINE: HKEY = -2147483646i32 as _;
pub const HKEY_USERS: HKEY = -2147483645i32 as _;
pub const INVALID_HANDLE_VALUE: HANDLE = -1i32 as _;
pub const KEY_READ: REG_SAM_FLAGS = 131097u32;
pub const KEY_WRITE: REG_SAM_FLAGS = 131078u32;
pub type PCWSTR = *const u16;
pub type PWSTR = *mut u16;
pub const REG_BINARY: REG_VALUE_TYPE = 3u32;
pub type REG_CREATE_KEY_DISPOSITION = u32;
pub const REG_DWORD: REG_VALUE_TYPE = 4u32;
pub const REG_EXPAND_SZ: REG_VALUE_TYPE = 2u32;
pub const REG_MULTI_SZ: REG_VALUE_TYPE = 7u32;
pub type REG_OPEN_CREATE_OPTIONS = u32;
pub const REG_OPTION_NON_VOLATILE: REG_OPEN_CREATE_OPTIONS = 0u32;
pub const REG_OPTION_VOLATILE: REG_OPEN_CREATE_OPTIONS = 1u32;
pub const REG_QWORD: REG_VALUE_TYPE = 11u32;
pub type REG_SAM_FLAGS = u32;
pub const REG_SZ: REG_VALUE_TYPE = 1u32;
pub type REG_VALUE_TYPE = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut core::ffi::c_void,
    pub bInheritHandle: BOOL,
}
impl Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WIN32_ERROR = u32;
