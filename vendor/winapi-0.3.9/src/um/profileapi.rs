// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::BOOL;
use um::winnt::LARGE_INTEGER;
extern "system" {
    pub fn QueryPerformanceCounter(
        lpPerformanceCount: *mut LARGE_INTEGER,
    ) -> BOOL;
    pub fn QueryPerformanceFrequency(
        lpFrequency: *mut LARGE_INTEGER,
    ) -> BOOL;
}
