// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{DWORD, UINT};
use um::mmsystem::{LPTIMECAPS, MMRESULT};
extern "system" {
    pub fn timeGetTime() -> DWORD;
    pub fn timeGetDevCaps(
        ptc: LPTIMECAPS,
        cbtc: UINT,
    ) -> MMRESULT;
    pub fn timeBeginPeriod(
        uPeriod: UINT,
    ) -> MMRESULT;
    pub fn timeEndPeriod(
        uPeriod: UINT,
    ) -> MMRESULT;
}
