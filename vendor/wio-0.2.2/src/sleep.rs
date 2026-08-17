// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use {k32, w};

pub fn sleep(ms: u32) {
    unsafe { k32::Sleep(ms) }
}
#[derive(Debug, Eq, PartialEq)]
pub enum WakeReason {
    TimedOut,
    CallbacksFired,
}
pub fn sleep_alertable(ms: u32) -> WakeReason {
    let ret = unsafe { k32::SleepEx(ms, w::TRUE) };
    match ret {
        0 => WakeReason::TimedOut,
        w::WAIT_IO_COMPLETION => WakeReason::CallbacksFired,
        _ => unreachable!("SleepEx returned weird value of {:?}", ret),
    }
}
