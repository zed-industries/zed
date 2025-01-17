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
        Foundation::{HANDLE, HWND, LPARAM, WPARAM},
        System::Threading::SetEvent,
        UI::WindowsAndMessaging::PostMessageW,
    },
};

use crate::{PlatformDispatcher, SafeHandle, TaskLabel};

use super::EVENT_DISPATCHED;

pub(crate) struct WindowsDispatcher {
    main_sender: Sender<Runnable>,
    platform_window_hwnd: SafeHwnd,
    parker: Mutex<Parker>,
    main_thread_id: ThreadId,
}

struct SafeHwnd(HWND);

unsafe impl Send for SafeHwnd {}
unsafe impl Sync for SafeHwnd {}

impl WindowsDispatcher {
    pub(crate) fn new(main_sender: Sender<Runnable>, platform_window_hwnd: HWND) -> Self {
        let parker = Mutex::new(Parker::new());
        let main_thread_id = current().id();

        WindowsDispatcher {
            main_sender,
            platform_window_hwnd: SafeHwnd(platform_window_hwnd),
            parker,
            main_thread_id,
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
        self.main_sender
            .send(runnable)
            .context("Dispatch on main thread failed")
            .log_err();
        unsafe {
            PostMessageW(
                self.platform_window_hwnd.0,
                EVENT_DISPATCHED,
                WPARAM(0),
                LPARAM(0),
            )
            .log_err()
        };
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
