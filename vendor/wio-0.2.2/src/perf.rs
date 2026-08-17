// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use {k32};

pub fn frequency() -> i64 {
    let mut freq = 0;
    unsafe { k32::QueryPerformanceFrequency(&mut freq) };
    freq
}
pub fn counter() -> i64 {
    let mut count = 0;
    unsafe { k32::QueryPerformanceCounter(&mut count) };
    count
}
