use crate::{
    GLOBAL_THREAD_TIMINGS, PlatformDispatcher, RunnableVariant, THREAD_TIMINGS, TaskLabel,
    TaskTiming, ThreadTaskTimings,
};
use calloop::{
    EventLoop,
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
    main_sender: Sender<RunnableVariant>,
    timer_sender: Sender<TimerAfter>,
    background_sender: flume::Sender<RunnableVariant>,
    _background_threads: Vec<thread::JoinHandle<()>>,
    main_thread_id: thread::ThreadId,
}

impl LinuxDispatcher {
    pub fn new(main_sender: Sender<RunnableVariant>) -> Self {
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

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant) {
        self.main_sender.send(runnable).unwrap_or_else(|runnable| {
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
