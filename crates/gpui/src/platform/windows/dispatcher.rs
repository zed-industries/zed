use crate::{log_windows_error, PlatformDispatcher, TaskLabel};
use async_task::Runnable;
use parking::{Parker, Unparker};
use parking_lot::Mutex;
use std::{panic, thread, time::Duration};
use windows::Win32::{
    Foundation::{BOOLEAN, HANDLE, HWND, LPARAM, WPARAM},
    System::Threading::{
        CreateThreadpool, CreateThreadpoolWork, CreateTimerQueue, CreateTimerQueueTimer,
        SetThreadpoolThreadMinimum, SubmitThreadpoolWork, PTP_CALLBACK_INSTANCE, PTP_POOL,
        PTP_WORK, WT_EXECUTEONLYONCE,
    },
    UI::WindowsAndMessaging::PostMessageW,
};

pub(crate) struct WindowsDispatcher {
    dispatch_window: HWND,
    parker: Mutex<Parker>,
    main_sender: flume::Sender<Runnable>,
    main_thread_id: thread::ThreadId,
    threadpool: PTP_POOL,
    // timer queue maybe not accuracy
    // https://www.virtualdub.org/blog2/entry_272.html
    timer_queue: HANDLE,
}

impl WindowsDispatcher {
    pub fn new(main_sender: flume::Sender<Runnable>, dispatch_window_handle: HWND) -> Self {
        let (threadpool, timer_queue) = dispatcher_init().expect("error init dispatcher");

        Self {
            dispatch_window: dispatch_window_handle,
            parker: Mutex::new(Parker::new()),
            main_sender,
            main_thread_id: thread::current().id(),
            threadpool,
            timer_queue,
        }
    }

    fn send_dispatch_message(&self) -> anyhow::Result<()> {
        unsafe {
            PostMessageW(
                self.dispatch_window,
                super::MAIN_DISPATCH,
                WPARAM::default(),
                LPARAM::default(),
            )
            .inspect_err(log_windows_error)?;
        }
        Ok(())
    }

    fn dispatch_on_threadpool(&self, runnable: Runnable) -> anyhow::Result<()> {
        unsafe {
            let ptr = Box::into_raw(Box::new(runnable));
            let work = CreateThreadpoolWork(Some(background_runner), Some(ptr as _), None)
                .inspect_err(log_windows_error)?;
            SubmitThreadpoolWork(work);
        }
        Ok(())
    }
}

impl PlatformDispatcher for WindowsDispatcher {
    fn is_main_thread(&self) -> bool {
        thread::current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: Runnable, _: Option<TaskLabel>) {
        // should panic ?
        let _ = self.dispatch_on_threadpool(runnable);
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        self.main_sender.send(runnable).unwrap();
        if self.send_dispatch_message().is_err() {
            self.send_dispatch_message().expect("Error sending message");
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
        // println!("Dispatched timed task {:#?}", duration);
        unsafe {
            let mut handle = std::mem::zeroed();
            let ptr = Box::into_raw(Box::new(runnable));
            CreateTimerQueueTimer(
                &mut handle,
                self.timer_queue,
                Some(timer_runner),
                Some(ptr as _),
                duration.as_millis() as u32,
                0,
                WT_EXECUTEONLYONCE,
            )
            .expect("error create timer task");
        }
    }

    fn tick(&self, _background_only: bool) -> bool {
        false
    }

    fn park(&self) {
        self.parker.lock().park()
    }

    fn unparker(&self) -> Unparker {
        self.parker.lock().unparker()
    }
}

fn dispatcher_init() -> anyhow::Result<(PTP_POOL, HANDLE)> {
    let threadpool = unsafe {
        let mut threadpool_handle = CreateThreadpool(None);
        if threadpool_handle.0 == 0 {
            log::error!("Windows error: {}", std::io::Error::last_os_error());
            threadpool_handle = CreateThreadpool(None);
            if threadpool_handle.0 == 0 {
                log::error!("Windows error: {}", std::io::Error::last_os_error());
                return anyhow::Result::Err(anyhow::anyhow!("Error init dispatcher"));
            }
        }
        SetThreadpoolThreadMinimum(threadpool_handle, 1).inspect_err(log_windows_error)?;
        threadpool_handle
    };
    let timer_queue = unsafe { CreateTimerQueue().inspect_err(log_windows_error)? };
    Ok((threadpool, timer_queue))
}

extern "system" fn background_runner(
    _: PTP_CALLBACK_INSTANCE,
    ptr: *mut std::ffi::c_void,
    _: PTP_WORK,
) {
    unsafe {
        let runnable = Box::from_raw(ptr as *mut Runnable);
        panic::catch_unwind(|| runnable.run()).expect("error running runnable");
    }
}

unsafe extern "system" fn timer_runner(ptr: *mut std::ffi::c_void, _: BOOLEAN) {
    unsafe {
        let runnable = Box::from_raw(ptr as *mut Runnable);
        panic::catch_unwind(|| runnable.run()).expect("error running runnable");
    }
}
