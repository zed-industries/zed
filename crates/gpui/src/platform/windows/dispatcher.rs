use std::{
    ffi::c_void,
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

macro_rules! generate_handler {
    ($handler_type: tt, $runnable: ident, $has_param: ident) => {{
        macro_rules! generate_func {
            (true, $function: block) => {
                $handler_type::new(move |_| $function)
            };
            (false, $function: block) => {
                $handler_type::new(move || $function)
            };
        }
        let runnable = $runnable;
        let task_wrapper = TaskWrapper(runnable.into_raw().as_ptr() as *mut c_void);
        generate_func!($has_param, {
            let task = unsafe {
                let captured = task_wrapper;
                Runnable::<()>::from_raw(std::ptr::NonNull::new_unchecked(captured.0 as *mut ()))
            };
            task.run();
            Ok(())
        })
    }};
}

pub(crate) struct WindowsDispatcher {
    controller: DispatcherQueueController,
    main_queue: DispatcherQueue,
    parker: Mutex<Parker>,
    main_thread_id: ThreadId,
}

unsafe impl Send for WindowsDispatcher {}
unsafe impl Sync for WindowsDispatcher {}

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
        let runner = generate_handler!(WorkItemHandler, runnable, true);
        ThreadPool::RunWithPriorityAndOptionsAsync(
            &runner,
            WorkItemPriority::High,
            WorkItemOptions::TimeSliced,
        )
        .log_err();
    }

    fn dispatch_on_threadpool_after(&self, runnable: Runnable, duration: Duration) {
        let runner = generate_handler!(TimerElapsedHandler, runnable, true);
        let delay = TimeSpan {
            // A time period expressed in 100-nanosecond units.
            // 10,000,000 ticks per second
            Duration: (duration.as_nanos() / 100) as i64,
        };
        ThreadPoolTimer::CreateTimer(&runner, delay).log_err();
    }
}

impl Drop for WindowsDispatcher {
    fn drop(&mut self) {
        self.controller.ShutdownQueueAsync().log_err();
    }
}

#[derive(Debug, Clone, Copy)]
struct TaskWrapper(*mut c_void);

unsafe impl Send for TaskWrapper {}
unsafe impl Sync for TaskWrapper {}

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
        let runner = generate_handler!(DispatcherQueueHandler, runnable, false);
        self.main_queue.TryEnqueue(&runner).log_err();
    }

    fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
        self.dispatch_on_threadpool_after(runnable, duration);
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
