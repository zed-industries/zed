#[inline]
pub unsafe fn EmptyWorkingSet<P0>(hprocess: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("psapi.dll" "system" fn EmptyWorkingSet(hprocess : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    EmptyWorkingSet(hprocess.param().abi()).ok()
}
#[inline]
pub unsafe fn EnumDeviceDrivers(lpimagebase: *mut *mut core::ffi::c_void, cb: u32, lpcbneeded: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn EnumDeviceDrivers(lpimagebase : *mut *mut core::ffi::c_void, cb : u32, lpcbneeded : *mut u32) -> super::super::Foundation:: BOOL);
    EnumDeviceDrivers(lpimagebase, cb, lpcbneeded).ok()
}
#[inline]
pub unsafe fn EnumPageFilesA(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKA, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    EnumPageFilesA(pcallbackroutine, pcontext).ok()
}
#[inline]
pub unsafe fn EnumPageFilesW(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKW, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    EnumPageFilesW(pcallbackroutine, pcontext).ok()
}
#[inline]
pub unsafe fn EnumProcessModules<P0>(hprocess: P0, lphmodule: *mut super::super::Foundation::HMODULE, cb: u32, lpcbneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("psapi.dll" "system" fn EnumProcessModules(hprocess : super::super::Foundation:: HANDLE, lphmodule : *mut super::super::Foundation:: HMODULE, cb : u32, lpcbneeded : *mut u32) -> super::super::Foundation:: BOOL);
    EnumProcessModules(hprocess.param().abi(), lphmodule, cb, lpcbneeded).ok()
}
#[inline]
pub unsafe fn EnumProcessModulesEx<P0>(hprocess: P0, lphmodule: *mut super::super::Foundation::HMODULE, cb: u32, lpcbneeded: *mut u32, dwfilterflag: ENUM_PROCESS_MODULES_EX_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("psapi.dll" "system" fn EnumProcessModulesEx(hprocess : super::super::Foundation:: HANDLE, lphmodule : *mut super::super::Foundation:: HMODULE, cb : u32, lpcbneeded : *mut u32, dwfilterflag : ENUM_PROCESS_MODULES_EX_FLAGS) -> super::super::Foundation:: BOOL);
    EnumProcessModulesEx(hprocess.param().abi(), lphmodule, cb, lpcbneeded, dwfilterflag).ok()
}
#[inline]
pub unsafe fn EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn EnumProcesses(lpidprocess : *mut u32, cb : u32, lpcbneeded : *mut u32) -> super::super::Foundation:: BOOL);
    EnumProcesses(lpidprocess, cb, lpcbneeded).ok()
}
#[inline]
pub unsafe fn GetDeviceDriverBaseNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetDeviceDriverBaseNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    GetDeviceDriverBaseNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetDeviceDriverBaseNameW(imagebase: *const core::ffi::c_void, lpbasename: &mut [u16]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetDeviceDriverBaseNameW(imagebase : *const core::ffi::c_void, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    GetDeviceDriverBaseNameW(imagebase, core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetDeviceDriverFileNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetDeviceDriverFileNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    GetDeviceDriverFileNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetDeviceDriverFileNameW(imagebase: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetDeviceDriverFileNameW(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    GetDeviceDriverFileNameW(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetMappedFileNameA<P0>(hprocess: P0, lpv: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("psapi.dll" "system" fn GetMappedFileNameA(hprocess : super::super::Foundation:: HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    GetMappedFileNameA(hprocess.param().abi(), lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetMappedFileNameW<P0>(hprocess: P0, lpv: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("psapi.dll" "system" fn GetMappedFileNameW(hprocess : super::super::Foundation:: HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    GetMappedFileNameW(hprocess.param().abi(), lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetModuleBaseNameA<P0, P1>(hprocess: P0, hmodule: P1, lpbasename: &mut [u8]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("psapi.dll" "system" fn GetModuleBaseNameA(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpbasename : windows_core::PSTR, nsize : u32) -> u32);
    GetModuleBaseNameA(hprocess.param().abi(), hmodule.param().abi(), core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetModuleBaseNameW<P0, P1>(hprocess: P0, hmodule: P1, lpbasename: &mut [u16]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("psapi.dll" "system" fn GetModuleBaseNameW(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    GetModuleBaseNameW(hprocess.param().abi(), hmodule.param().abi(), core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetModuleFileNameExA<P0, P1>(hprocess: P0, hmodule: P1, lpfilename: &mut [u8]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("psapi.dll" "system" fn GetModuleFileNameExA(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    GetModuleFileNameExA(hprocess.param().abi(), hmodule.param().abi(), core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetModuleFileNameExW<P0, P1>(hprocess: P0, hmodule: P1, lpfilename: &mut [u16]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("psapi.dll" "system" fn GetModuleFileNameExW(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    GetModuleFileNameExW(hprocess.param().abi(), hmodule.param().abi(), core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetModuleInformation<P0, P1>(hprocess: P0, hmodule: P1, lpmodinfo: *mut MODULEINFO, cb: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("psapi.dll" "system" fn GetModuleInformation(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpmodinfo : *mut MODULEINFO, cb : u32) -> super::super::Foundation:: BOOL);
    GetModuleInformation(hprocess.param().abi(), hmodule.param().abi(), lpmodinfo, cb).ok()
}
#[inline]
pub unsafe fn GetPerformanceInfo(pperformanceinformation: *mut PERFORMANCE_INFORMATION, cb: u32) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn GetPerformanceInfo(pperformanceinformation : *mut PERFORMANCE_INFORMATION, cb : u32) -> super::super::Foundation:: BOOL);
    GetPerformanceInfo(pperformanceinformation, cb).ok()
}
#[inline]
pub unsafe fn GetProcessImageFileNameA<P0>(hprocess: P0, lpimagefilename: &mut [u8]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("psapi.dll" "system" fn GetProcessImageFileNameA(hprocess : super::super::Foundation:: HANDLE, lpimagefilename : windows_core::PSTR, nsize : u32) -> u32);
    GetProcessImageFileNameA(hprocess.param().abi(), core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetProcessImageFileNameW<P0>(hprocess: P0, lpimagefilename: &mut [u16]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("psapi.dll" "system" fn GetProcessImageFileNameW(hprocess : super::super::Foundation:: HANDLE, lpimagefilename : windows_core::PWSTR, nsize : u32) -> u32);
    GetProcessImageFileNameW(hprocess.param().abi(), core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetProcessMemoryInfo<P0>(process: P0, ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS, cb: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("psapi.dll" "system" fn GetProcessMemoryInfo(process : super::super::Foundation:: HANDLE, ppsmemcounters : *mut PROCESS_MEMORY_COUNTERS, cb : u32) -> super::super::Foundation:: BOOL);
    GetProcessMemoryInfo(process.param().abi(), ppsmemcounters, cb).ok()
}
#[inline]
pub unsafe fn GetWsChanges<P0>(hprocess: P0, lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION, cb: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("psapi.dll" "system" fn GetWsChanges(hprocess : super::super::Foundation:: HANDLE, lpwatchinfo : *mut PSAPI_WS_WATCH_INFORMATION, cb : u32) -> super::super::Foundation:: BOOL);
    GetWsChanges(hprocess.param().abi(), lpwatchinfo, cb).ok()
}
#[inline]
pub unsafe fn GetWsChangesEx<P0>(hprocess: P0, lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX, cb: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("psapi.dll" "system" fn GetWsChangesEx(hprocess : super::super::Foundation:: HANDLE, lpwatchinfoex : *mut PSAPI_WS_WATCH_INFORMATION_EX, cb : *mut u32) -> super::super::Foundation:: BOOL);
    GetWsChangesEx(hprocess.param().abi(), lpwatchinfoex, cb).ok()
}
#[inline]
pub unsafe fn InitializeProcessForWsWatch<P0>(hprocess: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("psapi.dll" "system" fn InitializeProcessForWsWatch(hprocess : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    InitializeProcessForWsWatch(hprocess.param().abi()).ok()
}
#[inline]
pub unsafe fn K32EmptyWorkingSet<P0>(hprocess: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32EmptyWorkingSet(hprocess : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    K32EmptyWorkingSet(hprocess.param().abi())
}
#[inline]
pub unsafe fn K32EnumDeviceDrivers(lpimagebase: *mut *mut core::ffi::c_void, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32EnumDeviceDrivers(lpimagebase : *mut *mut core::ffi::c_void, cb : u32, lpcbneeded : *mut u32) -> super::super::Foundation:: BOOL);
    K32EnumDeviceDrivers(lpimagebase, cb, lpcbneeded)
}
#[inline]
pub unsafe fn K32EnumPageFilesA(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKA, pcontext: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    K32EnumPageFilesA(pcallbackroutine, pcontext)
}
#[inline]
pub unsafe fn K32EnumPageFilesW(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKW, pcontext: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    K32EnumPageFilesW(pcallbackroutine, pcontext)
}
#[inline]
pub unsafe fn K32EnumProcessModules<P0>(hprocess: P0, lphmodule: *mut super::super::Foundation::HMODULE, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32EnumProcessModules(hprocess : super::super::Foundation:: HANDLE, lphmodule : *mut super::super::Foundation:: HMODULE, cb : u32, lpcbneeded : *mut u32) -> super::super::Foundation:: BOOL);
    K32EnumProcessModules(hprocess.param().abi(), lphmodule, cb, lpcbneeded)
}
#[inline]
pub unsafe fn K32EnumProcessModulesEx<P0>(hprocess: P0, lphmodule: *mut super::super::Foundation::HMODULE, cb: u32, lpcbneeded: *mut u32, dwfilterflag: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32EnumProcessModulesEx(hprocess : super::super::Foundation:: HANDLE, lphmodule : *mut super::super::Foundation:: HMODULE, cb : u32, lpcbneeded : *mut u32, dwfilterflag : u32) -> super::super::Foundation:: BOOL);
    K32EnumProcessModulesEx(hprocess.param().abi(), lphmodule, cb, lpcbneeded, dwfilterflag)
}
#[inline]
pub unsafe fn K32EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32EnumProcesses(lpidprocess : *mut u32, cb : u32, lpcbneeded : *mut u32) -> super::super::Foundation:: BOOL);
    K32EnumProcesses(lpidprocess, cb, lpcbneeded)
}
#[inline]
pub unsafe fn K32GetDeviceDriverBaseNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetDeviceDriverBaseNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    K32GetDeviceDriverBaseNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetDeviceDriverBaseNameW(imagebase: *const core::ffi::c_void, lpbasename: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetDeviceDriverBaseNameW(imagebase : *const core::ffi::c_void, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    K32GetDeviceDriverBaseNameW(imagebase, core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetDeviceDriverFileNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetDeviceDriverFileNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    K32GetDeviceDriverFileNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetDeviceDriverFileNameW(imagebase: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetDeviceDriverFileNameW(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    K32GetDeviceDriverFileNameW(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetMappedFileNameA<P0>(hprocess: P0, lpv: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32GetMappedFileNameA(hprocess : super::super::Foundation:: HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    K32GetMappedFileNameA(hprocess.param().abi(), lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetMappedFileNameW<P0>(hprocess: P0, lpv: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32GetMappedFileNameW(hprocess : super::super::Foundation:: HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    K32GetMappedFileNameW(hprocess.param().abi(), lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetModuleBaseNameA<P0, P1>(hprocess: P0, hmodule: P1, lpbasename: &mut [u8]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32GetModuleBaseNameA(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpbasename : windows_core::PSTR, nsize : u32) -> u32);
    K32GetModuleBaseNameA(hprocess.param().abi(), hmodule.param().abi(), core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetModuleBaseNameW<P0, P1>(hprocess: P0, hmodule: P1, lpbasename: &mut [u16]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32GetModuleBaseNameW(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    K32GetModuleBaseNameW(hprocess.param().abi(), hmodule.param().abi(), core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetModuleFileNameExA<P0, P1>(hprocess: P0, hmodule: P1, lpfilename: &mut [u8]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32GetModuleFileNameExA(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    K32GetModuleFileNameExA(hprocess.param().abi(), hmodule.param().abi(), core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetModuleFileNameExW<P0, P1>(hprocess: P0, hmodule: P1, lpfilename: &mut [u16]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32GetModuleFileNameExW(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    K32GetModuleFileNameExW(hprocess.param().abi(), hmodule.param().abi(), core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetModuleInformation<P0, P1>(hprocess: P0, hmodule: P1, lpmodinfo: *mut MODULEINFO, cb: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32GetModuleInformation(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpmodinfo : *mut MODULEINFO, cb : u32) -> super::super::Foundation:: BOOL);
    K32GetModuleInformation(hprocess.param().abi(), hmodule.param().abi(), lpmodinfo, cb)
}
#[inline]
pub unsafe fn K32GetPerformanceInfo(pperformanceinformation: *mut PERFORMANCE_INFORMATION, cb: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32GetPerformanceInfo(pperformanceinformation : *mut PERFORMANCE_INFORMATION, cb : u32) -> super::super::Foundation:: BOOL);
    K32GetPerformanceInfo(pperformanceinformation, cb)
}
#[inline]
pub unsafe fn K32GetProcessImageFileNameA<P0>(hprocess: P0, lpimagefilename: &mut [u8]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32GetProcessImageFileNameA(hprocess : super::super::Foundation:: HANDLE, lpimagefilename : windows_core::PSTR, nsize : u32) -> u32);
    K32GetProcessImageFileNameA(hprocess.param().abi(), core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetProcessImageFileNameW<P0>(hprocess: P0, lpimagefilename: &mut [u16]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32GetProcessImageFileNameW(hprocess : super::super::Foundation:: HANDLE, lpimagefilename : windows_core::PWSTR, nsize : u32) -> u32);
    K32GetProcessImageFileNameW(hprocess.param().abi(), core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetProcessMemoryInfo<P0>(process: P0, ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS, cb: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32GetProcessMemoryInfo(process : super::super::Foundation:: HANDLE, ppsmemcounters : *mut PROCESS_MEMORY_COUNTERS, cb : u32) -> super::super::Foundation:: BOOL);
    K32GetProcessMemoryInfo(process.param().abi(), ppsmemcounters, cb)
}
#[inline]
pub unsafe fn K32GetWsChanges<P0>(hprocess: P0, lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION, cb: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32GetWsChanges(hprocess : super::super::Foundation:: HANDLE, lpwatchinfo : *mut PSAPI_WS_WATCH_INFORMATION, cb : u32) -> super::super::Foundation:: BOOL);
    K32GetWsChanges(hprocess.param().abi(), lpwatchinfo, cb)
}
#[inline]
pub unsafe fn K32GetWsChangesEx<P0>(hprocess: P0, lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX, cb: *mut u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32GetWsChangesEx(hprocess : super::super::Foundation:: HANDLE, lpwatchinfoex : *mut PSAPI_WS_WATCH_INFORMATION_EX, cb : *mut u32) -> super::super::Foundation:: BOOL);
    K32GetWsChangesEx(hprocess.param().abi(), lpwatchinfoex, cb)
}
#[inline]
pub unsafe fn K32InitializeProcessForWsWatch<P0>(hprocess: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32InitializeProcessForWsWatch(hprocess : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    K32InitializeProcessForWsWatch(hprocess.param().abi())
}
#[inline]
pub unsafe fn K32QueryWorkingSet<P0>(hprocess: P0, pv: *mut core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32QueryWorkingSet(hprocess : super::super::Foundation:: HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> super::super::Foundation:: BOOL);
    K32QueryWorkingSet(hprocess.param().abi(), pv, cb)
}
#[inline]
pub unsafe fn K32QueryWorkingSetEx<P0>(hprocess: P0, pv: *mut core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn K32QueryWorkingSetEx(hprocess : super::super::Foundation:: HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> super::super::Foundation:: BOOL);
    K32QueryWorkingSetEx(hprocess.param().abi(), pv, cb)
}
#[inline]
pub unsafe fn QueryWorkingSet<P0>(hprocess: P0, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("psapi.dll" "system" fn QueryWorkingSet(hprocess : super::super::Foundation:: HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> super::super::Foundation:: BOOL);
    QueryWorkingSet(hprocess.param().abi(), pv, cb).ok()
}
#[inline]
pub unsafe fn QueryWorkingSetEx<P0>(hprocess: P0, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("psapi.dll" "system" fn QueryWorkingSetEx(hprocess : super::super::Foundation:: HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> super::super::Foundation:: BOOL);
    QueryWorkingSetEx(hprocess.param().abi(), pv, cb).ok()
}
pub const LIST_MODULES_32BIT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(1u32);
pub const LIST_MODULES_64BIT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(2u32);
pub const LIST_MODULES_ALL: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(3u32);
pub const LIST_MODULES_DEFAULT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(0u32);
pub const PSAPI_VERSION: u32 = 2u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ENUM_PROCESS_MODULES_EX_FLAGS(pub u32);
impl windows_core::TypeKind for ENUM_PROCESS_MODULES_EX_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ENUM_PROCESS_MODULES_EX_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ENUM_PROCESS_MODULES_EX_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct ENUM_PAGE_FILE_INFORMATION {
    pub cb: u32,
    pub Reserved: u32,
    pub TotalSize: usize,
    pub TotalInUse: usize,
    pub PeakUsage: usize,
}
impl Copy for ENUM_PAGE_FILE_INFORMATION {}
impl Clone for ENUM_PAGE_FILE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for ENUM_PAGE_FILE_INFORMATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ENUM_PAGE_FILE_INFORMATION").field("cb", &self.cb).field("Reserved", &self.Reserved).field("TotalSize", &self.TotalSize).field("TotalInUse", &self.TotalInUse).field("PeakUsage", &self.PeakUsage).finish()
    }
}
impl windows_core::TypeKind for ENUM_PAGE_FILE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for ENUM_PAGE_FILE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.Reserved == other.Reserved && self.TotalSize == other.TotalSize && self.TotalInUse == other.TotalInUse && self.PeakUsage == other.PeakUsage
    }
}
impl Eq for ENUM_PAGE_FILE_INFORMATION {}
impl Default for ENUM_PAGE_FILE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MODULEINFO {
    pub lpBaseOfDll: *mut core::ffi::c_void,
    pub SizeOfImage: u32,
    pub EntryPoint: *mut core::ffi::c_void,
}
impl Copy for MODULEINFO {}
impl Clone for MODULEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for MODULEINFO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MODULEINFO").field("lpBaseOfDll", &self.lpBaseOfDll).field("SizeOfImage", &self.SizeOfImage).field("EntryPoint", &self.EntryPoint).finish()
    }
}
impl windows_core::TypeKind for MODULEINFO {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for MODULEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpBaseOfDll == other.lpBaseOfDll && self.SizeOfImage == other.SizeOfImage && self.EntryPoint == other.EntryPoint
    }
}
impl Eq for MODULEINFO {}
impl Default for MODULEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl Copy for PERFORMANCE_INFORMATION {}
impl Clone for PERFORMANCE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PERFORMANCE_INFORMATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PERFORMANCE_INFORMATION")
            .field("cb", &self.cb)
            .field("CommitTotal", &self.CommitTotal)
            .field("CommitLimit", &self.CommitLimit)
            .field("CommitPeak", &self.CommitPeak)
            .field("PhysicalTotal", &self.PhysicalTotal)
            .field("PhysicalAvailable", &self.PhysicalAvailable)
            .field("SystemCache", &self.SystemCache)
            .field("KernelTotal", &self.KernelTotal)
            .field("KernelPaged", &self.KernelPaged)
            .field("KernelNonpaged", &self.KernelNonpaged)
            .field("PageSize", &self.PageSize)
            .field("HandleCount", &self.HandleCount)
            .field("ProcessCount", &self.ProcessCount)
            .field("ThreadCount", &self.ThreadCount)
            .finish()
    }
}
impl windows_core::TypeKind for PERFORMANCE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PERFORMANCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.CommitTotal == other.CommitTotal && self.CommitLimit == other.CommitLimit && self.CommitPeak == other.CommitPeak && self.PhysicalTotal == other.PhysicalTotal && self.PhysicalAvailable == other.PhysicalAvailable && self.SystemCache == other.SystemCache && self.KernelTotal == other.KernelTotal && self.KernelPaged == other.KernelPaged && self.KernelNonpaged == other.KernelNonpaged && self.PageSize == other.PageSize && self.HandleCount == other.HandleCount && self.ProcessCount == other.ProcessCount && self.ThreadCount == other.ThreadCount
    }
}
impl Eq for PERFORMANCE_INFORMATION {}
impl Default for PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl Copy for PROCESS_MEMORY_COUNTERS {}
impl Clone for PROCESS_MEMORY_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PROCESS_MEMORY_COUNTERS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PROCESS_MEMORY_COUNTERS")
            .field("cb", &self.cb)
            .field("PageFaultCount", &self.PageFaultCount)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("QuotaPeakPagedPoolUsage", &self.QuotaPeakPagedPoolUsage)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field("QuotaPeakNonPagedPoolUsage", &self.QuotaPeakNonPagedPoolUsage)
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .finish()
    }
}
impl windows_core::TypeKind for PROCESS_MEMORY_COUNTERS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PROCESS_MEMORY_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.PageFaultCount == other.PageFaultCount && self.PeakWorkingSetSize == other.PeakWorkingSetSize && self.WorkingSetSize == other.WorkingSetSize && self.QuotaPeakPagedPoolUsage == other.QuotaPeakPagedPoolUsage && self.QuotaPagedPoolUsage == other.QuotaPagedPoolUsage && self.QuotaPeakNonPagedPoolUsage == other.QuotaPeakNonPagedPoolUsage && self.QuotaNonPagedPoolUsage == other.QuotaNonPagedPoolUsage && self.PagefileUsage == other.PagefileUsage && self.PeakPagefileUsage == other.PeakPagefileUsage
    }
}
impl Eq for PROCESS_MEMORY_COUNTERS {}
impl Default for PROCESS_MEMORY_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl Copy for PROCESS_MEMORY_COUNTERS_EX {}
impl Clone for PROCESS_MEMORY_COUNTERS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PROCESS_MEMORY_COUNTERS_EX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PROCESS_MEMORY_COUNTERS_EX")
            .field("cb", &self.cb)
            .field("PageFaultCount", &self.PageFaultCount)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("QuotaPeakPagedPoolUsage", &self.QuotaPeakPagedPoolUsage)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field("QuotaPeakNonPagedPoolUsage", &self.QuotaPeakNonPagedPoolUsage)
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("PrivateUsage", &self.PrivateUsage)
            .finish()
    }
}
impl windows_core::TypeKind for PROCESS_MEMORY_COUNTERS_EX {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PROCESS_MEMORY_COUNTERS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.PageFaultCount == other.PageFaultCount && self.PeakWorkingSetSize == other.PeakWorkingSetSize && self.WorkingSetSize == other.WorkingSetSize && self.QuotaPeakPagedPoolUsage == other.QuotaPeakPagedPoolUsage && self.QuotaPagedPoolUsage == other.QuotaPagedPoolUsage && self.QuotaPeakNonPagedPoolUsage == other.QuotaPeakNonPagedPoolUsage && self.QuotaNonPagedPoolUsage == other.QuotaNonPagedPoolUsage && self.PagefileUsage == other.PagefileUsage && self.PeakPagefileUsage == other.PeakPagefileUsage && self.PrivateUsage == other.PrivateUsage
    }
}
impl Eq for PROCESS_MEMORY_COUNTERS_EX {}
impl Default for PROCESS_MEMORY_COUNTERS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl Copy for PROCESS_MEMORY_COUNTERS_EX2 {}
impl Clone for PROCESS_MEMORY_COUNTERS_EX2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PROCESS_MEMORY_COUNTERS_EX2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PROCESS_MEMORY_COUNTERS_EX2")
            .field("cb", &self.cb)
            .field("PageFaultCount", &self.PageFaultCount)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("QuotaPeakPagedPoolUsage", &self.QuotaPeakPagedPoolUsage)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field("QuotaPeakNonPagedPoolUsage", &self.QuotaPeakNonPagedPoolUsage)
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("PrivateUsage", &self.PrivateUsage)
            .field("PrivateWorkingSetSize", &self.PrivateWorkingSetSize)
            .field("SharedCommitUsage", &self.SharedCommitUsage)
            .finish()
    }
}
impl windows_core::TypeKind for PROCESS_MEMORY_COUNTERS_EX2 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PROCESS_MEMORY_COUNTERS_EX2 {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.PageFaultCount == other.PageFaultCount && self.PeakWorkingSetSize == other.PeakWorkingSetSize && self.WorkingSetSize == other.WorkingSetSize && self.QuotaPeakPagedPoolUsage == other.QuotaPeakPagedPoolUsage && self.QuotaPagedPoolUsage == other.QuotaPagedPoolUsage && self.QuotaPeakNonPagedPoolUsage == other.QuotaPeakNonPagedPoolUsage && self.QuotaNonPagedPoolUsage == other.QuotaNonPagedPoolUsage && self.PagefileUsage == other.PagefileUsage && self.PeakPagefileUsage == other.PeakPagefileUsage && self.PrivateUsage == other.PrivateUsage && self.PrivateWorkingSetSize == other.PrivateWorkingSetSize && self.SharedCommitUsage == other.SharedCommitUsage
    }
}
impl Eq for PROCESS_MEMORY_COUNTERS_EX2 {}
impl Default for PROCESS_MEMORY_COUNTERS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union PSAPI_WORKING_SET_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_BLOCK_0,
}
impl Copy for PSAPI_WORKING_SET_BLOCK {}
impl Clone for PSAPI_WORKING_SET_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for PSAPI_WORKING_SET_BLOCK {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSAPI_WORKING_SET_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PSAPI_WORKING_SET_BLOCK_0 {
    pub _bitfield: usize,
}
impl Copy for PSAPI_WORKING_SET_BLOCK_0 {}
impl Clone for PSAPI_WORKING_SET_BLOCK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PSAPI_WORKING_SET_BLOCK_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PSAPI_WORKING_SET_BLOCK_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl windows_core::TypeKind for PSAPI_WORKING_SET_BLOCK_0 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PSAPI_WORKING_SET_BLOCK_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl Eq for PSAPI_WORKING_SET_BLOCK_0 {}
impl Default for PSAPI_WORKING_SET_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union PSAPI_WORKING_SET_EX_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0,
}
impl Copy for PSAPI_WORKING_SET_EX_BLOCK {}
impl Clone for PSAPI_WORKING_SET_EX_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for PSAPI_WORKING_SET_EX_BLOCK {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSAPI_WORKING_SET_EX_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union PSAPI_WORKING_SET_EX_BLOCK_0 {
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0_0,
    pub Invalid: PSAPI_WORKING_SET_EX_BLOCK_0_1,
}
impl Copy for PSAPI_WORKING_SET_EX_BLOCK_0 {}
impl Clone for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for PSAPI_WORKING_SET_EX_BLOCK_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub _bitfield: usize,
}
impl Copy for PSAPI_WORKING_SET_EX_BLOCK_0_0 {}
impl Clone for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PSAPI_WORKING_SET_EX_BLOCK_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl windows_core::TypeKind for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl Eq for PSAPI_WORKING_SET_EX_BLOCK_0_0 {}
impl Default for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    pub _bitfield: usize,
}
impl Copy for PSAPI_WORKING_SET_EX_BLOCK_0_1 {}
impl Clone for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PSAPI_WORKING_SET_EX_BLOCK_0_1").field("_bitfield", &self._bitfield).finish()
    }
}
impl windows_core::TypeKind for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl Eq for PSAPI_WORKING_SET_EX_BLOCK_0_1 {}
impl Default for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PSAPI_WORKING_SET_EX_INFORMATION {
    pub VirtualAddress: *mut core::ffi::c_void,
    pub VirtualAttributes: PSAPI_WORKING_SET_EX_BLOCK,
}
impl Copy for PSAPI_WORKING_SET_EX_INFORMATION {}
impl Clone for PSAPI_WORKING_SET_EX_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for PSAPI_WORKING_SET_EX_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSAPI_WORKING_SET_EX_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PSAPI_WORKING_SET_INFORMATION {
    pub NumberOfEntries: usize,
    pub WorkingSetInfo: [PSAPI_WORKING_SET_BLOCK; 1],
}
impl Copy for PSAPI_WORKING_SET_INFORMATION {}
impl Clone for PSAPI_WORKING_SET_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for PSAPI_WORKING_SET_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSAPI_WORKING_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PSAPI_WS_WATCH_INFORMATION {
    pub FaultingPc: *mut core::ffi::c_void,
    pub FaultingVa: *mut core::ffi::c_void,
}
impl Copy for PSAPI_WS_WATCH_INFORMATION {}
impl Clone for PSAPI_WS_WATCH_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PSAPI_WS_WATCH_INFORMATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PSAPI_WS_WATCH_INFORMATION").field("FaultingPc", &self.FaultingPc).field("FaultingVa", &self.FaultingVa).finish()
    }
}
impl windows_core::TypeKind for PSAPI_WS_WATCH_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PSAPI_WS_WATCH_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.FaultingPc == other.FaultingPc && self.FaultingVa == other.FaultingVa
    }
}
impl Eq for PSAPI_WS_WATCH_INFORMATION {}
impl Default for PSAPI_WS_WATCH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PSAPI_WS_WATCH_INFORMATION_EX {
    pub BasicInfo: PSAPI_WS_WATCH_INFORMATION,
    pub FaultingThreadId: usize,
    pub Flags: usize,
}
impl Copy for PSAPI_WS_WATCH_INFORMATION_EX {}
impl Clone for PSAPI_WS_WATCH_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for PSAPI_WS_WATCH_INFORMATION_EX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PSAPI_WS_WATCH_INFORMATION_EX").field("BasicInfo", &self.BasicInfo).field("FaultingThreadId", &self.FaultingThreadId).field("Flags", &self.Flags).finish()
    }
}
impl windows_core::TypeKind for PSAPI_WS_WATCH_INFORMATION_EX {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for PSAPI_WS_WATCH_INFORMATION_EX {
    fn eq(&self, other: &Self) -> bool {
        self.BasicInfo == other.BasicInfo && self.FaultingThreadId == other.FaultingThreadId && self.Flags == other.Flags
    }
}
impl Eq for PSAPI_WS_WATCH_INFORMATION_EX {}
impl Default for PSAPI_WS_WATCH_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PENUM_PAGE_FILE_CALLBACKA = Option<unsafe extern "system" fn(pcontext: *mut core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: windows_core::PCSTR) -> super::super::Foundation::BOOL>;
pub type PENUM_PAGE_FILE_CALLBACKW = Option<unsafe extern "system" fn(pcontext: *mut core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: windows_core::PCWSTR) -> super::super::Foundation::BOOL>;
