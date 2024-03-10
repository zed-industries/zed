#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// todo(linux): remove
#![allow(unused_variables)]

use crate::{PlatformDispatcher, TaskLabel};
use async_task::Runnable;
use calloop::{
    channel::{self, Sender},
    timer::TimeoutAction,
    EventLoop,
};
use parking::{Parker, Unparker};
use parking_lot::Mutex;
use std::{thread, time::Duration};
use util::ResultExt;

struct TimerAfter {
    duration: Duration,
    runnable: Runnable,
}

pub(crate) struct LinuxDispatcher {
    parker: Mutex<Parker>,
    main_sender: Sender<Runnable>,
    timer_sender: Sender<TimerAfter>,
    background_sender: flume::Sender<Runnable>,
    _background_threads: Vec<thread::JoinHandle<()>>,
    main_thread_id: thread::ThreadId,
}

impl LinuxDispatcher {
    pub fn new(main_sender: Sender<Runnable>) -> Self {
        let (background_sender, background_receiver) = flume::unbounded::<Runnable>();
        let thread_count = std::thread::available_parallelism()
            .map(|i| i.get())
            .unwrap_or(1);

        let mut background_threads = (0..thread_count)
            .map(|_| {
                let receiver = background_receiver.clone();
                std::thread::spawn(move || {
                    for runnable in receiver {
                        runnable.run();
                    }
                })
            })
            .collect::<Vec<_>>();

        let (timer_sender, timer_channel) = calloop::channel::channel::<TimerAfter>();
        let timer_thread = std::thread::spawn(|| {
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
                                move |e, _, _| {
                                    if let Some(runnable) = runnable.take() {
                                        runnable.run();
                                    }
                                    TimeoutAction::Drop
                                },
                            )
                            .expect("Failed to start timer");
                    }
                })
                .expect("Failed to start timer thread");

            event_loop.run(None, &mut (), |_| {}).log_err();
        });

        background_threads.push(timer_thread);

        Self {
            parker: Mutex::new(Parker::new()),
            main_sender,
            timer_sender,
            background_sender,
            _background_threads: background_threads,
            main_thread_id: thread::current().id(),
        }
    }
}

impl PlatformDispatcher for LinuxDispatcher {
    fn is_main_thread(&self) -> bool {
        thread::current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: Runnable, _: Option<TaskLabel>) {
        self.background_sender.send(runnable).unwrap();
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        self.main_sender
            .send(runnable)
            .expect("Main thread is gone");
    }

    fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
        self.timer_sender
            .send(TimerAfter { duration, runnable })
            .expect("Timer thread has died");
    }

    fn tick(&self, background_only: bool) -> bool {
        false
    }

    fn park(&self) {
        self.parker.lock().park()
    }

    fn unparker(&self) -> Unparker {
        self.parker.lock().unparker()
    }
}
