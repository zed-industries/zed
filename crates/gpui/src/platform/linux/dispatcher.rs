#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{PlatformDispatcher, TaskLabel};
use async_task::Runnable;
use parking::{Parker, Unparker};
use parking_lot::Mutex;
use std::{
    panic, thread,
    time::{Duration, Instant},
};

pub(crate) struct LinuxDispatcher {
    parker: Mutex<Parker>,
    timed_tasks: Mutex<Vec<(Instant, Runnable)>>,
    main_sender: flume::Sender<Runnable>,
    main_receiver: flume::Receiver<Runnable>,
    background_sender: flume::Sender<Runnable>,
    background_thread: thread::JoinHandle<()>,
    main_thread_id: thread::ThreadId,
}

impl Default for LinuxDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxDispatcher {
    pub fn new() -> Self {
        let (main_sender, main_receiver) = flume::unbounded::<Runnable>();
        let (background_sender, background_receiver) = flume::unbounded::<Runnable>();
        let background_thread = thread::spawn(move || {
            for runnable in background_receiver {
                let _ignore_panic = panic::catch_unwind(|| runnable.run());
            }
        });
        LinuxDispatcher {
            parker: Mutex::new(Parker::new()),
            timed_tasks: Mutex::new(Vec::new()),
            main_sender,
            main_receiver,
            background_sender,
            background_thread,
            main_thread_id: thread::current().id(),
        }
    }

    pub fn tick_main(&self) {
        assert!(self.is_main_thread());
        if let Ok(runnable) = self.main_receiver.try_recv() {
            runnable.run();
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
        self.main_sender.send(runnable).unwrap();
    }

    fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
        let moment = Instant::now() + duration;
        let mut timed_tasks = self.timed_tasks.lock();
        timed_tasks.push((moment, runnable));
        timed_tasks.sort_unstable_by(|&(ref a, _), &(ref b, _)| b.cmp(a));
    }

    fn tick(&self, background_only: bool) -> bool {
        let mut ran = false;
        if self.is_main_thread() && !background_only {
            for runnable in self.main_receiver.try_iter() {
                runnable.run();
                ran = true;
            }
        }
        let mut timed_tasks = self.timed_tasks.lock();
        while let Some(&(moment, _)) = timed_tasks.last() {
            if moment <= Instant::now() {
                let (_, runnable) = timed_tasks.pop().unwrap();
                runnable.run();
                ran = true;
            } else {
                break;
            }
        }
        ran
    }

    fn park(&self) {
        self.parker.lock().park()
    }

    fn unparker(&self) -> Unparker {
        self.parker.lock().unparker()
    }
}
