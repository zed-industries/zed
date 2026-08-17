// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-timezone-l1
use shared::minwindef::{BOOL, DWORD, FILETIME, LPDWORD, LPFILETIME, USHORT};
use um::minwinbase::{LPSYSTEMTIME, SYSTEMTIME};
use um::winnt::{BOOLEAN, LONG, WCHAR};
pub const TIME_ZONE_ID_INVALID: DWORD = 0xFFFFFFFF;
STRUCT!{struct TIME_ZONE_INFORMATION {
    Bias: LONG,
    StandardName: [WCHAR; 32],
    StandardDate: SYSTEMTIME,
    StandardBias: LONG,
    DaylightName: [WCHAR; 32],
    DaylightDate: SYSTEMTIME,
    DaylightBias: LONG,
}}
pub type PTIME_ZONE_INFORMATION = *mut TIME_ZONE_INFORMATION;
pub type LPTIME_ZONE_INFORMATION = *mut TIME_ZONE_INFORMATION;
STRUCT!{struct DYNAMIC_TIME_ZONE_INFORMATION {
    Bias: LONG,
    StandardName: [WCHAR; 32],
    StandardDate: SYSTEMTIME,
    StandardBias: LONG,
    DaylightName: [WCHAR; 32],
    DaylightDate: SYSTEMTIME,
    DaylightBias: LONG,
    TimeZoneKeyName: [WCHAR; 128],
    DynamicDaylightTimeDisabled: BOOLEAN,
}}
pub type PDYNAMIC_TIME_ZONE_INFORMATION = *mut DYNAMIC_TIME_ZONE_INFORMATION;
extern "system" {
    pub fn SystemTimeToTzSpecificLocalTime(
        lpTimeZoneInformation: *const TIME_ZONE_INFORMATION,
        lpUniversalTime: *const SYSTEMTIME,
        lpLocalTime: LPSYSTEMTIME,
    ) -> BOOL;
    pub fn TzSpecificLocalTimeToSystemTime(
        lpTimeZoneInformation: *const TIME_ZONE_INFORMATION,
        lpLocalTime: *const SYSTEMTIME,
        lpUniversalTime: LPSYSTEMTIME,
    ) -> BOOL;
    pub fn FileTimeToSystemTime(
        lpFileTime: *const FILETIME,
        lpSystemTime: LPSYSTEMTIME,
    ) -> BOOL;
    pub fn SystemTimeToFileTime(
        lpSystemTime: *const SYSTEMTIME,
        lpFileTime: LPFILETIME,
    ) -> BOOL;
    pub fn GetTimeZoneInformation(
        lpTimeZoneInformation: LPTIME_ZONE_INFORMATION,
    ) -> DWORD;
    pub fn SetTimeZoneInformation(
        lpTimeZoneInformation: *const TIME_ZONE_INFORMATION,
    ) -> BOOL;
    pub fn SetDynamicTimeZoneInformation(
        lpTimeZoneInformation: *const DYNAMIC_TIME_ZONE_INFORMATION,
    ) -> BOOL;
    pub fn GetDynamicTimeZoneInformation(
        pTimeZoneInformation: PDYNAMIC_TIME_ZONE_INFORMATION,
    ) -> DWORD;
    pub fn GetTimeZoneInformationForYear(
        wYear: USHORT,
        pdtzi: PDYNAMIC_TIME_ZONE_INFORMATION,
        ptzi: LPTIME_ZONE_INFORMATION,
    ) -> BOOL;
    pub fn EnumDynamicTimeZoneInformation(
        dwIndex: DWORD,
        lpTimeZoneInformation: PDYNAMIC_TIME_ZONE_INFORMATION,
    ) -> DWORD;
    pub fn GetDynamicTimeZoneInformationEffectiveYears(
        lpTimeZoneInformation: PDYNAMIC_TIME_ZONE_INFORMATION,
        FirstYear: LPDWORD,
        LastYear: LPDWORD,
    ) -> DWORD;
    pub fn SystemTimeToTzSpecificLocalTimeEx(
        lpTimeZoneInformation: *const DYNAMIC_TIME_ZONE_INFORMATION,
        lpUniversalTime: *const SYSTEMTIME,
        lpLocalTime: LPSYSTEMTIME,
    ) -> BOOL;
    pub fn TzSpecificLocalTimeToSystemTimeEx(
        lpTimeZoneInformation: *const DYNAMIC_TIME_ZONE_INFORMATION,
        lpLocalTime: *const SYSTEMTIME,
        lpUniversalTime: LPSYSTEMTIME,
    ) -> BOOL;
}
