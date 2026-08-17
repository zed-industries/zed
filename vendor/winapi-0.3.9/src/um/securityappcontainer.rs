// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{BOOL, PULONG, ULONG};
use um::winnt::{HANDLE, LPWSTR, PSID};
extern "system" {
    pub fn GetAppContainerNamedObjectPath(
        Token: HANDLE,
        AppContainerSid: PSID,
        ObjectPathLength: ULONG,
        ObjectPath: LPWSTR,
        ReturnLength: PULONG,
    ) -> BOOL;
}
