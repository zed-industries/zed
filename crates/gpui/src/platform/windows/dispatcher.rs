use std::{
    ffi::c_void,
    sync::{
        atomic::{AtomicIsize, Ordering},
        Arc,
    },
    thread::{current, ThreadId},
    time::Duration,
};

use async_task::Runnable;
use flume::Sender;
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
    Win32::{
        Foundation::*,
        System::{
            Threading::*,
            WinRT::{
                CreateDispatcherQueueController, DispatcherQueueOptions, DQTAT_COM_NONE,
                DQTYPE_THREAD_CURRENT,
            },
        },
    },
};

use crate::{PlatformDispatcher, TaskLabel};

pub(crate) struct WindowsDispatcher {
    controller: DispatcherQueueController,
    main_queue: DispatcherQueue,
    parker: Mutex<Parker>,
    main_thread_id: ThreadId,
}

unsafe impl Send for WindowsDispatcher {}
unsafe impl Sync for WindowsDispatcher {}

impl WindowsDispatcher {
    pub(crate) fn new(main_sender: Sender<Runnable>, dispatch_event: HANDLE) -> Self {
        let parker = Mutex::new(Parker::new());
        // Windows 10 Fall Creators Update (introduced in 10.0.16299.0)
        let controller = unsafe {
            let options = DispatcherQueueOptions {
                dwSize: std::mem::size_of::<DispatcherQueueOptions>() as u32,
                threadType: DQTYPE_THREAD_CURRENT,
                apartmentType: DQTAT_COM_NONE,
            };
            CreateDispatcherQueueController(options).unwrap()
        };
        let main_queue = controller.DispatcherQueue().unwrap();
        let main_thread_id = current().id();

        WindowsDispatcher {
            controller,
            main_queue,
            parker,
            main_thread_id,
        }
    }

    fn dispatch_on_threadpool(&self, runnable: Runnable, duration: Option<Duration>) {
        if let Some(duration) = duration {
            let task_wrapper = TaskWrapper(runnable.into_raw().as_ptr() as *mut c_void);
            let handler = TimerElapsedHandler::new(move |param| {
                let task = unsafe {
                    let captured = task_wrapper;
                    Runnable::<()>::from_raw(std::ptr::NonNull::new_unchecked(
                        captured.0 as *mut (),
                    ))
                };
                task.run();
                Ok(())
            });
            let timer =
            // A time period expressed in 100-nanosecond units.
            // 10,000,000 ticks per second
                ThreadPoolTimer::CreateTimer(&handler, TimeSpan { Duration: (duration.as_nanos() / 100) as i64}).unwrap();
        } else {
            let task_wrapper = TaskWrapper(runnable.into_raw().as_ptr() as *mut c_void);
            let handler = WorkItemHandler::new(move |param| {
                let task = unsafe {
                    let captured = task_wrapper;
                    Runnable::<()>::from_raw(std::ptr::NonNull::new_unchecked(
                        captured.0 as *mut (),
                    ))
                };
                task.run();
                Ok(())
            });
            let x = ThreadPool::RunWithPriorityAndOptionsAsync(
                &handler,
                WorkItemPriority::High,
                WorkItemOptions::TimeSliced,
            )
            .unwrap();
        }
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

#[derive(Debug)]
struct ThreadpoolTaskWrapper {
    runnable: Runnable,
    duration: Option<Duration>,
}

impl PlatformDispatcher for WindowsDispatcher {
    fn is_main_thread(&self) -> bool {
        current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: Runnable, label: Option<TaskLabel>) {
        self.dispatch_on_threadpool(runnable, None);
        if let Some(label) = label {
            log::debug!("TaskLabel: {label:?}");
        }
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        let task_wrapper = TaskWrapper(runnable.into_raw().as_ptr() as *mut c_void);
        let runner = DispatcherQueueHandler::new(move || {
            let task = unsafe {
                let captured = task_wrapper;
                Runnable::<()>::from_raw(std::ptr::NonNull::new_unchecked(captured.0 as *mut ()))
            };
            task.run();
            Ok(())
        });
        self.main_queue.TryEnqueue(&runner).log_err();
    }

    fn dispatch_after(&self, duration: std::time::Duration, runnable: Runnable) {
        self.dispatch_on_threadpool(runnable, Some(duration));
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

fn main_queue_runner() -> windows::core::Result<()> {
    Ok(())
}

extern "system" fn threadpool_runner(
    _: PTP_CALLBACK_INSTANCE,
    ptr: *mut std::ffi::c_void,
    _: PTP_WORK,
) {
    unsafe {
        let ThreadpoolTaskWrapper { runnable, duration } =
            *Box::from_raw(ptr as *mut ThreadpoolTaskWrapper);
        if let Some(duration) = duration {
            std::thread::sleep(duration);
        }
        runnable.run();
    }
}

#[inline]
fn get_threadpool_environment(pool: PTP_POOL) -> TP_CALLBACK_ENVIRON_V3 {
    TP_CALLBACK_ENVIRON_V3 {
        Version: 3, // Win7+, otherwise this value should be 1
        Pool: pool,
        CallbackPriority: TP_CALLBACK_PRIORITY_HIGH,
        Size: std::mem::size_of::<TP_CALLBACK_ENVIRON_V3>() as _,
        ..Default::default()
    }
}
