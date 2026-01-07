#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{
    GLOBAL_THREAD_TIMINGS, PlatformDispatcher, Priority, RunnableVariant, THREAD_TIMINGS,
    TaskTiming, ThreadTaskTimings,
};

use objc::{
    class, msg_send,
    runtime::{BOOL, YES},
    sel, sel_impl,
};
use std::{
    ffi::c_void,
    ptr::addr_of,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
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

pub(crate) struct MacDispatcher {
    closed: Arc<AtomicBool>,
}

impl MacDispatcher {
    pub fn new() -> Self {
        Self {
            closed: Arc::new(AtomicBool::new(false)),
        }
    }
}

struct DispatchedRunnable {
    runnable: RunnableVariant,
    closed: Arc<AtomicBool>,
}

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

    fn dispatch(&self, runnable: RunnableVariant, priority: Priority) {
        if self.closed.load(Ordering::SeqCst) {
            return;
        }

        let dispatched = Box::new(DispatchedRunnable {
            runnable,
            closed: self.closed.clone(),
        });
        let context = Box::into_raw(dispatched) as *mut c_void;
        let trampoline = Some(trampoline as unsafe extern "C" fn(*mut c_void));

        let queue_priority = match priority {
            Priority::High => DISPATCH_QUEUE_PRIORITY_HIGH as isize,
            Priority::Medium => DISPATCH_QUEUE_PRIORITY_DEFAULT as isize,
            Priority::Low => DISPATCH_QUEUE_PRIORITY_LOW as isize,
        };

        unsafe {
            dispatch_async_f(
                dispatch_get_global_queue(queue_priority, 0),
                context,
                trampoline,
            );
        }
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        if self.closed.load(Ordering::SeqCst) {
            return;
        }

        let dispatched = Box::new(DispatchedRunnable {
            runnable,
            closed: self.closed.clone(),
        });
        let context = Box::into_raw(dispatched) as *mut c_void;
        let trampoline = Some(trampoline as unsafe extern "C" fn(*mut c_void));
        unsafe {
            dispatch_async_f(dispatch_get_main_queue(), context, trampoline);
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        if self.closed.load(Ordering::SeqCst) {
            return;
        }

        let dispatched = Box::new(DispatchedRunnable {
            runnable,
            closed: self.closed.clone(),
        });
        let context = Box::into_raw(dispatched) as *mut c_void;
        let trampoline = Some(trampoline as unsafe extern "C" fn(*mut c_void));
        unsafe {
            let queue =
                dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH.try_into().unwrap(), 0);
            let when = dispatch_time(DISPATCH_TIME_NOW as u64, duration.as_nanos() as i64);
            dispatch_after_f(when, queue, context, trampoline);
        }
    }

    fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
    }
}

extern "C" fn trampoline(context: *mut c_void) {
    let dispatched = unsafe { Box::from_raw(context as *mut DispatchedRunnable) };

    // Check if dispatcher was closed - if so, drop the task without running
    if dispatched.closed.load(Ordering::SeqCst) {
        return;
    }

    let runnable = dispatched.runnable;
    let metadata = runnable.metadata();
    let location = metadata.location;

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

    runnable.run();
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
