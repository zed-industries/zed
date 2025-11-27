#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{
    GLOBAL_THREAD_TIMINGS, PlatformDispatcher, RunnableMeta, RunnableVariant, THREAD_TIMINGS,
    TaskLabel, TaskTiming, ThreadTaskTimings,
};

use async_task::Runnable;
use objc::{
    class, msg_send,
    runtime::{BOOL, YES},
    sel, sel_impl,
};
use std::{
    ffi::c_void,
    ptr::{NonNull, addr_of},
    time::{Duration, Instant},
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

pub(crate) struct MacDispatcher;

impl PlatformDispatcher for MacDispatcher {
    fn get_all_timings(&self) -> Vec<ThreadTaskTimings> {
        let global_timings = GLOBAL_THREAD_TIMINGS.lock();
        ThreadTaskTimings::convert(&global_timings)
    }

    fn get_current_thread_timings(&self) -> Vec<TaskTiming> {
        THREAD_TIMINGS.with(|timings| {
            let timings = &timings.lock().timings;

            let mut vec = Vec::with_capacity(timings.len());

            let (s1, s2) = timings.as_slices();
            vec.extend_from_slice(s1);
            vec.extend_from_slice(s2);
            vec
        })
    }

    fn is_main_thread(&self) -> bool {
        let is_main_thread: BOOL = unsafe { msg_send![class!(NSThread), isMainThread] };
        is_main_thread == YES
    }

    fn dispatch(&self, runnable: RunnableVariant, _: Option<TaskLabel>) {
        let (context, trampoline) = match runnable {
            RunnableVariant::Meta(runnable) => (
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline as unsafe extern "C" fn(*mut c_void)),
            ),
            RunnableVariant::Compat(runnable) => (
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline_compat as unsafe extern "C" fn(*mut c_void)),
            ),
        };
        unsafe {
            dispatch_async_f(
                dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH.try_into().unwrap(), 0),
                context,
                trampoline,
            );
        }
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant) {
        let (context, trampoline) = match runnable {
            RunnableVariant::Meta(runnable) => (
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline as unsafe extern "C" fn(*mut c_void)),
            ),
            RunnableVariant::Compat(runnable) => (
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline_compat as unsafe extern "C" fn(*mut c_void)),
            ),
        };
        unsafe {
            dispatch_async_f(dispatch_get_main_queue(), context, trampoline);
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let (context, trampoline) = match runnable {
            RunnableVariant::Meta(runnable) => (
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline as unsafe extern "C" fn(*mut c_void)),
            ),
            RunnableVariant::Compat(runnable) => (
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline_compat as unsafe extern "C" fn(*mut c_void)),
            ),
        };
        unsafe {
            let queue =
                dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH.try_into().unwrap(), 0);
            let when = dispatch_time(DISPATCH_TIME_NOW as u64, duration.as_nanos() as i64);
            dispatch_after_f(when, queue, context, trampoline);
        }
    }
}

extern "C" fn trampoline(runnable: *mut c_void) {
    let task =
        unsafe { Runnable::<RunnableMeta>::from_raw(NonNull::new_unchecked(runnable as *mut ())) };

    let location = task.metadata().location;

    let start = Instant::now();
    let timing = TaskTiming {
        location,
        start,
        end: None,
    };

    THREAD_TIMINGS.with(|timings| {
        let mut timings = timings.lock();
        let timings = &mut timings.timings;
        if let Some(last_timing) = timings.iter_mut().rev().next() {
            if last_timing.location == timing.location {
                return;
            }
        }

        timings.push_back(timing);
    });

    task.run();
    let end = Instant::now();

    THREAD_TIMINGS.with(|timings| {
        let mut timings = timings.lock();
        let timings = &mut timings.timings;
        let Some(last_timing) = timings.iter_mut().rev().next() else {
            return;
        };
        last_timing.end = Some(end);
    });
}

extern "C" fn trampoline_compat(runnable: *mut c_void) {
    let task = unsafe { Runnable::<()>::from_raw(NonNull::new_unchecked(runnable as *mut ())) };

    let location = core::panic::Location::caller();

    let start = Instant::now();
    let timing = TaskTiming {
        location,
        start,
        end: None,
    };
    THREAD_TIMINGS.with(|timings| {
        let mut timings = timings.lock();
        let timings = &mut timings.timings;
        if let Some(last_timing) = timings.iter_mut().rev().next() {
            if last_timing.location == timing.location {
                return;
            }
        }

        timings.push_back(timing);
    });

    task.run();
    let end = Instant::now();

    THREAD_TIMINGS.with(|timings| {
        let mut timings = timings.lock();
        let timings = &mut timings.timings;
        let Some(last_timing) = timings.iter_mut().rev().next() else {
            return;
        };
        last_timing.end = Some(end);
    });
}
