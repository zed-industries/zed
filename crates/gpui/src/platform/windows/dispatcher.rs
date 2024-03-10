use std::{
    cmp::Ordering,
    thread::{current, JoinHandle, ThreadId},
    time::{Duration, Instant},
};

use async_task::Runnable;
use collections::BinaryHeap;
use flume::{RecvTimeoutError, Sender};
use parking::Parker;
use parking_lot::Mutex;
use windows::Win32::{Foundation::HANDLE, System::Threading::SetEvent};

use crate::{PlatformDispatcher, TaskLabel};

pub(crate) struct WindowsDispatcher {
    background_sender: Sender<(Runnable, Option<TaskLabel>)>,
    main_sender: Sender<Runnable>,
    timer_sender: Sender<(Runnable, Duration)>,
    background_threads: Vec<JoinHandle<()>>,
    timer_thread: JoinHandle<()>,
    parker: Mutex<Parker>,
    main_thread_id: ThreadId,
    event: HANDLE,
}

impl WindowsDispatcher {
    pub(crate) fn new(main_sender: Sender<Runnable>, event: HANDLE) -> Self {
        let parker = Mutex::new(Parker::new());
        let (background_sender, background_receiver) =
            flume::unbounded::<(Runnable, Option<TaskLabel>)>();
        let background_threads = (0..std::thread::available_parallelism()
            .map(|i| i.get())
            .unwrap_or(1))
            .map(|_| {
                let receiver = background_receiver.clone();
                std::thread::spawn(move || {
                    for (runnable, label) in receiver {
                        if let Some(label) = label {
                            log::debug!("TaskLabel: {label:?}");
                        }
                        runnable.run();
                    }
                })
            })
            .collect::<Vec<_>>();
        let (timer_sender, timer_receiver) = flume::unbounded::<(Runnable, Duration)>();
        let timer_thread = std::thread::spawn(move || {
            let mut runnables = BinaryHeap::<RunnableAfter>::new();
            let mut timeout_dur = None;
            loop {
                let recv = if let Some(dur) = timeout_dur {
                    match timer_receiver.recv_timeout(dur) {
                        Ok(recv) => Some(recv),
                        Err(RecvTimeoutError::Timeout) => None,
                        Err(RecvTimeoutError::Disconnected) => break,
                    }
                } else if let Ok(recv) = timer_receiver.recv() {
                    Some(recv)
                } else {
                    break;
                };
                let now = Instant::now();
                if let Some((runnable, dur)) = recv {
                    runnables.push(RunnableAfter {
                        runnable,
                        instant: now + dur,
                    });
                    while let Ok((runnable, dur)) = timer_receiver.try_recv() {
                        runnables.push(RunnableAfter {
                            runnable,
                            instant: now + dur,
                        })
                    }
                }
                while runnables.peek().is_some_and(|entry| entry.instant <= now) {
                    runnables.pop().unwrap().runnable.run();
                }
                timeout_dur = runnables.peek().map(|entry| entry.instant - now);
            }
        });
        let main_thread_id = current().id();
        Self {
            background_sender,
            main_sender,
            timer_sender,
            background_threads,
            timer_thread,
            parker,
            main_thread_id,
            event,
        }
    }
}

impl PlatformDispatcher for WindowsDispatcher {
    fn is_main_thread(&self) -> bool {
        current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: Runnable, label: Option<TaskLabel>) {
        self.background_sender
            .send((runnable, label))
            .inspect_err(|e| log::error!("Dispatch failed: {e}"))
            .ok();
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        self.main_sender
            .send(runnable)
            .inspect_err(|e| log::error!("Dispatch failed: {e}"))
            .ok();
        unsafe { SetEvent(self.event) }.ok();
    }

    fn dispatch_after(&self, duration: std::time::Duration, runnable: Runnable) {
        self.timer_sender
            .send((runnable, duration))
            .inspect_err(|e| log::error!("Dispatch failed: {e}"))
            .ok();
    }

    fn tick(&self, _background_only: bool) -> bool {
        false
    }

    fn park(&self) {
        self.parker.lock().park();
    }

    fn unparker(&self) -> parking::Unparker {
        self.parker.lock().unparker()
    }
}

struct RunnableAfter {
    runnable: Runnable,
    instant: Instant,
}

impl PartialEq for RunnableAfter {
    fn eq(&self, other: &Self) -> bool {
        self.instant == other.instant
    }
}

impl Eq for RunnableAfter {}

impl Ord for RunnableAfter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.instant.cmp(&other.instant).reverse()
    }
}

impl PartialOrd for RunnableAfter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
