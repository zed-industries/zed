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
use std::ffi::c_void;

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

    fn run_on_main_thread(&self, runnable: Runnable) {
        unsafe {
            dispatch_async_f(
                dispatch_get_main_queue(),
                runnable.into_raw() as *mut c_void,
                Some(trampoline),
            );
        }

        extern "C" fn trampoline(runnable: *mut c_void) {
            let task = unsafe { Runnable::from_raw(runnable as *mut ()) };
            task.run();
        }
    }
}
