use crate::{PlatformDispatcher, RunnableMeta};
use async_task::Runnable;
use chrono::{DateTime, Utc};
use futures::channel::oneshot;
use scheduler::Instant;
use scheduler::{
    Clock, LocalExecutor, Priority, Scheduler, SessionId, Task, TestScheduler, Timer,
    spawn_dedicated_thread,
};
#[cfg(not(target_family = "wasm"))]
use std::task::{Context, Poll};
use std::{
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicU16, Ordering},
    },
    time::Duration,
};

/// A production implementation of [`Scheduler`] that wraps a [`PlatformDispatcher`].
///
/// This allows GPUI to use the scheduler crate's executor types with the platform's
/// native dispatch mechanisms (e.g., Grand Central Dispatch on macOS).
pub struct PlatformScheduler {
    dispatcher: Arc<dyn PlatformDispatcher>,
    clock: Arc<PlatformClock>,
    next_session_id: AtomicU16,
}

impl PlatformScheduler {
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        Self {
            dispatcher: dispatcher.clone(),
            clock: Arc::new(PlatformClock { dispatcher }),
            next_session_id: AtomicU16::new(0),
        }
    }

    pub fn allocate_session_id(&self) -> SessionId {
        SessionId::new(self.next_session_id.fetch_add(1, Ordering::SeqCst))
    }

    /// Spawn work on a fresh OS thread that's exclusive to the returned
    /// task and anything spawned on the executor it provides. Blocking
    /// syscalls inside that work don't disturb any other executor in the
    /// process.
    ///
    /// `f` is called on the dedicated thread with a [`LocalExecutor`]
    /// pinned to it. The future `f` returns may freely be `!Send`. The
    /// returned `Task` is that future's task: dropping it cancels the
    /// root, but detached children keep running until they finish. The
    /// thread shuts down once the executor and every task on it are gone.
    ///
    /// Motivating use case: a single-threaded actor owning `!Send` state
    /// (e.g. a CRDT replica behind `Rc<RefCell<…>>`) doing blocking
    /// filesystem I/O.
    pub fn spawn_dedicated<F, Fut>(self: &Arc<Self>, f: F) -> Task<Fut::Output>
    where
        F: FnOnce(LocalExecutor) -> Fut + Send + 'static,
        Fut: Future + 'static,
        Fut::Output: Send + 'static,
    {
        spawn_dedicated_thread(self.allocate_session_id(), self.clone(), f)
    }
}

impl Scheduler for PlatformScheduler {
    fn block(
        &self,
        _session_id: Option<SessionId>,
        #[cfg_attr(target_family = "wasm", allow(unused_mut))] mut future: Pin<
            &mut dyn Future<Output = ()>,
        >,
        #[cfg_attr(target_family = "wasm", allow(unused_variables))] timeout: Option<Duration>,
    ) -> bool {
        #[cfg(target_family = "wasm")]
        {
            let _ = (&future, &timeout);
            panic!("Cannot block on wasm")
        }
        #[cfg(not(target_family = "wasm"))]
        {
            use waker_fn::waker_fn;
            let deadline = timeout.map(|t| Instant::now() + t);
            let parker = parking::Parker::new();
            let unparker = parker.unparker();
            let waker = waker_fn(move || {
                unparker.unpark();
            });
            let mut cx = Context::from_waker(&waker);
            if let Poll::Ready(()) = future.as_mut().poll(&mut cx) {
                return true;
            }

            let park_deadline = |deadline: Instant| {
                // Timer expirations are only delivered every ~15.6 milliseconds by default on Windows.
                // We increase the resolution during this wait so that short timeouts stay reasonably short.
                let _timer_guard = self.dispatcher.increase_timer_resolution();
                parker.park_deadline(deadline)
            };

            loop {
                match deadline {
                    Some(deadline) if !park_deadline(deadline) && deadline <= Instant::now() => {
                        return false;
                    }
                    Some(_) => (),
                    None => parker.park(),
                }
                if let Poll::Ready(()) = future.as_mut().poll(&mut cx) {
                    break true;
                }
            }
        }
    }

    fn schedule_local(&self, _session_id: SessionId, runnable: Runnable<RunnableMeta>) {
        self.dispatcher
            .dispatch_on_main_thread(runnable, Priority::default());
    }

