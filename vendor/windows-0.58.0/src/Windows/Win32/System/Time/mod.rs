#[inline]
pub unsafe fn EnumDynamicTimeZoneInformation(dwindex: u32, lptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32 {
    windows_targets::link!("advapi32.dll" "system" fn EnumDynamicTimeZoneInformation(dwindex : u32, lptimezoneinformation : *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32);
    EnumDynamicTimeZoneInformation(dwindex, lptimezoneinformation)
}
#[inline]
pub unsafe fn FileTimeToSystemTime(lpfiletime: *const super::super::Foundation::FILETIME, lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn FileTimeToSystemTime(lpfiletime : *const super::super::Foundation:: FILETIME, lpsystemtime : *mut super::super::Foundation:: SYSTEMTIME) -> super::super::Foundation:: BOOL);
    FileTimeToSystemTime(lpfiletime, lpsystemtime).ok()
}
#[inline]
pub unsafe fn GetDynamicTimeZoneInformation(ptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetDynamicTimeZoneInformation(ptimezoneinformation : *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32);
    GetDynamicTimeZoneInformation(ptimezoneinformation)
}
#[inline]
pub unsafe fn GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, firstyear: *mut u32, lastyear: *mut u32) -> u32 {
    windows_targets::link!("advapi32.dll" "system" fn GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, firstyear : *mut u32, lastyear : *mut u32) -> u32);
    GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation, firstyear, lastyear)
}
#[inline]
pub unsafe fn GetTimeZoneInformation(lptimezoneinformation: *mut TIME_ZONE_INFORMATION) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetTimeZoneInformation(lptimezoneinformation : *mut TIME_ZONE_INFORMATION) -> u32);
    GetTimeZoneInformation(lptimezoneinformation)
}
#[inline]
pub unsafe fn GetTimeZoneInformationForYear(wyear: u16, pdtzi: Option<*const DYNAMIC_TIME_ZONE_INFORMATION>, ptzi: *mut TIME_ZONE_INFORMATION) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetTimeZoneInformationForYear(wyear : u16, pdtzi : *const DYNAMIC_TIME_ZONE_INFORMATION, ptzi : *mut TIME_ZONE_INFORMATION) -> super::super::Foundation:: BOOL);
    GetTimeZoneInformationForYear(wyear, core::mem::transmute(pdtzi.unwrap_or(std::ptr::null())), ptzi).ok()
}
#[inline]
pub unsafe fn LocalFileTimeToLocalSystemTime(timezoneinformation: Option<*const TIME_ZONE_INFORMATION>, localfiletime: *const super::super::Foundation::FILETIME, localsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn LocalFileTimeToLocalSystemTime(timezoneinformation : *const TIME_ZONE_INFORMATION, localfiletime : *const super::super::Foundation:: FILETIME, localsystemtime : *mut super::super::Foundation:: SYSTEMTIME) -> super::super::Foundation:: BOOL);
    LocalFileTimeToLocalSystemTime(core::mem::transmute(timezoneinformation.unwrap_or(std::ptr::null())), localfiletime, localsystemtime)
}
#[inline]
pub unsafe fn LocalSystemTimeToLocalFileTime(timezoneinformation: Option<*const TIME_ZONE_INFORMATION>, localsystemtime: *const super::super::Foundation::SYSTEMTIME, localfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn LocalSystemTimeToLocalFileTime(timezoneinformation : *const TIME_ZONE_INFORMATION, localsystemtime : *const super::super::Foundation:: SYSTEMTIME, localfiletime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: BOOL);
    LocalSystemTimeToLocalFileTime(core::mem::transmute(timezoneinformation.unwrap_or(std::ptr::null())), localsystemtime, localfiletime)
}
#[inline]
pub unsafe fn SetDynamicTimeZoneInformation(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetDynamicTimeZoneInformation(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION) -> super::super::Foundation:: BOOL);
    SetDynamicTimeZoneInformation(lptimezoneinformation).ok()
}
#[inline]
pub unsafe fn SetTimeZoneInformation(lptimezoneinformation: *const TIME_ZONE_INFORMATION) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetTimeZoneInformation(lptimezoneinformation : *const TIME_ZONE_INFORMATION) -> super::super::Foundation:: BOOL);
    SetTimeZoneInformation(lptimezoneinformation).ok()
}
#[inline]
pub unsafe fn SystemTimeToFileTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME, lpfiletime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SystemTimeToFileTime(lpsystemtime : *const super::super::Foundation:: SYSTEMTIME, lpfiletime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: BOOL);
    SystemTimeToFileTime(lpsystemtime, lpfiletime).ok()
}
#[inline]
pub unsafe fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation: Option<*const TIME_ZONE_INFORMATION>, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lpuniversaltime : *const super::super::Foundation:: SYSTEMTIME, lplocaltime : *mut super::super::Foundation:: SYSTEMTIME) -> super::super::Foundation:: BOOL);
    SystemTimeToTzSpecificLocalTime(core::mem::transmute(lptimezoneinformation.unwrap_or(std::ptr::null())), lpuniversaltime, lplocaltime).ok()
}
#[inline]
pub unsafe fn SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation: Option<*const DYNAMIC_TIME_ZONE_INFORMATION>, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, lpuniversaltime : *const super::super::Foundation:: SYSTEMTIME, lplocaltime : *mut super::super::Foundation:: SYSTEMTIME) -> super::super::Foundation:: BOOL);
    SystemTimeToTzSpecificLocalTimeEx(core::mem::transmute(lptimezoneinformation.unwrap_or(std::ptr::null())), lpuniversaltime, lplocaltime).ok()
}
#[inline]
pub unsafe fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation: Option<*const TIME_ZONE_INFORMATION>, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lplocaltime : *const super::super::Foundation:: SYSTEMTIME, lpuniversaltime : *mut super::super::Foundation:: SYSTEMTIME) -> super::super::Foundation:: BOOL);
    TzSpecificLocalTimeToSystemTime(core::mem::transmute(lptimezoneinformation.unwrap_or(std::ptr::null())), lplocaltime, lpuniversaltime).ok()
}
#[inline]
pub unsafe fn TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation: Option<*const DYNAMIC_TIME_ZONE_INFORMATION>, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, lplocaltime : *const super::super::Foundation:: SYSTEMTIME, lpuniversaltime : *mut super::super::Foundation:: SYSTEMTIME) -> super::super::Foundation:: BOOL);
    TzSpecificLocalTimeToSystemTimeEx(core::mem::transmute(lptimezoneinformation.unwrap_or(std::ptr::null())), lplocaltime, lpuniversaltime).ok()
}
pub const TIME_ZONE_ID_INVALID: u32 = 4294967295u32;
pub const TSF_Authenticated: u32 = 2u32;
pub const TSF_Hardware: u32 = 1u32;
pub const TSF_IPv6: u32 = 4u32;
pub const TSF_SignatureAuthenticated: u32 = 8u32;
pub const wszW32TimeRegKeyPolicyTimeProviders: windows_core::PCWSTR = windows_core::w!("Software\\Policies\\Microsoft\\W32Time\\TimeProviders");
pub const wszW32TimeRegKeyTimeProviders: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\W32Time\\TimeProviders");
pub const wszW32TimeRegValueDllName: windows_core::PCWSTR = windows_core::w!("DllName");
pub const wszW32TimeRegValueEnabled: windows_core::PCWSTR = windows_core::w!("Enabled");
pub const wszW32TimeRegValueInputProvider: windows_core::PCWSTR = windows_core::w!("InputProvider");
pub const wszW32TimeRegValueMetaDataProvider: windows_core::PCWSTR = windows_core::w!("MetaDataProvider");
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: super::super::Foundation::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: super::super::Foundation::SYSTEMTIME,
    pub DaylightBias: i32,
    pub TimeZoneKeyName: [u16; 128],
    pub DynamicDaylightTimeDisabled: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for DYNAMIC_TIME_ZONE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DYNAMIC_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: super::super::Foundation::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: super::super::Foundation::SYSTEMTIME,
    pub DaylightBias: i32,
}
impl windows_core::TypeKind for TIME_ZONE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
