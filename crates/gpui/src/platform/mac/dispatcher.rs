#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{BackgroundExecutor, ForegroundExecutor, PlatformDispatcher, TaskLabel};
use async_task::Runnable;
use parking::{Parker, Unparker};
use parking_lot::Mutex;
use scheduler::Scheduler;
use std::{
    ffi::c_void,
    ptr::{NonNull, addr_of},
    sync::{
        Arc,
        atomic::{AtomicU16, Ordering::SeqCst},
    },
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

pub(crate) struct MacScheduler {
    next_session_id: AtomicU16,
}

impl MacScheduler {
    pub fn new() -> Self {
        MacScheduler {
            next_session_id: AtomicU16::new(0),
        }
    }

    pub fn background(self: &Arc<Self>) -> BackgroundExecutor {
        BackgroundExecutor::new(scheduler::BackgroundExecutor::new(self.clone()))
    }

    pub fn foreground(self: &Arc<Self>) -> ForegroundExecutor {
        let session_id = scheduler::SessionId::new(self.next_session_id.fetch_add(1, SeqCst));
        ForegroundExecutor::new(scheduler::ForegroundExecutor::new(session_id, self.clone()))
    }
}

impl Scheduler for MacScheduler {
    fn block(
        &self,
        session_id: scheduler::SessionId,
        future: futures::future::LocalBoxFuture<()>,
        timeout: Option<Duration>,
    ) {
        todo!()
    }

    fn schedule_foreground(&self, session_id: scheduler::SessionId, runnable: Runnable) {
        todo!()
    }

    fn schedule_background(&self, runnable: Runnable) {
        todo!()
    }

    fn timer(&self, timeout: Duration) -> scheduler::Timer {
        todo!()
    }

    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        todo!()
    }
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
        unsafe {
            dispatch_async_f(
                dispatch_get_main_queue(),
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline),
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
