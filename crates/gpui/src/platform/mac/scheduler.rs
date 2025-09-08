#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{BackgroundExecutor, ForegroundExecutor};
use async_task::Runnable;
use chrono::{DateTime, Utc};
use futures::{
    channel::oneshot,
    future::{self, LocalBoxFuture},
};
use scheduler::{Scheduler, SessionId, Timer};
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
        let session_id = SessionId::new(self.next_session_id.fetch_add(1, SeqCst));
        ForegroundExecutor::new(scheduler::ForegroundExecutor::new(session_id, self.clone()))
    }
}

impl Scheduler for MacScheduler {
    fn block(
        &self,
        _session_id: Option<SessionId>,
        future: LocalBoxFuture<()>,
        timeout: Option<Duration>,
    ) {
        if let Some(timeout) = timeout {
            let timer = self.timer(timeout);
            futures::executor::block_on(future::select(timer, future));
        } else {
            futures::executor::block_on(future);
        }
    }

    fn schedule_foreground(&self, _session_id: SessionId, runnable: Runnable) {
        unsafe {
            dispatch_async_f(
                dispatch_get_main_queue(),
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline),
            );
        }
    }

    fn schedule_background(&self, runnable: Runnable) {
        unsafe {
            dispatch_async_f(
                dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH.try_into().unwrap(), 0),
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline),
            );
        }
    }

    fn timer(&self, timeout: Duration) -> Timer {
        let (tx, rx) = oneshot::channel();
        let (runnable, task) = async_task::spawn(
            async move {
                tx.send(()).ok();
            },
            move |runnable: Runnable| unsafe {
                let queue =
                    dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH.try_into().unwrap(), 0);
                let when = dispatch_time(DISPATCH_TIME_NOW as u64, timeout.as_nanos() as i64);
                dispatch_after_f(
                    when,
                    queue,
                    runnable.into_raw().as_ptr() as *mut c_void,
                    Some(trampoline),
                );
            },
        );
        runnable.schedule();
        task.detach();

        Timer::new(rx)
    }

    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

extern "C" fn trampoline(runnable: *mut c_void) {
    let task = unsafe { Runnable::<()>::from_raw(NonNull::new_unchecked(runnable as *mut ())) };
    task.run();
}
