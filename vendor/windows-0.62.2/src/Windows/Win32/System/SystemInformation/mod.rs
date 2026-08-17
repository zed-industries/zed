#[inline]
pub unsafe fn DnsHostnameToComputerNameExW<P0>(hostname: P0, computername: Option<windows_core::PWSTR>, nsize: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameExW(hostname : windows_core::PCWSTR, computername : windows_core::PWSTR, nsize : *mut u32) -> windows_core::BOOL);
    unsafe { DnsHostnameToComputerNameExW(hostname.param().abi(), computername.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[inline]
pub unsafe fn EnumSystemFirmwareTables(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, pfirmwaretableenumbuffer: Option<&mut [u8]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn EnumSystemFirmwareTables(firmwaretableprovidersignature : FIRMWARE_TABLE_PROVIDER, pfirmwaretableenumbuffer : *mut u8, buffersize : u32) -> u32);
    unsafe { EnumSystemFirmwareTables(firmwaretableprovidersignature, core::mem::transmute(pfirmwaretableenumbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pfirmwaretableenumbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: Option<windows_core::PSTR>, nsize: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetComputerNameExA(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_core::PSTR, nsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetComputerNameExA(nametype, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _).ok() }
}
#[inline]
pub unsafe fn GetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: Option<windows_core::PWSTR>, nsize: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetComputerNameExW(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_core::PWSTR, nsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetComputerNameExW(nametype, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _).ok() }
}
#[inline]
pub unsafe fn GetDeveloperDriveEnablementState() -> DEVELOPER_DRIVE_ENABLEMENT_STATE {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-6.dll" "system" fn GetDeveloperDriveEnablementState() -> DEVELOPER_DRIVE_ENABLEMENT_STATE);
    unsafe { GetDeveloperDriveEnablementState() }
}
#[inline]
pub unsafe fn GetFirmwareType(firmwaretype: *mut FIRMWARE_TYPE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetFirmwareType(firmwaretype : *mut FIRMWARE_TYPE) -> windows_core::BOOL);
    unsafe { GetFirmwareType(firmwaretype as _).ok() }
}
#[inline]
pub unsafe fn GetIntegratedDisplaySize() -> windows_core::Result<f64> {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-3.dll" "system" fn GetIntegratedDisplaySize(sizeininches : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetIntegratedDisplaySize(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetLocalTime() -> super::super::Foundation::SYSTEMTIME {
    windows_core::link!("kernel32.dll" "system" fn GetLocalTime(lpsystemtime : *mut super::super::Foundation:: SYSTEMTIME));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetLocalTime(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn GetLogicalProcessorInformation(buffer: Option<*mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION>, returnedlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetLogicalProcessorInformation(buffer : *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION, returnedlength : *mut u32) -> windows_core::BOOL);
    unsafe { GetLogicalProcessorInformation(buffer.unwrap_or(core::mem::zeroed()) as _, returnedlength as _).ok() }
}
#[inline]
pub unsafe fn GetLogicalProcessorInformationEx(relationshiptype: LOGICAL_PROCESSOR_RELATIONSHIP, buffer: Option<*mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX>, returnedlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetLogicalProcessorInformationEx(relationshiptype : LOGICAL_PROCESSOR_RELATIONSHIP, buffer : *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, returnedlength : *mut u32) -> windows_core::BOOL);
    unsafe { GetLogicalProcessorInformationEx(relationshiptype, buffer.unwrap_or(core::mem::zeroed()) as _, returnedlength as _).ok() }
}
#[inline]
pub unsafe fn GetNativeSystemInfo(lpsysteminfo: *mut SYSTEM_INFO) {
    windows_core::link!("kernel32.dll" "system" fn GetNativeSystemInfo(lpsysteminfo : *mut SYSTEM_INFO));
    unsafe { GetNativeSystemInfo(lpsysteminfo as _) }
}
#[inline]
pub unsafe fn GetOsManufacturingMode(pbenabled: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-3.dll" "system" fn GetOsManufacturingMode(pbenabled : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetOsManufacturingMode(pbenabled as _) }
}
#[inline]
pub unsafe fn GetOsSafeBootMode(flags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-0.dll" "system" fn GetOsSafeBootMode(flags : *mut u32) -> windows_core::BOOL);
    unsafe { GetOsSafeBootMode(flags as _) }
}
#[inline]
pub unsafe fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes: *mut u64) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes : *mut u64) -> windows_core::BOOL);
    unsafe { GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes as _).ok() }
}
#[inline]
pub unsafe fn GetProcessorSystemCycleTime(group: u16, buffer: Option<*mut SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION>, returnedlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetProcessorSystemCycleTime(group : u16, buffer : *mut SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, returnedlength : *mut u32) -> windows_core::BOOL);
    unsafe { GetProcessorSystemCycleTime(group, buffer.unwrap_or(core::mem::zeroed()) as _, returnedlength as _).ok() }
}
#[inline]
pub unsafe fn GetProductInfo(dwosmajorversion: u32, dwosminorversion: u32, dwspmajorversion: u32, dwspminorversion: u32, pdwreturnedproducttype: *mut OS_PRODUCT_TYPE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProductInfo(dwosmajorversion : u32, dwosminorversion : u32, dwspmajorversion : u32, dwspminorversion : u32, pdwreturnedproducttype : *mut OS_PRODUCT_TYPE) -> windows_core::BOOL);
    unsafe { GetProductInfo(dwosmajorversion, dwosminorversion, dwspmajorversion, dwspminorversion, pdwreturnedproducttype as _) }
}
#[inline]
pub unsafe fn GetSystemCpuSetInformation(information: Option<*mut SYSTEM_CPU_SET_INFORMATION>, bufferlength: u32, returnedlength: *mut u32, process: Option<super::super::Foundation::HANDLE>, flags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetSystemCpuSetInformation(information : *mut SYSTEM_CPU_SET_INFORMATION, bufferlength : u32, returnedlength : *mut u32, process : super::super::Foundation:: HANDLE, flags : u32) -> windows_core::BOOL);
    unsafe { GetSystemCpuSetInformation(information.unwrap_or(core::mem::zeroed()) as _, bufferlength, returnedlength as _, process.unwrap_or(core::mem::zeroed()) as _, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE {
    windows_core::link!("kernel32.dll" "system" fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE);
    unsafe { GetSystemDEPPolicy() }
}
#[inline]
pub unsafe fn GetSystemDirectoryA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemDirectoryA(lpbuffer : windows_core::PSTR, usize : u32) -> u32);
    unsafe { GetSystemDirectoryA(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetSystemDirectoryW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemDirectoryW(lpbuffer : windows_core::PWSTR, usize : u32) -> u32);
    unsafe { GetSystemDirectoryW(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetSystemFirmwareTable(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, firmwaretableid: u32, pfirmwaretablebuffer: Option<&mut [u8]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemFirmwareTable(firmwaretableprovidersignature : FIRMWARE_TABLE_PROVIDER, firmwaretableid : u32, pfirmwaretablebuffer : *mut u8, buffersize : u32) -> u32);
    unsafe { GetSystemFirmwareTable(firmwaretableprovidersignature, firmwaretableid, core::mem::transmute(pfirmwaretablebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pfirmwaretablebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetSystemInfo(lpsysteminfo: *mut SYSTEM_INFO) {
    windows_core::link!("kernel32.dll" "system" fn GetSystemInfo(lpsysteminfo : *mut SYSTEM_INFO));
    unsafe { GetSystemInfo(lpsysteminfo as _) }
}
#[inline]
pub unsafe fn GetSystemLeapSecondInformation(enabled: *mut windows_core::BOOL, flags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetSystemLeapSecondInformation(enabled : *mut windows_core::BOOL, flags : *mut u32) -> windows_core::BOOL);
    unsafe { GetSystemLeapSecondInformation(enabled as _, flags as _) }
}
#[inline]
pub unsafe fn GetSystemTime() -> super::super::Foundation::SYSTEMTIME {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTime(lpsystemtime : *mut super::super::Foundation:: SYSTEMTIME));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetSystemTime(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn GetSystemTimeAdjustment(lptimeadjustment: *mut u32, lptimeincrement: *mut u32, lptimeadjustmentdisabled: *mut windows_core::BOOL) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTimeAdjustment(lptimeadjustment : *mut u32, lptimeincrement : *mut u32, lptimeadjustmentdisabled : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetSystemTimeAdjustment(lptimeadjustment as _, lptimeincrement as _, lptimeadjustmentdisabled as _).ok() }
}
#[inline]
pub unsafe fn GetSystemTimeAdjustmentPrecise(lptimeadjustment: *mut u64, lptimeincrement: *mut u64, lptimeadjustmentdisabled: *mut windows_core::BOOL) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-4.dll" "system" fn GetSystemTimeAdjustmentPrecise(lptimeadjustment : *mut u64, lptimeincrement : *mut u64, lptimeadjustmentdisabled : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetSystemTimeAdjustmentPrecise(lptimeadjustment as _, lptimeincrement as _, lptimeadjustmentdisabled as _).ok() }
}
#[inline]
pub unsafe fn GetSystemTimeAsFileTime() -> super::super::Foundation::FILETIME {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime : *mut super::super::Foundation:: FILETIME));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetSystemTimeAsFileTime(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn GetSystemTimePreciseAsFileTime() -> super::super::Foundation::FILETIME {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime : *mut super::super::Foundation:: FILETIME));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetSystemTimePreciseAsFileTime(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn GetSystemWindowsDirectoryA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemWindowsDirectoryA(lpbuffer : windows_core::PSTR, usize : u32) -> u32);
    unsafe { GetSystemWindowsDirectoryA(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetSystemWindowsDirectoryW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemWindowsDirectoryW(lpbuffer : windows_core::PWSTR, usize : u32) -> u32);
    unsafe { GetSystemWindowsDirectoryW(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetSystemWow64Directory2A(lpbuffer: Option<&mut [u8]>, imagefilemachinetype: IMAGE_FILE_MACHINE) -> u32 {
    windows_core::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn GetSystemWow64Directory2A(lpbuffer : windows_core::PSTR, usize : u32, imagefilemachinetype : IMAGE_FILE_MACHINE) -> u32);
    unsafe { GetSystemWow64Directory2A(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), imagefilemachinetype) }
}
#[inline]
pub unsafe fn GetSystemWow64Directory2W(lpbuffer: Option<&mut [u16]>, imagefilemachinetype: IMAGE_FILE_MACHINE) -> u32 {
    windows_core::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn GetSystemWow64Directory2W(lpbuffer : windows_core::PWSTR, usize : u32, imagefilemachinetype : IMAGE_FILE_MACHINE) -> u32);
    unsafe { GetSystemWow64Directory2W(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), imagefilemachinetype) }
}
#[inline]
pub unsafe fn GetSystemWow64DirectoryA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemWow64DirectoryA(lpbuffer : windows_core::PSTR, usize : u32) -> u32);
    unsafe { GetSystemWow64DirectoryA(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetSystemWow64DirectoryW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemWow64DirectoryW(lpbuffer : windows_core::PWSTR, usize : u32) -> u32);
    unsafe { GetSystemWow64DirectoryW(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetTickCount() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
    unsafe { GetTickCount() }
}
#[inline]
pub unsafe fn GetTickCount64() -> u64 {
    windows_core::link!("kernel32.dll" "system" fn GetTickCount64() -> u64);
    unsafe { GetTickCount64() }
}
#[inline]
pub unsafe fn GetVersion() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetVersion() -> u32);
    unsafe { GetVersion() }
}
#[inline]
pub unsafe fn GetVersionExA(lpversioninformation: *mut OSVERSIONINFOA) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetVersionExA(lpversioninformation : *mut OSVERSIONINFOA) -> windows_core::BOOL);
    unsafe { GetVersionExA(lpversioninformation as _).ok() }
}
#[inline]
pub unsafe fn GetVersionExW(lpversioninformation: *mut OSVERSIONINFOW) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetVersionExW(lpversioninformation : *mut OSVERSIONINFOW) -> windows_core::BOOL);
    unsafe { GetVersionExW(lpversioninformation as _).ok() }
}
#[inline]
pub unsafe fn GetWindowsDirectoryA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetWindowsDirectoryA(lpbuffer : windows_core::PSTR, usize : u32) -> u32);
    unsafe { GetWindowsDirectoryA(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetWindowsDirectoryW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetWindowsDirectoryW(lpbuffer : windows_core::PWSTR, usize : u32) -> u32);
    unsafe { GetWindowsDirectoryW(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GlobalMemoryStatus(lpbuffer: *mut MEMORYSTATUS) {
    windows_core::link!("kernel32.dll" "system" fn GlobalMemoryStatus(lpbuffer : *mut MEMORYSTATUS));
    unsafe { GlobalMemoryStatus(lpbuffer as _) }
}
#[inline]
pub unsafe fn GlobalMemoryStatusEx(lpbuffer: *mut MEMORYSTATUSEX) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GlobalMemoryStatusEx(lpbuffer : *mut MEMORYSTATUSEX) -> windows_core::BOOL);
    unsafe { GlobalMemoryStatusEx(lpbuffer as _).ok() }
}
#[inline]
pub unsafe fn IsUserCetAvailableInEnvironment(usercetenvironment: USER_CET_ENVIRONMENT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsUserCetAvailableInEnvironment(usercetenvironment : USER_CET_ENVIRONMENT) -> windows_core::BOOL);
    unsafe { IsUserCetAvailableInEnvironment(usercetenvironment) }
}
#[inline]
pub unsafe fn IsWow64GuestMachineSupported(wowguestmachine: IMAGE_FILE_MACHINE) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("kernel32.dll" "system" fn IsWow64GuestMachineSupported(wowguestmachine : IMAGE_FILE_MACHINE, machineissupported : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IsWow64GuestMachineSupported(wowguestmachine, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize: *mut u32, puldeviceformbuffersize: *mut u32, devicefamily: windows_core::PWSTR, deviceform: windows_core::PWSTR) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize : *mut u32, puldeviceformbuffersize : *mut u32, devicefamily : windows_core::PWSTR, deviceform : windows_core::PWSTR) -> u32);
    unsafe { RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize as _, puldeviceformbuffersize as _, core::mem::transmute(devicefamily), core::mem::transmute(deviceform)) }
}
#[inline]
pub unsafe fn RtlGetDeviceFamilyInfoEnum(pulluapinfo: Option<*mut u64>, puldevicefamily: Option<*mut DEVICEFAMILYINFOENUM>, puldeviceform: Option<*mut DEVICEFAMILYDEVICEFORM>) {
    windows_core::link!("ntdll.dll" "system" fn RtlGetDeviceFamilyInfoEnum(pulluapinfo : *mut u64, puldevicefamily : *mut DEVICEFAMILYINFOENUM, puldeviceform : *mut DEVICEFAMILYDEVICEFORM));
    unsafe { RtlGetDeviceFamilyInfoEnum(pulluapinfo.unwrap_or(core::mem::zeroed()) as _, puldevicefamily.unwrap_or(core::mem::zeroed()) as _, puldeviceform.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RtlGetProductInfo(osmajorversion: u32, osminorversion: u32, spmajorversion: u32, spminorversion: u32, returnedproducttype: *mut u32) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlGetProductInfo(osmajorversion : u32, osminorversion : u32, spmajorversion : u32, spminorversion : u32, returnedproducttype : *mut u32) -> bool);
    unsafe { RtlGetProductInfo(osmajorversion, osminorversion, spmajorversion, spminorversion, returnedproducttype as _) }
}
#[inline]
pub unsafe fn RtlGetSystemGlobalData(dataid: RTL_SYSTEM_GLOBAL_DATA_ID, buffer: *mut core::ffi::c_void, size: u32) -> u32 {
    windows_core::link!("ntdllk.dll" "system" fn RtlGetSystemGlobalData(dataid : RTL_SYSTEM_GLOBAL_DATA_ID, buffer : *mut core::ffi::c_void, size : u32) -> u32);
    unsafe { RtlGetSystemGlobalData(dataid, buffer as _, size) }
}
#[inline]
pub unsafe fn RtlOsDeploymentState(flags: u32) -> OS_DEPLOYEMENT_STATE_VALUES {
    windows_core::link!("ntdll.dll" "system" fn RtlOsDeploymentState(flags : u32) -> OS_DEPLOYEMENT_STATE_VALUES);
    unsafe { RtlOsDeploymentState(flags) }
}
#[inline]
pub unsafe fn RtlSwitchedVVI(versioninfo: *const OSVERSIONINFOEXW, typemask: u32, conditionmask: u64) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlSwitchedVVI(versioninfo : *const OSVERSIONINFOEXW, typemask : u32, conditionmask : u64) -> u32);
    unsafe { RtlSwitchedVVI(versioninfo, typemask, conditionmask) }
}
#[inline]
pub unsafe fn SetComputerNameA<P0>(lpcomputername: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetComputerNameA(lpcomputername : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetComputerNameA(lpcomputername.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetComputerNameEx2W<P2>(nametype: COMPUTER_NAME_FORMAT, flags: u32, lpbuffer: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetComputerNameEx2W(nametype : COMPUTER_NAME_FORMAT, flags : u32, lpbuffer : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetComputerNameEx2W(nametype, flags, lpbuffer.param().abi()) }
}
#[inline]
pub unsafe fn SetComputerNameExA<P1>(nametype: COMPUTER_NAME_FORMAT, lpbuffer: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetComputerNameExA(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetComputerNameExA(nametype, lpbuffer.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetComputerNameExW<P1>(nametype: COMPUTER_NAME_FORMAT, lpbuffer: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetComputerNameExW(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetComputerNameExW(nametype, lpbuffer.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetComputerNameW<P0>(lpcomputername: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetComputerNameW(lpcomputername : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetComputerNameW(lpcomputername.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetLocalTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetLocalTime(lpsystemtime : *const super::super::Foundation:: SYSTEMTIME) -> windows_core::BOOL);
    unsafe { SetLocalTime(lpsystemtime).ok() }
}
#[inline]
pub unsafe fn SetSystemTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetSystemTime(lpsystemtime : *const super::super::Foundation:: SYSTEMTIME) -> windows_core::BOOL);
    unsafe { SetSystemTime(lpsystemtime).ok() }
}
#[inline]
pub unsafe fn SetSystemTimeAdjustment(dwtimeadjustment: u32, btimeadjustmentdisabled: bool) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetSystemTimeAdjustment(dwtimeadjustment : u32, btimeadjustmentdisabled : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetSystemTimeAdjustment(dwtimeadjustment, btimeadjustmentdisabled.into()).ok() }
}
#[inline]
pub unsafe fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment: u64, btimeadjustmentdisabled: bool) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-4.dll" "system" fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment : u64, btimeadjustmentdisabled : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetSystemTimeAdjustmentPrecise(dwtimeadjustment, btimeadjustmentdisabled.into()).ok() }
}
#[inline]
pub unsafe fn VerSetConditionMask(conditionmask: u64, typemask: VER_FLAGS, condition: u8) -> u64 {
    windows_core::link!("kernel32.dll" "system" fn VerSetConditionMask(conditionmask : u64, typemask : VER_FLAGS, condition : u8) -> u64);
    unsafe { VerSetConditionMask(conditionmask, typemask, condition) }
}
#[inline]
pub unsafe fn VerifyVersionInfoA(lpversioninformation: *mut OSVERSIONINFOEXA, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn VerifyVersionInfoA(lpversioninformation : *mut OSVERSIONINFOEXA, dwtypemask : VER_FLAGS, dwlconditionmask : u64) -> windows_core::BOOL);
    unsafe { VerifyVersionInfoA(lpversioninformation as _, dwtypemask, dwlconditionmask).ok() }
}
#[inline]
pub unsafe fn VerifyVersionInfoW(lpversioninformation: *mut OSVERSIONINFOEXW, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn VerifyVersionInfoW(lpversioninformation : *mut OSVERSIONINFOEXW, dwtypemask : VER_FLAGS, dwlconditionmask : u64) -> windows_core::BOOL);
    unsafe { VerifyVersionInfoW(lpversioninformation as _, dwtypemask, dwlconditionmask).ok() }
}
pub const ACPI: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1094930505u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CACHE_DESCRIPTOR {
    pub Level: u8,
    pub Associativity: u8,
    pub LineSize: u16,
    pub Size: u32,
    pub Type: PROCESSOR_CACHE_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CACHE_RELATIONSHIP {
    pub Level: u8,
    pub Associativity: u8,
    pub LineSize: u16,
    pub CacheSize: u32,
    pub Type: PROCESSOR_CACHE_TYPE,
    pub Reserved: [u8; 18],
    pub GroupCount: u16,
    pub Anonymous: CACHE_RELATIONSHIP_0,
}
impl Default for CACHE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CACHE_RELATIONSHIP_0 {
    pub GroupMask: GROUP_AFFINITY,
    pub GroupMasks: [GROUP_AFFINITY; 1],
}
impl Default for CACHE_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMPUTER_NAME_FORMAT(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CPU_SET_INFORMATION_TYPE(pub i32);
pub const CacheData: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(2i32);
pub const CacheInstruction: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(1i32);
pub const CacheTrace: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(3i32);
pub const CacheUnified: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(0i32);
pub const ComputerNameDnsDomain: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(2i32);
pub const ComputerNameDnsFullyQualified: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(3i32);
pub const ComputerNameDnsHostname: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(1i32);
pub const ComputerNameMax: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(8i32);
pub const ComputerNameNetBIOS: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(0i32);
pub const ComputerNamePhysicalDnsDomain: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(6i32);
pub const ComputerNamePhysicalDnsFullyQualified: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(7i32);
pub const ComputerNamePhysicalDnsHostname: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(5i32);
pub const ComputerNamePhysicalNetBIOS: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(4i32);
pub const CpuSetInformation: CPU_SET_INFORMATION_TYPE = CPU_SET_INFORMATION_TYPE(0i32);
pub const DEPPolicyAlwaysOff: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(0i32);
pub const DEPPolicyAlwaysOn: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(1i32);
pub const DEPPolicyOptIn: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(2i32);
pub const DEPPolicyOptOut: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(3i32);
pub const DEPTotalPolicyCount: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEP_SYSTEM_POLICY_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVELOPER_DRIVE_ENABLEMENT_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICEFAMILYDEVICEFORM(pub u32);
pub const DEVICEFAMILYDEVICEFORM_ALLINONE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(7u32);
pub const DEVICEFAMILYDEVICEFORM_BANKING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(14u32);
pub const DEVICEFAMILYDEVICEFORM_BUILDING_AUTOMATION: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(15u32);
pub const DEVICEFAMILYDEVICEFORM_CONVERTIBLE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(5u32);
pub const DEVICEFAMILYDEVICEFORM_DESKTOP: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(3u32);
pub const DEVICEFAMILYDEVICEFORM_DETACHABLE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(6u32);
pub const DEVICEFAMILYDEVICEFORM_DIGITAL_SIGNAGE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(16u32);
pub const DEVICEFAMILYDEVICEFORM_GAMING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(17u32);
pub const DEVICEFAMILYDEVICEFORM_HMD: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(11u32);
pub const DEVICEFAMILYDEVICEFORM_HOME_AUTOMATION: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(18u32);
pub const DEVICEFAMILYDEVICEFORM_INDUSTRIAL_AUTOMATION: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(19u32);
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_HANDHELD: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(12u32);
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_OTHER: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(29u32);
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_TABLET: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(13u32);
pub const DEVICEFAMILYDEVICEFORM_KIOSK: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(20u32);
pub const DEVICEFAMILYDEVICEFORM_LARGESCREEN: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(10u32);
pub const DEVICEFAMILYDEVICEFORM_MAKER_BOARD: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(21u32);
pub const DEVICEFAMILYDEVICEFORM_MAX: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(45u32);
pub const DEVICEFAMILYDEVICEFORM_MEDICAL: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(22u32);
pub const DEVICEFAMILYDEVICEFORM_NETWORKING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(23u32);
pub const DEVICEFAMILYDEVICEFORM_NOTEBOOK: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(4u32);
pub const DEVICEFAMILYDEVICEFORM_PHONE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(1u32);
pub const DEVICEFAMILYDEVICEFORM_POINT_OF_SERVICE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(24u32);
pub const DEVICEFAMILYDEVICEFORM_PRINTING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(25u32);
pub const DEVICEFAMILYDEVICEFORM_PUCK: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(9u32);
pub const DEVICEFAMILYDEVICEFORM_STICKPC: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(8u32);
pub const DEVICEFAMILYDEVICEFORM_TABLET: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(2u32);
pub const DEVICEFAMILYDEVICEFORM_THIN_CLIENT: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(26u32);
pub const DEVICEFAMILYDEVICEFORM_TOY: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(27u32);
pub const DEVICEFAMILYDEVICEFORM_UNKNOWN: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(0u32);
pub const DEVICEFAMILYDEVICEFORM_VENDING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(28u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(30u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_S: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(31u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(32u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X_DEVKIT: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(33u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_01: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(37u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_02: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(38u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_03: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(39u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_04: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(40u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_05: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(41u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_06: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(42u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_07: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(43u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_08: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(44u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_09: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(45u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_S: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(36u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(34u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X_DEVKIT: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(35u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICEFAMILYINFOENUM(pub u32);
pub const DEVICEFAMILYINFOENUM_7067329: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(15u32);
pub const DEVICEFAMILYINFOENUM_8828080: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(14u32);
pub const DEVICEFAMILYINFOENUM_DESKTOP: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(3u32);
pub const DEVICEFAMILYINFOENUM_HOLOGRAPHIC: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(10u32);
pub const DEVICEFAMILYINFOENUM_IOT: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(7u32);
pub const DEVICEFAMILYINFOENUM_IOT_HEADLESS: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(8u32);
pub const DEVICEFAMILYINFOENUM_MAX: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(17u32);
pub const DEVICEFAMILYINFOENUM_MOBILE: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(4u32);
pub const DEVICEFAMILYINFOENUM_SERVER: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(9u32);
pub const DEVICEFAMILYINFOENUM_SERVER_NANO: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(13u32);
pub const DEVICEFAMILYINFOENUM_TEAM: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(6u32);
pub const DEVICEFAMILYINFOENUM_UAP: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(0u32);
pub const DEVICEFAMILYINFOENUM_WINDOWS_8X: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(1u32);
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(16u32);
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE_HEADLESS: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(17u32);
pub const DEVICEFAMILYINFOENUM_WINDOWS_PHONE_8X: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(2u32);
pub const DEVICEFAMILYINFOENUM_XBOX: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(5u32);
pub const DEVICEFAMILYINFOENUM_XBOXERA: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(12u32);
pub const DEVICEFAMILYINFOENUM_XBOXSRA: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(11u32);
pub const DeveloperDriveDisabledByGroupPolicy: DEVELOPER_DRIVE_ENABLEMENT_STATE = DEVELOPER_DRIVE_ENABLEMENT_STATE(3i32);
pub const DeveloperDriveDisabledBySystemPolicy: DEVELOPER_DRIVE_ENABLEMENT_STATE = DEVELOPER_DRIVE_ENABLEMENT_STATE(2i32);
pub const DeveloperDriveEnabled: DEVELOPER_DRIVE_ENABLEMENT_STATE = DEVELOPER_DRIVE_ENABLEMENT_STATE(1i32);
pub const DeveloperDriveEnablementStateError: DEVELOPER_DRIVE_ENABLEMENT_STATE = DEVELOPER_DRIVE_ENABLEMENT_STATE(0i32);
pub const FIRM: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1179210317u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FIRMWARE_TABLE_PROVIDER(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FIRMWARE_TYPE(pub i32);
pub const FirmwareTypeBios: FIRMWARE_TYPE = FIRMWARE_TYPE(1i32);
pub const FirmwareTypeMax: FIRMWARE_TYPE = FIRMWARE_TYPE(3i32);
pub const FirmwareTypeUefi: FIRMWARE_TYPE = FIRMWARE_TYPE(2i32);
pub const FirmwareTypeUnknown: FIRMWARE_TYPE = FIRMWARE_TYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_AFFINITY {
    pub Mask: usize,
    pub Group: u16,
    pub Reserved: [u16; 3],
}
impl Default for GROUP_AFFINITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_RELATIONSHIP {
    pub MaximumGroupCount: u16,
    pub ActiveGroupCount: u16,
    pub Reserved: [u8; 20],
    pub GroupInfo: [PROCESSOR_GROUP_INFO; 1],
}
impl Default for GROUP_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GlobalDataIdConsoleSharedDataFlags: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(14i32);
pub const GlobalDataIdCyclesPerYield: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(11i32);
pub const GlobalDataIdImageNumberHigh: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(5i32);
pub const GlobalDataIdImageNumberLow: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(4i32);
pub const GlobalDataIdInterruptTime: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(2i32);
pub const GlobalDataIdKdDebuggerEnabled: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(10i32);
pub const GlobalDataIdLastSystemRITEventTickCount: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(13i32);
pub const GlobalDataIdNtMajorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(7i32);
pub const GlobalDataIdNtMinorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(8i32);
pub const GlobalDataIdNtSystemRootDrive: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(15i32);
pub const GlobalDataIdQpcBias: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(19i32);
pub const GlobalDataIdQpcBypassEnabled: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(17i32);
pub const GlobalDataIdQpcData: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(18i32);
pub const GlobalDataIdQpcShift: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(16i32);
pub const GlobalDataIdRngSeedVersion: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(1i32);
pub const GlobalDataIdSafeBootMode: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(12i32);
pub const GlobalDataIdSystemExpirationDate: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(9i32);
pub const GlobalDataIdTimeZoneBias: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(3i32);
pub const GlobalDataIdTimeZoneId: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(6i32);
pub const GlobalDataIdUnknown: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGE_FILE_MACHINE(pub u16);
pub const IMAGE_FILE_MACHINE_ALPHA: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(388u16);
pub const IMAGE_FILE_MACHINE_ALPHA64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(644u16);
pub const IMAGE_FILE_MACHINE_AM33: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(467u16);
pub const IMAGE_FILE_MACHINE_AMD64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(34404u16);
pub const IMAGE_FILE_MACHINE_ARM: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(448u16);
pub const IMAGE_FILE_MACHINE_ARM64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(43620u16);
pub const IMAGE_FILE_MACHINE_ARMNT: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(452u16);
pub const IMAGE_FILE_MACHINE_AXP64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(644u16);
pub const IMAGE_FILE_MACHINE_CEE: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(49390u16);
pub const IMAGE_FILE_MACHINE_CEF: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(3311u16);
pub const IMAGE_FILE_MACHINE_EBC: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(3772u16);
pub const IMAGE_FILE_MACHINE_I386: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(332u16);
pub const IMAGE_FILE_MACHINE_IA64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(512u16);
pub const IMAGE_FILE_MACHINE_M32R: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(36929u16);
pub const IMAGE_FILE_MACHINE_MIPS16: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(614u16);
pub const IMAGE_FILE_MACHINE_MIPSFPU: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(870u16);
pub const IMAGE_FILE_MACHINE_MIPSFPU16: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(1126u16);
pub const IMAGE_FILE_MACHINE_POWERPC: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(496u16);
pub const IMAGE_FILE_MACHINE_POWERPCFP: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(497u16);
pub const IMAGE_FILE_MACHINE_R10000: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(360u16);
pub const IMAGE_FILE_MACHINE_R3000: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(354u16);
pub const IMAGE_FILE_MACHINE_R4000: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(358u16);
pub const IMAGE_FILE_MACHINE_SH3: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(418u16);
pub const IMAGE_FILE_MACHINE_SH3DSP: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(419u16);
pub const IMAGE_FILE_MACHINE_SH3E: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(420u16);
pub const IMAGE_FILE_MACHINE_SH4: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(422u16);
pub const IMAGE_FILE_MACHINE_SH5: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(424u16);
pub const IMAGE_FILE_MACHINE_TARGET_HOST: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(1u16);
pub const IMAGE_FILE_MACHINE_THUMB: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(450u16);
pub const IMAGE_FILE_MACHINE_TRICORE: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(1312u16);
pub const IMAGE_FILE_MACHINE_UNKNOWN: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(0u16);
pub const IMAGE_FILE_MACHINE_WCEMIPSV2: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(361u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOGICAL_PROCESSOR_RELATIONSHIP(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEMORYSTATUS {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub dwTotalPhys: usize,
    pub dwAvailPhys: usize,
    pub dwTotalPageFile: usize,
    pub dwAvailPageFile: usize,
    pub dwTotalVirtual: usize,
    pub dwAvailVirtual: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEMORYSTATUSEX {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub ullTotalPhys: u64,
    pub ullAvailPhys: u64,
    pub ullTotalPageFile: u64,
    pub ullAvailPageFile: u64,
    pub ullTotalVirtual: u64,
    pub ullAvailVirtual: u64,
    pub ullAvailExtendedVirtual: u64,
}
pub const NTDDI_LONGHORN: u32 = 100663296u32;
pub const NTDDI_VERSION: u32 = 167772172u32;
pub const NTDDI_VISTA: u32 = 100663296u32;
pub const NTDDI_VISTASP1: u32 = 100663552u32;
pub const NTDDI_VISTASP2: u32 = 100663808u32;
pub const NTDDI_VISTASP3: u32 = 100664064u32;
pub const NTDDI_VISTASP4: u32 = 100664320u32;
pub const NTDDI_WIN10: u32 = 167772160u32;
pub const NTDDI_WIN10_19H1: u32 = 167772167u32;
pub const NTDDI_WIN10_CO: u32 = 167772171u32;
pub const NTDDI_WIN10_FE: u32 = 167772170u32;
pub const NTDDI_WIN10_MN: u32 = 167772169u32;
pub const NTDDI_WIN10_NI: u32 = 167772172u32;
pub const NTDDI_WIN10_RS1: u32 = 167772162u32;
pub const NTDDI_WIN10_RS2: u32 = 167772163u32;
pub const NTDDI_WIN10_RS3: u32 = 167772164u32;
pub const NTDDI_WIN10_RS4: u32 = 167772165u32;
pub const NTDDI_WIN10_RS5: u32 = 167772166u32;
pub const NTDDI_WIN10_TH2: u32 = 167772161u32;
pub const NTDDI_WIN10_VB: u32 = 167772168u32;
pub const NTDDI_WIN2K: u32 = 83886080u32;
pub const NTDDI_WIN2KSP1: u32 = 83886336u32;
pub const NTDDI_WIN2KSP2: u32 = 83886592u32;
pub const NTDDI_WIN2KSP3: u32 = 83886848u32;
pub const NTDDI_WIN2KSP4: u32 = 83887104u32;
pub const NTDDI_WIN4: u32 = 67108864u32;
pub const NTDDI_WIN6: u32 = 100663296u32;
pub const NTDDI_WIN6SP1: u32 = 100663552u32;
pub const NTDDI_WIN6SP2: u32 = 100663808u32;
pub const NTDDI_WIN6SP3: u32 = 100664064u32;
pub const NTDDI_WIN6SP4: u32 = 100664320u32;
pub const NTDDI_WIN7: u32 = 100728832u32;
pub const NTDDI_WIN8: u32 = 100794368u32;
pub const NTDDI_WINBLUE: u32 = 100859904u32;
pub const NTDDI_WINTHRESHOLD: u32 = 167772160u32;
pub const NTDDI_WINXP: u32 = 83951616u32;
pub const NTDDI_WINXPSP1: u32 = 83951872u32;
pub const NTDDI_WINXPSP2: u32 = 83952128u32;
pub const NTDDI_WINXPSP3: u32 = 83952384u32;
pub const NTDDI_WINXPSP4: u32 = 83952640u32;
pub const NTDDI_WS03: u32 = 84017152u32;
pub const NTDDI_WS03SP1: u32 = 84017408u32;
pub const NTDDI_WS03SP2: u32 = 84017664u32;
pub const NTDDI_WS03SP3: u32 = 84017920u32;
pub const NTDDI_WS03SP4: u32 = 84018176u32;
pub const NTDDI_WS08: u32 = 100663552u32;
pub const NTDDI_WS08SP2: u32 = 100663808u32;
pub const NTDDI_WS08SP3: u32 = 100664064u32;
pub const NTDDI_WS08SP4: u32 = 100664320u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NUMA_NODE_RELATIONSHIP {
    pub NodeNumber: u32,
    pub Reserved: [u8; 18],
    pub GroupCount: u16,
    pub Anonymous: NUMA_NODE_RELATIONSHIP_0,
}
impl Default for NUMA_NODE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NUMA_NODE_RELATIONSHIP_0 {
    pub GroupMask: GROUP_AFFINITY,
    pub GroupMasks: [GROUP_AFFINITY; 1],
}
impl Default for NUMA_NODE_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OSVERSIONINFOA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [i8; 128],
}
impl Default for OSVERSIONINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OSVERSIONINFOEXA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [i8; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
impl Default for OSVERSIONINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const OSVERSION_MASK: u32 = 4294901760u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OS_DEPLOYEMENT_STATE_VALUES(pub i32);
pub const OS_DEPLOYMENT_COMPACT: OS_DEPLOYEMENT_STATE_VALUES = OS_DEPLOYEMENT_STATE_VALUES(2i32);
pub const OS_DEPLOYMENT_STANDARD: OS_DEPLOYEMENT_STATE_VALUES = OS_DEPLOYEMENT_STATE_VALUES(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OS_PRODUCT_TYPE(pub u32);
pub type PGET_SYSTEM_WOW64_DIRECTORY_A = Option<unsafe extern "system" fn(lpbuffer: windows_core::PSTR, usize: u32) -> u32>;
pub type PGET_SYSTEM_WOW64_DIRECTORY_W = Option<unsafe extern "system" fn(lpbuffer: windows_core::PWSTR, usize: u32) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESSOR_ARCHITECTURE(pub u16);
pub const PROCESSOR_ARCHITECTURE_ALPHA: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(2u16);
pub const PROCESSOR_ARCHITECTURE_ALPHA64: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(7u16);
pub const PROCESSOR_ARCHITECTURE_AMD64: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(9u16);
pub const PROCESSOR_ARCHITECTURE_ARM: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(5u16);
pub const PROCESSOR_ARCHITECTURE_ARM32_ON_WIN64: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(13u16);
pub const PROCESSOR_ARCHITECTURE_ARM64: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(12u16);
pub const PROCESSOR_ARCHITECTURE_IA32_ON_ARM64: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(14u16);
pub const PROCESSOR_ARCHITECTURE_IA32_ON_WIN64: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(10u16);
pub const PROCESSOR_ARCHITECTURE_IA64: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(6u16);
pub const PROCESSOR_ARCHITECTURE_INTEL: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(0u16);
pub const PROCESSOR_ARCHITECTURE_MIPS: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(1u16);
pub const PROCESSOR_ARCHITECTURE_MSIL: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(8u16);
pub const PROCESSOR_ARCHITECTURE_NEUTRAL: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(11u16);
pub const PROCESSOR_ARCHITECTURE_PPC: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(3u16);
pub const PROCESSOR_ARCHITECTURE_SHX: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(4u16);
pub const PROCESSOR_ARCHITECTURE_UNKNOWN: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(65535u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESSOR_CACHE_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESSOR_GROUP_INFO {
    pub MaximumProcessorCount: u8,
    pub ActiveProcessorCount: u8,
    pub Reserved: [u8; 38],
    pub ActiveProcessorMask: usize,
}
impl Default for PROCESSOR_GROUP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESSOR_RELATIONSHIP {
    pub Flags: u8,
    pub EfficiencyClass: u8,
    pub Reserved: [u8; 20],
    pub GroupCount: u16,
    pub GroupMask: [GROUP_AFFINITY; 1],
}
impl Default for PROCESSOR_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRODUCT_BUSINESS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(6u32);
pub const PRODUCT_BUSINESS_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(16u32);
pub const PRODUCT_CLUSTER_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(18u32);
pub const PRODUCT_CLUSTER_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(64u32);
pub const PRODUCT_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(101u32);
pub const PRODUCT_CORE_COUNTRYSPECIFIC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(99u32);
pub const PRODUCT_CORE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(98u32);
pub const PRODUCT_CORE_SINGLELANGUAGE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(100u32);
pub const PRODUCT_DATACENTER_A_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(145u32);
pub const PRODUCT_DATACENTER_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(80u32);
pub const PRODUCT_DATACENTER_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(8u32);
pub const PRODUCT_DATACENTER_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(12u32);
pub const PRODUCT_DATACENTER_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(39u32);
pub const PRODUCT_DATACENTER_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(37u32);
pub const PRODUCT_EDUCATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(121u32);
pub const PRODUCT_EDUCATION_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(122u32);
pub const PRODUCT_ENTERPRISE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(4u32);
pub const PRODUCT_ENTERPRISE_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(70u32);
pub const PRODUCT_ENTERPRISE_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(72u32);
pub const PRODUCT_ENTERPRISE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(27u32);
pub const PRODUCT_ENTERPRISE_N_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(84u32);
pub const PRODUCT_ENTERPRISE_S: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(125u32);
pub const PRODUCT_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(10u32);
pub const PRODUCT_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(14u32);
pub const PRODUCT_ENTERPRISE_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(41u32);
pub const PRODUCT_ENTERPRISE_SERVER_IA64: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(15u32);
pub const PRODUCT_ENTERPRISE_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(38u32);
pub const PRODUCT_ENTERPRISE_S_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(129u32);
pub const PRODUCT_ENTERPRISE_S_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(126u32);
pub const PRODUCT_ENTERPRISE_S_N_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(130u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(60u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(62u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(59u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(61u32);
pub const PRODUCT_HOME_BASIC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(2u32);
pub const PRODUCT_HOME_BASIC_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(67u32);
pub const PRODUCT_HOME_BASIC_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(5u32);
pub const PRODUCT_HOME_PREMIUM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(3u32);
pub const PRODUCT_HOME_PREMIUM_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(68u32);
pub const PRODUCT_HOME_PREMIUM_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(26u32);
pub const PRODUCT_HOME_PREMIUM_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(34u32);
pub const PRODUCT_HOME_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(19u32);
pub const PRODUCT_HYPERV: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(42u32);
pub const PRODUCT_IOTUAP: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(123u32);
pub const PRODUCT_IOTUAPCOMMERCIAL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(131u32);
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(30u32);
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(32u32);
pub const PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(31u32);
pub const PRODUCT_MOBILE_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(104u32);
pub const PRODUCT_MOBILE_ENTERPRISE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(133u32);
pub const PRODUCT_MULTIPOINT_PREMIUM_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(77u32);
pub const PRODUCT_MULTIPOINT_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(76u32);
pub const PRODUCT_PROFESSIONAL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(48u32);
pub const PRODUCT_PROFESSIONAL_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(69u32);
pub const PRODUCT_PROFESSIONAL_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(49u32);
pub const PRODUCT_PROFESSIONAL_WMC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(103u32);
pub const PRODUCT_PRO_WORKSTATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(161u32);
pub const PRODUCT_PRO_WORKSTATION_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(162u32);
pub const PRODUCT_SB_SOLUTION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(50u32);
pub const PRODUCT_SB_SOLUTION_SERVER_EM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(54u32);
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(51u32);
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(55u32);
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(24u32);
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(35u32);
pub const PRODUCT_SERVER_FOUNDATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(33u32);
pub const PRODUCT_SMALLBUSINESS_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(9u32);
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(25u32);
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(63u32);
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(56u32);
pub const PRODUCT_STANDARD_A_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(146u32);
pub const PRODUCT_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(79u32);
pub const PRODUCT_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(7u32);
pub const PRODUCT_STANDARD_SERVER_CORE_: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(13u32);
pub const PRODUCT_STANDARD_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(40u32);
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(52u32);
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(53u32);
pub const PRODUCT_STANDARD_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(36u32);
pub const PRODUCT_STARTER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(11u32);
pub const PRODUCT_STARTER_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(66u32);
pub const PRODUCT_STARTER_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(47u32);
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(23u32);
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(46u32);
pub const PRODUCT_STORAGE_EXPRESS_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(20u32);
pub const PRODUCT_STORAGE_EXPRESS_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(43u32);
pub const PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(96u32);
pub const PRODUCT_STORAGE_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(21u32);
pub const PRODUCT_STORAGE_STANDARD_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(44u32);
pub const PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(95u32);
pub const PRODUCT_STORAGE_WORKGROUP_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(22u32);
pub const PRODUCT_STORAGE_WORKGROUP_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(45u32);
pub const PRODUCT_ULTIMATE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(1u32);
pub const PRODUCT_ULTIMATE_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(71u32);
pub const PRODUCT_ULTIMATE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(28u32);
pub const PRODUCT_UNDEFINED: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(0u32);
pub const PRODUCT_WEB_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(17u32);
pub const PRODUCT_WEB_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(29u32);
pub const RSMB: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1381190978u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTL_SYSTEM_GLOBAL_DATA_ID(pub i32);
pub const RelationAll: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(65535i32);
pub const RelationCache: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(2i32);
pub const RelationGroup: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(4i32);
pub const RelationNumaNode: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(1i32);
pub const RelationNumaNodeEx: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(6i32);
pub const RelationProcessorCore: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(0i32);
pub const RelationProcessorDie: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(5i32);
pub const RelationProcessorModule: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(7i32);
pub const RelationProcessorPackage: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(3i32);
pub const SCEX2_ALT_NETBIOS_NAME: u32 = 1u32;
pub const SPVERSION_MASK: u32 = 65280u32;
pub const SUBVERSION_MASK: u32 = 255u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_CPU_SET_INFORMATION {
    pub Size: u32,
    pub Type: CPU_SET_INFORMATION_TYPE,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0,
}
impl Default for SYSTEM_CPU_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_CPU_SET_INFORMATION_0 {
    pub CpuSet: SYSTEM_CPU_SET_INFORMATION_0_0,
}
impl Default for SYSTEM_CPU_SET_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_CPU_SET_INFORMATION_0_0 {
    pub Id: u32,
    pub Group: u16,
    pub LogicalProcessorIndex: u8,
    pub CoreIndex: u8,
    pub LastLevelCacheIndex: u8,
    pub NumaNodeIndex: u8,
    pub EfficiencyClass: u8,
    pub Anonymous1: SYSTEM_CPU_SET_INFORMATION_0_0_0,
    pub Anonymous2: SYSTEM_CPU_SET_INFORMATION_0_0_1,
    pub AllocationTag: u64,
}
impl Default for SYSTEM_CPU_SET_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    pub AllFlags: u8,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0_0_0_0,
}
impl Default for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    pub Reserved: u32,
    pub SchedulingClass: u8,
}
impl Default for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED: u32 = 2u32;
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED_TO_TARGET_PROCESS: u32 = 4u32;
pub const SYSTEM_CPU_SET_INFORMATION_PARKED: u32 = 1u32;
pub const SYSTEM_CPU_SET_INFORMATION_REALTIME: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_INFO {
    pub Anonymous: SYSTEM_INFO_0,
    pub dwPageSize: u32,
    pub lpMinimumApplicationAddress: *mut core::ffi::c_void,
    pub lpMaximumApplicationAddress: *mut core::ffi::c_void,
    pub dwActiveProcessorMask: usize,
    pub dwNumberOfProcessors: u32,
    pub dwProcessorType: u32,
    pub dwAllocationGranularity: u32,
    pub wProcessorLevel: u16,
    pub wProcessorRevision: u16,
}
impl Default for SYSTEM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_INFO_0 {
    pub dwOemId: u32,
    pub Anonymous: SYSTEM_INFO_0_0,
}
impl Default for SYSTEM_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_INFO_0_0 {
    pub wProcessorArchitecture: PROCESSOR_ARCHITECTURE,
    pub wReserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    pub ProcessorMask: usize,
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0,
}
impl Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    pub ProcessorCore: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0,
    pub NumaNode: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1,
    pub Cache: CACHE_DESCRIPTOR,
    pub Reserved: [u64; 2],
}
impl Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    pub NodeNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    pub Flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Size: u32,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0,
}
impl Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    pub Processor: PROCESSOR_RELATIONSHIP,
    pub NumaNode: NUMA_NODE_RELATIONSHIP,
    pub Cache: CACHE_RELATIONSHIP,
    pub Group: GROUP_RELATIONSHIP,
}
impl Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_POOL_ZEROING_INFORMATION {
    pub PoolZeroingSupportPresent: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    pub CycleTime: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    pub _bitfield: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_CET_ENVIRONMENT(pub u32);
pub const USER_CET_ENVIRONMENT_SGX2_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(2u32);
pub const USER_CET_ENVIRONMENT_VBS_BASIC_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(17u32);
pub const USER_CET_ENVIRONMENT_VBS_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(16u32);
pub const USER_CET_ENVIRONMENT_WIN32_PROCESS: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(0u32);
pub const VER_BUILDNUMBER: VER_FLAGS = VER_FLAGS(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VER_FLAGS(pub u32);
impl VER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const VER_MAJORVERSION: VER_FLAGS = VER_FLAGS(2u32);
pub const VER_MINORVERSION: VER_FLAGS = VER_FLAGS(1u32);
pub const VER_PLATFORMID: VER_FLAGS = VER_FLAGS(8u32);
pub const VER_PRODUCT_TYPE: VER_FLAGS = VER_FLAGS(128u32);
pub const VER_SERVICEPACKMAJOR: VER_FLAGS = VER_FLAGS(32u32);
pub const VER_SERVICEPACKMINOR: VER_FLAGS = VER_FLAGS(16u32);
pub const VER_SUITENAME: VER_FLAGS = VER_FLAGS(64u32);
pub const WDK_NTDDI_VERSION: u32 = 167772172u32;
pub const _WIN32_IE_IE100: u32 = 2560u32;
pub const _WIN32_IE_IE110: u32 = 2560u32;
pub const _WIN32_IE_IE20: u32 = 512u32;
pub const _WIN32_IE_IE30: u32 = 768u32;
pub const _WIN32_IE_IE302: u32 = 770u32;
pub const _WIN32_IE_IE40: u32 = 1024u32;
pub const _WIN32_IE_IE401: u32 = 1025u32;
pub const _WIN32_IE_IE50: u32 = 1280u32;
pub const _WIN32_IE_IE501: u32 = 1281u32;
pub const _WIN32_IE_IE55: u32 = 1360u32;
pub const _WIN32_IE_IE60: u32 = 1536u32;
pub const _WIN32_IE_IE60SP1: u32 = 1537u32;
pub const _WIN32_IE_IE60SP2: u32 = 1539u32;
pub const _WIN32_IE_IE70: u32 = 1792u32;
pub const _WIN32_IE_IE80: u32 = 2048u32;
pub const _WIN32_IE_IE90: u32 = 2304u32;
pub const _WIN32_IE_LONGHORN: u32 = 1792u32;
pub const _WIN32_IE_NT4: u32 = 512u32;
pub const _WIN32_IE_NT4SP1: u32 = 512u32;
pub const _WIN32_IE_NT4SP2: u32 = 512u32;
pub const _WIN32_IE_NT4SP3: u32 = 770u32;
pub const _WIN32_IE_NT4SP4: u32 = 1025u32;
pub const _WIN32_IE_NT4SP5: u32 = 1025u32;
pub const _WIN32_IE_NT4SP6: u32 = 1280u32;
pub const _WIN32_IE_WIN10: u32 = 2560u32;
pub const _WIN32_IE_WIN2K: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP1: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP2: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP3: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP4: u32 = 1281u32;
pub const _WIN32_IE_WIN6: u32 = 1792u32;
pub const _WIN32_IE_WIN7: u32 = 2048u32;
pub const _WIN32_IE_WIN8: u32 = 2560u32;
pub const _WIN32_IE_WIN98: u32 = 1025u32;
pub const _WIN32_IE_WIN98SE: u32 = 1280u32;
pub const _WIN32_IE_WINBLUE: u32 = 2560u32;
pub const _WIN32_IE_WINME: u32 = 1360u32;
pub const _WIN32_IE_WINTHRESHOLD: u32 = 2560u32;
pub const _WIN32_IE_WS03: u32 = 1538u32;
pub const _WIN32_IE_WS03SP1: u32 = 1539u32;
pub const _WIN32_IE_XP: u32 = 1536u32;
pub const _WIN32_IE_XPSP1: u32 = 1537u32;
pub const _WIN32_IE_XPSP2: u32 = 1539u32;
pub const _WIN32_WINNT_LONGHORN: u32 = 1536u32;
pub const _WIN32_WINNT_NT4: u32 = 1024u32;
pub const _WIN32_WINNT_VISTA: u32 = 1536u32;
pub const _WIN32_WINNT_WIN10: u32 = 2560u32;
pub const _WIN32_WINNT_WIN2K: u32 = 1280u32;
pub const _WIN32_WINNT_WIN6: u32 = 1536u32;
pub const _WIN32_WINNT_WIN7: u32 = 1537u32;
pub const _WIN32_WINNT_WIN8: u32 = 1538u32;
pub const _WIN32_WINNT_WINBLUE: u32 = 1539u32;
pub const _WIN32_WINNT_WINTHRESHOLD: u32 = 2560u32;
pub const _WIN32_WINNT_WINXP: u32 = 1281u32;
pub const _WIN32_WINNT_WS03: u32 = 1282u32;
pub const _WIN32_WINNT_WS08: u32 = 1536u32;
