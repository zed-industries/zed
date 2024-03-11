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
use windows::Win32::{
    Foundation::{BOOLEAN, HANDLE},
    System::Threading::{
        CreateThreadpool, CreateThreadpoolWork, CreateTimerQueueTimer, DeleteTimerQueueTimer,
        SetEvent, SetThreadpoolThreadMinimum, SubmitThreadpoolWork, PTP_CALLBACK_INSTANCE,
        PTP_POOL, PTP_WORK, WT_EXECUTEONLYONCE,
    },
};

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
        let threadpool = init();
        let main_thread_id = current().id();
        WindowsDispatcher {
            threadpool,
            main_sender,
            parker,
            main_thread_id,
            dispatch_event,
        }
    }

    fn dispatch_on_threadpool(
        &self,
        runnable: Runnable,
        label: Option<TaskLabel>,
    ) -> anyhow::Result<()> {
        unsafe {
            let ptr = Box::into_raw(Box::new(runnable));
            let work = CreateThreadpoolWork(Some(threadpool_runner), Some(ptr as _), None)
                .inspect_err(|_| {
                    log::error!(
                        "unable to dispatch work on thread pool: {}",
                        std::io::Error::last_os_error()
                    )
                })?;
            SubmitThreadpoolWork(work);
            if let Some(label) = label {
                log::debug!("TaskLabel: {label:?}");
            }
        }
        Ok(())
    }
}

impl PlatformDispatcher for WindowsDispatcher {
    fn is_main_thread(&self) -> bool {
        current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: Runnable, label: Option<TaskLabel>) {
        let _ = self.dispatch_on_threadpool(runnable, label);
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
            let _ = self.dispatch_on_threadpool(runnable, None);
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
                    "unable to dispatch timed task: {}",
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

fn init() -> PTP_POOL {
    unsafe {
        let threadpool = CreateThreadpool(None);
        if threadpool.0 == 0 {
            log::error!(
                "unable to initialize a thread pool: {}",
                std::io::Error::last_os_error()
            );
            panic!();
        }
        let _ = SetThreadpoolThreadMinimum(threadpool, 1)
            .inspect_err(|_| log::error!("unable to configure thread pool"));
        threadpool
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
