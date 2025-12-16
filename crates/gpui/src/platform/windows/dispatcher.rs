use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread::{ThreadId, current},
    time::{Duration, Instant},
};

use anyhow::Context;
use util::ResultExt;
use windows::{
    System::Threading::{
        ThreadPool, ThreadPoolTimer, TimerElapsedHandler, WorkItemHandler, WorkItemPriority,
    },
    Win32::{
        Foundation::{LPARAM, WPARAM},
        System::Threading::{
            GetCurrentThread, HIGH_PRIORITY_CLASS, SetPriorityClass, SetThreadPriority,
            THREAD_PRIORITY_HIGHEST, THREAD_PRIORITY_TIME_CRITICAL,
        },
        UI::WindowsAndMessaging::PostMessageW,
    },
};

use crate::{
    GLOBAL_THREAD_TIMINGS, HWND, PlatformDispatcher, Priority, PriorityQueueSender,
    RealtimePriority, RunnableVariant, SafeHwnd, THREAD_TIMINGS, TaskLabel, TaskTiming,
    ThreadTaskTimings, WM_GPUI_TASK_DISPATCHED_ON_MAIN_THREAD, profiler,
};

pub(crate) struct WindowsDispatcher {
    pub(crate) wake_posted: AtomicBool,
    main_sender: PriorityQueueSender<RunnableVariant>,
    main_thread_id: ThreadId,
    pub(crate) platform_window_handle: SafeHwnd,
    validation_number: usize,
}

impl WindowsDispatcher {
    pub(crate) fn new(
        main_sender: PriorityQueueSender<RunnableVariant>,
        platform_window_handle: HWND,
        validation_number: usize,
    ) -> Self {
        let main_thread_id = current().id();
        let platform_window_handle = platform_window_handle.into();

        WindowsDispatcher {
            main_sender,
            main_thread_id,
            platform_window_handle,
            validation_number,
            wake_posted: AtomicBool::new(false),
        }
    }

    fn dispatch_on_threadpool(&self, priority: WorkItemPriority, runnable: RunnableVariant) {
        let handler = {
            let mut task_wrapper = Some(runnable);
            WorkItemHandler::new(move |_| {
                Self::execute_runnable(task_wrapper.take().unwrap());
                Ok(())
            })
        };

        ThreadPool::RunWithPriorityAsync(&handler, priority).log_err();
    }

    fn dispatch_on_threadpool_after(&self, runnable: RunnableVariant, duration: Duration) {
        let handler = {
            let mut task_wrapper = Some(runnable);
            TimerElapsedHandler::new(move |_| {
                Self::execute_runnable(task_wrapper.take().unwrap());
                Ok(())
            })
        };
        ThreadPoolTimer::CreateTimer(&handler, duration.into()).log_err();
    }

    #[inline(always)]
    pub(crate) fn execute_runnable(runnable: RunnableVariant) {
        let start = Instant::now();

        let mut timing = match runnable {
            RunnableVariant::Meta(runnable) => {
                let location = runnable.metadata().location;
                let timing = TaskTiming {
                    location,
                    start,
                    end: None,
                };
                profiler::add_task_timing(timing);

                runnable.run();

                timing
            }
            RunnableVariant::Compat(runnable) => {
                let timing = TaskTiming {
                    location: core::panic::Location::caller(),
                    start,
                    end: None,
                };
                profiler::add_task_timing(timing);

                runnable.run();

                timing
            }
        };

        let end = Instant::now();
        timing.end = Some(end);

        profiler::add_task_timing(timing);
    }
}

impl PlatformDispatcher for WindowsDispatcher {
    fn get_all_timings(&self) -> Vec<ThreadTaskTimings> {
        let global_thread_timings = GLOBAL_THREAD_TIMINGS.lock();
        ThreadTaskTimings::convert(&global_thread_timings)
    }

    fn get_current_thread_timings(&self) -> Vec<crate::TaskTiming> {
        THREAD_TIMINGS.with(|timings| {
            let timings = timings.lock();
            let timings = &timings.timings;

            let mut vec = Vec::with_capacity(timings.len());

            let (s1, s2) = timings.as_slices();
            vec.extend_from_slice(s1);
            vec.extend_from_slice(s2);
            vec
        })
    }

    fn is_main_thread(&self) -> bool {
        current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: RunnableVariant, label: Option<TaskLabel>, priority: Priority) {
        let priority = match priority {
            Priority::Realtime(_) => unreachable!(),
            Priority::High => WorkItemPriority::High,
            Priority::Medium => WorkItemPriority::Normal,
            Priority::Low => WorkItemPriority::Low,
        };
        self.dispatch_on_threadpool(priority, runnable);

        if let Some(label) = label {
            log::debug!("TaskLabel: {label:?}");
        }
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, priority: Priority) {
        match self.main_sender.send(priority, runnable) {
            Ok(_) => {
                if !self.wake_posted.swap(true, Ordering::AcqRel) {
                    unsafe {
                        PostMessageW(
                            Some(self.platform_window_handle.as_raw()),
                            WM_GPUI_TASK_DISPATCHED_ON_MAIN_THREAD,
                            WPARAM(self.validation_number),
                            LPARAM(0),
                        )
                        .log_err();
                    }
                }
            }
            Err(runnable) => {
                // NOTE: Runnable may wrap a Future that is !Send.
                //
                // This is usually safe because we only poll it on the main thread.
                // However if the send fails, we know that:
                // 1. main_receiver has been dropped (which implies the app is shutting down)
                // 2. we are on a background thread.
                // It is not safe to drop something !Send on the wrong thread, and
                // the app will exit soon anyway, so we must forget the runnable.
                std::mem::forget(runnable);
            }
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        self.dispatch_on_threadpool_after(runnable, duration);
    }

    fn spawn_realtime(&self, priority: RealtimePriority, f: Box<dyn FnOnce() + Send>) {
        std::thread::spawn(move || {
            // SAFETY: always safe to call
            let thread_handle = unsafe { GetCurrentThread() };

            let thread_priority = match priority {
                RealtimePriority::Audio => THREAD_PRIORITY_TIME_CRITICAL,
                RealtimePriority::Other => THREAD_PRIORITY_HIGHEST,
            };

            // SAFETY: thread_handle is a valid handle to a thread
            unsafe { SetPriorityClass(thread_handle, HIGH_PRIORITY_CLASS) }
                .context("thread priority class")
                .log_err();

            // SAFETY: thread_handle is a valid handle to a thread
            unsafe { SetThreadPriority(thread_handle, thread_priority) }
                .context("thread priority")
                .log_err();

            f();
        });
    }
}
