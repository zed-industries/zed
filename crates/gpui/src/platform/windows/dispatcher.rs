use std::{
    thread::{current, ThreadId},
    time::Duration,
};

use anyhow::Context;
use async_task::Runnable;
use flume::Sender;
use parking::Parker;
use parking_lot::Mutex;
use util::ResultExt;
use windows::{
    Foundation::TimeSpan,
    System::Threading::{
        ThreadPool, ThreadPoolTimer, TimerElapsedHandler, WorkItemHandler, WorkItemOptions,
        WorkItemPriority,
    },
    Win32::{
        Foundation::{LPARAM, WPARAM},
        UI::WindowsAndMessaging::PostThreadMessageW,
    },
};

use crate::{PlatformDispatcher, TaskLabel, WM_GPUI_TASK_DISPATCHED_ON_MAIN_THREAD};

pub(crate) struct WindowsDispatcher {
    main_sender: Sender<Runnable>,
    parker: Mutex<Parker>,
    main_thread_id: ThreadId,
    main_thread_id_win32: u32,
    validation_number: usize,
}

impl WindowsDispatcher {
    pub(crate) fn new(
        main_sender: Sender<Runnable>,
        main_thread_id_win32: u32,
        validation_number: usize,
    ) -> Self {
        let parker = Mutex::new(Parker::new());
        let main_thread_id = current().id();

        WindowsDispatcher {
            main_sender,
            parker,
            main_thread_id,
            main_thread_id_win32,
            validation_number,
        }
    }

    fn dispatch_on_threadpool(&self, runnable: Runnable) {
        let handler = {
            let mut task_wrapper = Some(runnable);
            WorkItemHandler::new(move |_| {
                task_wrapper.take().unwrap().run();
                Ok(())
            })
        };
        ThreadPool::RunWithPriorityAndOptionsAsync(
            &handler,
            WorkItemPriority::High,
            WorkItemOptions::TimeSliced,
        )
        .log_err();
    }

    fn dispatch_on_threadpool_after(&self, runnable: Runnable, duration: Duration) {
        let handler = {
            let mut task_wrapper = Some(runnable);
            TimerElapsedHandler::new(move |_| {
                task_wrapper.take().unwrap().run();
                Ok(())
            })
        };
        let delay = TimeSpan {
            // A time period expressed in 100-nanosecond units.
            // 10,000,000 ticks per second
            Duration: (duration.as_nanos() / 100) as i64,
        };
        ThreadPoolTimer::CreateTimer(&handler, delay).log_err();
    }
}

impl PlatformDispatcher for WindowsDispatcher {
    fn is_main_thread(&self) -> bool {
        current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: Runnable, label: Option<TaskLabel>) {
        self.dispatch_on_threadpool(runnable);
        if let Some(label) = label {
            log::debug!("TaskLabel: {label:?}");
        }
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        if self
            .main_sender
            .send(runnable)
            .context("Dispatch on main thread failed")
            .log_err()
            .is_some()
        {
            unsafe {
                PostThreadMessageW(
                    self.main_thread_id_win32,
                    WM_GPUI_TASK_DISPATCHED_ON_MAIN_THREAD,
                    WPARAM(self.validation_number),
                    LPARAM(0),
                )
                .log_err();
            }
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
        self.dispatch_on_threadpool_after(runnable, duration);
    }

    fn park(&self, timeout: Option<Duration>) -> bool {
        if let Some(timeout) = timeout {
            self.parker.lock().park_timeout(timeout)
        } else {
            self.parker.lock().park();
            true
        }
    }

    fn unparker(&self) -> parking::Unparker {
        self.parker.lock().unparker()
    }
}
