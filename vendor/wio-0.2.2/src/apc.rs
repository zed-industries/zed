// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use {Result, k32, last_error, w};
use std::os::windows::io::{AsRawHandle};
use thread::{Thread};

pub fn queue<T>(func: T, thread: &Thread) -> Result<()> where T: FnOnce() + 'static {
    unsafe extern "system" fn helper<T: FnOnce() + 'static>(thing: w::ULONG_PTR) {
        let func = Box::from_raw(thing as *mut T);
        func()
    }
    let thing = Box::into_raw(Box::new(func)) as w::ULONG_PTR;
    match unsafe { k32::QueueUserAPC(Some(helper::<T>), thread.as_raw_handle(), thing) } {
        0 => {
            // If it fails we still need to deallocate the function
            unsafe { Box::from_raw(thing as *mut T) };
            last_error()
        },
        _ => Ok(()),
    }
}
pub fn queue_current<T>(func: T) -> Result<()> where T: FnOnce() + 'static {
    unsafe extern "system" fn helper<T: FnOnce() + 'static>(thing: w::ULONG_PTR) {
        let func = Box::from_raw(thing as *mut T);
        func()
    }
    let thing = Box::into_raw(Box::new(func)) as w::ULONG_PTR;
    match unsafe { k32::QueueUserAPC(Some(helper::<T>), k32::GetCurrentThread(), thing) } {
        0 => {
            // If it fails we still need to deallocate the function
            unsafe { Box::from_raw(thing as *mut T) };
            last_error()
        },
        _ => Ok(()),
    }
}
