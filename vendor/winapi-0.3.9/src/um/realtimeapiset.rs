// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::PULONG64;
use shared::minwindef::{BOOL, PULONG, USHORT};
use um::winnt::{HANDLE, PULONGLONG};
extern "system" {
    pub fn QueryThreadCycleTime(
        ThreadHandle: HANDLE,
        CycleTime: PULONG64,
    ) -> BOOL;
    pub fn QueryProcessCycleTime(
        ProcessHandle: HANDLE,
        CycleTime: PULONG64,
    ) -> BOOL;
    pub fn QueryIdleProcessorCycleTime(
        BufferLength: PULONG,
        ProcessorIdleCycleTime: PULONG64,
    ) -> BOOL;
    pub fn QueryIdleProcessorCycleTimeEx(
        Group: USHORT,
        BufferLength: PULONG,
        ProcessorIdleCycleTime: PULONG64,
    ) -> BOOL;
    pub fn QueryUnbiasedInterruptTime(
        UnbiasedTime: PULONGLONG,
    ) -> BOOL;
}
