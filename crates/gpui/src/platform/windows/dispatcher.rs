use std::{
    thread::{current, ThreadId},
    time::Duration,
};

use async_task::Runnable;
use parking::Parker;
use parking_lot::Mutex;
use util::ResultExt;
use windows::{
    Foundation::TimeSpan,
    System::{
        DispatcherQueue, DispatcherQueueController, DispatcherQueueHandler,
        Threading::{
            ThreadPool, ThreadPoolTimer, TimerElapsedHandler, WorkItemHandler, WorkItemOptions,
            WorkItemPriority,
        },
    },
    Win32::System::WinRT::{
        CreateDispatcherQueueController, DispatcherQueueOptions, DQTAT_COM_NONE,
        DQTYPE_THREAD_CURRENT,
    },
};

use crate::{PlatformDispatcher, TaskLabel};

pub(crate) struct WindowsDispatcher {
    controller: DispatcherQueueController,
    main_queue: DispatcherQueue,
    parker: Mutex<Parker>,
    main_thread_id: ThreadId,
}

impl WindowsDispatcher {
    pub(crate) fn new() -> Self {
        let controller = unsafe {
            let options = DispatcherQueueOptions {
                dwSize: std::mem::size_of::<DispatcherQueueOptions>() as u32,
                threadType: DQTYPE_THREAD_CURRENT,
                apartmentType: DQTAT_COM_NONE,
            };
            CreateDispatcherQueueController(options).unwrap()
        };
        let main_queue = controller.DispatcherQueue().unwrap();
        let parker = Mutex::new(Parker::new());
        let main_thread_id = current().id();

        WindowsDispatcher {
            controller,
            main_queue,
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

impl Drop for WindowsDispatcher {
    fn drop(&mut self) {
        self.controller.ShutdownQueueAsync().log_err();
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
        let handler = {
            let mut task_wrapper = Some(runnable);
            DispatcherQueueHandler::new(move || {
                task_wrapper.take().unwrap().run();
                Ok(())
            })
        };
        self.main_queue.TryEnqueue(&handler).log_err();
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
