windows_link::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> BOOL);
windows_link::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const SECURITY_ATTRIBUTES, bmanualreset : BOOL, binitialstate : BOOL, lpname : PCWSTR) -> HANDLE);
windows_link::link!("kernel32.dll" "system" fn SetEvent(hevent : HANDLE) -> BOOL);
windows_link::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle : HANDLE, dwmilliseconds : u32) -> WAIT_EVENT);
pub type BOOL = i32;
pub type HANDLE = *mut core::ffi::c_void;
pub type PCWSTR = *const u16;
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
pub type WAIT_EVENT = u32;
