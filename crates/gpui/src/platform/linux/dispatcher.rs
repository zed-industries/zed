use crate::{
    GLOBAL_THREAD_TIMINGS, PlatformDispatcher, RunnableVariant, THREAD_TIMINGS, TaskLabel,
    TaskTiming, ThreadTaskTimings,
};
use calloop::{
    EventLoop, PostAction,
    channel::{self, Sender},
    timer::TimeoutAction,
};
use std::{
    thread,
    time::{Duration, Instant},
};
use util::ResultExt;

struct TimerAfter {
    duration: Duration,
    runnable: RunnableVariant,
}

pub(crate) struct LinuxDispatcher {
    main_sender: BadPriorityQueueSender<RunnableVariant>,
    timer_sender: Sender<TimerAfter>,
    background_sender: flume::Sender<RunnableVariant>,
    _background_threads: Vec<thread::JoinHandle<()>>,
    main_thread_id: thread::ThreadId,
}

impl LinuxDispatcher {
    pub fn new(main_sender: BadPriorityQueueSender<RunnableVariant>) -> Self {
        let (background_sender, background_receiver) = flume::unbounded::<RunnableVariant>();
        let thread_count = std::thread::available_parallelism()
            .map(|i| i.get())
            .unwrap_or(1);

        let mut background_threads = (0..thread_count)
            .map(|i| {
                let receiver = background_receiver.clone();
                std::thread::Builder::new()
                    .name(format!("Worker-{i}"))
                    .spawn(move || {
                        for runnable in receiver {
                            let start = Instant::now();

                            let mut location = match runnable {
                                RunnableVariant::Meta(runnable) => {
                                    let location = runnable.metadata().location;
                                    let timing = TaskTiming {
                                        location,
                                        start,
                                        end: None,
                                    };
                                    Self::add_task_timing(timing);

                                    runnable.run();
                                    timing
                                }
                                RunnableVariant::Compat(runnable) => {
                                    let location = core::panic::Location::caller();
                                    let timing = TaskTiming {
                                        location,
                                        start,
                                        end: None,
                                    };
                                    Self::add_task_timing(timing);

                                    runnable.run();
                                    timing
                                }
                            };

                            let end = Instant::now();
                            location.end = Some(end);
                            Self::add_task_timing(location);

                            log::trace!(
                                "background thread {}: ran runnable. took: {:?}",
                                i,
                                start.elapsed()
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
                                            let start = Instant::now();
                                            let mut timing = match runnable {
                                                RunnableVariant::Meta(runnable) => {
                                                    let location = runnable.metadata().location;
                                                    let timing = TaskTiming {
                                                        location,
                                                        start,
                                                        end: None,
                                                    };
                                                    Self::add_task_timing(timing);

                                                    runnable.run();
                                                    timing
                                                }
                                                RunnableVariant::Compat(runnable) => {
                                                    let timing = TaskTiming {
                                                        location: core::panic::Location::caller(),
                                                        start,
                                                        end: None,
                                                    };
                                                    Self::add_task_timing(timing);

                                                    runnable.run();
                                                    timing
                                                }
                                            };
                                            let end = Instant::now();

                                            timing.end = Some(end);
                                            Self::add_task_timing(timing);
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

    pub(crate) fn add_task_timing(timing: TaskTiming) {
        THREAD_TIMINGS.with(|timings| {
            let mut timings = timings.lock();
            let timings = &mut timings.timings;

            if let Some(last_timing) = timings.iter_mut().rev().next() {
                if last_timing.location == timing.location {
                    last_timing.end = timing.end;
                    return;
                }
            }

            timings.push_back(timing);
        });
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

    fn dispatch(&self, runnable: RunnableVariant, _: Option<TaskLabel>) {
        self.background_sender.send(runnable).unwrap();
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, priority: Priority) {
        self.main_sender
            .send(priority, runnable)
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

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        self.timer_sender
            .send(TimerAfter { duration, runnable })
            .ok();
    }
}

#[derive(Clone, Copy, Debug, Default, Ord, PartialOrd, PartialEq, Eq)]
#[repr(u8)]
pub enum Priority {
    High,
    #[default]
    Medium,
    Low,
}

impl Priority {
    fn ticket_percentage(&self) -> f32 {
        match self {
            Priority::High => 0.6f32,
            Priority::Medium => 0.3f32,
            Priority::Low => 0.1f32,
        }
    }
}

pub struct BadPriorityQueueSender<T> {
    sender: flume::Sender<(Priority, T)>,
    ping: calloop::ping::Ping,
}

impl<T> BadPriorityQueueSender<T> {
    fn new(tx: flume::Sender<(Priority, T)>, ping: calloop::ping::Ping) -> Self {
        Self { sender: tx, ping }
    }

    fn send(&self, priority: Priority, item: T) -> Result<(), flume::SendError<(Priority, T)>> {
        let res = self.sender.send((priority, item));
        if res.is_ok() {
            self.ping.ping();
        }
        res
    }
}

impl<T> Drop for BadPriorityQueueSender<T> {
    fn drop(&mut self) {
        self.ping.ping();
    }
}

struct BadReceiverState<T> {
    receiver: flume::Receiver<(Priority, T)>,
    high_priority: Vec<T>,
    medium_priority: Vec<T>,
    low_priority: Vec<T>,
    disconnected: bool,
}

impl<T> BadReceiverState<T> {
    const TICKET_COUNT: usize = 100;

    fn new(receiver: flume::Receiver<(Priority, T)>) -> Self {
        BadReceiverState {
            receiver,
            high_priority: Vec::new(),
            medium_priority: Vec::new(),
            low_priority: Vec::new(),
            disconnected: false,
        }
    }

    fn pop(&mut self) -> impl Iterator<Item = T> {
        let mut max_count = Self::TICKET_COUNT;
        loop {
            match self.receiver.try_recv() {
                Ok((priority, item)) => {
                    max_count -= 1;
                    match priority {
                        Priority::High => self.high_priority.push(item),
                        Priority::Medium => self.medium_priority.push(item),
                        Priority::Low => self.low_priority.push(item),
                    }

                    if max_count == 0 {
                        break;
                    }
                }
                Err(flume::TryRecvError::Empty) => {
                    break;
                }
                Err(flume::TryRecvError::Disconnected) => {
                    self.disconnected = true;
                    break;
                }
            }
        }

        let mut results = Vec::new();

        let mut ticket_count = Self::TICKET_COUNT;

        // todo(kate): make it also consider the other way around where there's
        // a lot of high priority tasks and no low priority ones i have it a
        // math problem and will solve it later
        // todo(for kate): tests, soo many tests
        let high_percentage = Priority::High.ticket_percentage();
        let medium_percentage = Priority::Medium.ticket_percentage() / (1.0f32 - high_percentage);
        let low_percentage = (Priority::Low.ticket_percentage() / (1.0f32 - high_percentage))
            / (1.0f32 - medium_percentage);

        let high_taken = (ticket_count as f32 * high_percentage).ceil() as usize;
        ticket_count -= high_taken;

        let medium_taken = (ticket_count as f32 / medium_percentage).ceil() as usize;
        ticket_count -= medium_taken;

        let low_taken = (ticket_count as f32 / low_percentage).ceil() as usize;

            let high_priority = self.high_priority
                .drain(..high_taken.min(self.high_priority.len()));
            let medium_priority =
            self.medium_priority
                .drain(..medium_taken.min(self.medium_priority.len()));
            let low_priority =
            self.low_priority
                .drain(..low_taken.min(self.low_priority.len()));

        results
    }
}

pub struct BadPriorityQueueReceiver<T> {
    receiver: BadReceiverState<T>,
    source: calloop::ping::PingSource,
    ping: calloop::ping::Ping,
}

impl<T> BadPriorityQueueReceiver<T> {
    pub fn new() -> (BadPriorityQueueSender<T>, Self) {
        let (ping, source) = calloop::ping::make_ping().expect("Failed to create a Ping.");

        let (tx, rx) = flume::unbounded();

        (
            BadPriorityQueueSender::new(tx, ping.clone()),
            Self {
                receiver: BadReceiverState::new(rx),
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

impl<T> calloop::EventSource for BadPriorityQueueReceiver<T> {
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

        let action = self
            .source
            .process_events(readiness, token, |(), &mut ()| {
                let runnables = self.receiver.pop();
                if runnables.is_empty() {
                    clear_readiness = true;
                }

                for runnable in runnables {
                    callback(Event::Msg(runnable), &mut ())
                }

                if self.receiver.disconnected {
                    callback(Event::Closed, &mut ())
                }
            })
            .map_err(ChannelError)?;

        if self.receiver.disconnected {
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
    fn tomato() {
        let mut event_loop = calloop::EventLoop::try_new().unwrap();
        let handle = event_loop.handle();

        let (tx, rx) = BadPriorityQueueReceiver::new();

        struct Data {
            got_msg: bool,
            got_closed: bool,
        }

        let mut data = Data {
            got_msg: false,
            got_closed: false,
        };

        let _channel_token = handle
            .insert_source(rx, move |evt, &mut (), data: &mut Data| match dbg!(evt) {
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
