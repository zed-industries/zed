// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_int;
use um::winnt::{LPCUWSTR, PCUWSTR, PUWSTR, WCHAR};
use vc::vcruntime::size_t;
extern "system" {
    pub fn uaw_lstrcmpW(
        String1: PCUWSTR,
        String2: PCUWSTR,
    ) -> c_int;
    pub fn uaw_lstrcmpiW(
        String1: PCUWSTR,
        String2: PCUWSTR,
    ) -> c_int;
    pub fn uaw_lstrlenW(
        String: LPCUWSTR,
    ) -> c_int;
    pub fn uaw_wcschr(
        String: PCUWSTR,
        Character: WCHAR,
    ) -> PUWSTR;
    pub fn uaw_wcscpy(
        Destination: PUWSTR,
        Source: PCUWSTR,
    ) -> PUWSTR;
    pub fn uaw_wcsicmp(
        String1: PCUWSTR,
        String2: PCUWSTR,
    ) -> c_int;
    pub fn uaw_wcslen(
        String: PCUWSTR,
    ) -> size_t;
    pub fn uaw_wcsrchr(
        String: PCUWSTR,
        Character: WCHAR,
    ) -> PUWSTR;
}
