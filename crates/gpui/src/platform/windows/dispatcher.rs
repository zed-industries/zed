use std::{
    sync::{
        atomic::{AtomicIsize, Ordering},
        Arc,
    },
    thread::{current, ThreadId},
};

use async_task::Runnable;
use flume::Sender;
use parking::Parker;
use parking_lot::Mutex;
use windows::Win32::{Foundation::*, System::Threading::*};

use crate::{PlatformDispatcher, TaskLabel};

pub(crate) struct WindowsDispatcher {
    threadpool: PTP_POOL,
    main_sender: Sender<Runnable>,
    parker: Mutex<Parker>,
    main_thread_id: ThreadId,
    dispatch_event: HANDLE,
}

impl WindowsDispatcher {
    pub(crate) fn new(main_sender: Sender<Runnable>, dispatch_event: HANDLE) -> Self {
        let parker = Mutex::new(Parker::new());
        let threadpool = unsafe {
            let ret = CreateThreadpool(None);
            if ret.0 == 0 {
                panic!(
                    "unable to initialize a thread pool: {}",
                    std::io::Error::last_os_error()
                );
            }
            // set minimum 1 thread in threadpool
            let _ = SetThreadpoolThreadMinimum(ret, 1)
                .inspect_err(|_| log::error!("unable to configure thread pool"));

            ret
        };
        let main_thread_id = current().id();
        WindowsDispatcher {
            threadpool,
            main_sender,
            parker,
            main_thread_id,
            dispatch_event,
        }
    }

    fn dispatch_on_threadpool(&self, runnable: Runnable) {
        unsafe {
            let ptr = Box::into_raw(Box::new(runnable));
            let environment = get_threadpool_environment(self.threadpool);
            let Ok(work) =
                CreateThreadpoolWork(Some(threadpool_runner), Some(ptr as _), Some(&environment))
                    .inspect_err(|_| {
                        log::error!(
                            "unable to dispatch work on thread pool: {}",
                            std::io::Error::last_os_error()
                        )
                    })
            else {
                return;
            };
            SubmitThreadpoolWork(work);
        }
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
            .inspect_err(|e| log::error!("Dispatch failed: {e}"))
            .ok();
        unsafe { SetEvent(self.dispatch_event) }.ok();
    }

    fn dispatch_after(&self, duration: std::time::Duration, runnable: Runnable) {
        if duration.as_millis() == 0 {
            self.dispatch_on_threadpool(runnable);
            return;
        }
        unsafe {
            let mut handle = std::mem::zeroed();
            let task = Arc::new(DelayedTask::new(runnable));
            let _ = CreateTimerQueueTimer(
                &mut handle,
                None,
                Some(timer_queue_runner),
                Some(Arc::into_raw(task.clone()) as _),
                duration.as_millis() as u32,
                0,
                WT_EXECUTEONLYONCE,
            )
            .inspect_err(|_| {
                log::error!(
                    "unable to dispatch delayed task: {}",
                    std::io::Error::last_os_error()
                )
            });
            task.raw_timer_handle.store(handle.0, Ordering::SeqCst);
        }
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

extern "system" fn threadpool_runner(
    _: PTP_CALLBACK_INSTANCE,
    ptr: *mut std::ffi::c_void,
    _: PTP_WORK,
) {
    unsafe {
        let runnable = Box::from_raw(ptr as *mut Runnable);
        runnable.run();
    }
}

unsafe extern "system" fn timer_queue_runner(ptr: *mut std::ffi::c_void, _: BOOLEAN) {
    let task = Arc::from_raw(ptr as *mut DelayedTask);
    task.runnable.lock().take().unwrap().run();
    unsafe {
        let timer = task.raw_timer_handle.load(Ordering::SeqCst);
        let _ = DeleteTimerQueueTimer(None, HANDLE(timer), None);
    }
}

struct DelayedTask {
    runnable: Mutex<Option<Runnable>>,
    raw_timer_handle: AtomicIsize,
}

impl DelayedTask {
    pub fn new(runnable: Runnable) -> Self {
        DelayedTask {
            runnable: Mutex::new(Some(runnable)),
            raw_timer_handle: AtomicIsize::new(0),
        }
    }
}

#[inline]
fn get_threadpool_environment(pool: PTP_POOL) -> TP_CALLBACK_ENVIRON_V3 {
    TP_CALLBACK_ENVIRON_V3 {
        Version: 3, // Win7+, otherwise this value should be 1
        Pool: pool,
        CallbackPriority: TP_CALLBACK_PRIORITY_NORMAL,
        Size: std::mem::size_of::<TP_CALLBACK_ENVIRON_V3>() as _,
        ..Default::default()
    }
}
