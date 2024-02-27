use std::thread::JoinHandle;

use async_task::Runnable;
use flume::Sender;
use parking::Parker;
use parking_lot::Mutex;
use windows::Win32::System::Threading::GetCurrentThreadId;

use crate::{PlatformDispatcher, TaskLabel};

pub(crate) struct WindowsDispatcher {
    background_sender: Sender<(Runnable, Option<TaskLabel>)>,
    main_sender: Sender<Runnable>,
    background_threads: Vec<JoinHandle<()>>,
    parker: Mutex<Parker>,
}

impl WindowsDispatcher {
    pub(crate) fn new(main_sender: Sender<Runnable>) -> Self {
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
        Self {
            background_sender,
            main_sender,
            background_threads,
            parker,
        }
    }
}

extern "C" {
    static MAIN_THREAD_ID: u32;
}

impl PlatformDispatcher for WindowsDispatcher {
    fn is_main_thread(&self) -> bool {
        unsafe { GetCurrentThreadId() == MAIN_THREAD_ID }
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
    }

    fn dispatch_after(&self, duration: std::time::Duration, runnable: Runnable) {
        let time = std::time::Instant::now() + duration;
        let future = std::future::poll_fn(move |_| {
            let now = std::time::Instant::now();
            if now >= time {
                std::task::Poll::Ready(())
            } else {
                std::task::Poll::Pending
            }
        });
        let sender = self.background_sender.clone();
        let (runnable, task) = async_task::spawn(
            async {
                future.await;
                runnable.run();
            },
            move |runnable| {
                sender.send((runnable, None)).unwrap();
            },
        );
        self.background_sender
            .send((runnable, None))
            .inspect_err(|e| log::error!("Dispatch failed: {e}"))
            .ok();
        task.detach();
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
