// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{BOOL, PUSHORT};
use um::winnt::{GROUP_AFFINITY, HANDLE, PGROUP_AFFINITY};
extern "system" {
    pub fn GetProcessGroupAffinity(
        hProcess: HANDLE,
        GroupCount: PUSHORT,
        GroupArray: PUSHORT,
    ) -> BOOL;
    pub fn GetThreadGroupAffinity(
        hThread: HANDLE,
        GroupAffinity: PGROUP_AFFINITY,
    ) -> BOOL;
    pub fn SetThreadGroupAffinity(
        hThread: HANDLE,
        GroupAffinity: *const GROUP_AFFINITY,
        PreviousGroupAffinity: PGROUP_AFFINITY,
    ) -> BOOL;
}
