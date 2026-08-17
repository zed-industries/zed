windows_link::link!("advapi32.dll" "system" fn RegGetValueA(hkey : HKEY, lpsubkey : PCSTR, lpvalue : PCSTR, dwflags : REG_ROUTINE_FLAGS, pdwtype : *mut REG_VALUE_TYPE, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> WIN32_ERROR);
windows_link::link!("ntdll.dll" "system" fn RtlGetVersion(lpversioninformation : *mut OSVERSIONINFOW) -> NTSTATUS);
pub type HKEY = *mut core::ffi::c_void;
pub const HKEY_LOCAL_MACHINE: HKEY = -2147483646i32 as _;
pub type NTSTATUS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OSVERSIONINFOEXW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
impl Default for OSVERSIONINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OSVERSIONINFOW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
}
impl Default for OSVERSIONINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PCSTR = *const u8;
pub type REG_ROUTINE_FLAGS = u32;
pub type REG_VALUE_TYPE = u32;
pub const RRF_RT_REG_DWORD: REG_ROUTINE_FLAGS = 16u32;
pub const VER_NT_WORKSTATION: u32 = 1u32;
pub type WIN32_ERROR = u32;
