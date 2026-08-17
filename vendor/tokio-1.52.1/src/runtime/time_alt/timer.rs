use super::{EntryHandle, TempLocalContext};
use crate::runtime::scheduler::Handle as SchedulerHandle;
use crate::time::Instant;

use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(any(feature = "rt", feature = "rt-multi-thread"))]
use crate::util::error::RUNTIME_SHUTTING_DOWN_ERROR;

pub(crate) struct Timer {
    sched_handle: SchedulerHandle,

    /// The entry in the timing wheel.
    ///
    /// - `Some` if the timer is registered / pending / woken up / cancelling.
    /// - `None` if the timer is unregistered.
    entry: Option<EntryHandle>,

    /// The deadline for the timer.
    deadline: Instant,
}

impl std::fmt::Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timer")
            .field("deadline", &self.deadline)
            .finish()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        if let Some(entry) = self.entry.take() {
            entry.cancel();
        }
    }
}

impl Timer {
    #[track_caller]
    pub(crate) fn new(sched_hdl: SchedulerHandle, deadline: Instant) -> Self {
        // Panic if the time driver is not enabled
        let _ = sched_hdl.driver().time();
        Timer {
            sched_handle: sched_hdl,
            entry: None,
            deadline,
        }
    }

    pub(crate) fn deadline(&self) -> Instant {
        self.deadline
    }

    pub(crate) fn is_elapsed(&self) -> bool {
        self.entry.as_ref().is_some_and(|entry| entry.is_woken_up())
    }

    fn register(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();

        with_current_temp_local_context(&this.sched_handle, |maybe_time_cx| {
            let deadline = deadline_to_tick(&this.sched_handle, this.deadline);

            match maybe_time_cx {
                Some(TempLocalContext::Running {
                    registration_queue: _,
                    elapsed,
                }) if deadline <= elapsed => Poll::Ready(()),

                Some(TempLocalContext::Running {
                    registration_queue,
                    elapsed: _,
                }) => {
                    let hdl = EntryHandle::new(deadline, cx.waker().clone());
                    this.entry = Some(hdl.clone());
                    unsafe {
                        registration_queue.push_front(hdl);
                    }
                    Poll::Pending
                }
                #[cfg(feature = "rt-multi-thread")]
                Some(TempLocalContext::Shutdown) => panic!("{RUNTIME_SHUTTING_DOWN_ERROR}"),

                _ => {
                    let hdl = EntryHandle::new(deadline, cx.waker().clone());
                    this.entry = Some(hdl.clone());
                    push_from_remote(&this.sched_handle, hdl);
                    Poll::Pending
                }
            }
        })
    }

    pub(crate) fn poll_elapsed(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match self.entry.as_ref() {
            Some(entry) if entry.is_woken_up() => Poll::Ready(()),
            Some(entry) => {
                entry.register_waker(cx.waker().clone());
                Poll::Pending
            }
            None => self.register(cx),
        }
    }

    pub(crate) fn scheduler_handle(&self) -> &SchedulerHandle {
        &self.sched_handle
    }

    #[cfg(all(tokio_unstable, feature = "tracing"))]
    pub(crate) fn driver(&self) -> &crate::runtime::time::Handle {
        self.sched_handle.driver().time()
    }

    #[cfg(all(tokio_unstable, feature = "tracing"))]
    pub(crate) fn clock(&self) -> &crate::time::Clock {
        self.sched_handle.driver().clock()
    }
}

fn with_current_temp_local_context<F, R>(hdl: &SchedulerHandle, f: F) -> R
where
    F: FnOnce(Option<TempLocalContext<'_>>) -> R,
{
    #[cfg(not(feature = "rt"))]
    {
        let (_, _) = (hdl, f);
        panic!("Tokio runtime is not enabled, cannot access the current wheel");
    }

    #[cfg(feature = "rt")]
    {
        use crate::runtime::context;

        let is_same_rt =
            context::with_current(|cur_hdl| cur_hdl.is_same_runtime(hdl)).unwrap_or_default();

        if !is_same_rt {
            // We don't want to create the timer in one runtime,
            // but register it in a different runtime's timer wheel.
            f(None)
        } else {
            context::with_scheduler(|maybe_cx| match maybe_cx {
                Some(cx) => cx.with_time_temp_local_context(f),
                None => f(None),
            })
        }
    }
}

fn push_from_remote(sched_hdl: &SchedulerHandle, entry_hdl: EntryHandle) {
    #[cfg(not(feature = "rt"))]
    {
        let (_, _) = (sched_hdl, entry_hdl);
        panic!("Tokio runtime is not enabled, cannot access the current wheel");
    }

    #[cfg(feature = "rt")]
    {
        assert!(!sched_hdl.is_shutdown(), "{RUNTIME_SHUTTING_DOWN_ERROR}");
        sched_hdl.push_remote_timer(entry_hdl);
    }
}

fn deadline_to_tick(sched_hdl: &SchedulerHandle, deadline: Instant) -> u64 {
    let time_hdl = sched_hdl.driver().time();
    time_hdl.time_source().deadline_to_tick(deadline)
}