    fn schedule_background_with_priority(
        &self,
        runnable: Runnable<RunnableMeta>,
        priority: Priority,
    ) {
        self.dispatcher.dispatch(runnable, priority);
    }

    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>) {
        self.dispatcher.spawn_realtime(f);
    }

    #[track_caller]
    fn timer(&self, duration: Duration) -> Timer {
        let (tx, rx) = oneshot::channel();
        let dispatcher = self.dispatcher.clone();

        // Create a runnable that will send the completion signal
        let location = std::panic::Location::caller();
        let (runnable, _task) = async_task::Builder::new()
            .metadata(RunnableMeta { location })
            .spawn(
                move |_| async move {
                    let _ = tx.send(());
                },
                move |runnable| {
                    dispatcher.dispatch_after(duration, runnable);
                },
            );
        runnable.schedule();

        Timer::new(rx)
    }

    fn clock(&self) -> Arc<dyn Clock> {
        self.clock.clone()
    }

    fn as_test(&self) -> Option<&TestScheduler> {
        None
    }
}

/// A production clock that uses the platform dispatcher's time.
struct PlatformClock {
    dispatcher: Arc<dyn PlatformDispatcher>,
}

impl Clock for PlatformClock {
    fn utc_now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    fn now(&self) -> Instant {
        self.dispatcher.now()
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use crate::{RunnableVariant, ThreadTaskTimings};
    use std::time::Instant as StdInstant;

    // `spawn_dedicated` shouldn't touch the platform dispatcher at all;
    // panicking on every method ensures the test catches it if it does.
    struct SmokeDispatcher;

    impl PlatformDispatcher for SmokeDispatcher {
        fn get_all_timings(&self) -> Vec<ThreadTaskTimings> {
            Vec::new()
        }
        fn get_current_thread_timings(&self) -> ThreadTaskTimings {
            ThreadTaskTimings {
                thread_name: None,
                thread_id: std::thread::current().id(),
                timings: Vec::new(),
                total_pushed: 0,
            }
        }
        fn is_main_thread(&self) -> bool {
            false
        }
        fn dispatch(&self, _runnable: RunnableVariant, _priority: Priority) {
            panic!("SmokeDispatcher should not be asked to dispatch in this test");
        }
        fn dispatch_on_main_thread(&self, _runnable: RunnableVariant, _priority: Priority) {
            panic!("SmokeDispatcher does not implement a main thread");
        }
        fn dispatch_after(&self, _duration: Duration, _runnable: RunnableVariant) {
            panic!("SmokeDispatcher does not implement timers");
        }
        fn spawn_realtime(&self, _f: Box<dyn FnOnce() + Send>) {
            panic!("SmokeDispatcher does not implement realtime");
        }
    }

    #[test]
    fn spawn_dedicated_runs_on_a_real_separate_thread() {
        let scheduler = Arc::new(PlatformScheduler::new(Arc::new(SmokeDispatcher)));
        let started = StdInstant::now();
        let task = scheduler.spawn_dedicated(|_executor| async move {
            // A genuine blocking syscall on the dedicated thread. If
            // `spawn_dedicated` were running the future on any shared
            // executor, this would stall that executor.
            let thread_id_before = std::thread::current().id();
            std::thread::sleep(Duration::from_millis(50));
            let thread_id_after = std::thread::current().id();
            assert_eq!(thread_id_before, thread_id_after);
            (thread_id_before, "slept")
        });
        let (dedicated_thread_id, message) = futures::executor::block_on(task);
        let elapsed = started.elapsed();
        assert_eq!(message, "slept");
        assert_ne!(
            dedicated_thread_id,
            std::thread::current().id(),
            "dedicated future ran on the test thread"
        );
        assert!(
            elapsed >= Duration::from_millis(40),
            "expected the dedicated thread to genuinely sleep, elapsed = {:?}",
            elapsed
        );
    }

    #[test]
    fn spawn_dedicated_returns_not_send_future_output() {
        // The whole point of `spawn_dedicated` is that the future can be
        // `!Send`. Constructing one with `Rc<RefCell<_>>` ensures the
        // signature actually permits it.
        use std::cell::RefCell;
        use std::rc::Rc;

        let scheduler = Arc::new(PlatformScheduler::new(Arc::new(SmokeDispatcher)));
        let task = scheduler.spawn_dedicated(|_executor| async move {
            let state = Rc::new(RefCell::new(0_i32));
            for _ in 0..3 {
                *state.borrow_mut() += 1;
            }
            *state.borrow()
        });
        let output = futures::executor::block_on(task);
        assert_eq!(output, 3);
    }
}
