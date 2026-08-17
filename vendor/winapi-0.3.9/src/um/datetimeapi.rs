// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_int;
use shared::minwindef::DWORD;
use um::minwinbase::SYSTEMTIME;
use um::winnt::{LCID, LPCSTR, LPCWSTR, LPSTR, LPWSTR};
extern "system" {
    pub fn GetDateFormatA(
        Locale: LCID,
        dwFlags: DWORD,
        lpDate: *const SYSTEMTIME,
        lpFormat: LPCSTR,
        lpDateStr: LPSTR,
        cchDate: c_int,
    ) -> c_int;
    pub fn GetDateFormatW(
        Locale: LCID,
        dwFlags: DWORD,
        lpDate: *const SYSTEMTIME,
        lpFormat: LPCWSTR,
        lpDateStr: LPWSTR,
        cchDate: c_int,
    ) -> c_int;
    pub fn GetTimeFormatA(
        Locale: LCID,
        dwFlags: DWORD,
        lpTime: *const SYSTEMTIME,
        lpFormat: LPCSTR,
        lpTimeStr: LPSTR,
        cchTime: c_int,
    ) -> c_int;
    pub fn GetTimeFormatW(
        Locale: LCID,
        dwFlags: DWORD,
        lpTime: *const SYSTEMTIME,
        lpFormat: LPCWSTR,
        lpTimeStr: LPWSTR,
        cchTime: c_int,
    ) -> c_int;
    pub fn GetTimeFormatEx(
        lpLocaleName: LPCWSTR,
        dwFlags: DWORD,
        lpTime: *const SYSTEMTIME,
        lpFormat: LPCWSTR,
        lpTimeStr: LPWSTR,
        cchTime: c_int,
    ) -> c_int;
    pub fn GetDateFormatEx(
        lpLocaleName: LPCWSTR,
        dwFlags: DWORD,
        lpDate: *const SYSTEMTIME,
        lpFormat: LPCWSTR,
        lpDateStr: LPWSTR,
        cchDate: c_int,
        lpCalendar: LPCWSTR,
    ) -> c_int;
}
