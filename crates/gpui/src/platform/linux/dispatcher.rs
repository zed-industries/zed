use calloop::{
    EventLoop, PostAction,
    channel::{self, Sender},
    timer::TimeoutAction,
};
use util::ResultExt;

use std::{mem::MaybeUninit, thread, time::Duration};

use crate::{
    GLOBAL_THREAD_TIMINGS, GpuiRunnable, PlatformDispatcher, Priority, PriorityQueueReceiver,
    PriorityQueueSender, RealtimePriority, THREAD_TIMINGS, TaskLabel, ThreadTaskTimings,
};

struct TimerAfter {
    duration: Duration,
    runnable: GpuiRunnable,
}

pub(crate) struct LinuxDispatcher {
    main_sender: PriorityQueueCalloopSender<GpuiRunnable>,
    timer_sender: Sender<TimerAfter>,
    background_sender: PriorityQueueSender<GpuiRunnable>,
    _background_threads: Vec<thread::JoinHandle<()>>,
    main_thread_id: thread::ThreadId,
}

const MIN_THREADS: usize = 2;

impl LinuxDispatcher {
    pub fn new(main_sender: PriorityQueueCalloopSender<GpuiRunnable>) -> Self {
        let (background_sender, background_receiver) = PriorityQueueReceiver::new();
        let thread_count =
            std::thread::available_parallelism().map_or(MIN_THREADS, |i| i.get().max(MIN_THREADS));

        // These thread should really be lower prio then the foreground
        // executor
        let mut background_threads = (0..thread_count)
            .map(|i| {
                let mut receiver: PriorityQueueReceiver<GpuiRunnable> = background_receiver.clone();
                std::thread::Builder::new()
                    .name(format!("Worker-{i}"))
                    .spawn(move || {
                        for runnable in receiver.iter() {
                            let started = runnable.run_and_profile();
                            log::trace!(
                                "background thread {}: ran runnable. took: {:?}",
                                i,
                                started.elapsed()
                            );
                        }
                    })
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let (timer_sender, timer_channel) = calloop::channel::channel::<TimerAfter>();
        let timer_thread = std::thread::Builder::new()
            .name("Timer".to_owned())
            .spawn(|| {
                let mut event_loop: EventLoop<()> =
                    EventLoop::try_new().expect("Failed to initialize timer loop!");

                let handle = event_loop.handle();
                let timer_handle = event_loop.handle();
                handle
                    .insert_source(timer_channel, move |e, _, _| {
                        if let channel::Event::Msg(timer) = e {
                            // This has to be in an option to satisfy the borrow checker. The callback below should only be scheduled once.
                            let mut runnable = Some(timer.runnable);
                            timer_handle
                                .insert_source(
                                    calloop::timer::Timer::from_duration(timer.duration),
                                    move |_, _, _| {
                                        if let Some(runnable) = runnable.take() {
                                            runnable.run_and_profile();
                                        }
                                        TimeoutAction::Drop
                                    },
                                )
                                .expect("Failed to start timer");
                        }
                    })
                    .expect("Failed to start timer thread");

                event_loop.run(None, &mut (), |_| {}).log_err();
            })
            .unwrap();

        background_threads.push(timer_thread);

        Self {
            main_sender,
            timer_sender,
            background_sender,
            _background_threads: background_threads,
            main_thread_id: thread::current().id(),
        }
    }
}

impl PlatformDispatcher for LinuxDispatcher {
    fn get_all_timings(&self) -> Vec<crate::ThreadTaskTimings> {
        let global_timings = GLOBAL_THREAD_TIMINGS.lock();
        ThreadTaskTimings::convert(&global_timings)
    }

    fn get_current_thread_timings(&self) -> Vec<crate::TaskTiming> {
        THREAD_TIMINGS.with(|timings| {
            let timings = timings.lock();
            let timings = &timings.timings;

            let mut vec = Vec::with_capacity(timings.len());

            let (s1, s2) = timings.as_slices();
            vec.extend_from_slice(s1);
            vec.extend_from_slice(s2);
            vec
        })
    }

    fn is_main_thread(&self) -> bool {
        thread::current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: GpuiRunnable, _: Option<TaskLabel>) {
        self.background_sender
            .send(runnable.priority(), runnable)
            .unwrap_or_else(|_| panic!("blocking sender returned without value"));
    }

    fn dispatch_on_main_thread(&self, runnable: GpuiRunnable) {
        self.main_sender
            .send(runnable.priority(), runnable)
            .unwrap_or_else(|runnable| {
                // NOTE: Runnable may wrap a Future that is !Send.
                //
                // This is usually safe because we only poll it on the main thread.
                // However if the send fails, we know that:
                // 1. main_receiver has been dropped (which implies the app is shutting down)
                // 2. we are on a background thread.
                // It is not safe to drop something !Send on the wrong thread, and
                // the app will exit soon anyway, so we must forget the runnable.
                std::mem::forget(runnable);
            });
    }

    fn dispatch_after(&self, duration: Duration, runnable: GpuiRunnable) {
        self.timer_sender
            .send(TimerAfter { duration, runnable })
            .ok();
    }

    fn spawn_realtime(&self, priority: RealtimePriority, f: Box<dyn FnOnce() + Send>) {
        std::thread::spawn(move || {
            // SAFETY: always safe to call
            let thread_id = unsafe { libc::pthread_self() };

            let policy = match priority {
                RealtimePriority::Audio => libc::SCHED_FIFO,
                RealtimePriority::Other => libc::SCHED_RR,
            };
            let sched_priority = match priority {
                RealtimePriority::Audio => 65,
                RealtimePriority::Other => 45,
            };

            // SAFETY: all sched_param members are valid when initialized to zero.
            let mut sched_param =
                unsafe { MaybeUninit::<libc::sched_param>::zeroed().assume_init() };
            sched_param.sched_priority = sched_priority;
            // SAFETY: sched_param is a valid initialized structure
            let result = unsafe { libc::pthread_setschedparam(thread_id, policy, &sched_param) };
            if result != 0 {
                log::warn!("failed to set realtime thread priority to {:?}", priority);
            }

            f();
        });
    }
}

pub struct PriorityQueueCalloopSender<T> {
    sender: PriorityQueueSender<T>,
    ping: calloop::ping::Ping,
}

impl<T> PriorityQueueCalloopSender<T> {
    fn new(tx: PriorityQueueSender<T>, ping: calloop::ping::Ping) -> Self {
        Self { sender: tx, ping }
    }

    fn send(&self, priority: Priority, item: T) -> Result<(), crate::queue::SendError<T>> {
        let res = self.sender.send(priority, item);
        if res.is_ok() {
            self.ping.ping();
        }
        res
    }
}

impl<T> Drop for PriorityQueueCalloopSender<T> {
    fn drop(&mut self) {
        self.ping.ping();
    }
}

pub struct PriorityQueueCalloopReceiver<T> {
    receiver: PriorityQueueReceiver<T>,
    source: calloop::ping::PingSource,
    ping: calloop::ping::Ping,
}

impl<T> PriorityQueueCalloopReceiver<T> {
    pub fn new() -> (PriorityQueueCalloopSender<T>, Self) {
        let (ping, source) = calloop::ping::make_ping().expect("Failed to create a Ping.");

        let (tx, rx) = PriorityQueueReceiver::new();

        (
            PriorityQueueCalloopSender::new(tx, ping.clone()),
            Self {
                receiver: rx,
                source,
                ping,
            },
        )
    }
}

use calloop::channel::Event;

#[derive(Debug)]
pub struct ChannelError(calloop::ping::PingError);

impl std::fmt::Display for ChannelError {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for ChannelError {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl<T> calloop::EventSource for PriorityQueueCalloopReceiver<T> {
    type Event = Event<T>;
    type Metadata = ();
    type Ret = ();
    type Error = ChannelError;

    fn process_events<F>(
        &mut self,
        readiness: calloop::Readiness,
        token: calloop::Token,
        mut callback: F,
    ) -> Result<calloop::PostAction, Self::Error>
    where
        F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        let mut clear_readiness = false;
        let mut disconnected = false;

        let action = self
            .source
            .process_events(readiness, token, |(), &mut ()| {
                let mut is_empty = true;

                let mut receiver = self.receiver.clone();
                for runnable in receiver.try_iter() {
                    match runnable {
                        Ok(r) => {
                            callback(Event::Msg(r), &mut ());
                            is_empty = false;
                        }
                        Err(_) => {
                            disconnected = true;
                        }
                    }
                }

                if disconnected {
                    callback(Event::Closed, &mut ());
                }

                if is_empty {
                    clear_readiness = true;
                }
            })
            .map_err(ChannelError)?;

        if disconnected {
            Ok(PostAction::Remove)
        } else if clear_readiness {
            Ok(action)
        } else {
            // Re-notify the ping source so we can try again.
            self.ping.ping();
            Ok(PostAction::Continue)
        }
    }

    fn register(
        &mut self,
        poll: &mut calloop::Poll,
        token_factory: &mut calloop::TokenFactory,
    ) -> calloop::Result<()> {
        self.source.register(poll, token_factory)
    }

    fn reregister(
        &mut self,
        poll: &mut calloop::Poll,
        token_factory: &mut calloop::TokenFactory,
    ) -> calloop::Result<()> {
        self.source.reregister(poll, token_factory)
    }

    fn unregister(&mut self, poll: &mut calloop::Poll) -> calloop::Result<()> {
        self.source.unregister(poll)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calloop_works() {
        let mut event_loop = calloop::EventLoop::try_new().unwrap();
        let handle = event_loop.handle();

        let (tx, rx) = PriorityQueueCalloopReceiver::new();

        struct Data {
            got_msg: bool,
            got_closed: bool,
        }

        let mut data = Data {
            got_msg: false,
            got_closed: false,
        };

        let _channel_token = handle
            .insert_source(rx, move |evt, &mut (), data: &mut Data| match evt {
                Event::Msg(()) => {
                    data.got_msg = true;
                }

                Event::Closed => {
                    data.got_closed = true;
                }
            })
            .unwrap();

        // nothing is sent, nothing is received
        event_loop
            .dispatch(Some(::std::time::Duration::ZERO), &mut data)
            .unwrap();

        assert!(!data.got_msg);
        assert!(!data.got_closed);
        // a message is send

        tx.send(Priority::Medium, ()).unwrap();
        event_loop
            .dispatch(Some(::std::time::Duration::ZERO), &mut data)
            .unwrap();

        assert!(data.got_msg);
        assert!(!data.got_closed);

        // the sender is dropped
        drop(tx);
        event_loop
            .dispatch(Some(::std::time::Duration::ZERO), &mut data)
            .unwrap();

        assert!(data.got_msg);
        assert!(data.got_closed);
    }
}

// running 1 test
// test platform::linux::dispatcher::tests::tomato ... FAILED

// failures:

// ---- platform::linux::dispatcher::tests::tomato stdout ----
// [crates/gpui/src/platform/linux/dispatcher.rs:262:9]
// returning 1 tasks to process
// [crates/gpui/src/platform/linux/dispatcher.rs:480:75] evt = Msg(
//     (),
// )
// returning 0 tasks to process

// thread 'platform::linux::dispatcher::tests::tomato' (478301) panicked at crates/gpui/src/platform/linux/dispatcher.rs:515:9:
// assertion failed: data.got_closed
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
