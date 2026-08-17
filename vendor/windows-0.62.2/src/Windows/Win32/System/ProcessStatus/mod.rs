#[inline]
pub unsafe fn EmptyWorkingSet(hprocess: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn EmptyWorkingSet(hprocess : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { EmptyWorkingSet(hprocess).ok() }
}
#[inline]
pub unsafe fn EnumDeviceDrivers(lpimagebase: *mut *mut core::ffi::c_void, cb: u32, lpcbneeded: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn EnumDeviceDrivers(lpimagebase : *mut *mut core::ffi::c_void, cb : u32, lpcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { EnumDeviceDrivers(lpimagebase as _, cb, lpcbneeded as _).ok() }
}
#[inline]
pub unsafe fn EnumPageFilesA(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKA, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { EnumPageFilesA(pcallbackroutine, pcontext as _).ok() }
}
#[inline]
pub unsafe fn EnumPageFilesW(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKW, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { EnumPageFilesW(pcallbackroutine, pcontext as _).ok() }
}
#[inline]
pub unsafe fn EnumProcessModules(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HMODULE, cb: u32, lpcbneeded: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn EnumProcessModules(hprocess : super::super::Foundation:: HANDLE, lphmodule : *mut super::super::Foundation:: HMODULE, cb : u32, lpcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { EnumProcessModules(hprocess, lphmodule as _, cb, lpcbneeded as _).ok() }
}
#[inline]
pub unsafe fn EnumProcessModulesEx(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HMODULE, cb: u32, lpcbneeded: *mut u32, dwfilterflag: ENUM_PROCESS_MODULES_EX_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn EnumProcessModulesEx(hprocess : super::super::Foundation:: HANDLE, lphmodule : *mut super::super::Foundation:: HMODULE, cb : u32, lpcbneeded : *mut u32, dwfilterflag : ENUM_PROCESS_MODULES_EX_FLAGS) -> windows_core::BOOL);
    unsafe { EnumProcessModulesEx(hprocess, lphmodule as _, cb, lpcbneeded as _, dwfilterflag).ok() }
}
#[inline]
pub unsafe fn EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn EnumProcesses(lpidprocess : *mut u32, cb : u32, lpcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { EnumProcesses(lpidprocess as _, cb, lpcbneeded as _).ok() }
}
#[inline]
pub unsafe fn GetDeviceDriverBaseNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("psapi.dll" "system" fn GetDeviceDriverBaseNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetDeviceDriverBaseNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetDeviceDriverBaseNameW(imagebase: *const core::ffi::c_void, lpbasename: &mut [u16]) -> u32 {
    windows_core::link!("psapi.dll" "system" fn GetDeviceDriverBaseNameW(imagebase : *const core::ffi::c_void, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetDeviceDriverBaseNameW(imagebase, core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetDeviceDriverFileNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("psapi.dll" "system" fn GetDeviceDriverFileNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetDeviceDriverFileNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetDeviceDriverFileNameW(imagebase: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_core::link!("psapi.dll" "system" fn GetDeviceDriverFileNameW(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetDeviceDriverFileNameW(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetMappedFileNameA(hprocess: super::super::Foundation::HANDLE, lpv: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("psapi.dll" "system" fn GetMappedFileNameA(hprocess : super::super::Foundation:: HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetMappedFileNameA(hprocess, lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetMappedFileNameW(hprocess: super::super::Foundation::HANDLE, lpv: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_core::link!("psapi.dll" "system" fn GetMappedFileNameW(hprocess : super::super::Foundation:: HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetMappedFileNameW(hprocess, lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetModuleBaseNameA(hprocess: super::super::Foundation::HANDLE, hmodule: Option<super::super::Foundation::HMODULE>, lpbasename: &mut [u8]) -> u32 {
    windows_core::link!("psapi.dll" "system" fn GetModuleBaseNameA(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpbasename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetModuleBaseNameA(hprocess, hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetModuleBaseNameW(hprocess: super::super::Foundation::HANDLE, hmodule: Option<super::super::Foundation::HMODULE>, lpbasename: &mut [u16]) -> u32 {
    windows_core::link!("psapi.dll" "system" fn GetModuleBaseNameW(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetModuleBaseNameW(hprocess, hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetModuleFileNameExA(hprocess: Option<super::super::Foundation::HANDLE>, hmodule: Option<super::super::Foundation::HMODULE>, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("psapi.dll" "system" fn GetModuleFileNameExA(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetModuleFileNameExA(hprocess.unwrap_or(core::mem::zeroed()) as _, hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetModuleFileNameExW(hprocess: Option<super::super::Foundation::HANDLE>, hmodule: Option<super::super::Foundation::HMODULE>, lpfilename: &mut [u16]) -> u32 {
    windows_core::link!("psapi.dll" "system" fn GetModuleFileNameExW(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetModuleFileNameExW(hprocess.unwrap_or(core::mem::zeroed()) as _, hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetModuleInformation(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HMODULE, lpmodinfo: *mut MODULEINFO, cb: u32) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn GetModuleInformation(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpmodinfo : *mut MODULEINFO, cb : u32) -> windows_core::BOOL);
    unsafe { GetModuleInformation(hprocess, hmodule, lpmodinfo as _, cb).ok() }
}
#[inline]
pub unsafe fn GetPerformanceInfo(pperformanceinformation: *mut PERFORMANCE_INFORMATION, cb: u32) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn GetPerformanceInfo(pperformanceinformation : *mut PERFORMANCE_INFORMATION, cb : u32) -> windows_core::BOOL);
    unsafe { GetPerformanceInfo(pperformanceinformation as _, cb).ok() }
}
#[inline]
pub unsafe fn GetProcessImageFileNameA(hprocess: super::super::Foundation::HANDLE, lpimagefilename: &mut [u8]) -> u32 {
    windows_core::link!("psapi.dll" "system" fn GetProcessImageFileNameA(hprocess : super::super::Foundation:: HANDLE, lpimagefilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetProcessImageFileNameA(hprocess, core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetProcessImageFileNameW(hprocess: super::super::Foundation::HANDLE, lpimagefilename: &mut [u16]) -> u32 {
    windows_core::link!("psapi.dll" "system" fn GetProcessImageFileNameW(hprocess : super::super::Foundation:: HANDLE, lpimagefilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetProcessImageFileNameW(hprocess, core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetProcessMemoryInfo(process: super::super::Foundation::HANDLE, ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS, cb: u32) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn GetProcessMemoryInfo(process : super::super::Foundation:: HANDLE, ppsmemcounters : *mut PROCESS_MEMORY_COUNTERS, cb : u32) -> windows_core::BOOL);
    unsafe { GetProcessMemoryInfo(process, ppsmemcounters as _, cb).ok() }
}
#[inline]
pub unsafe fn GetWsChanges(hprocess: super::super::Foundation::HANDLE, lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION, cb: u32) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn GetWsChanges(hprocess : super::super::Foundation:: HANDLE, lpwatchinfo : *mut PSAPI_WS_WATCH_INFORMATION, cb : u32) -> windows_core::BOOL);
    unsafe { GetWsChanges(hprocess, lpwatchinfo as _, cb).ok() }
}
#[inline]
pub unsafe fn GetWsChangesEx(hprocess: super::super::Foundation::HANDLE, lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX, cb: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn GetWsChangesEx(hprocess : super::super::Foundation:: HANDLE, lpwatchinfoex : *mut PSAPI_WS_WATCH_INFORMATION_EX, cb : *mut u32) -> windows_core::BOOL);
    unsafe { GetWsChangesEx(hprocess, lpwatchinfoex as _, cb as _).ok() }
}
#[inline]
pub unsafe fn InitializeProcessForWsWatch(hprocess: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn InitializeProcessForWsWatch(hprocess : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { InitializeProcessForWsWatch(hprocess).ok() }
}
#[inline]
pub unsafe fn K32EmptyWorkingSet(hprocess: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EmptyWorkingSet(hprocess : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { K32EmptyWorkingSet(hprocess) }
}
#[inline]
pub unsafe fn K32EnumDeviceDrivers(lpimagebase: *mut *mut core::ffi::c_void, cb: u32, lpcbneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EnumDeviceDrivers(lpimagebase : *mut *mut core::ffi::c_void, cb : u32, lpcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { K32EnumDeviceDrivers(lpimagebase as _, cb, lpcbneeded as _) }
}
#[inline]
pub unsafe fn K32EnumPageFilesA(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKA, pcontext: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { K32EnumPageFilesA(pcallbackroutine, pcontext as _) }
}
#[inline]
pub unsafe fn K32EnumPageFilesW(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKW, pcontext: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { K32EnumPageFilesW(pcallbackroutine, pcontext as _) }
}
#[inline]
pub unsafe fn K32EnumProcessModules(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HMODULE, cb: u32, lpcbneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EnumProcessModules(hprocess : super::super::Foundation:: HANDLE, lphmodule : *mut super::super::Foundation:: HMODULE, cb : u32, lpcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { K32EnumProcessModules(hprocess, lphmodule as _, cb, lpcbneeded as _) }
}
#[inline]
pub unsafe fn K32EnumProcessModulesEx(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HMODULE, cb: u32, lpcbneeded: *mut u32, dwfilterflag: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EnumProcessModulesEx(hprocess : super::super::Foundation:: HANDLE, lphmodule : *mut super::super::Foundation:: HMODULE, cb : u32, lpcbneeded : *mut u32, dwfilterflag : u32) -> windows_core::BOOL);
    unsafe { K32EnumProcessModulesEx(hprocess, lphmodule as _, cb, lpcbneeded as _, dwfilterflag) }
}
#[inline]
pub unsafe fn K32EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EnumProcesses(lpidprocess : *mut u32, cb : u32, lpcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { K32EnumProcesses(lpidprocess as _, cb, lpcbneeded as _) }
}
#[inline]
pub unsafe fn K32GetDeviceDriverBaseNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn K32GetDeviceDriverBaseNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { K32GetDeviceDriverBaseNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetDeviceDriverBaseNameW(imagebase: *const core::ffi::c_void, lpbasename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn K32GetDeviceDriverBaseNameW(imagebase : *const core::ffi::c_void, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { K32GetDeviceDriverBaseNameW(imagebase, core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetDeviceDriverFileNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn K32GetDeviceDriverFileNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { K32GetDeviceDriverFileNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetDeviceDriverFileNameW(imagebase: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn K32GetDeviceDriverFileNameW(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { K32GetDeviceDriverFileNameW(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetMappedFileNameA(hprocess: super::super::Foundation::HANDLE, lpv: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn K32GetMappedFileNameA(hprocess : super::super::Foundation:: HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { K32GetMappedFileNameA(hprocess, lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetMappedFileNameW(hprocess: super::super::Foundation::HANDLE, lpv: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn K32GetMappedFileNameW(hprocess : super::super::Foundation:: HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { K32GetMappedFileNameW(hprocess, lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetModuleBaseNameA(hprocess: super::super::Foundation::HANDLE, hmodule: Option<super::super::Foundation::HMODULE>, lpbasename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn K32GetModuleBaseNameA(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpbasename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { K32GetModuleBaseNameA(hprocess, hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetModuleBaseNameW(hprocess: super::super::Foundation::HANDLE, hmodule: Option<super::super::Foundation::HMODULE>, lpbasename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn K32GetModuleBaseNameW(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { K32GetModuleBaseNameW(hprocess, hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetModuleFileNameExA(hprocess: Option<super::super::Foundation::HANDLE>, hmodule: Option<super::super::Foundation::HMODULE>, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn K32GetModuleFileNameExA(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { K32GetModuleFileNameExA(hprocess.unwrap_or(core::mem::zeroed()) as _, hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetModuleFileNameExW(hprocess: Option<super::super::Foundation::HANDLE>, hmodule: Option<super::super::Foundation::HMODULE>, lpfilename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn K32GetModuleFileNameExW(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { K32GetModuleFileNameExW(hprocess.unwrap_or(core::mem::zeroed()) as _, hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetModuleInformation(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HMODULE, lpmodinfo: *mut MODULEINFO, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32GetModuleInformation(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpmodinfo : *mut MODULEINFO, cb : u32) -> windows_core::BOOL);
    unsafe { K32GetModuleInformation(hprocess, hmodule, lpmodinfo as _, cb) }
}
#[inline]
pub unsafe fn K32GetPerformanceInfo(pperformanceinformation: *mut PERFORMANCE_INFORMATION, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32GetPerformanceInfo(pperformanceinformation : *mut PERFORMANCE_INFORMATION, cb : u32) -> windows_core::BOOL);
    unsafe { K32GetPerformanceInfo(pperformanceinformation as _, cb) }
}
#[inline]
pub unsafe fn K32GetProcessImageFileNameA(hprocess: super::super::Foundation::HANDLE, lpimagefilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn K32GetProcessImageFileNameA(hprocess : super::super::Foundation:: HANDLE, lpimagefilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { K32GetProcessImageFileNameA(hprocess, core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetProcessImageFileNameW(hprocess: super::super::Foundation::HANDLE, lpimagefilename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn K32GetProcessImageFileNameW(hprocess : super::super::Foundation:: HANDLE, lpimagefilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { K32GetProcessImageFileNameW(hprocess, core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetProcessMemoryInfo(process: super::super::Foundation::HANDLE, ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32GetProcessMemoryInfo(process : super::super::Foundation:: HANDLE, ppsmemcounters : *mut PROCESS_MEMORY_COUNTERS, cb : u32) -> windows_core::BOOL);
    unsafe { K32GetProcessMemoryInfo(process, ppsmemcounters as _, cb) }
}
#[inline]
pub unsafe fn K32GetWsChanges(hprocess: super::super::Foundation::HANDLE, lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32GetWsChanges(hprocess : super::super::Foundation:: HANDLE, lpwatchinfo : *mut PSAPI_WS_WATCH_INFORMATION, cb : u32) -> windows_core::BOOL);
    unsafe { K32GetWsChanges(hprocess, lpwatchinfo as _, cb) }
}
#[inline]
pub unsafe fn K32GetWsChangesEx(hprocess: super::super::Foundation::HANDLE, lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX, cb: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32GetWsChangesEx(hprocess : super::super::Foundation:: HANDLE, lpwatchinfoex : *mut PSAPI_WS_WATCH_INFORMATION_EX, cb : *mut u32) -> windows_core::BOOL);
    unsafe { K32GetWsChangesEx(hprocess, lpwatchinfoex as _, cb as _) }
}
#[inline]
pub unsafe fn K32InitializeProcessForWsWatch(hprocess: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32InitializeProcessForWsWatch(hprocess : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { K32InitializeProcessForWsWatch(hprocess) }
}
#[inline]
pub unsafe fn K32QueryWorkingSet(hprocess: super::super::Foundation::HANDLE, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32QueryWorkingSet(hprocess : super::super::Foundation:: HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> windows_core::BOOL);
    unsafe { K32QueryWorkingSet(hprocess, pv as _, cb) }
}
#[inline]
pub unsafe fn K32QueryWorkingSetEx(hprocess: super::super::Foundation::HANDLE, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32QueryWorkingSetEx(hprocess : super::super::Foundation:: HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> windows_core::BOOL);
    unsafe { K32QueryWorkingSetEx(hprocess, pv as _, cb) }
}
#[inline]
pub unsafe fn QueryWorkingSet(hprocess: super::super::Foundation::HANDLE, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn QueryWorkingSet(hprocess : super::super::Foundation:: HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> windows_core::BOOL);
    unsafe { QueryWorkingSet(hprocess, pv as _, cb).ok() }
}
#[inline]
pub unsafe fn QueryWorkingSetEx(hprocess: super::super::Foundation::HANDLE, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::Result<()> {
    windows_core::link!("psapi.dll" "system" fn QueryWorkingSetEx(hprocess : super::super::Foundation:: HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> windows_core::BOOL);
    unsafe { QueryWorkingSetEx(hprocess, pv as _, cb).ok() }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENUM_PAGE_FILE_INFORMATION {
    pub cb: u32,
    pub Reserved: u32,
    pub TotalSize: usize,
    pub TotalInUse: usize,
    pub PeakUsage: usize,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENUM_PROCESS_MODULES_EX_FLAGS(pub u32);
pub const LIST_MODULES_32BIT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(1u32);
pub const LIST_MODULES_64BIT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(2u32);
pub const LIST_MODULES_ALL: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(3u32);
pub const LIST_MODULES_DEFAULT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(0u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MODULEINFO {
    pub lpBaseOfDll: *mut core::ffi::c_void,
    pub SizeOfImage: u32,
    pub EntryPoint: *mut core::ffi::c_void,
}
impl Default for MODULEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PENUM_PAGE_FILE_CALLBACKA = Option<unsafe extern "system" fn(pcontext: *mut core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: windows_core::PCSTR) -> windows_core::BOOL>;
pub type PENUM_PAGE_FILE_CALLBACKW = Option<unsafe extern "system" fn(pcontext: *mut core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: windows_core::PCWSTR) -> windows_core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERFORMANCE_INFORMATION {
    pub cb: u32,
    pub CommitTotal: usize,
    pub CommitLimit: usize,
    pub CommitPeak: usize,
    pub PhysicalTotal: usize,
    pub PhysicalAvailable: usize,
    pub SystemCache: usize,
    pub KernelTotal: usize,
    pub KernelPaged: usize,
    pub KernelNonpaged: usize,
    pub PageSize: usize,
    pub HandleCount: u32,
    pub ProcessCount: u32,
    pub ThreadCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MEMORY_COUNTERS {
    pub cb: u32,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MEMORY_COUNTERS_EX {
    pub cb: u32,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivateUsage: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MEMORY_COUNTERS_EX2 {
    pub cb: u32,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivateUsage: usize,
    pub PrivateWorkingSetSize: usize,
    pub SharedCommitUsage: u64,
}
pub const PSAPI_VERSION: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_BLOCK_0,
}
impl Default for PSAPI_WORKING_SET_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PSAPI_WORKING_SET_BLOCK_0 {
    pub _bitfield: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0,
}
impl Default for PSAPI_WORKING_SET_EX_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK_0 {
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0_0,
    pub Invalid: PSAPI_WORKING_SET_EX_BLOCK_0_1,
}
impl Default for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub _bitfield: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    pub _bitfield: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PSAPI_WORKING_SET_EX_INFORMATION {
    pub VirtualAddress: *mut core::ffi::c_void,
    pub VirtualAttributes: PSAPI_WORKING_SET_EX_BLOCK,
}
impl Default for PSAPI_WORKING_SET_EX_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PSAPI_WORKING_SET_INFORMATION {
    pub NumberOfEntries: usize,
    pub WorkingSetInfo: [PSAPI_WORKING_SET_BLOCK; 1],
}
impl Default for PSAPI_WORKING_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PSAPI_WS_WATCH_INFORMATION {
    pub FaultingPc: *mut core::ffi::c_void,
    pub FaultingVa: *mut core::ffi::c_void,
}
impl Default for PSAPI_WS_WATCH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PSAPI_WS_WATCH_INFORMATION_EX {
    pub BasicInfo: PSAPI_WS_WATCH_INFORMATION,
    pub FaultingThreadId: usize,
    pub Flags: usize,
}
