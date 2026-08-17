// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{ULONG, USHORT};
use um::winnt::{PSLIST_ENTRY, PSLIST_HEADER};
extern "system" {
    pub fn InitializeSListHead(
        ListHead: PSLIST_HEADER,
    );
    pub fn InterlockedPopEntrySList(
        ListHead: PSLIST_HEADER,
    ) -> PSLIST_ENTRY;
    pub fn InterlockedPushEntrySList(
        ListHead: PSLIST_HEADER,
        ListEntry: PSLIST_ENTRY,
    ) -> PSLIST_ENTRY;
    pub fn InterlockedPushListSListEx(
        ListHead: PSLIST_HEADER,
        List: PSLIST_ENTRY,
        ListEnd: PSLIST_ENTRY,
        Count: ULONG,
    ) -> PSLIST_ENTRY;
    pub fn InterlockedFlushSList(
        ListHead: PSLIST_HEADER,
    ) -> PSLIST_ENTRY;
    pub fn QueryDepthSList(
        ListHead: PSLIST_HEADER,
    ) -> USHORT;
}
