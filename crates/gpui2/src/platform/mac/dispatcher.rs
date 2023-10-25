#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::PlatformDispatcher;
use async_task::Runnable;
use objc::{
    class, msg_send,
    runtime::{BOOL, YES},
    sel, sel_impl,
};
use std::{
    ffi::c_void,
    time::{Duration, SystemTime},
};

include!(concat!(env!("OUT_DIR"), "/dispatch_sys.rs"));

pub fn dispatch_get_main_queue() -> dispatch_queue_t {
    unsafe { &_dispatch_main_q as *const _ as dispatch_queue_t }
}

pub struct MacDispatcher;

impl PlatformDispatcher for MacDispatcher {
    fn is_main_thread(&self) -> bool {
        let is_main_thread: BOOL = unsafe { msg_send![class!(NSThread), isMainThread] };
        is_main_thread == YES
    }

    fn dispatch(&self, runnable: Runnable) {
        unsafe {
            dispatch_async_f(
                dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT.try_into().unwrap(), 0),
                runnable.into_raw() as *mut c_void,
                Some(trampoline),
            );
        }
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        unsafe {
            dispatch_async_f(
                dispatch_get_main_queue(),
                runnable.into_raw() as *mut c_void,
                Some(trampoline),
            );
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
        let now = SystemTime::now();
        let after_duration = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
            + duration.as_nanos() as u64;
        unsafe {
            let queue =
                dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT.try_into().unwrap(), 0);
            let when = dispatch_time(0, after_duration as i64);
            dispatch_after_f(
                when,
                queue,
                runnable.into_raw() as *mut c_void,
                Some(trampoline),
            );
        }
    }

    fn poll(&self) -> bool {
        false
    }

    #[cfg(any(test, feature = "test-support"))]
    fn advance_clock(&self, _: Duration) {
        unimplemented!()
    }
}

extern "C" fn trampoline(runnable: *mut c_void) {
    let task = unsafe { Runnable::from_raw(runnable as *mut ()) };
    task.run();
}

// #include <dispatch/dispatch.h>

// int main(void) {

//     dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
//         // Do some lengthy background work here...
//         printf("Background Work\n");

//         dispatch_async(dispatch_get_main_queue(), ^{
//             // Once done, update your UI on the main queue here.
//             printf("UI Updated\n");

//         });
//     });

//     sleep(3);  // prevent the program from terminating immediately

//     return 0;
// }
// ```
