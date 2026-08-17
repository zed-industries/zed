#[inline]
pub unsafe fn CallNtPowerInformation(informationlevel: POWER_INFORMATION_LEVEL, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32) -> super::super::Foundation::NTSTATUS {
    windows_targets::link!("powrprof.dll" "system" fn CallNtPowerInformation(informationlevel : POWER_INFORMATION_LEVEL, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::super::Foundation:: NTSTATUS);
    CallNtPowerInformation(informationlevel, core::mem::transmute(inputbuffer.unwrap_or(std::ptr::null())), inputbufferlength, core::mem::transmute(outputbuffer.unwrap_or(std::ptr::null_mut())), outputbufferlength)
}
#[inline]
pub unsafe fn CanUserWritePwrScheme() -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn CanUserWritePwrScheme() -> super::super::Foundation:: BOOLEAN);
    CanUserWritePwrScheme()
}
#[inline]
pub unsafe fn DeletePwrScheme(uiid: u32) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn DeletePwrScheme(uiid : u32) -> super::super::Foundation:: BOOLEAN);
    DeletePwrScheme(uiid)
}
#[inline]
pub unsafe fn DevicePowerClose() -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn DevicePowerClose() -> super::super::Foundation:: BOOLEAN);
    DevicePowerClose()
}
#[inline]
pub unsafe fn DevicePowerEnumDevices(queryindex: u32, queryinterpretationflags: u32, queryflags: u32, preturnbuffer: Option<*mut u8>, pbuffersize: *mut u32) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn DevicePowerEnumDevices(queryindex : u32, queryinterpretationflags : u32, queryflags : u32, preturnbuffer : *mut u8, pbuffersize : *mut u32) -> super::super::Foundation:: BOOLEAN);
    DevicePowerEnumDevices(queryindex, queryinterpretationflags, queryflags, core::mem::transmute(preturnbuffer.unwrap_or(std::ptr::null_mut())), pbuffersize)
}
#[inline]
pub unsafe fn DevicePowerOpen(debugmask: u32) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn DevicePowerOpen(debugmask : u32) -> super::super::Foundation:: BOOLEAN);
    DevicePowerOpen(debugmask)
}
#[inline]
pub unsafe fn DevicePowerSetDeviceState<P0>(devicedescription: P0, setflags: u32, setdata: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("powrprof.dll" "system" fn DevicePowerSetDeviceState(devicedescription : windows_core::PCWSTR, setflags : u32, setdata : *const core::ffi::c_void) -> u32);
    DevicePowerSetDeviceState(devicedescription.param().abi(), setflags, core::mem::transmute(setdata.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn EnumPwrSchemes<P0>(lpfn: PWRSCHEMESENUMPROC, lparam: P0) -> super::super::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("powrprof.dll" "system" fn EnumPwrSchemes(lpfn : PWRSCHEMESENUMPROC, lparam : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOLEAN);
    EnumPwrSchemes(lpfn, lparam.param().abi())
}
#[inline]
pub unsafe fn GetActivePwrScheme(puiid: *mut u32) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn GetActivePwrScheme(puiid : *mut u32) -> super::super::Foundation:: BOOLEAN);
    GetActivePwrScheme(puiid)
}
#[inline]
pub unsafe fn GetCurrentPowerPolicies(pglobalpowerpolicy: *mut GLOBAL_POWER_POLICY, ppowerpolicy: *mut POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn GetCurrentPowerPolicies(pglobalpowerpolicy : *mut GLOBAL_POWER_POLICY, ppowerpolicy : *mut POWER_POLICY) -> super::super::Foundation:: BOOLEAN);
    GetCurrentPowerPolicies(pglobalpowerpolicy, ppowerpolicy)
}
#[inline]
pub unsafe fn GetDevicePowerState<P0>(hdevice: P0, pfon: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetDevicePowerState(hdevice : super::super::Foundation:: HANDLE, pfon : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetDevicePowerState(hdevice.param().abi(), pfon)
}
#[inline]
pub unsafe fn GetPwrCapabilities(lpspc: *mut SYSTEM_POWER_CAPABILITIES) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn GetPwrCapabilities(lpspc : *mut SYSTEM_POWER_CAPABILITIES) -> super::super::Foundation:: BOOLEAN);
    GetPwrCapabilities(lpspc)
}
#[inline]
pub unsafe fn GetPwrDiskSpindownRange(puimax: *mut u32, puimin: *mut u32) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn GetPwrDiskSpindownRange(puimax : *mut u32, puimin : *mut u32) -> super::super::Foundation:: BOOLEAN);
    GetPwrDiskSpindownRange(puimax, puimin)
}
#[inline]
pub unsafe fn GetSystemPowerStatus(lpsystempowerstatus: *mut SYSTEM_POWER_STATUS) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetSystemPowerStatus(lpsystempowerstatus : *mut SYSTEM_POWER_STATUS) -> super::super::Foundation:: BOOL);
    GetSystemPowerStatus(lpsystempowerstatus).ok()
}
#[inline]
pub unsafe fn IsAdminOverrideActive(papp: *const ADMINISTRATOR_POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn IsAdminOverrideActive(papp : *const ADMINISTRATOR_POWER_POLICY) -> super::super::Foundation:: BOOLEAN);
    IsAdminOverrideActive(papp)
}
#[inline]
pub unsafe fn IsPwrHibernateAllowed() -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn IsPwrHibernateAllowed() -> super::super::Foundation:: BOOLEAN);
    IsPwrHibernateAllowed()
}
#[inline]
pub unsafe fn IsPwrShutdownAllowed() -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn IsPwrShutdownAllowed() -> super::super::Foundation:: BOOLEAN);
    IsPwrShutdownAllowed()
}
#[inline]
pub unsafe fn IsPwrSuspendAllowed() -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn IsPwrSuspendAllowed() -> super::super::Foundation:: BOOLEAN);
    IsPwrSuspendAllowed()
}
#[inline]
pub unsafe fn IsSystemResumeAutomatic() -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn IsSystemResumeAutomatic() -> super::super::Foundation:: BOOL);
    IsSystemResumeAutomatic()
}
#[inline]
pub unsafe fn PowerCanRestoreIndividualDefaultPowerScheme(schemeguid: *const windows_core::GUID) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("powrprof.dll" "system" fn PowerCanRestoreIndividualDefaultPowerScheme(schemeguid : *const windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    PowerCanRestoreIndividualDefaultPowerScheme(schemeguid)
}
#[inline]
pub unsafe fn PowerClearRequest<P0>(powerrequest: P0, requesttype: POWER_REQUEST_TYPE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn PowerClearRequest(powerrequest : super::super::Foundation:: HANDLE, requesttype : POWER_REQUEST_TYPE) -> super::super::Foundation:: BOOL);
    PowerClearRequest(powerrequest.param().abi(), requesttype).ok()
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerCreatePossibleSetting<P0>(rootsystempowerkey: P0, subgroupofpowersettingsguid: *const windows_core::GUID, powersettingguid: *const windows_core::GUID, possiblesettingindex: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerCreatePossibleSetting(rootsystempowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, possiblesettingindex : u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerCreatePossibleSetting(rootsystempowerkey.param().abi(), subgroupofpowersettingsguid, powersettingguid, possiblesettingindex)
}
#[cfg(feature = "Win32_System_Threading")]
#[inline]
pub unsafe fn PowerCreateRequest(context: *const super::Threading::REASON_CONTEXT) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("kernel32.dll" "system" fn PowerCreateRequest(context : *const super::Threading:: REASON_CONTEXT) -> super::super::Foundation:: HANDLE);
    let result__ = PowerCreateRequest(context);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerCreateSetting<P0>(rootsystempowerkey: P0, subgroupofpowersettingsguid: *const windows_core::GUID, powersettingguid: *const windows_core::GUID) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerCreateSetting(rootsystempowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    PowerCreateSetting(rootsystempowerkey.param().abi(), subgroupofpowersettingsguid, powersettingguid)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerDeleteScheme<P0>(rootpowerkey: P0, schemeguid: *const windows_core::GUID) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerDeleteScheme(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    PowerDeleteScheme(rootpowerkey.param().abi(), schemeguid)
}
#[inline]
pub unsafe fn PowerDeterminePlatformRole() -> POWER_PLATFORM_ROLE {
    windows_targets::link!("powrprof.dll" "system" fn PowerDeterminePlatformRole() -> POWER_PLATFORM_ROLE);
    PowerDeterminePlatformRole()
}
#[inline]
pub unsafe fn PowerDeterminePlatformRoleEx(version: POWER_PLATFORM_ROLE_VERSION) -> POWER_PLATFORM_ROLE {
    windows_targets::link!("powrprof.dll" "system" fn PowerDeterminePlatformRoleEx(version : POWER_PLATFORM_ROLE_VERSION) -> POWER_PLATFORM_ROLE);
    PowerDeterminePlatformRoleEx(version)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerDuplicateScheme<P0>(rootpowerkey: P0, sourceschemeguid: *const windows_core::GUID, destinationschemeguid: *mut *mut windows_core::GUID) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerDuplicateScheme(rootpowerkey : super::Registry:: HKEY, sourceschemeguid : *const windows_core::GUID, destinationschemeguid : *mut *mut windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    PowerDuplicateScheme(rootpowerkey.param().abi(), sourceschemeguid, destinationschemeguid)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerEnumerate<P0>(rootpowerkey: P0, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, accessflags: POWER_DATA_ACCESSOR, index: u32, buffer: Option<*mut u8>, buffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerEnumerate(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, accessflags : POWER_DATA_ACCESSOR, index : u32, buffer : *mut u8, buffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerEnumerate(rootpowerkey.param().abi(), core::mem::transmute(schemeguid.unwrap_or(std::ptr::null())), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), accessflags, index, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerGetActiveScheme<P0>(userrootpowerkey: P0, activepolicyguid: *mut *mut windows_core::GUID) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerGetActiveScheme(userrootpowerkey : super::Registry:: HKEY, activepolicyguid : *mut *mut windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    PowerGetActiveScheme(userrootpowerkey.param().abi(), activepolicyguid)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerImportPowerScheme<P0, P1>(rootpowerkey: P0, importfilenamepath: P1, destinationschemeguid: *mut *mut windows_core::GUID) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerImportPowerScheme(rootpowerkey : super::Registry:: HKEY, importfilenamepath : windows_core::PCWSTR, destinationschemeguid : *mut *mut windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    PowerImportPowerScheme(rootpowerkey.param().abi(), importfilenamepath.param().abi(), destinationschemeguid)
}
#[inline]
pub unsafe fn PowerIsSettingRangeDefined(subkeyguid: Option<*const windows_core::GUID>, settingguid: Option<*const windows_core::GUID>) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn PowerIsSettingRangeDefined(subkeyguid : *const windows_core::GUID, settingguid : *const windows_core::GUID) -> super::super::Foundation:: BOOLEAN);
    PowerIsSettingRangeDefined(core::mem::transmute(subkeyguid.unwrap_or(std::ptr::null())), core::mem::transmute(settingguid.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerOpenSystemPowerKey<P0>(phsystempowerkey: *mut super::Registry::HKEY, access: u32, openexisting: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerOpenSystemPowerKey(phsystempowerkey : *mut super::Registry:: HKEY, access : u32, openexisting : super::super::Foundation:: BOOL) -> u32);
    PowerOpenSystemPowerKey(phsystempowerkey, access, openexisting.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerOpenUserPowerKey<P0>(phuserpowerkey: *mut super::Registry::HKEY, access: u32, openexisting: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerOpenUserPowerKey(phuserpowerkey : *mut super::Registry:: HKEY, access : u32, openexisting : super::super::Foundation:: BOOL) -> u32);
    PowerOpenUserPowerKey(phuserpowerkey, access, openexisting.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadACDefaultIndex<P0>(rootpowerkey: P0, schemepersonalityguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: *const windows_core::GUID, acdefaultindex: *mut u32) -> u32
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadACDefaultIndex(rootpowerkey : super::Registry:: HKEY, schemepersonalityguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, acdefaultindex : *mut u32) -> u32);
    PowerReadACDefaultIndex(rootpowerkey.param().abi(), schemepersonalityguid, core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), powersettingguid, acdefaultindex)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadACValue<P0>(rootpowerkey: P0, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, r#type: Option<*mut u32>, buffer: Option<*mut u8>, buffersize: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadACValue(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, r#type : *mut u32, buffer : *mut u8, buffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerReadACValue(rootpowerkey.param().abi(), core::mem::transmute(schemeguid.unwrap_or(std::ptr::null())), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), core::mem::transmute(r#type.unwrap_or(std::ptr::null_mut())), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(buffersize.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadACValueIndex<P0>(rootpowerkey: P0, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, acvalueindex: *mut u32) -> u32
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadACValueIndex(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, acvalueindex : *mut u32) -> u32);
    PowerReadACValueIndex(rootpowerkey.param().abi(), core::mem::transmute(schemeguid.unwrap_or(std::ptr::null())), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), acvalueindex)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadDCDefaultIndex<P0>(rootpowerkey: P0, schemepersonalityguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: *const windows_core::GUID, dcdefaultindex: *mut u32) -> u32
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadDCDefaultIndex(rootpowerkey : super::Registry:: HKEY, schemepersonalityguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, dcdefaultindex : *mut u32) -> u32);
    PowerReadDCDefaultIndex(rootpowerkey.param().abi(), schemepersonalityguid, core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), powersettingguid, dcdefaultindex)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadDCValue<P0>(rootpowerkey: P0, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, r#type: Option<*mut u32>, buffer: Option<*mut u8>, buffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadDCValue(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, r#type : *mut u32, buffer : *mut u8, buffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerReadDCValue(rootpowerkey.param().abi(), core::mem::transmute(schemeguid.unwrap_or(std::ptr::null())), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), core::mem::transmute(r#type.unwrap_or(std::ptr::null_mut())), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadDCValueIndex<P0>(rootpowerkey: P0, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, dcvalueindex: *mut u32) -> u32
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadDCValueIndex(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, dcvalueindex : *mut u32) -> u32);
    PowerReadDCValueIndex(rootpowerkey.param().abi(), core::mem::transmute(schemeguid.unwrap_or(std::ptr::null())), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), dcvalueindex)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadDescription<P0>(rootpowerkey: P0, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: Option<*mut u8>, buffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadDescription(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *mut u8, buffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerReadDescription(rootpowerkey.param().abi(), core::mem::transmute(schemeguid.unwrap_or(std::ptr::null())), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadFriendlyName<P0>(rootpowerkey: P0, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: Option<*mut u8>, buffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadFriendlyName(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *mut u8, buffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerReadFriendlyName(rootpowerkey.param().abi(), core::mem::transmute(schemeguid.unwrap_or(std::ptr::null())), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadIconResourceSpecifier<P0>(rootpowerkey: P0, schemeguid: Option<*const windows_core::GUID>, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: Option<*mut u8>, buffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadIconResourceSpecifier(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *mut u8, buffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerReadIconResourceSpecifier(rootpowerkey.param().abi(), core::mem::transmute(schemeguid.unwrap_or(std::ptr::null())), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadPossibleDescription<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, possiblesettingindex: u32, buffer: Option<*mut u8>, buffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadPossibleDescription(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, possiblesettingindex : u32, buffer : *mut u8, buffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerReadPossibleDescription(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), possiblesettingindex, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadPossibleFriendlyName<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, possiblesettingindex: u32, buffer: Option<*mut u8>, buffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadPossibleFriendlyName(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, possiblesettingindex : u32, buffer : *mut u8, buffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerReadPossibleFriendlyName(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), possiblesettingindex, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadPossibleValue<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, r#type: Option<*mut u32>, possiblesettingindex: u32, buffer: Option<*mut u8>, buffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadPossibleValue(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, r#type : *mut u32, possiblesettingindex : u32, buffer : *mut u8, buffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerReadPossibleValue(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), core::mem::transmute(r#type.unwrap_or(std::ptr::null_mut())), possiblesettingindex, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize)
}
#[inline]
pub unsafe fn PowerReadSettingAttributes(subgroupguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>) -> u32 {
    windows_targets::link!("powrprof.dll" "system" fn PowerReadSettingAttributes(subgroupguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID) -> u32);
    PowerReadSettingAttributes(core::mem::transmute(subgroupguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadValueIncrement<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, valueincrement: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadValueIncrement(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, valueincrement : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerReadValueIncrement(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), valueincrement)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadValueMax<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, valuemaximum: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadValueMax(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, valuemaximum : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerReadValueMax(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), valuemaximum)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadValueMin<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, valueminimum: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadValueMin(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, valueminimum : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerReadValueMin(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), valueminimum)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerReadValueUnitsSpecifier<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: Option<*mut u8>, buffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerReadValueUnitsSpecifier(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *mut u8, buffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerReadValueUnitsSpecifier(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize)
}
#[inline]
pub unsafe fn PowerRegisterForEffectivePowerModeNotifications(version: u32, callback: EFFECTIVE_POWER_MODE_CALLBACK, context: Option<*const core::ffi::c_void>, registrationhandle: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("powrprof.dll" "system" fn PowerRegisterForEffectivePowerModeNotifications(version : u32, callback : EFFECTIVE_POWER_MODE_CALLBACK, context : *const core::ffi::c_void, registrationhandle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PowerRegisterForEffectivePowerModeNotifications(version, callback, core::mem::transmute(context.unwrap_or(std::ptr::null())), registrationhandle).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn PowerRegisterSuspendResumeNotification<P0>(flags: super::super::UI::WindowsAndMessaging::REGISTER_NOTIFICATION_FLAGS, recipient: P0, registrationhandle: *mut *mut core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerRegisterSuspendResumeNotification(flags : super::super::UI::WindowsAndMessaging:: REGISTER_NOTIFICATION_FLAGS, recipient : super::super::Foundation:: HANDLE, registrationhandle : *mut *mut core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    PowerRegisterSuspendResumeNotification(flags, recipient.param().abi(), registrationhandle)
}
#[inline]
pub unsafe fn PowerRemovePowerSetting(powersettingsubkeyguid: *const windows_core::GUID, powersettingguid: *const windows_core::GUID) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("powrprof.dll" "system" fn PowerRemovePowerSetting(powersettingsubkeyguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    PowerRemovePowerSetting(powersettingsubkeyguid, powersettingguid)
}
#[inline]
pub unsafe fn PowerReplaceDefaultPowerSchemes() -> u32 {
    windows_targets::link!("powrprof.dll" "system" fn PowerReplaceDefaultPowerSchemes() -> u32);
    PowerReplaceDefaultPowerSchemes()
}
#[inline]
pub unsafe fn PowerReportThermalEvent(event: *const THERMAL_EVENT) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("powrprof.dll" "system" fn PowerReportThermalEvent(event : *const THERMAL_EVENT) -> super::super::Foundation:: WIN32_ERROR);
    PowerReportThermalEvent(event)
}
#[inline]
pub unsafe fn PowerRestoreDefaultPowerSchemes() -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("powrprof.dll" "system" fn PowerRestoreDefaultPowerSchemes() -> super::super::Foundation:: WIN32_ERROR);
    PowerRestoreDefaultPowerSchemes()
}
#[inline]
pub unsafe fn PowerRestoreIndividualDefaultPowerScheme(schemeguid: *const windows_core::GUID) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("powrprof.dll" "system" fn PowerRestoreIndividualDefaultPowerScheme(schemeguid : *const windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    PowerRestoreIndividualDefaultPowerScheme(schemeguid)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerSetActiveScheme<P0>(userrootpowerkey: P0, schemeguid: Option<*const windows_core::GUID>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerSetActiveScheme(userrootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    PowerSetActiveScheme(userrootpowerkey.param().abi(), core::mem::transmute(schemeguid.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn PowerSetRequest<P0>(powerrequest: P0, requesttype: POWER_REQUEST_TYPE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn PowerSetRequest(powerrequest : super::super::Foundation:: HANDLE, requesttype : POWER_REQUEST_TYPE) -> super::super::Foundation:: BOOL);
    PowerSetRequest(powerrequest.param().abi(), requesttype).ok()
}
#[inline]
pub unsafe fn PowerSettingAccessCheck(accessflags: POWER_DATA_ACCESSOR, powerguid: Option<*const windows_core::GUID>) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("powrprof.dll" "system" fn PowerSettingAccessCheck(accessflags : POWER_DATA_ACCESSOR, powerguid : *const windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    PowerSettingAccessCheck(accessflags, core::mem::transmute(powerguid.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerSettingAccessCheckEx(accessflags: POWER_DATA_ACCESSOR, powerguid: Option<*const windows_core::GUID>, accesstype: super::Registry::REG_SAM_FLAGS) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("powrprof.dll" "system" fn PowerSettingAccessCheckEx(accessflags : POWER_DATA_ACCESSOR, powerguid : *const windows_core::GUID, accesstype : super::Registry:: REG_SAM_FLAGS) -> super::super::Foundation:: WIN32_ERROR);
    PowerSettingAccessCheckEx(accessflags, core::mem::transmute(powerguid.unwrap_or(std::ptr::null())), accesstype)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn PowerSettingRegisterNotification<P0>(settingguid: *const windows_core::GUID, flags: super::super::UI::WindowsAndMessaging::REGISTER_NOTIFICATION_FLAGS, recipient: P0, registrationhandle: *mut *mut core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerSettingRegisterNotification(settingguid : *const windows_core::GUID, flags : super::super::UI::WindowsAndMessaging:: REGISTER_NOTIFICATION_FLAGS, recipient : super::super::Foundation:: HANDLE, registrationhandle : *mut *mut core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    PowerSettingRegisterNotification(settingguid, flags, recipient.param().abi(), registrationhandle)
}
#[inline]
pub unsafe fn PowerSettingUnregisterNotification<P0>(registrationhandle: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HPOWERNOTIFY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerSettingUnregisterNotification(registrationhandle : HPOWERNOTIFY) -> super::super::Foundation:: WIN32_ERROR);
    PowerSettingUnregisterNotification(registrationhandle.param().abi())
}
#[inline]
pub unsafe fn PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("powrprof.dll" "system" fn PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle : *const core::ffi::c_void) -> windows_core::HRESULT);
    PowerUnregisterFromEffectivePowerModeNotifications(registrationhandle).ok()
}
#[inline]
pub unsafe fn PowerUnregisterSuspendResumeNotification<P0>(registrationhandle: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HPOWERNOTIFY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerUnregisterSuspendResumeNotification(registrationhandle : HPOWERNOTIFY) -> super::super::Foundation:: WIN32_ERROR);
    PowerUnregisterSuspendResumeNotification(registrationhandle.param().abi())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteACDefaultIndex<P0>(rootsystempowerkey: P0, schemepersonalityguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: *const windows_core::GUID, defaultacindex: u32) -> u32
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWriteACDefaultIndex(rootsystempowerkey : super::Registry:: HKEY, schemepersonalityguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, defaultacindex : u32) -> u32);
    PowerWriteACDefaultIndex(rootsystempowerkey.param().abi(), schemepersonalityguid, core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), powersettingguid, defaultacindex)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteACValueIndex<P0>(rootpowerkey: P0, schemeguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, acvalueindex: u32) -> u32
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWriteACValueIndex(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, acvalueindex : u32) -> u32);
    PowerWriteACValueIndex(rootpowerkey.param().abi(), schemeguid, core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), acvalueindex)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteDCDefaultIndex<P0>(rootsystempowerkey: P0, schemepersonalityguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: *const windows_core::GUID, defaultdcindex: u32) -> u32
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWriteDCDefaultIndex(rootsystempowerkey : super::Registry:: HKEY, schemepersonalityguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, defaultdcindex : u32) -> u32);
    PowerWriteDCDefaultIndex(rootsystempowerkey.param().abi(), schemepersonalityguid, core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), powersettingguid, defaultdcindex)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteDCValueIndex<P0>(rootpowerkey: P0, schemeguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, dcvalueindex: u32) -> u32
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWriteDCValueIndex(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, dcvalueindex : u32) -> u32);
    PowerWriteDCValueIndex(rootpowerkey.param().abi(), schemeguid, core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), dcvalueindex)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteDescription<P0>(rootpowerkey: P0, schemeguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: &[u8]) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWriteDescription(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *const u8, buffersize : u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerWriteDescription(rootpowerkey.param().abi(), schemeguid, core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteFriendlyName<P0>(rootpowerkey: P0, schemeguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: &[u8]) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWriteFriendlyName(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *const u8, buffersize : u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerWriteFriendlyName(rootpowerkey.param().abi(), schemeguid, core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteIconResourceSpecifier<P0>(rootpowerkey: P0, schemeguid: *const windows_core::GUID, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: &[u8]) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWriteIconResourceSpecifier(rootpowerkey : super::Registry:: HKEY, schemeguid : *const windows_core::GUID, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *const u8, buffersize : u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerWriteIconResourceSpecifier(rootpowerkey.param().abi(), schemeguid, core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWritePossibleDescription<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, possiblesettingindex: u32, buffer: &[u8]) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWritePossibleDescription(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, possiblesettingindex : u32, buffer : *const u8, buffersize : u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerWritePossibleDescription(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), possiblesettingindex, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWritePossibleFriendlyName<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, possiblesettingindex: u32, buffer: &[u8]) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWritePossibleFriendlyName(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, possiblesettingindex : u32, buffer : *const u8, buffersize : u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerWritePossibleFriendlyName(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), possiblesettingindex, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap())
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWritePossibleValue<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, r#type: u32, possiblesettingindex: u32, buffer: &[u8]) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWritePossibleValue(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, r#type : u32, possiblesettingindex : u32, buffer : *const u8, buffersize : u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerWritePossibleValue(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), r#type, possiblesettingindex, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap())
}
#[inline]
pub unsafe fn PowerWriteSettingAttributes(subgroupguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, attributes: u32) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("powrprof.dll" "system" fn PowerWriteSettingAttributes(subgroupguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, attributes : u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerWriteSettingAttributes(core::mem::transmute(subgroupguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), attributes)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteValueIncrement<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, valueincrement: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWriteValueIncrement(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, valueincrement : u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerWriteValueIncrement(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), valueincrement)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteValueMax<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, valuemaximum: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWriteValueMax(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, valuemaximum : u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerWriteValueMax(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), valuemaximum)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteValueMin<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, valueminimum: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWriteValueMin(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, valueminimum : u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerWriteValueMin(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), valueminimum)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PowerWriteValueUnitsSpecifier<P0>(rootpowerkey: P0, subgroupofpowersettingsguid: Option<*const windows_core::GUID>, powersettingguid: Option<*const windows_core::GUID>, buffer: &[u8]) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::Registry::HKEY>,
{
    windows_targets::link!("powrprof.dll" "system" fn PowerWriteValueUnitsSpecifier(rootpowerkey : super::Registry:: HKEY, subgroupofpowersettingsguid : *const windows_core::GUID, powersettingguid : *const windows_core::GUID, buffer : *const u8, buffersize : u32) -> super::super::Foundation:: WIN32_ERROR);
    PowerWriteValueUnitsSpecifier(rootpowerkey.param().abi(), core::mem::transmute(subgroupofpowersettingsguid.unwrap_or(std::ptr::null())), core::mem::transmute(powersettingguid.unwrap_or(std::ptr::null())), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap())
}
#[inline]
pub unsafe fn ReadGlobalPwrPolicy(pglobalpowerpolicy: *const GLOBAL_POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn ReadGlobalPwrPolicy(pglobalpowerpolicy : *const GLOBAL_POWER_POLICY) -> super::super::Foundation:: BOOLEAN);
    ReadGlobalPwrPolicy(pglobalpowerpolicy)
}
#[inline]
pub unsafe fn ReadProcessorPwrScheme(uiid: u32, pmachineprocessorpowerpolicy: *mut MACHINE_PROCESSOR_POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn ReadProcessorPwrScheme(uiid : u32, pmachineprocessorpowerpolicy : *mut MACHINE_PROCESSOR_POWER_POLICY) -> super::super::Foundation:: BOOLEAN);
    ReadProcessorPwrScheme(uiid, pmachineprocessorpowerpolicy)
}
#[inline]
pub unsafe fn ReadPwrScheme(uiid: u32, ppowerpolicy: *mut POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn ReadPwrScheme(uiid : u32, ppowerpolicy : *mut POWER_POLICY) -> super::super::Foundation:: BOOLEAN);
    ReadPwrScheme(uiid, ppowerpolicy)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn RegisterPowerSettingNotification<P0>(hrecipient: P0, powersettingguid: *const windows_core::GUID, flags: super::super::UI::WindowsAndMessaging::REGISTER_NOTIFICATION_FLAGS) -> windows_core::Result<HPOWERNOTIFY>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn RegisterPowerSettingNotification(hrecipient : super::super::Foundation:: HANDLE, powersettingguid : *const windows_core::GUID, flags : super::super::UI::WindowsAndMessaging:: REGISTER_NOTIFICATION_FLAGS) -> HPOWERNOTIFY);
    let result__ = RegisterPowerSettingNotification(hrecipient.param().abi(), powersettingguid, flags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn RegisterSuspendResumeNotification<P0>(hrecipient: P0, flags: super::super::UI::WindowsAndMessaging::REGISTER_NOTIFICATION_FLAGS) -> windows_core::Result<HPOWERNOTIFY>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn RegisterSuspendResumeNotification(hrecipient : super::super::Foundation:: HANDLE, flags : super::super::UI::WindowsAndMessaging:: REGISTER_NOTIFICATION_FLAGS) -> HPOWERNOTIFY);
    let result__ = RegisterSuspendResumeNotification(hrecipient.param().abi(), flags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn RequestWakeupLatency(latency: LATENCY_TIME) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn RequestWakeupLatency(latency : LATENCY_TIME) -> super::super::Foundation:: BOOL);
    RequestWakeupLatency(latency)
}
#[inline]
pub unsafe fn SetActivePwrScheme(uiid: u32, pglobalpowerpolicy: Option<*const GLOBAL_POWER_POLICY>, ppowerpolicy: Option<*const POWER_POLICY>) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn SetActivePwrScheme(uiid : u32, pglobalpowerpolicy : *const GLOBAL_POWER_POLICY, ppowerpolicy : *const POWER_POLICY) -> super::super::Foundation:: BOOLEAN);
    SetActivePwrScheme(uiid, core::mem::transmute(pglobalpowerpolicy.unwrap_or(std::ptr::null())), core::mem::transmute(ppowerpolicy.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn SetSuspendState<P0, P1, P2>(bhibernate: P0, bforce: P1, bwakeupeventsdisabled: P2) -> super::super::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::Foundation::BOOLEAN>,
    P1: windows_core::Param<super::super::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("powrprof.dll" "system" fn SetSuspendState(bhibernate : super::super::Foundation:: BOOLEAN, bforce : super::super::Foundation:: BOOLEAN, bwakeupeventsdisabled : super::super::Foundation:: BOOLEAN) -> super::super::Foundation:: BOOLEAN);
    SetSuspendState(bhibernate.param().abi(), bforce.param().abi(), bwakeupeventsdisabled.param().abi())
}
#[inline]
pub unsafe fn SetSystemPowerState<P0, P1>(fsuspend: P0, fforce: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetSystemPowerState(fsuspend : super::super::Foundation:: BOOL, fforce : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    SetSystemPowerState(fsuspend.param().abi(), fforce.param().abi()).ok()
}
#[inline]
pub unsafe fn SetThreadExecutionState(esflags: EXECUTION_STATE) -> EXECUTION_STATE {
    windows_targets::link!("kernel32.dll" "system" fn SetThreadExecutionState(esflags : EXECUTION_STATE) -> EXECUTION_STATE);
    SetThreadExecutionState(esflags)
}
#[inline]
pub unsafe fn UnregisterPowerSettingNotification<P0>(handle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HPOWERNOTIFY>,
{
    windows_targets::link!("user32.dll" "system" fn UnregisterPowerSettingNotification(handle : HPOWERNOTIFY) -> super::super::Foundation:: BOOL);
    UnregisterPowerSettingNotification(handle.param().abi()).ok()
}
#[inline]
pub unsafe fn UnregisterSuspendResumeNotification<P0>(handle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HPOWERNOTIFY>,
{
    windows_targets::link!("user32.dll" "system" fn UnregisterSuspendResumeNotification(handle : HPOWERNOTIFY) -> super::super::Foundation:: BOOL);
    UnregisterSuspendResumeNotification(handle.param().abi()).ok()
}
#[inline]
pub unsafe fn ValidatePowerPolicies(pglobalpowerpolicy: Option<*mut GLOBAL_POWER_POLICY>, ppowerpolicy: Option<*mut POWER_POLICY>) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn ValidatePowerPolicies(pglobalpowerpolicy : *mut GLOBAL_POWER_POLICY, ppowerpolicy : *mut POWER_POLICY) -> super::super::Foundation:: BOOLEAN);
    ValidatePowerPolicies(core::mem::transmute(pglobalpowerpolicy.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppowerpolicy.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn WriteGlobalPwrPolicy(pglobalpowerpolicy: *const GLOBAL_POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn WriteGlobalPwrPolicy(pglobalpowerpolicy : *const GLOBAL_POWER_POLICY) -> super::super::Foundation:: BOOLEAN);
    WriteGlobalPwrPolicy(pglobalpowerpolicy)
}
#[inline]
pub unsafe fn WriteProcessorPwrScheme(uiid: u32, pmachineprocessorpowerpolicy: *const MACHINE_PROCESSOR_POWER_POLICY) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("powrprof.dll" "system" fn WriteProcessorPwrScheme(uiid : u32, pmachineprocessorpowerpolicy : *const MACHINE_PROCESSOR_POWER_POLICY) -> super::super::Foundation:: BOOLEAN);
    WriteProcessorPwrScheme(uiid, pmachineprocessorpowerpolicy)
}
#[inline]
pub unsafe fn WritePwrScheme<P0, P1>(puiid: *const u32, lpszschemename: P0, lpszdescription: P1, lpscheme: *const POWER_POLICY) -> super::super::Foundation::BOOLEAN
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("powrprof.dll" "system" fn WritePwrScheme(puiid : *const u32, lpszschemename : windows_core::PCWSTR, lpszdescription : windows_core::PCWSTR, lpscheme : *const POWER_POLICY) -> super::super::Foundation:: BOOLEAN);
    WritePwrScheme(puiid, lpszschemename.param().abi(), lpszdescription.param().abi(), lpscheme)
}
pub const ACCESS_ACTIVE_OVERLAY_SCHEME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(27i32);
pub const ACCESS_ACTIVE_SCHEME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(19i32);
pub const ACCESS_AC_POWER_SETTING_INDEX: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(0i32);
pub const ACCESS_AC_POWER_SETTING_MAX: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(21i32);
pub const ACCESS_AC_POWER_SETTING_MIN: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(23i32);
pub const ACCESS_ATTRIBUTES: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(15i32);
pub const ACCESS_CREATE_SCHEME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(20i32);
pub const ACCESS_DC_POWER_SETTING_INDEX: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(1i32);
pub const ACCESS_DC_POWER_SETTING_MAX: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(22i32);
pub const ACCESS_DC_POWER_SETTING_MIN: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(24i32);
pub const ACCESS_DEFAULT_AC_POWER_SETTING: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(7i32);
pub const ACCESS_DEFAULT_DC_POWER_SETTING: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(8i32);
pub const ACCESS_DEFAULT_SECURITY_DESCRIPTOR: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(14i32);
pub const ACCESS_DESCRIPTION: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(3i32);
pub const ACCESS_FRIENDLY_NAME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(2i32);
pub const ACCESS_ICON_RESOURCE: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(13i32);
pub const ACCESS_INDIVIDUAL_SETTING: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(18i32);
pub const ACCESS_OVERLAY_SCHEME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(26i32);
pub const ACCESS_POSSIBLE_POWER_SETTING: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(4i32);
pub const ACCESS_POSSIBLE_POWER_SETTING_DESCRIPTION: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(6i32);
pub const ACCESS_POSSIBLE_POWER_SETTING_FRIENDLY_NAME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(5i32);
pub const ACCESS_POSSIBLE_VALUE_INCREMENT: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(11i32);
pub const ACCESS_POSSIBLE_VALUE_MAX: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(10i32);
pub const ACCESS_POSSIBLE_VALUE_MIN: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(9i32);
pub const ACCESS_POSSIBLE_VALUE_UNITS: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(12i32);
pub const ACCESS_PROFILE: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(25i32);
pub const ACCESS_SCHEME: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(16i32);
pub const ACCESS_SUBGROUP: POWER_DATA_ACCESSOR = POWER_DATA_ACCESSOR(17i32);
pub const ACPI_TIME_ADJUST_DAYLIGHT: u32 = 1u32;
pub const ACPI_TIME_IN_DAYLIGHT: u32 = 2u32;
pub const ACPI_TIME_ZONE_UNKNOWN: u32 = 2047u32;
pub const ACTIVE_COOLING: u32 = 0u32;
pub const ALTITUDE_GROUP_POLICY: POWER_SETTING_ALTITUDE = POWER_SETTING_ALTITUDE(0i32);
pub const ALTITUDE_INTERNAL_OVERRIDE: POWER_SETTING_ALTITUDE = POWER_SETTING_ALTITUDE(5i32);
pub const ALTITUDE_OEM_CUSTOMIZATION: POWER_SETTING_ALTITUDE = POWER_SETTING_ALTITUDE(4i32);
pub const ALTITUDE_OS_DEFAULT: POWER_SETTING_ALTITUDE = POWER_SETTING_ALTITUDE(6i32);
pub const ALTITUDE_PROVISIONING: POWER_SETTING_ALTITUDE = POWER_SETTING_ALTITUDE(3i32);
pub const ALTITUDE_RUNTIME_OVERRIDE: POWER_SETTING_ALTITUDE = POWER_SETTING_ALTITUDE(2i32);
pub const ALTITUDE_USER: POWER_SETTING_ALTITUDE = POWER_SETTING_ALTITUDE(1i32);
pub const AcpiTimeResolutionMax: ACPI_TIME_RESOLUTION = ACPI_TIME_RESOLUTION(2i32);
pub const AcpiTimeResolutionMilliseconds: ACPI_TIME_RESOLUTION = ACPI_TIME_RESOLUTION(0i32);
pub const AcpiTimeResolutionSeconds: ACPI_TIME_RESOLUTION = ACPI_TIME_RESOLUTION(1i32);
pub const AdministratorPowerPolicy: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(9i32);
pub const BATTERY_CAPACITY_RELATIVE: u32 = 1073741824u32;
pub const BATTERY_CHARGING: u32 = 4u32;
pub const BATTERY_CLASS_MAJOR_VERSION: u32 = 1u32;
pub const BATTERY_CLASS_MINOR_VERSION: u32 = 0u32;
pub const BATTERY_CLASS_MINOR_VERSION_1: u32 = 1u32;
pub const BATTERY_CRITICAL: u32 = 8u32;
pub const BATTERY_CYCLE_COUNT_WMI_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xef98db24_0014_4c25_a50b_c724ae5cd371);
pub const BATTERY_DISCHARGING: u32 = 2u32;
pub const BATTERY_FULL_CHARGED_CAPACITY_WMI_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x40b40565_96f7_4435_8694_97e0e4395905);
pub const BATTERY_IS_SHORT_TERM: u32 = 536870912u32;
pub const BATTERY_MINIPORT_UPDATE_DATA_VER_1: u32 = 1u32;
pub const BATTERY_MINIPORT_UPDATE_DATA_VER_2: u32 = 2u32;
pub const BATTERY_POWER_ON_LINE: u32 = 1u32;
pub const BATTERY_RUNTIME_WMI_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x535a3767_1ac2_49bc_a077_3f7a02e40aec);
pub const BATTERY_SEALED: u32 = 268435456u32;
pub const BATTERY_SET_CHARGER_ID_SUPPORTED: u32 = 8u32;
pub const BATTERY_SET_CHARGE_SUPPORTED: u32 = 1u32;
pub const BATTERY_SET_CHARGINGSOURCE_SUPPORTED: u32 = 4u32;
pub const BATTERY_SET_DISCHARGE_SUPPORTED: u32 = 2u32;
pub const BATTERY_STATIC_DATA_WMI_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x05e1e463_e4e2_4ea9_80cb_9bd4b3ca0655);
pub const BATTERY_STATUS_CHANGE_WMI_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcddfa0c3_7c5b_4e43_a034_059fa5b84364);
pub const BATTERY_STATUS_WMI_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfc4670d1_ebbf_416e_87ce_374a4ebc111a);
pub const BATTERY_SYSTEM_BATTERY: u32 = 2147483648u32;
pub const BATTERY_TAG_CHANGE_WMI_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5e1f6e19_8786_4d23_94fc_9e746bd5d888);
pub const BATTERY_TAG_INVALID: u32 = 0u32;
pub const BATTERY_TEMPERATURE_WMI_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1a52a14d_adce_4a44_9a3e_c8d8f15ff2c2);
pub const BATTERY_UNKNOWN_CAPACITY: u32 = 4294967295u32;
pub const BATTERY_UNKNOWN_CURRENT: u32 = 4294967295u32;
pub const BATTERY_UNKNOWN_RATE: u32 = 2147483648u32;
pub const BATTERY_UNKNOWN_TIME: u32 = 4294967295u32;
pub const BATTERY_UNKNOWN_VOLTAGE: u32 = 4294967295u32;
pub const BATTERY_USB_CHARGER_STATUS_FN_DEFAULT_USB: u32 = 1u32;
pub const BATTERY_USB_CHARGER_STATUS_UCM_PD: u32 = 2u32;
pub const BatteryCharge: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(1i32);
pub const BatteryChargerId: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(4i32);
pub const BatteryChargerStatus: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(5i32);
pub const BatteryChargingSource: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(3i32);
pub const BatteryChargingSourceType_AC: BATTERY_CHARGING_SOURCE_TYPE = BATTERY_CHARGING_SOURCE_TYPE(1i32);
pub const BatteryChargingSourceType_Max: BATTERY_CHARGING_SOURCE_TYPE = BATTERY_CHARGING_SOURCE_TYPE(4i32);
pub const BatteryChargingSourceType_USB: BATTERY_CHARGING_SOURCE_TYPE = BATTERY_CHARGING_SOURCE_TYPE(2i32);
pub const BatteryChargingSourceType_Wireless: BATTERY_CHARGING_SOURCE_TYPE = BATTERY_CHARGING_SOURCE_TYPE(3i32);
pub const BatteryCriticalBias: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(0i32);
pub const BatteryDeviceName: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(4i32);
pub const BatteryDeviceState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(86i32);
pub const BatteryDischarge: BATTERY_SET_INFORMATION_LEVEL = BATTERY_SET_INFORMATION_LEVEL(2i32);
pub const BatteryEstimatedTime: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(3i32);
pub const BatteryGranularityInformation: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(1i32);
pub const BatteryInformation: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(0i32);
pub const BatteryManufactureDate: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(5i32);
pub const BatteryManufactureName: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(6i32);
pub const BatterySerialNumber: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(8i32);
pub const BatteryTemperature: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(2i32);
pub const BatteryUniqueID: BATTERY_QUERY_INFORMATION_LEVEL = BATTERY_QUERY_INFORMATION_LEVEL(7i32);
pub const BlackBoxRecorderDirectAccessBuffer: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(97i32);
pub const CsDeviceNotification: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(74i32);
pub const DEVICEPOWER_AND_OPERATION: u32 = 1073741824u32;
pub const DEVICEPOWER_CLEAR_WAKEENABLED: u32 = 2u32;
pub const DEVICEPOWER_FILTER_DEVICES_PRESENT: u32 = 536870912u32;
pub const DEVICEPOWER_FILTER_HARDWARE: u32 = 268435456u32;
pub const DEVICEPOWER_FILTER_ON_NAME: u32 = 33554432u32;
pub const DEVICEPOWER_FILTER_WAKEENABLED: u32 = 134217728u32;
pub const DEVICEPOWER_FILTER_WAKEPROGRAMMABLE: u32 = 67108864u32;
pub const DEVICEPOWER_HARDWAREID: u32 = 2147483648u32;
pub const DEVICEPOWER_SET_WAKEENABLED: u32 = 1u32;
pub const DisplayBurst: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(77i32);
pub const EFFECTIVE_POWER_MODE_V1: u32 = 1u32;
pub const EFFECTIVE_POWER_MODE_V2: u32 = 2u32;
pub const EMI_NAME_MAX: u32 = 16u32;
pub const EMI_VERSION_V1: u32 = 1u32;
pub const EMI_VERSION_V2: u32 = 2u32;
pub const ES_AWAYMODE_REQUIRED: EXECUTION_STATE = EXECUTION_STATE(64u32);
pub const ES_CONTINUOUS: EXECUTION_STATE = EXECUTION_STATE(2147483648u32);
pub const ES_DISPLAY_REQUIRED: EXECUTION_STATE = EXECUTION_STATE(2u32);
pub const ES_SYSTEM_REQUIRED: EXECUTION_STATE = EXECUTION_STATE(1u32);
pub const ES_USER_PRESENT: EXECUTION_STATE = EXECUTION_STATE(4u32);
pub const EffectivePowerModeBalanced: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(2i32);
pub const EffectivePowerModeBatterySaver: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(0i32);
pub const EffectivePowerModeBetterBattery: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(1i32);
pub const EffectivePowerModeGameMode: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(5i32);
pub const EffectivePowerModeHighPerformance: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(3i32);
pub const EffectivePowerModeMaxPerformance: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(4i32);
pub const EffectivePowerModeMixedReality: EFFECTIVE_POWER_MODE = EFFECTIVE_POWER_MODE(6i32);
pub const EmiMeasurementUnitPicowattHours: EMI_MEASUREMENT_UNIT = EMI_MEASUREMENT_UNIT(0i32);
pub const EnableMultiBatteryDisplay: u32 = 2u32;
pub const EnablePasswordLogon: u32 = 4u32;
pub const EnableSysTrayBatteryMeter: u32 = 1u32;
pub const EnableVideoDimDisplay: u32 = 16u32;
pub const EnableWakeOnRing: u32 = 8u32;
pub const EnergyTrackerCreate: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(92i32);
pub const EnergyTrackerQuery: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(93i32);
pub const ExitLatencySamplingPercentage: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(78i32);
pub const FirmwareTableInformationRegistered: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(69i32);
pub const GUID_CLASS_INPUT: windows_core::GUID = windows_core::GUID::from_u128(0x4d1e55b2_f16f_11cf_88cb_001111000030);
pub const GUID_DEVICE_ACPI_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x97f99bf6_4497_4f18_bb22_4b9fb2fbef9c);
pub const GUID_DEVICE_APPLICATIONLAUNCH_BUTTON: windows_core::GUID = windows_core::GUID::from_u128(0x629758ee_986e_4d9e_8e47_de27f8ab054d);
pub const GUID_DEVICE_BATTERY: windows_core::GUID = windows_core::GUID::from_u128(0x72631e54_78a4_11d0_bcf7_00aa00b7b32a);
pub const GUID_DEVICE_ENERGY_METER: windows_core::GUID = windows_core::GUID::from_u128(0x45bd8344_7ed6_49cf_a440_c276c933b053);
pub const GUID_DEVICE_FAN: windows_core::GUID = windows_core::GUID::from_u128(0x05ecd13d_81da_4a2a_8a4c_524f23dd4dc9);
pub const GUID_DEVICE_LID: windows_core::GUID = windows_core::GUID::from_u128(0x4afa3d52_74a7_11d0_be5e_00a0c9062857);
pub const GUID_DEVICE_MEMORY: windows_core::GUID = windows_core::GUID::from_u128(0x3fd0f03d_92e0_45fb_b75c_5ed8ffb01021);
pub const GUID_DEVICE_MESSAGE_INDICATOR: windows_core::GUID = windows_core::GUID::from_u128(0xcd48a365_fa94_4ce2_a232_a1b764e5d8b4);
pub const GUID_DEVICE_PROCESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x97fadb10_4e33_40ae_359c_8bef029dbdd0);
pub const GUID_DEVICE_SYS_BUTTON: windows_core::GUID = windows_core::GUID::from_u128(0x4afa3d53_74a7_11d0_be5e_00a0c9062857);
pub const GUID_DEVICE_THERMAL_ZONE: windows_core::GUID = windows_core::GUID::from_u128(0x4afa3d51_74a7_11d0_be5e_00a0c9062857);
pub const GUID_DEVINTERFACE_THERMAL_COOLING: windows_core::GUID = windows_core::GUID::from_u128(0xdbe4373d_3c81_40cb_ace4_e0e5d05f0c9f);
pub const GUID_DEVINTERFACE_THERMAL_MANAGER: windows_core::GUID = windows_core::GUID::from_u128(0x927ec093_69a4_4bc0_bd02_711664714463);
pub const GetPowerRequestList: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(45i32);
pub const GetPowerSettingValue: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(59i32);
pub const GroupPark: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(48i32);
pub const IOCTL_ACPI_GET_REAL_TIME: u32 = 2703888u32;
pub const IOCTL_ACPI_SET_REAL_TIME: u32 = 2720276u32;
pub const IOCTL_BATTERY_CHARGING_SOURCE_CHANGE: u32 = 2703440u32;
pub const IOCTL_BATTERY_QUERY_INFORMATION: u32 = 2703428u32;
pub const IOCTL_BATTERY_QUERY_STATUS: u32 = 2703436u32;
pub const IOCTL_BATTERY_QUERY_TAG: u32 = 2703424u32;
pub const IOCTL_BATTERY_SET_INFORMATION: u32 = 2719816u32;
pub const IOCTL_EMI_GET_MEASUREMENT: u32 = 2244620u32;
pub const IOCTL_EMI_GET_METADATA: u32 = 2244616u32;
pub const IOCTL_EMI_GET_METADATA_SIZE: u32 = 2244612u32;
pub const IOCTL_EMI_GET_VERSION: u32 = 2244608u32;
pub const IOCTL_GET_ACPI_TIME_AND_ALARM_CAPABILITIES: u32 = 2703900u32;
pub const IOCTL_GET_PROCESSOR_OBJ_INFO: u32 = 2703744u32;
pub const IOCTL_GET_SYS_BUTTON_CAPS: u32 = 2703680u32;
pub const IOCTL_GET_SYS_BUTTON_EVENT: u32 = 2703684u32;
pub const IOCTL_GET_WAKE_ALARM_POLICY: u32 = 2736652u32;
pub const IOCTL_GET_WAKE_ALARM_SYSTEM_POWERSTATE: u32 = 2703896u32;
pub const IOCTL_GET_WAKE_ALARM_VALUE: u32 = 2736648u32;
pub const IOCTL_NOTIFY_SWITCH_EVENT: u32 = 2703616u32;
pub const IOCTL_QUERY_LID: u32 = 2703552u32;
pub const IOCTL_RUN_ACTIVE_COOLING_METHOD: u32 = 2719880u32;
pub const IOCTL_SET_SYS_MESSAGE_INDICATOR: u32 = 2720192u32;
pub const IOCTL_SET_WAKE_ALARM_POLICY: u32 = 2720260u32;
pub const IOCTL_SET_WAKE_ALARM_VALUE: u32 = 2720256u32;
pub const IOCTL_THERMAL_QUERY_INFORMATION: u32 = 2703488u32;
pub const IOCTL_THERMAL_READ_POLICY: u32 = 2703508u32;
pub const IOCTL_THERMAL_READ_TEMPERATURE: u32 = 2703504u32;
pub const IOCTL_THERMAL_SET_COOLING_POLICY: u32 = 2719876u32;
pub const IOCTL_THERMAL_SET_PASSIVE_LIMIT: u32 = 2719884u32;
pub const IdleResiliency: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(60i32);
pub const LT_DONT_CARE: LATENCY_TIME = LATENCY_TIME(0i32);
pub const LT_LOWEST_LATENCY: LATENCY_TIME = LATENCY_TIME(1i32);
pub const LastResumePerformance: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(76i32);
pub const LastSleepTime: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(15i32);
pub const LastWakeTime: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(14i32);
pub const LogicalProcessorIdling: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(56i32);
pub const MAX_ACTIVE_COOLING_LEVELS: u32 = 10u32;
pub const MAX_BATTERY_STRING_SIZE: u32 = 128u32;
pub const MonitorCapabilities: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(40i32);
pub const MonitorInvocation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(68i32);
pub const MonitorRequestReasonAcDcDisplayBurst: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(5i32);
pub const MonitorRequestReasonAcDcDisplayBurstSuppressed: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(28i32);
pub const MonitorRequestReasonBatteryCountChange: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(16i32);
pub const MonitorRequestReasonBatteryCountChangeSuppressed: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(49i32);
pub const MonitorRequestReasonBatteryPreCritical: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(53i32);
pub const MonitorRequestReasonBuiltinPanel: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(47i32);
pub const MonitorRequestReasonDP: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(19i32);
pub const MonitorRequestReasonDim: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(46i32);
pub const MonitorRequestReasonDirectedDrips: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(45i32);
pub const MonitorRequestReasonDisplayRequiredUnDim: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(48i32);
pub const MonitorRequestReasonFullWake: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(9i32);
pub const MonitorRequestReasonGracePeriod: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(17i32);
pub const MonitorRequestReasonIdleTimeout: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(12i32);
pub const MonitorRequestReasonLid: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(15i32);
pub const MonitorRequestReasonMax: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(55i32);
pub const MonitorRequestReasonNearProximity: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(22i32);
pub const MonitorRequestReasonPdcSignal: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(27i32);
pub const MonitorRequestReasonPdcSignalFingerprint: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(44i32);
pub const MonitorRequestReasonPdcSignalHeyCortana: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(42i32);
pub const MonitorRequestReasonPdcSignalHolographicShell: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(43i32);
pub const MonitorRequestReasonPdcSignalSensorsHumanPresence: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(52i32);
pub const MonitorRequestReasonPdcSignalWindowsMobilePwrNotif: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(40i32);
pub const MonitorRequestReasonPdcSignalWindowsMobileShell: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(41i32);
pub const MonitorRequestReasonPnP: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(18i32);
pub const MonitorRequestReasonPoSetSystemState: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(7i32);
pub const MonitorRequestReasonPolicyChange: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(13i32);
pub const MonitorRequestReasonPowerButton: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(1i32);
pub const MonitorRequestReasonRemoteConnection: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(2i32);
pub const MonitorRequestReasonResumeModernStandby: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(50i32);
pub const MonitorRequestReasonResumePdc: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(24i32);
pub const MonitorRequestReasonResumeS4: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(25i32);
pub const MonitorRequestReasonScMonitorpower: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(3i32);
pub const MonitorRequestReasonScreenOffRequest: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(11i32);
pub const MonitorRequestReasonSessionUnlock: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(10i32);
pub const MonitorRequestReasonSetThreadExecutionState: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(8i32);
pub const MonitorRequestReasonSleepButton: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(14i32);
pub const MonitorRequestReasonSxTransition: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(20i32);
pub const MonitorRequestReasonSystemIdle: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(21i32);
pub const MonitorRequestReasonSystemStateEntered: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(29i32);
pub const MonitorRequestReasonTerminal: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(26i32);
pub const MonitorRequestReasonTerminalInit: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(51i32);
pub const MonitorRequestReasonThermalStandby: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(23i32);
pub const MonitorRequestReasonUnknown: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(0i32);
pub const MonitorRequestReasonUserDisplayBurst: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(6i32);
pub const MonitorRequestReasonUserInput: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(4i32);
pub const MonitorRequestReasonUserInputAccelerometer: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(35i32);
pub const MonitorRequestReasonUserInputHid: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(36i32);
pub const MonitorRequestReasonUserInputInitialization: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(39i32);
pub const MonitorRequestReasonUserInputKeyboard: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(31i32);
pub const MonitorRequestReasonUserInputMouse: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(32i32);
pub const MonitorRequestReasonUserInputPen: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(34i32);
pub const MonitorRequestReasonUserInputPoUserPresent: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(37i32);
pub const MonitorRequestReasonUserInputSessionSwitch: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(38i32);
pub const MonitorRequestReasonUserInputTouch: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(54i32);
pub const MonitorRequestReasonUserInputTouchpad: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(33i32);
pub const MonitorRequestReasonWinrt: POWER_MONITOR_REQUEST_REASON = POWER_MONITOR_REQUEST_REASON(30i32);
pub const MonitorRequestTypeOff: POWER_MONITOR_REQUEST_TYPE = POWER_MONITOR_REQUEST_TYPE(0i32);
pub const MonitorRequestTypeOnAndPresent: POWER_MONITOR_REQUEST_TYPE = POWER_MONITOR_REQUEST_TYPE(1i32);
pub const MonitorRequestTypeToggleOn: POWER_MONITOR_REQUEST_TYPE = POWER_MONITOR_REQUEST_TYPE(2i32);
pub const NotifyUserModeLegacyPowerEvent: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(47i32);
pub const NotifyUserPowerSetting: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(26i32);
pub const PASSIVE_COOLING: u32 = 1u32;
pub const PDCAP_S0_SUPPORTED: u32 = 65536u32;
pub const PDCAP_S1_SUPPORTED: u32 = 131072u32;
pub const PDCAP_S2_SUPPORTED: u32 = 262144u32;
pub const PDCAP_S3_SUPPORTED: u32 = 524288u32;
pub const PDCAP_S4_SUPPORTED: u32 = 16777216u32;
pub const PDCAP_S5_SUPPORTED: u32 = 33554432u32;
pub const PDCAP_WAKE_FROM_S0_SUPPORTED: u32 = 1048576u32;
pub const PDCAP_WAKE_FROM_S1_SUPPORTED: u32 = 2097152u32;
pub const PDCAP_WAKE_FROM_S2_SUPPORTED: u32 = 4194304u32;
pub const PDCAP_WAKE_FROM_S3_SUPPORTED: u32 = 8388608u32;
pub const POWER_ATTRIBUTE_HIDE: u32 = 1u32;
pub const POWER_ATTRIBUTE_SHOW_AOAC: u32 = 2u32;
pub const POWER_FORCE_TRIGGER_RESET: POWER_ACTION_POLICY_EVENT_CODE = POWER_ACTION_POLICY_EVENT_CODE(2147483648u32);
pub const POWER_LEVEL_USER_NOTIFY_EXEC: POWER_ACTION_POLICY_EVENT_CODE = POWER_ACTION_POLICY_EVENT_CODE(4u32);
pub const POWER_LEVEL_USER_NOTIFY_SOUND: POWER_ACTION_POLICY_EVENT_CODE = POWER_ACTION_POLICY_EVENT_CODE(2u32);
pub const POWER_LEVEL_USER_NOTIFY_TEXT: POWER_ACTION_POLICY_EVENT_CODE = POWER_ACTION_POLICY_EVENT_CODE(1u32);
pub const POWER_PLATFORM_ROLE_V1: POWER_PLATFORM_ROLE_VERSION = POWER_PLATFORM_ROLE_VERSION(1u32);
pub const POWER_PLATFORM_ROLE_V2: POWER_PLATFORM_ROLE_VERSION = POWER_PLATFORM_ROLE_VERSION(2u32);
pub const POWER_USER_NOTIFY_BUTTON: POWER_ACTION_POLICY_EVENT_CODE = POWER_ACTION_POLICY_EVENT_CODE(8u32);
pub const POWER_USER_NOTIFY_SHUTDOWN: POWER_ACTION_POLICY_EVENT_CODE = POWER_ACTION_POLICY_EVENT_CODE(16u32);
pub const PO_TZ_ACTIVE: POWER_COOLING_MODE = POWER_COOLING_MODE(0u16);
pub const PO_TZ_INVALID_MODE: POWER_COOLING_MODE = POWER_COOLING_MODE(2u16);
pub const PO_TZ_PASSIVE: POWER_COOLING_MODE = POWER_COOLING_MODE(1u16);
pub const PPM_FIRMWARE_ACPI1C2: u32 = 1u32;
pub const PPM_FIRMWARE_ACPI1C3: u32 = 2u32;
pub const PPM_FIRMWARE_ACPI1TSTATES: u32 = 4u32;
pub const PPM_FIRMWARE_CPC: u32 = 262144u32;
pub const PPM_FIRMWARE_CSD: u32 = 16u32;
pub const PPM_FIRMWARE_CST: u32 = 8u32;
pub const PPM_FIRMWARE_LPI: u32 = 524288u32;
pub const PPM_FIRMWARE_OSC: u32 = 65536u32;
pub const PPM_FIRMWARE_PCCH: u32 = 16384u32;
pub const PPM_FIRMWARE_PCCP: u32 = 32768u32;
pub const PPM_FIRMWARE_PCT: u32 = 32u32;
pub const PPM_FIRMWARE_PDC: u32 = 131072u32;
pub const PPM_FIRMWARE_PPC: u32 = 256u32;
pub const PPM_FIRMWARE_PSD: u32 = 512u32;
pub const PPM_FIRMWARE_PSS: u32 = 64u32;
pub const PPM_FIRMWARE_PTC: u32 = 1024u32;
pub const PPM_FIRMWARE_TPC: u32 = 4096u32;
pub const PPM_FIRMWARE_TSD: u32 = 8192u32;
pub const PPM_FIRMWARE_TSS: u32 = 2048u32;
pub const PPM_FIRMWARE_XPSS: u32 = 128u32;
pub const PPM_IDLESTATES_DATA_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xba138e10_e250_4ad7_8616_cf1a7ad410e7);
pub const PPM_IDLESTATE_CHANGE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4838fe4f_f71c_4e51_9ecc_8430a7ac4c6c);
pub const PPM_IDLE_ACCOUNTING_EX_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd67abd39_81f8_4a5e_8152_72e31ec912ee);
pub const PPM_IDLE_ACCOUNTING_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe2a26f78_ae07_4ee0_a30f_ce54f55a94cd);
pub const PPM_IDLE_IMPLEMENTATION_CSTATES: u32 = 1u32;
pub const PPM_IDLE_IMPLEMENTATION_LPISTATES: u32 = 4u32;
pub const PPM_IDLE_IMPLEMENTATION_MICROPEP: u32 = 3u32;
pub const PPM_IDLE_IMPLEMENTATION_NONE: u32 = 0u32;
pub const PPM_IDLE_IMPLEMENTATION_PEP: u32 = 2u32;
pub const PPM_PERFMON_PERFSTATE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7fd18652_0cfe_40d2_b0a1_0b066a87759e);
pub const PPM_PERFORMANCE_IMPLEMENTATION_CPPC: u32 = 3u32;
pub const PPM_PERFORMANCE_IMPLEMENTATION_NONE: u32 = 0u32;
pub const PPM_PERFORMANCE_IMPLEMENTATION_PCCV1: u32 = 2u32;
pub const PPM_PERFORMANCE_IMPLEMENTATION_PEP: u32 = 4u32;
pub const PPM_PERFORMANCE_IMPLEMENTATION_PSTATES: u32 = 1u32;
pub const PPM_PERFSTATES_DATA_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5708cc20_7d40_4bf4_b4aa_2b01338d0126);
pub const PPM_PERFSTATE_CHANGE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa5b32ddd_7f39_4abc_b892_900e43b59ebb);
pub const PPM_PERFSTATE_DOMAIN_CHANGE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x995e6b7f_d653_497a_b978_36a30c29bf01);
pub const PPM_THERMALCONSTRAINT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa852c2c8_1a4c_423b_8c2c_f30d82931a88);
pub const PPM_THERMAL_POLICY_CHANGE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x48f377b8_6880_4c7b_8bdc_380176c6654d);
#[cfg(feature = "Win32_Devices_Properties")]
pub const PROCESSOR_NUMBER_PKEY: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x5724c81d_d5af_4c1f_a103_a06e28f204c6), pid: 1 };
pub const PdcInvocation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(67i32);
pub const PhysicalPowerButtonPress: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(90i32);
pub const PlatformIdleStates: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(80i32);
pub const PlatformIdleVeto: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(82i32);
pub const PlatformInformation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(66i32);
pub const PlatformRole: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(75i32);
pub const PlatformRoleAppliancePC: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(6i32);
pub const PlatformRoleDesktop: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(1i32);
pub const PlatformRoleEnterpriseServer: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(4i32);
pub const PlatformRoleMaximum: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(9i32);
pub const PlatformRoleMobile: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(2i32);
pub const PlatformRolePerformanceServer: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(7i32);
pub const PlatformRoleSOHOServer: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(5i32);
pub const PlatformRoleSlate: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(8i32);
pub const PlatformRoleUnspecified: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(0i32);
pub const PlatformRoleWorkstation: POWER_PLATFORM_ROLE = POWER_PLATFORM_ROLE(3i32);
pub const PlmPowerRequestCreate: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(72i32);
pub const PoAc: SYSTEM_POWER_CONDITION = SYSTEM_POWER_CONDITION(0i32);
pub const PoConditionMaximum: SYSTEM_POWER_CONDITION = SYSTEM_POWER_CONDITION(3i32);
pub const PoDc: SYSTEM_POWER_CONDITION = SYSTEM_POWER_CONDITION(1i32);
pub const PoHot: SYSTEM_POWER_CONDITION = SYSTEM_POWER_CONDITION(2i32);
pub const PowerActionDisplayOff: POWER_ACTION = POWER_ACTION(8i32);
pub const PowerActionHibernate: POWER_ACTION = POWER_ACTION(3i32);
pub const PowerActionNone: POWER_ACTION = POWER_ACTION(0i32);
pub const PowerActionReserved: POWER_ACTION = POWER_ACTION(1i32);
pub const PowerActionShutdown: POWER_ACTION = POWER_ACTION(4i32);
pub const PowerActionShutdownOff: POWER_ACTION = POWER_ACTION(6i32);
pub const PowerActionShutdownReset: POWER_ACTION = POWER_ACTION(5i32);
pub const PowerActionSleep: POWER_ACTION = POWER_ACTION(2i32);
pub const PowerActionWarmEject: POWER_ACTION = POWER_ACTION(7i32);
pub const PowerDeviceD0: DEVICE_POWER_STATE = DEVICE_POWER_STATE(1i32);
pub const PowerDeviceD1: DEVICE_POWER_STATE = DEVICE_POWER_STATE(2i32);
pub const PowerDeviceD2: DEVICE_POWER_STATE = DEVICE_POWER_STATE(3i32);
pub const PowerDeviceD3: DEVICE_POWER_STATE = DEVICE_POWER_STATE(4i32);
pub const PowerDeviceMaximum: DEVICE_POWER_STATE = DEVICE_POWER_STATE(5i32);
pub const PowerDeviceUnspecified: DEVICE_POWER_STATE = DEVICE_POWER_STATE(0i32);
pub const PowerInformationInternal: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(87i32);
pub const PowerInformationLevelMaximum: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(98i32);
pub const PowerInformationLevelUnused0: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(27i32);
pub const PowerRequestAction: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(44i32);
pub const PowerRequestActionInternal: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(85i32);
pub const PowerRequestAwayModeRequired: POWER_REQUEST_TYPE = POWER_REQUEST_TYPE(2i32);
pub const PowerRequestCreate: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(43i32);
pub const PowerRequestDisplayRequired: POWER_REQUEST_TYPE = POWER_REQUEST_TYPE(0i32);
pub const PowerRequestExecutionRequired: POWER_REQUEST_TYPE = POWER_REQUEST_TYPE(3i32);
pub const PowerRequestSystemRequired: POWER_REQUEST_TYPE = POWER_REQUEST_TYPE(1i32);
pub const PowerSettingNotificationName: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(58i32);
pub const PowerShutdownNotification: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(39i32);
pub const PowerSystemHibernate: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(5i32);
pub const PowerSystemMaximum: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(7i32);
pub const PowerSystemShutdown: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(6i32);
pub const PowerSystemSleeping1: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(2i32);
pub const PowerSystemSleeping2: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(3i32);
pub const PowerSystemSleeping3: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(4i32);
pub const PowerSystemUnspecified: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(0i32);
pub const PowerSystemWorking: SYSTEM_POWER_STATE = SYSTEM_POWER_STATE(1i32);
pub const PowerUserInactive: USER_ACTIVITY_PRESENCE = USER_ACTIVITY_PRESENCE(2i32);
pub const PowerUserInvalid: USER_ACTIVITY_PRESENCE = USER_ACTIVITY_PRESENCE(3i32);
pub const PowerUserMaximum: USER_ACTIVITY_PRESENCE = USER_ACTIVITY_PRESENCE(3i32);
pub const PowerUserNotPresent: USER_ACTIVITY_PRESENCE = USER_ACTIVITY_PRESENCE(1i32);
pub const PowerUserPresent: USER_ACTIVITY_PRESENCE = USER_ACTIVITY_PRESENCE(0i32);
pub const ProcessorCap: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(34i32);
pub const ProcessorIdleDomains: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(49i32);
pub const ProcessorIdleStates: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(33i32);
pub const ProcessorIdleStatesHv: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(52i32);
pub const ProcessorIdleVeto: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(81i32);
pub const ProcessorInformation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(11i32);
pub const ProcessorInformationEx: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(46i32);
pub const ProcessorLoad: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(38i32);
pub const ProcessorPerfCapHv: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(54i32);
pub const ProcessorPerfStates: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(32i32);
pub const ProcessorPerfStatesHv: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(53i32);
pub const ProcessorPowerPolicyAc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(18i32);
pub const ProcessorPowerPolicyCurrent: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(22i32);
pub const ProcessorPowerPolicyDc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(19i32);
pub const ProcessorSetIdle: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(55i32);
pub const ProcessorStateHandler: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(7i32);
pub const ProcessorStateHandler2: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(13i32);
pub const QueryPotentialDripsConstraint: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(91i32);
pub const RegisterSpmPowerSettings: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(79i32);
pub const SYS_BUTTON_LID: u32 = 4u32;
pub const SYS_BUTTON_LID_CHANGED: u32 = 524288u32;
pub const SYS_BUTTON_LID_CLOSED: u32 = 131072u32;
pub const SYS_BUTTON_LID_INITIAL: u32 = 262144u32;
pub const SYS_BUTTON_LID_OPEN: u32 = 65536u32;
pub const SYS_BUTTON_LID_STATE_MASK: u32 = 196608u32;
pub const SYS_BUTTON_POWER: u32 = 1u32;
pub const SYS_BUTTON_SLEEP: u32 = 2u32;
pub const SYS_BUTTON_WAKE: u32 = 2147483648u32;
pub const ScreenOff: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(73i32);
pub const SendSuspendResumeNotification: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(96i32);
pub const SessionAllowExternalDmaDevices: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(95i32);
pub const SessionConnectNotification: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(62i32);
pub const SessionDisplayState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(42i32);
pub const SessionLockState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(64i32);
pub const SessionPowerCleanup: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(63i32);
pub const SessionPowerInit: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(41i32);
pub const SessionRITState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(61i32);
pub const SetPowerSettingValue: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(25i32);
pub const SetShutdownSelectedTime: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(70i32);
pub const SuspendResumeInvocation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(71i32);
pub const SystemBatteryState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(5i32);
pub const SystemBatteryStatePrecise: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(83i32);
pub const SystemExecutionState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(16i32);
pub const SystemHiberFileInformation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(36i32);
pub const SystemHiberFileSize: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(51i32);
pub const SystemHiberFileType: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(89i32);
pub const SystemHiberbootState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(65i32);
pub const SystemMonitorHiberBootPowerOff: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(28i32);
pub const SystemPowerCapabilities: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(4i32);
pub const SystemPowerInformation: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(12i32);
pub const SystemPowerLoggingEntry: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(24i32);
pub const SystemPowerPolicyAc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(0i32);
pub const SystemPowerPolicyCurrent: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(8i32);
pub const SystemPowerPolicyDc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(1i32);
pub const SystemPowerStateHandler: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(6i32);
pub const SystemPowerStateLogging: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(23i32);
pub const SystemPowerStateNotifyHandler: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(17i32);
pub const SystemReserveHiberFile: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(10i32);
pub const SystemVideoState: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(29i32);
pub const SystemWakeSource: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(35i32);
pub const THERMAL_COOLING_INTERFACE_VERSION: u32 = 1u32;
pub const THERMAL_DEVICE_INTERFACE_VERSION: u32 = 1u32;
pub const THERMAL_EVENT_VERSION: u32 = 1u32;
pub const THERMAL_POLICY_VERSION_1: u32 = 1u32;
pub const THERMAL_POLICY_VERSION_2: u32 = 2u32;
pub const TZ_ACTIVATION_REASON_CURRENT: u32 = 2u32;
pub const TZ_ACTIVATION_REASON_THERMAL: u32 = 1u32;
pub const ThermalEvent: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(84i32);
pub const ThermalStandby: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(88i32);
pub const TraceApplicationPowerMessage: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(30i32);
pub const TraceApplicationPowerMessageEnd: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(31i32);
pub const TraceServicePowerMessage: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(37i32);
pub const UNKNOWN_CAPACITY: u32 = 4294967295u32;
pub const UNKNOWN_CURRENT: u32 = 4294967295u32;
pub const UNKNOWN_RATE: u32 = 2147483648u32;
pub const UNKNOWN_VOLTAGE: u32 = 4294967295u32;
pub const UpdateBlackBoxRecorder: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(94i32);
pub const UsbChargerPort_Legacy: USB_CHARGER_PORT = USB_CHARGER_PORT(0i32);
pub const UsbChargerPort_Max: USB_CHARGER_PORT = USB_CHARGER_PORT(2i32);
pub const UsbChargerPort_TypeC: USB_CHARGER_PORT = USB_CHARGER_PORT(1i32);
pub const UserNotPresent: POWER_USER_PRESENCE_TYPE = POWER_USER_PRESENCE_TYPE(0i32);
pub const UserPresence: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(57i32);
pub const UserPresent: POWER_USER_PRESENCE_TYPE = POWER_USER_PRESENCE_TYPE(1i32);
pub const UserUnknown: POWER_USER_PRESENCE_TYPE = POWER_USER_PRESENCE_TYPE(255i32);
pub const VerifyProcessorPowerPolicyAc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(20i32);
pub const VerifyProcessorPowerPolicyDc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(21i32);
pub const VerifySystemPolicyAc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(2i32);
pub const VerifySystemPolicyDc: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(3i32);
pub const WakeTimerList: POWER_INFORMATION_LEVEL = POWER_INFORMATION_LEVEL(50i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACPI_TIME_RESOLUTION(pub i32);
impl windows_core::TypeKind for ACPI_TIME_RESOLUTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACPI_TIME_RESOLUTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACPI_TIME_RESOLUTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BATTERY_CHARGING_SOURCE_TYPE(pub i32);
impl windows_core::TypeKind for BATTERY_CHARGING_SOURCE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BATTERY_CHARGING_SOURCE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BATTERY_CHARGING_SOURCE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BATTERY_QUERY_INFORMATION_LEVEL(pub i32);
impl windows_core::TypeKind for BATTERY_QUERY_INFORMATION_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BATTERY_QUERY_INFORMATION_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BATTERY_QUERY_INFORMATION_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BATTERY_SET_INFORMATION_LEVEL(pub i32);
impl windows_core::TypeKind for BATTERY_SET_INFORMATION_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BATTERY_SET_INFORMATION_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BATTERY_SET_INFORMATION_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DEVICE_POWER_STATE(pub i32);
impl windows_core::TypeKind for DEVICE_POWER_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DEVICE_POWER_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DEVICE_POWER_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EFFECTIVE_POWER_MODE(pub i32);
impl windows_core::TypeKind for EFFECTIVE_POWER_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EFFECTIVE_POWER_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EFFECTIVE_POWER_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EMI_MEASUREMENT_UNIT(pub i32);
impl windows_core::TypeKind for EMI_MEASUREMENT_UNIT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EMI_MEASUREMENT_UNIT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EMI_MEASUREMENT_UNIT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EXECUTION_STATE(pub u32);
impl windows_core::TypeKind for EXECUTION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EXECUTION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EXECUTION_STATE").field(&self.0).finish()
    }
}
impl EXECUTION_STATE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for EXECUTION_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for EXECUTION_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for EXECUTION_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for EXECUTION_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for EXECUTION_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LATENCY_TIME(pub i32);
impl windows_core::TypeKind for LATENCY_TIME {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LATENCY_TIME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LATENCY_TIME").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POWER_ACTION(pub i32);
impl windows_core::TypeKind for POWER_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POWER_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POWER_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POWER_ACTION_POLICY_EVENT_CODE(pub u32);
impl windows_core::TypeKind for POWER_ACTION_POLICY_EVENT_CODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POWER_ACTION_POLICY_EVENT_CODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POWER_ACTION_POLICY_EVENT_CODE").field(&self.0).finish()
    }
}
impl POWER_ACTION_POLICY_EVENT_CODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for POWER_ACTION_POLICY_EVENT_CODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for POWER_ACTION_POLICY_EVENT_CODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for POWER_ACTION_POLICY_EVENT_CODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for POWER_ACTION_POLICY_EVENT_CODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for POWER_ACTION_POLICY_EVENT_CODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POWER_COOLING_MODE(pub u16);
impl windows_core::TypeKind for POWER_COOLING_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POWER_COOLING_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POWER_COOLING_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POWER_DATA_ACCESSOR(pub i32);
impl windows_core::TypeKind for POWER_DATA_ACCESSOR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POWER_DATA_ACCESSOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POWER_DATA_ACCESSOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POWER_INFORMATION_LEVEL(pub i32);
impl windows_core::TypeKind for POWER_INFORMATION_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POWER_INFORMATION_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POWER_INFORMATION_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POWER_MONITOR_REQUEST_REASON(pub i32);
impl windows_core::TypeKind for POWER_MONITOR_REQUEST_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POWER_MONITOR_REQUEST_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POWER_MONITOR_REQUEST_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POWER_MONITOR_REQUEST_TYPE(pub i32);
impl windows_core::TypeKind for POWER_MONITOR_REQUEST_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POWER_MONITOR_REQUEST_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POWER_MONITOR_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POWER_PLATFORM_ROLE(pub i32);
impl windows_core::TypeKind for POWER_PLATFORM_ROLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POWER_PLATFORM_ROLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POWER_PLATFORM_ROLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POWER_PLATFORM_ROLE_VERSION(pub u32);
impl windows_core::TypeKind for POWER_PLATFORM_ROLE_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POWER_PLATFORM_ROLE_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POWER_PLATFORM_ROLE_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POWER_REQUEST_TYPE(pub i32);
impl windows_core::TypeKind for POWER_REQUEST_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POWER_REQUEST_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POWER_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POWER_SETTING_ALTITUDE(pub i32);
impl windows_core::TypeKind for POWER_SETTING_ALTITUDE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POWER_SETTING_ALTITUDE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POWER_SETTING_ALTITUDE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POWER_USER_PRESENCE_TYPE(pub i32);
impl windows_core::TypeKind for POWER_USER_PRESENCE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POWER_USER_PRESENCE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POWER_USER_PRESENCE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYSTEM_POWER_CONDITION(pub i32);
impl windows_core::TypeKind for SYSTEM_POWER_CONDITION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYSTEM_POWER_CONDITION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYSTEM_POWER_CONDITION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYSTEM_POWER_STATE(pub i32);
impl windows_core::TypeKind for SYSTEM_POWER_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYSTEM_POWER_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYSTEM_POWER_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USB_CHARGER_PORT(pub i32);
impl windows_core::TypeKind for USB_CHARGER_PORT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USB_CHARGER_PORT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USB_CHARGER_PORT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USER_ACTIVITY_PRESENCE(pub i32);
impl windows_core::TypeKind for USER_ACTIVITY_PRESENCE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USER_ACTIVITY_PRESENCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USER_ACTIVITY_PRESENCE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACPI_REAL_TIME {
    pub Year: u16,
    pub Month: u8,
    pub Day: u8,
    pub Hour: u8,
    pub Minute: u8,
    pub Second: u8,
    pub Valid: u8,
    pub Milliseconds: u16,
    pub TimeZone: i16,
    pub DayLight: u8,
    pub Reserved1: [u8; 3],
}
impl windows_core::TypeKind for ACPI_REAL_TIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACPI_REAL_TIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACPI_TIME_AND_ALARM_CAPABILITIES {
    pub AcWakeSupported: super::super::Foundation::BOOLEAN,
    pub DcWakeSupported: super::super::Foundation::BOOLEAN,
    pub S4AcWakeSupported: super::super::Foundation::BOOLEAN,
    pub S4DcWakeSupported: super::super::Foundation::BOOLEAN,
    pub S5AcWakeSupported: super::super::Foundation::BOOLEAN,
    pub S5DcWakeSupported: super::super::Foundation::BOOLEAN,
    pub S4S5WakeStatusSupported: super::super::Foundation::BOOLEAN,
    pub DeepestWakeSystemState: u32,
    pub RealTimeFeaturesSupported: super::super::Foundation::BOOLEAN,
    pub RealTimeResolution: ACPI_TIME_RESOLUTION,
}
impl windows_core::TypeKind for ACPI_TIME_AND_ALARM_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACPI_TIME_AND_ALARM_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADMINISTRATOR_POWER_POLICY {
    pub MinSleep: SYSTEM_POWER_STATE,
    pub MaxSleep: SYSTEM_POWER_STATE,
    pub MinVideoTimeout: u32,
    pub MaxVideoTimeout: u32,
    pub MinSpindownTimeout: u32,
    pub MaxSpindownTimeout: u32,
}
impl windows_core::TypeKind for ADMINISTRATOR_POWER_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADMINISTRATOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BATTERY_CHARGER_STATUS {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub VaData: [u32; 1],
}
impl windows_core::TypeKind for BATTERY_CHARGER_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for BATTERY_CHARGER_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BATTERY_CHARGING_SOURCE {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub MaxCurrent: u32,
}
impl windows_core::TypeKind for BATTERY_CHARGING_SOURCE {
    type TypeKind = windows_core::CopyType;
}
impl Default for BATTERY_CHARGING_SOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BATTERY_CHARGING_SOURCE_INFORMATION {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub SourceOnline: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for BATTERY_CHARGING_SOURCE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for BATTERY_CHARGING_SOURCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BATTERY_INFORMATION {
    pub Capabilities: u32,
    pub Technology: u8,
    pub Reserved: [u8; 3],
    pub Chemistry: [u8; 4],
    pub DesignedCapacity: u32,
    pub FullChargedCapacity: u32,
    pub DefaultAlert1: u32,
    pub DefaultAlert2: u32,
    pub CriticalBias: u32,
    pub CycleCount: u32,
}
impl windows_core::TypeKind for BATTERY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for BATTERY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BATTERY_MANUFACTURE_DATE {
    pub Day: u8,
    pub Month: u8,
    pub Year: u16,
}
impl windows_core::TypeKind for BATTERY_MANUFACTURE_DATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for BATTERY_MANUFACTURE_DATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BATTERY_QUERY_INFORMATION {
    pub BatteryTag: u32,
    pub InformationLevel: BATTERY_QUERY_INFORMATION_LEVEL,
    pub AtRate: u32,
}
impl windows_core::TypeKind for BATTERY_QUERY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for BATTERY_QUERY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BATTERY_REPORTING_SCALE {
    pub Granularity: u32,
    pub Capacity: u32,
}
impl windows_core::TypeKind for BATTERY_REPORTING_SCALE {
    type TypeKind = windows_core::CopyType;
}
impl Default for BATTERY_REPORTING_SCALE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BATTERY_SET_INFORMATION {
    pub BatteryTag: u32,
    pub InformationLevel: BATTERY_SET_INFORMATION_LEVEL,
    pub Buffer: [u8; 1],
}
impl windows_core::TypeKind for BATTERY_SET_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for BATTERY_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BATTERY_STATUS {
    pub PowerState: u32,
    pub Capacity: u32,
    pub Voltage: u32,
    pub Rate: i32,
}
impl windows_core::TypeKind for BATTERY_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for BATTERY_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BATTERY_USB_CHARGER_STATUS {
    pub Type: BATTERY_CHARGING_SOURCE_TYPE,
    pub Reserved: u32,
    pub Flags: u32,
    pub MaxCurrent: u32,
    pub Voltage: u32,
    pub PortType: USB_CHARGER_PORT,
    pub PortId: u64,
    pub PowerSourceInformation: *mut core::ffi::c_void,
    pub OemCharger: windows_core::GUID,
}
impl windows_core::TypeKind for BATTERY_USB_CHARGER_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for BATTERY_USB_CHARGER_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BATTERY_WAIT_STATUS {
    pub BatteryTag: u32,
    pub Timeout: u32,
    pub PowerState: u32,
    pub LowCapacity: u32,
    pub HighCapacity: u32,
}
impl windows_core::TypeKind for BATTERY_WAIT_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for BATTERY_WAIT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CM_POWER_DATA {
    pub PD_Size: u32,
    pub PD_MostRecentPowerState: DEVICE_POWER_STATE,
    pub PD_Capabilities: u32,
    pub PD_D1Latency: u32,
    pub PD_D2Latency: u32,
    pub PD_D3Latency: u32,
    pub PD_PowerStateMapping: [DEVICE_POWER_STATE; 7],
    pub PD_DeepestSystemWake: SYSTEM_POWER_STATE,
}
impl windows_core::TypeKind for CM_POWER_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for CM_POWER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    pub Callback: PDEVICE_NOTIFY_CALLBACK_ROUTINE,
    pub Context: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EMI_CHANNEL_MEASUREMENT_DATA {
    pub AbsoluteEnergy: u64,
    pub AbsoluteTime: u64,
}
impl windows_core::TypeKind for EMI_CHANNEL_MEASUREMENT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for EMI_CHANNEL_MEASUREMENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EMI_CHANNEL_V2 {
    pub MeasurementUnit: EMI_MEASUREMENT_UNIT,
    pub ChannelNameSize: u16,
    pub ChannelName: [u16; 1],
}
impl windows_core::TypeKind for EMI_CHANNEL_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for EMI_CHANNEL_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EMI_MEASUREMENT_DATA_V2 {
    pub ChannelData: [EMI_CHANNEL_MEASUREMENT_DATA; 1],
}
impl windows_core::TypeKind for EMI_MEASUREMENT_DATA_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for EMI_MEASUREMENT_DATA_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EMI_METADATA_SIZE {
    pub MetadataSize: u32,
}
impl windows_core::TypeKind for EMI_METADATA_SIZE {
    type TypeKind = windows_core::CopyType;
}
impl Default for EMI_METADATA_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EMI_METADATA_V1 {
    pub MeasurementUnit: EMI_MEASUREMENT_UNIT,
    pub HardwareOEM: [u16; 16],
    pub HardwareModel: [u16; 16],
    pub HardwareRevision: u16,
    pub MeteredHardwareNameSize: u16,
    pub MeteredHardwareName: [u16; 1],
}
impl windows_core::TypeKind for EMI_METADATA_V1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for EMI_METADATA_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EMI_METADATA_V2 {
    pub HardwareOEM: [u16; 16],
    pub HardwareModel: [u16; 16],
    pub HardwareRevision: u16,
    pub ChannelCount: u16,
    pub Channels: [EMI_CHANNEL_V2; 1],
}
impl windows_core::TypeKind for EMI_METADATA_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for EMI_METADATA_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EMI_VERSION {
    pub EmiVersion: u16,
}
impl windows_core::TypeKind for EMI_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl Default for EMI_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GLOBAL_MACHINE_POWER_POLICY {
    pub Revision: u32,
    pub LidOpenWakeAc: SYSTEM_POWER_STATE,
    pub LidOpenWakeDc: SYSTEM_POWER_STATE,
    pub BroadcastCapacityResolution: u32,
}
impl windows_core::TypeKind for GLOBAL_MACHINE_POWER_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for GLOBAL_MACHINE_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GLOBAL_POWER_POLICY {
    pub user: GLOBAL_USER_POWER_POLICY,
    pub mach: GLOBAL_MACHINE_POWER_POLICY,
}
impl windows_core::TypeKind for GLOBAL_POWER_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for GLOBAL_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GLOBAL_USER_POWER_POLICY {
    pub Revision: u32,
    pub PowerButtonAc: POWER_ACTION_POLICY,
    pub PowerButtonDc: POWER_ACTION_POLICY,
    pub SleepButtonAc: POWER_ACTION_POLICY,
    pub SleepButtonDc: POWER_ACTION_POLICY,
    pub LidCloseAc: POWER_ACTION_POLICY,
    pub LidCloseDc: POWER_ACTION_POLICY,
    pub DischargePolicy: [SYSTEM_POWER_LEVEL; 4],
    pub GlobalFlags: u32,
}
impl windows_core::TypeKind for GLOBAL_USER_POWER_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for GLOBAL_USER_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPOWERNOTIFY(pub isize);
impl HPOWERNOTIFY {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HPOWERNOTIFY {
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = UnregisterPowerSettingNotification(*self);
        }
    }
}
impl Default for HPOWERNOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HPOWERNOTIFY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MACHINE_POWER_POLICY {
    pub Revision: u32,
    pub MinSleepAc: SYSTEM_POWER_STATE,
    pub MinSleepDc: SYSTEM_POWER_STATE,
    pub ReducedLatencySleepAc: SYSTEM_POWER_STATE,
    pub ReducedLatencySleepDc: SYSTEM_POWER_STATE,
    pub DozeTimeoutAc: u32,
    pub DozeTimeoutDc: u32,
    pub DozeS4TimeoutAc: u32,
    pub DozeS4TimeoutDc: u32,
    pub MinThrottleAc: u8,
    pub MinThrottleDc: u8,
    pub pad1: [u8; 2],
    pub OverThrottledAc: POWER_ACTION_POLICY,
    pub OverThrottledDc: POWER_ACTION_POLICY,
}
impl windows_core::TypeKind for MACHINE_POWER_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for MACHINE_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MACHINE_PROCESSOR_POWER_POLICY {
    pub Revision: u32,
    pub ProcessorPolicyAc: PROCESSOR_POWER_POLICY,
    pub ProcessorPolicyDc: PROCESSOR_POWER_POLICY,
}
impl windows_core::TypeKind for MACHINE_PROCESSOR_POWER_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for MACHINE_PROCESSOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POWERBROADCAST_SETTING {
    pub PowerSetting: windows_core::GUID,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
impl windows_core::TypeKind for POWERBROADCAST_SETTING {
    type TypeKind = windows_core::CopyType;
}
impl Default for POWERBROADCAST_SETTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POWER_ACTION_POLICY {
    pub Action: POWER_ACTION,
    pub Flags: u32,
    pub EventCode: POWER_ACTION_POLICY_EVENT_CODE,
}
impl windows_core::TypeKind for POWER_ACTION_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for POWER_ACTION_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POWER_IDLE_RESILIENCY {
    pub CoalescingTimeout: u32,
    pub IdleResiliencyPeriod: u32,
}
impl windows_core::TypeKind for POWER_IDLE_RESILIENCY {
    type TypeKind = windows_core::CopyType;
}
impl Default for POWER_IDLE_RESILIENCY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POWER_MONITOR_INVOCATION {
    pub Console: super::super::Foundation::BOOLEAN,
    pub RequestReason: POWER_MONITOR_REQUEST_REASON,
}
impl windows_core::TypeKind for POWER_MONITOR_INVOCATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for POWER_MONITOR_INVOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POWER_PLATFORM_INFORMATION {
    pub AoAc: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for POWER_PLATFORM_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for POWER_PLATFORM_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POWER_POLICY {
    pub user: USER_POWER_POLICY,
    pub mach: MACHINE_POWER_POLICY,
}
impl windows_core::TypeKind for POWER_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {
    pub IsAllowed: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {
    type TypeKind = windows_core::CopyType;
}
impl Default for POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POWER_SESSION_CONNECT {
    pub Connected: super::super::Foundation::BOOLEAN,
    pub Console: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for POWER_SESSION_CONNECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for POWER_SESSION_CONNECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POWER_SESSION_RIT_STATE {
    pub Active: super::super::Foundation::BOOLEAN,
    pub LastInputTime: u64,
}
impl windows_core::TypeKind for POWER_SESSION_RIT_STATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for POWER_SESSION_RIT_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POWER_SESSION_TIMEOUTS {
    pub InputTimeout: u32,
    pub DisplayTimeout: u32,
}
impl windows_core::TypeKind for POWER_SESSION_TIMEOUTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for POWER_SESSION_TIMEOUTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POWER_SESSION_WINLOGON {
    pub SessionId: u32,
    pub Console: super::super::Foundation::BOOLEAN,
    pub Locked: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for POWER_SESSION_WINLOGON {
    type TypeKind = windows_core::CopyType;
}
impl Default for POWER_SESSION_WINLOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POWER_USER_PRESENCE {
    pub UserPresence: POWER_USER_PRESENCE_TYPE,
}
impl windows_core::TypeKind for POWER_USER_PRESENCE {
    type TypeKind = windows_core::CopyType;
}
impl Default for POWER_USER_PRESENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_IDLESTATE_EVENT {
    pub NewState: u32,
    pub OldState: u32,
    pub Processors: u64,
}
impl windows_core::TypeKind for PPM_IDLESTATE_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_IDLESTATE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_IDLE_ACCOUNTING {
    pub StateCount: u32,
    pub TotalTransitions: u32,
    pub ResetCount: u32,
    pub StartTime: u64,
    pub State: [PPM_IDLE_STATE_ACCOUNTING; 1],
}
impl windows_core::TypeKind for PPM_IDLE_ACCOUNTING {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_IDLE_ACCOUNTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_IDLE_ACCOUNTING_EX {
    pub StateCount: u32,
    pub TotalTransitions: u32,
    pub ResetCount: u32,
    pub AbortCount: u32,
    pub StartTime: u64,
    pub State: [PPM_IDLE_STATE_ACCOUNTING_EX; 1],
}
impl windows_core::TypeKind for PPM_IDLE_ACCOUNTING_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_IDLE_ACCOUNTING_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_IDLE_STATE_ACCOUNTING {
    pub IdleTransitions: u32,
    pub FailedTransitions: u32,
    pub InvalidBucketIndex: u32,
    pub TotalTime: u64,
    pub IdleTimeBuckets: [u32; 6],
}
impl windows_core::TypeKind for PPM_IDLE_STATE_ACCOUNTING {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_IDLE_STATE_ACCOUNTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_IDLE_STATE_ACCOUNTING_EX {
    pub TotalTime: u64,
    pub IdleTransitions: u32,
    pub FailedTransitions: u32,
    pub InvalidBucketIndex: u32,
    pub MinTimeUs: u32,
    pub MaxTimeUs: u32,
    pub CancelledTransitions: u32,
    pub IdleTimeBuckets: [PPM_IDLE_STATE_BUCKET_EX; 16],
}
impl windows_core::TypeKind for PPM_IDLE_STATE_ACCOUNTING_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_IDLE_STATE_ACCOUNTING_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_IDLE_STATE_BUCKET_EX {
    pub TotalTimeUs: u64,
    pub MinTimeUs: u32,
    pub MaxTimeUs: u32,
    pub Count: u32,
}
impl windows_core::TypeKind for PPM_IDLE_STATE_BUCKET_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_IDLE_STATE_BUCKET_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_PERFSTATE_DOMAIN_EVENT {
    pub State: u32,
    pub Latency: u32,
    pub Speed: u32,
    pub Processors: u64,
}
impl windows_core::TypeKind for PPM_PERFSTATE_DOMAIN_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_PERFSTATE_DOMAIN_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_PERFSTATE_EVENT {
    pub State: u32,
    pub Status: u32,
    pub Latency: u32,
    pub Speed: u32,
    pub Processor: u32,
}
impl windows_core::TypeKind for PPM_PERFSTATE_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_PERFSTATE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_THERMALCHANGE_EVENT {
    pub ThermalConstraint: u32,
    pub Processors: u64,
}
impl windows_core::TypeKind for PPM_THERMALCHANGE_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_THERMALCHANGE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_THERMAL_POLICY_EVENT {
    pub Mode: u8,
    pub Processors: u64,
}
impl windows_core::TypeKind for PPM_THERMAL_POLICY_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_THERMAL_POLICY_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_WMI_IDLE_STATE {
    pub Latency: u32,
    pub Power: u32,
    pub TimeCheck: u32,
    pub PromotePercent: u8,
    pub DemotePercent: u8,
    pub StateType: u8,
    pub Reserved: u8,
    pub StateFlags: u32,
    pub Context: u32,
    pub IdleHandler: u32,
    pub Reserved1: u32,
}
impl windows_core::TypeKind for PPM_WMI_IDLE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_WMI_IDLE_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_WMI_IDLE_STATES {
    pub Type: u32,
    pub Count: u32,
    pub TargetState: u32,
    pub OldState: u32,
    pub TargetProcessors: u64,
    pub State: [PPM_WMI_IDLE_STATE; 1],
}
impl windows_core::TypeKind for PPM_WMI_IDLE_STATES {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_WMI_IDLE_STATES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_WMI_IDLE_STATES_EX {
    pub Type: u32,
    pub Count: u32,
    pub TargetState: u32,
    pub OldState: u32,
    pub TargetProcessors: *mut core::ffi::c_void,
    pub State: [PPM_WMI_IDLE_STATE; 1],
}
impl windows_core::TypeKind for PPM_WMI_IDLE_STATES_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_WMI_IDLE_STATES_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_WMI_LEGACY_PERFSTATE {
    pub Frequency: u32,
    pub Flags: u32,
    pub PercentFrequency: u32,
}
impl windows_core::TypeKind for PPM_WMI_LEGACY_PERFSTATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_WMI_LEGACY_PERFSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_WMI_PERF_STATE {
    pub Frequency: u32,
    pub Power: u32,
    pub PercentFrequency: u8,
    pub IncreaseLevel: u8,
    pub DecreaseLevel: u8,
    pub Type: u8,
    pub IncreaseTime: u32,
    pub DecreaseTime: u32,
    pub Control: u64,
    pub Status: u64,
    pub HitCount: u32,
    pub Reserved1: u32,
    pub Reserved2: u64,
    pub Reserved3: u64,
}
impl windows_core::TypeKind for PPM_WMI_PERF_STATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_WMI_PERF_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_WMI_PERF_STATES {
    pub Count: u32,
    pub MaxFrequency: u32,
    pub CurrentState: u32,
    pub MaxPerfState: u32,
    pub MinPerfState: u32,
    pub LowestPerfState: u32,
    pub ThermalConstraint: u32,
    pub BusyAdjThreshold: u8,
    pub PolicyType: u8,
    pub Type: u8,
    pub Reserved: u8,
    pub TimerInterval: u32,
    pub TargetProcessors: u64,
    pub PStateHandler: u32,
    pub PStateContext: u32,
    pub TStateHandler: u32,
    pub TStateContext: u32,
    pub FeedbackHandler: u32,
    pub Reserved1: u32,
    pub Reserved2: u64,
    pub State: [PPM_WMI_PERF_STATE; 1],
}
impl windows_core::TypeKind for PPM_WMI_PERF_STATES {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_WMI_PERF_STATES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPM_WMI_PERF_STATES_EX {
    pub Count: u32,
    pub MaxFrequency: u32,
    pub CurrentState: u32,
    pub MaxPerfState: u32,
    pub MinPerfState: u32,
    pub LowestPerfState: u32,
    pub ThermalConstraint: u32,
    pub BusyAdjThreshold: u8,
    pub PolicyType: u8,
    pub Type: u8,
    pub Reserved: u8,
    pub TimerInterval: u32,
    pub TargetProcessors: *mut core::ffi::c_void,
    pub PStateHandler: u32,
    pub PStateContext: u32,
    pub TStateHandler: u32,
    pub TStateContext: u32,
    pub FeedbackHandler: u32,
    pub Reserved1: u32,
    pub Reserved2: u64,
    pub State: [PPM_WMI_PERF_STATE; 1],
}
impl windows_core::TypeKind for PPM_WMI_PERF_STATES_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPM_WMI_PERF_STATES_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROCESSOR_OBJECT_INFO {
    pub PhysicalID: u32,
    pub PBlkAddress: u32,
    pub PBlkLength: u8,
}
impl windows_core::TypeKind for PROCESSOR_OBJECT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROCESSOR_OBJECT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROCESSOR_OBJECT_INFO_EX {
    pub PhysicalID: u32,
    pub PBlkAddress: u32,
    pub PBlkLength: u8,
    pub InitialApicId: u32,
}
impl windows_core::TypeKind for PROCESSOR_OBJECT_INFO_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROCESSOR_OBJECT_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROCESSOR_POWER_INFORMATION {
    pub Number: u32,
    pub MaxMhz: u32,
    pub CurrentMhz: u32,
    pub MhzLimit: u32,
    pub MaxIdleState: u32,
    pub CurrentIdleState: u32,
}
impl windows_core::TypeKind for PROCESSOR_POWER_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROCESSOR_POWER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROCESSOR_POWER_POLICY {
    pub Revision: u32,
    pub DynamicThrottle: u8,
    pub Spare: [u8; 3],
    pub _bitfield: u32,
    pub PolicyCount: u32,
    pub Policy: [PROCESSOR_POWER_POLICY_INFO; 3],
}
impl windows_core::TypeKind for PROCESSOR_POWER_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROCESSOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROCESSOR_POWER_POLICY_INFO {
    pub TimeCheck: u32,
    pub DemoteLimit: u32,
    pub PromoteLimit: u32,
    pub DemotePercent: u8,
    pub PromotePercent: u8,
    pub Spare: [u8; 2],
    pub _bitfield: u32,
}
impl windows_core::TypeKind for PROCESSOR_POWER_POLICY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROCESSOR_POWER_POLICY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RESUME_PERFORMANCE {
    pub PostTimeMs: u32,
    pub TotalResumeTimeMs: u64,
    pub ResumeCompleteTimestamp: u64,
}
impl windows_core::TypeKind for RESUME_PERFORMANCE {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESUME_PERFORMANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SET_POWER_SETTING_VALUE {
    pub Version: u32,
    pub Guid: windows_core::GUID,
    pub PowerCondition: SYSTEM_POWER_CONDITION,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
impl windows_core::TypeKind for SET_POWER_SETTING_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SET_POWER_SETTING_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_BATTERY_STATE {
    pub AcOnLine: super::super::Foundation::BOOLEAN,
    pub BatteryPresent: super::super::Foundation::BOOLEAN,
    pub Charging: super::super::Foundation::BOOLEAN,
    pub Discharging: super::super::Foundation::BOOLEAN,
    pub Spare1: [super::super::Foundation::BOOLEAN; 3],
    pub Tag: u8,
    pub MaxCapacity: u32,
    pub RemainingCapacity: u32,
    pub Rate: u32,
    pub EstimatedTime: u32,
    pub DefaultAlert1: u32,
    pub DefaultAlert2: u32,
}
impl windows_core::TypeKind for SYSTEM_BATTERY_STATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_BATTERY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_POWER_CAPABILITIES {
    pub PowerButtonPresent: super::super::Foundation::BOOLEAN,
    pub SleepButtonPresent: super::super::Foundation::BOOLEAN,
    pub LidPresent: super::super::Foundation::BOOLEAN,
    pub SystemS1: super::super::Foundation::BOOLEAN,
    pub SystemS2: super::super::Foundation::BOOLEAN,
    pub SystemS3: super::super::Foundation::BOOLEAN,
    pub SystemS4: super::super::Foundation::BOOLEAN,
    pub SystemS5: super::super::Foundation::BOOLEAN,
    pub HiberFilePresent: super::super::Foundation::BOOLEAN,
    pub FullWake: super::super::Foundation::BOOLEAN,
    pub VideoDimPresent: super::super::Foundation::BOOLEAN,
    pub ApmPresent: super::super::Foundation::BOOLEAN,
    pub UpsPresent: super::super::Foundation::BOOLEAN,
    pub ThermalControl: super::super::Foundation::BOOLEAN,
    pub ProcessorThrottle: super::super::Foundation::BOOLEAN,
    pub ProcessorMinThrottle: u8,
    pub ProcessorMaxThrottle: u8,
    pub FastSystemS4: super::super::Foundation::BOOLEAN,
    pub Hiberboot: super::super::Foundation::BOOLEAN,
    pub WakeAlarmPresent: super::super::Foundation::BOOLEAN,
    pub AoAc: super::super::Foundation::BOOLEAN,
    pub DiskSpinDown: super::super::Foundation::BOOLEAN,
    pub HiberFileType: u8,
    pub AoAcConnectivitySupported: super::super::Foundation::BOOLEAN,
    pub spare3: [u8; 6],
    pub SystemBatteriesPresent: super::super::Foundation::BOOLEAN,
    pub BatteriesAreShortTerm: super::super::Foundation::BOOLEAN,
    pub BatteryScale: [BATTERY_REPORTING_SCALE; 3],
    pub AcOnLineWake: SYSTEM_POWER_STATE,
    pub SoftLidWake: SYSTEM_POWER_STATE,
    pub RtcWake: SYSTEM_POWER_STATE,
    pub MinDeviceWakeState: SYSTEM_POWER_STATE,
    pub DefaultLowLatencyWake: SYSTEM_POWER_STATE,
}
impl windows_core::TypeKind for SYSTEM_POWER_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_POWER_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_POWER_INFORMATION {
    pub MaxIdlenessAllowed: u32,
    pub Idleness: u32,
    pub TimeRemaining: u32,
    pub CoolingMode: POWER_COOLING_MODE,
}
impl windows_core::TypeKind for SYSTEM_POWER_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_POWER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_POWER_LEVEL {
    pub Enable: super::super::Foundation::BOOLEAN,
    pub Spare: [u8; 3],
    pub BatteryLevel: u32,
    pub PowerPolicy: POWER_ACTION_POLICY,
    pub MinSystemState: SYSTEM_POWER_STATE,
}
impl windows_core::TypeKind for SYSTEM_POWER_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_POWER_LEVEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_POWER_POLICY {
    pub Revision: u32,
    pub PowerButton: POWER_ACTION_POLICY,
    pub SleepButton: POWER_ACTION_POLICY,
    pub LidClose: POWER_ACTION_POLICY,
    pub LidOpenWake: SYSTEM_POWER_STATE,
    pub Reserved: u32,
    pub Idle: POWER_ACTION_POLICY,
    pub IdleTimeout: u32,
    pub IdleSensitivity: u8,
    pub DynamicThrottle: u8,
    pub Spare2: [u8; 2],
    pub MinSleep: SYSTEM_POWER_STATE,
    pub MaxSleep: SYSTEM_POWER_STATE,
    pub ReducedLatencySleep: SYSTEM_POWER_STATE,
    pub WinLogonFlags: u32,
    pub Spare3: u32,
    pub DozeS4Timeout: u32,
    pub BroadcastCapacityResolution: u32,
    pub DischargePolicy: [SYSTEM_POWER_LEVEL; 4],
    pub VideoTimeout: u32,
    pub VideoDimDisplay: super::super::Foundation::BOOLEAN,
    pub VideoReserved: [u32; 3],
    pub SpindownTimeout: u32,
    pub OptimizeForPower: super::super::Foundation::BOOLEAN,
    pub FanThrottleTolerance: u8,
    pub ForcedThrottle: u8,
    pub MinThrottle: u8,
    pub OverThrottled: POWER_ACTION_POLICY,
}
impl windows_core::TypeKind for SYSTEM_POWER_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_POWER_STATUS {
    pub ACLineStatus: u8,
    pub BatteryFlag: u8,
    pub BatteryLifePercent: u8,
    pub SystemStatusFlag: u8,
    pub BatteryLifeTime: u32,
    pub BatteryFullLifeTime: u32,
}
impl windows_core::TypeKind for SYSTEM_POWER_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYSTEM_POWER_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct THERMAL_EVENT {
    pub Version: u32,
    pub Size: u32,
    pub Type: u32,
    pub Temperature: u32,
    pub TripPointTemperature: u32,
    pub Initiator: windows_core::PWSTR,
}
impl windows_core::TypeKind for THERMAL_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for THERMAL_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct THERMAL_INFORMATION {
    pub ThermalStamp: u32,
    pub ThermalConstant1: u32,
    pub ThermalConstant2: u32,
    pub Processors: usize,
    pub SamplingPeriod: u32,
    pub CurrentTemperature: u32,
    pub PassiveTripPoint: u32,
    pub CriticalTripPoint: u32,
    pub ActiveTripPointCount: u8,
    pub ActiveTripPoint: [u32; 10],
}
impl windows_core::TypeKind for THERMAL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for THERMAL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct THERMAL_POLICY {
    pub Version: u32,
    pub WaitForUpdate: super::super::Foundation::BOOLEAN,
    pub Hibernate: super::super::Foundation::BOOLEAN,
    pub Critical: super::super::Foundation::BOOLEAN,
    pub ThermalStandby: super::super::Foundation::BOOLEAN,
    pub ActivationReasons: u32,
    pub PassiveLimit: u32,
    pub ActiveLevel: u32,
    pub OverThrottled: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for THERMAL_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for THERMAL_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct THERMAL_WAIT_READ {
    pub Timeout: u32,
    pub LowTemperature: u32,
    pub HighTemperature: u32,
}
impl windows_core::TypeKind for THERMAL_WAIT_READ {
    type TypeKind = windows_core::CopyType;
}
impl Default for THERMAL_WAIT_READ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_POWER_POLICY {
    pub Revision: u32,
    pub IdleAc: POWER_ACTION_POLICY,
    pub IdleDc: POWER_ACTION_POLICY,
    pub IdleTimeoutAc: u32,
    pub IdleTimeoutDc: u32,
    pub IdleSensitivityAc: u8,
    pub IdleSensitivityDc: u8,
    pub ThrottlePolicyAc: u8,
    pub ThrottlePolicyDc: u8,
    pub MaxSleepAc: SYSTEM_POWER_STATE,
    pub MaxSleepDc: SYSTEM_POWER_STATE,
    pub Reserved: [u32; 2],
    pub VideoTimeoutAc: u32,
    pub VideoTimeoutDc: u32,
    pub SpindownTimeoutAc: u32,
    pub SpindownTimeoutDc: u32,
    pub OptimizeForPowerAc: super::super::Foundation::BOOLEAN,
    pub OptimizeForPowerDc: super::super::Foundation::BOOLEAN,
    pub FanThrottleToleranceAc: u8,
    pub FanThrottleToleranceDc: u8,
    pub ForcedThrottleAc: u8,
    pub ForcedThrottleDc: u8,
}
impl windows_core::TypeKind for USER_POWER_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WAKE_ALARM_INFORMATION {
    pub TimerIdentifier: u32,
    pub Timeout: u32,
}
impl windows_core::TypeKind for WAKE_ALARM_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for WAKE_ALARM_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EFFECTIVE_POWER_MODE_CALLBACK = Option<unsafe extern "system" fn(mode: EFFECTIVE_POWER_MODE, context: *const core::ffi::c_void)>;
pub type PDEVICE_NOTIFY_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, r#type: u32, setting: *const core::ffi::c_void) -> u32>;
pub type PWRSCHEMESENUMPROC = Option<unsafe extern "system" fn(index: u32, namesize: u32, name: windows_core::PCWSTR, descriptionsize: u32, description: windows_core::PCWSTR, policy: *const POWER_POLICY, context: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOLEAN>;
pub type PWRSCHEMESENUMPROC_V1 = Option<unsafe extern "system" fn(index: u32, namesize: u32, name: *const i8, descriptionsize: u32, description: *const i8, policy: *const POWER_POLICY, context: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOLEAN>;
