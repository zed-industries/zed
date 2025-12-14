use crate::{PlatformDispatcher, Priority as GpuiPriority, RunnableVariant};
use async_task::Runnable;
use futures::FutureExt as _;
use futures::future::LocalBoxFuture;
use parking::Parker;
use scheduler::{Clock, Priority, RunnableMeta, Scheduler, SessionId, SystemClock, Timer};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use waker_fn::waker_fn;

/// Adapter that implements the `Scheduler` trait by wrapping a `PlatformDispatcher`.
/// This is used for production (non-test) environments.
pub struct PlatformScheduler {
    dispatcher: Arc<dyn PlatformDispatcher>,
    clock: Arc<SystemClock>,
}

impl PlatformScheduler {
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        Self {
            dispatcher,
            clock: Arc::new(SystemClock),
        }
    }
}

impl Scheduler for PlatformScheduler {
    fn block(
        &self,
        _session_id: Option<SessionId>,
        mut future: LocalBoxFuture<()>,
        timeout: Option<Duration>,
    ) {
        let deadline = timeout.map(|t| Instant::now() + t);

        let parker = Parker::new();
        let unparker = parker.unparker();
        let waker = waker_fn(move || {
            unparker.unpark();
        });
        let mut cx = Context::from_waker(&waker);

        loop {
            match future.poll_unpin(&mut cx) {
                Poll::Ready(()) => return,
                Poll::Pending => {
                    if let Some(deadline) = deadline {
                        let now = Instant::now();
                        if now >= deadline {
                            return;
                        }
                        let timeout = deadline.saturating_duration_since(now);
                        parker.park_timeout(timeout);
                    } else {
                        parker.park();
                    }
                }
            }
        }
    }

    fn schedule_foreground(&self, _session_id: SessionId, runnable: Runnable<RunnableMeta>) {
        self.dispatcher.dispatch_on_main_thread(
            RunnableVariant::Scheduler(runnable),
            GpuiPriority::default(),
        );
    }

    fn schedule_background_with_priority(
        &self,
        runnable: Runnable<RunnableMeta>,
        priority: Priority,
    ) {
        let gpui_priority = match priority {
            Priority::High => GpuiPriority::High,
            Priority::Medium => GpuiPriority::Medium,
            Priority::Low => GpuiPriority::Low,
        };
        self.dispatcher
            .dispatch(RunnableVariant::Scheduler(runnable), None, gpui_priority);
    }

    fn timer(&self, duration: Duration) -> Timer {
        let (tx, rx) = futures::channel::oneshot::channel();
        let (runnable, task) = async_task::spawn(
            async move {
                let _ = tx.send(());
            },
            {
                let dispatcher = self.dispatcher.clone();
                move |runnable: async_task::Runnable| {
                    dispatcher.dispatch_after(duration, RunnableVariant::Compat(runnable))
                }
            },
        );
        runnable.schedule();
        task.detach();
        Timer::new(rx)
    }

    fn clock(&self) -> Arc<dyn Clock> {
        self.clock.clone()
    }
}
