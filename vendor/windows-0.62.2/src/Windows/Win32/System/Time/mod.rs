#[inline]
pub unsafe fn EnumDynamicTimeZoneInformation(dwindex: u32, lptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EnumDynamicTimeZoneInformation(dwindex : u32, lptimezoneinformation : *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32);
    unsafe { EnumDynamicTimeZoneInformation(dwindex, lptimezoneinformation as _) }
}
#[inline]
pub unsafe fn FileTimeToSystemTime(lpfiletime: *const super::super::Foundation::FILETIME, lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn FileTimeToSystemTime(lpfiletime : *const super::super::Foundation:: FILETIME, lpsystemtime : *mut super::super::Foundation:: SYSTEMTIME) -> windows_core::BOOL);
    unsafe { FileTimeToSystemTime(lpfiletime, lpsystemtime as _).ok() }
}
#[inline]
pub unsafe fn GetDynamicTimeZoneInformation(ptimezoneinformation: *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetDynamicTimeZoneInformation(ptimezoneinformation : *mut DYNAMIC_TIME_ZONE_INFORMATION) -> u32);
    unsafe { GetDynamicTimeZoneInformation(ptimezoneinformation as _) }
}
#[inline]
pub unsafe fn GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION, firstyear: *mut u32, lastyear: *mut u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, firstyear : *mut u32, lastyear : *mut u32) -> u32);
    unsafe { GetDynamicTimeZoneInformationEffectiveYears(lptimezoneinformation, firstyear as _, lastyear as _) }
}
#[inline]
pub unsafe fn GetTimeZoneInformation(lptimezoneinformation: *mut TIME_ZONE_INFORMATION) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetTimeZoneInformation(lptimezoneinformation : *mut TIME_ZONE_INFORMATION) -> u32);
    unsafe { GetTimeZoneInformation(lptimezoneinformation as _) }
}
#[inline]
pub unsafe fn GetTimeZoneInformationForYear(wyear: u16, pdtzi: Option<*const DYNAMIC_TIME_ZONE_INFORMATION>, ptzi: *mut TIME_ZONE_INFORMATION) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetTimeZoneInformationForYear(wyear : u16, pdtzi : *const DYNAMIC_TIME_ZONE_INFORMATION, ptzi : *mut TIME_ZONE_INFORMATION) -> windows_core::BOOL);
    unsafe { GetTimeZoneInformationForYear(wyear, pdtzi.unwrap_or(core::mem::zeroed()) as _, ptzi as _).ok() }
}
#[inline]
pub unsafe fn LocalFileTimeToLocalSystemTime(timezoneinformation: Option<*const TIME_ZONE_INFORMATION>, localfiletime: *const super::super::Foundation::FILETIME, localsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn LocalFileTimeToLocalSystemTime(timezoneinformation : *const TIME_ZONE_INFORMATION, localfiletime : *const super::super::Foundation:: FILETIME, localsystemtime : *mut super::super::Foundation:: SYSTEMTIME) -> windows_core::BOOL);
    unsafe { LocalFileTimeToLocalSystemTime(timezoneinformation.unwrap_or(core::mem::zeroed()) as _, localfiletime, localsystemtime as _) }
}
#[inline]
pub unsafe fn LocalSystemTimeToLocalFileTime(timezoneinformation: Option<*const TIME_ZONE_INFORMATION>, localsystemtime: *const super::super::Foundation::SYSTEMTIME, localfiletime: *mut super::super::Foundation::FILETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn LocalSystemTimeToLocalFileTime(timezoneinformation : *const TIME_ZONE_INFORMATION, localsystemtime : *const super::super::Foundation:: SYSTEMTIME, localfiletime : *mut super::super::Foundation:: FILETIME) -> windows_core::BOOL);
    unsafe { LocalSystemTimeToLocalFileTime(timezoneinformation.unwrap_or(core::mem::zeroed()) as _, localsystemtime, localfiletime as _) }
}
#[inline]
pub unsafe fn SetDynamicTimeZoneInformation(lptimezoneinformation: *const DYNAMIC_TIME_ZONE_INFORMATION) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetDynamicTimeZoneInformation(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION) -> windows_core::BOOL);
    unsafe { SetDynamicTimeZoneInformation(lptimezoneinformation).ok() }
}
#[inline]
pub unsafe fn SetTimeZoneInformation(lptimezoneinformation: *const TIME_ZONE_INFORMATION) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetTimeZoneInformation(lptimezoneinformation : *const TIME_ZONE_INFORMATION) -> windows_core::BOOL);
    unsafe { SetTimeZoneInformation(lptimezoneinformation).ok() }
}
#[inline]
pub unsafe fn SystemTimeToFileTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME, lpfiletime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SystemTimeToFileTime(lpsystemtime : *const super::super::Foundation:: SYSTEMTIME, lpfiletime : *mut super::super::Foundation:: FILETIME) -> windows_core::BOOL);
    unsafe { SystemTimeToFileTime(lpsystemtime, lpfiletime as _).ok() }
}
#[inline]
pub unsafe fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation: Option<*const TIME_ZONE_INFORMATION>, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lpuniversaltime : *const super::super::Foundation:: SYSTEMTIME, lplocaltime : *mut super::super::Foundation:: SYSTEMTIME) -> windows_core::BOOL);
    unsafe { SystemTimeToTzSpecificLocalTime(lptimezoneinformation.unwrap_or(core::mem::zeroed()) as _, lpuniversaltime, lplocaltime as _).ok() }
}
#[inline]
pub unsafe fn SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation: Option<*const DYNAMIC_TIME_ZONE_INFORMATION>, lpuniversaltime: *const super::super::Foundation::SYSTEMTIME, lplocaltime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, lpuniversaltime : *const super::super::Foundation:: SYSTEMTIME, lplocaltime : *mut super::super::Foundation:: SYSTEMTIME) -> windows_core::BOOL);
    unsafe { SystemTimeToTzSpecificLocalTimeEx(lptimezoneinformation.unwrap_or(core::mem::zeroed()) as _, lpuniversaltime, lplocaltime as _).ok() }
}
#[inline]
pub unsafe fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation: Option<*const TIME_ZONE_INFORMATION>, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lplocaltime : *const super::super::Foundation:: SYSTEMTIME, lpuniversaltime : *mut super::super::Foundation:: SYSTEMTIME) -> windows_core::BOOL);
    unsafe { TzSpecificLocalTimeToSystemTime(lptimezoneinformation.unwrap_or(core::mem::zeroed()) as _, lplocaltime, lpuniversaltime as _).ok() }
}
#[inline]
pub unsafe fn TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation: Option<*const DYNAMIC_TIME_ZONE_INFORMATION>, lplocaltime: *const super::super::Foundation::SYSTEMTIME, lpuniversaltime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation : *const DYNAMIC_TIME_ZONE_INFORMATION, lplocaltime : *const super::super::Foundation:: SYSTEMTIME, lpuniversaltime : *mut super::super::Foundation:: SYSTEMTIME) -> windows_core::BOOL);
    unsafe { TzSpecificLocalTimeToSystemTimeEx(lptimezoneinformation.unwrap_or(core::mem::zeroed()) as _, lplocaltime, lpuniversaltime as _).ok() }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: super::super::Foundation::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: super::super::Foundation::SYSTEMTIME,
    pub DaylightBias: i32,
    pub TimeZoneKeyName: [u16; 128],
    pub DynamicDaylightTimeDisabled: bool,
}
impl Default for DYNAMIC_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TIME_ZONE_ID_INVALID: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: super::super::Foundation::SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: super::super::Foundation::SYSTEMTIME,
    pub DaylightBias: i32,
}
impl Default for TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
