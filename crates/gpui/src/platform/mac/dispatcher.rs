#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{PlatformDispatcher, TaskLabel};
use async_task::Runnable;
use block::{Block, ConcreteBlock, RcBlock};
use core_foundation::{
    base::CFTypeRef,
    runloop::{
        CFRunLoopRef, CFRunLoopRunInMode, CFRunLoopWakeUp, kCFRunLoopCommonModes,
        kCFRunLoopDefaultMode,
    },
};
use objc::{
    class, msg_send,
    runtime::{BOOL, YES},
    sel, sel_impl,
};
use parking::{Parker, Unparker};
use parking_lot::Mutex;
use smol::io::BlockOn;
use std::{
    cell::Cell,
    ffi::c_void,
    ptr::{NonNull, addr_of},
    sync::Arc,
    time::Duration,
};

/// All items in the generated file are marked as pub, so we're gonna wrap it in a separate mod to prevent
/// these pub items from leaking into public API.
pub(crate) mod dispatch_sys {
    include!(concat!(env!("OUT_DIR"), "/dispatch_sys.rs"));
}

use dispatch_sys::*;
pub(crate) fn dispatch_get_main_queue() -> dispatch_queue_t {
    addr_of!(_dispatch_main_q) as *const _ as dispatch_queue_t
}

pub(crate) struct MacDispatcher {
    parker: Arc<Mutex<Parker>>,
}

impl Default for MacDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl MacDispatcher {
    pub fn new() -> Self {
        MacDispatcher {
            parker: Arc::new(Mutex::new(Parker::new())),
        }
    }
}

impl PlatformDispatcher for MacDispatcher {
    fn is_main_thread(&self) -> bool {
        let is_main_thread: BOOL = unsafe { msg_send![class!(NSThread), isMainThread] };
        is_main_thread == YES
    }

    fn dispatch(&self, runnable: Runnable, _: Option<TaskLabel>) {
        unsafe {
            dispatch_async_f(
                dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH.try_into().unwrap(), 0),
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline),
            );
        }
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        use core_foundation::runloop::CFRunLoopGetMain;

        unsafe {
            let mut runnable = Cell::new(Some(runnable));
            let main_run_loop = CFRunLoopGetMain();
            let block = ConcreteBlock::new(move || {
                if let Some(runnable) = runnable.take() {
                    runnable.run();
                }
            })
            .copy();
            CFRunLoopPerformBlock(
                main_run_loop,
                kCFRunLoopDefaultMode as _,
                &*block as *const Block<_, _> as _,
            );
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
        unsafe {
            let queue =
                dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH.try_into().unwrap(), 0);
            let when = dispatch_time(DISPATCH_TIME_NOW as u64, duration.as_nanos() as i64);
            dispatch_after_f(
                when,
                queue,
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline),
            );
        }
    }

    fn tick(&self, background_only: bool) -> bool {
        unsafe {
            CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0., 0);
        }
        true
    }

    fn park(&self, timeout: Option<Duration>) -> bool {
        if let Some(timeout) = timeout {
            self.parker.lock().park_timeout(timeout)
        } else {
            self.parker.lock().park();
            true
        }
    }

    fn unparker(&self) -> Unparker {
        self.parker.lock().unparker()
    }
}

extern "C" fn trampoline(runnable: *mut c_void) {
    let task = unsafe { Runnable::<()>::from_raw(NonNull::new_unchecked(runnable as *mut ())) };
    task.run();
}

unsafe extern "C" {
    fn CFRunLoopPerformBlock(rl: CFRunLoopRef, mode: CFTypeRef, block: *const c_void);
}
