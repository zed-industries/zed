use async_task::Runnable;
use dispatch2::{DispatchQueue, DispatchQueueGlobalPriority, DispatchTime, GlobalQueueIdentifier};
use gpui::{PlatformDispatcher, Priority, RunnableMeta, RunnableVariant, ThreadTaskTimings};
use std::{ffi::c_void, ptr::NonNull, time::Duration};

pub(crate) struct IosDispatcher;

impl IosDispatcher {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformDispatcher for IosDispatcher {
    fn get_all_timings(&self) -> Vec<ThreadTaskTimings> {
        Vec::new()
    }

    fn get_current_thread_timings(&self) -> ThreadTaskTimings {
        ThreadTaskTimings {
            thread_name: None,
            thread_id: std::thread::current().id(),
            timings: Vec::new(),
            total_pushed: 0,
        }
    }

    fn is_main_thread(&self) -> bool {
        // GCD main queue check: dispatch_queue_get_label(DISPATCH_CURRENT_QUEUE_LABEL)
        // compares equal to dispatch_queue_get_label(dispatch_get_main_queue()).
        // Using the same approach as the macOS dispatcher via objc runtime.
        unsafe extern "C" {
            fn pthread_main_np() -> std::ffi::c_int;
        }
        unsafe { pthread_main_np() != 0 }
    }

    fn dispatch(&self, runnable: RunnableVariant, priority: Priority) {
        let context = runnable.into_raw().as_ptr() as *mut c_void;

        let queue_priority = match priority {
            Priority::RealtimeAudio => {
                panic!("RealtimeAudio priority should use spawn_realtime, not dispatch")
            }
            Priority::High => DispatchQueueGlobalPriority::High,
            Priority::Medium => DispatchQueueGlobalPriority::Default,
            Priority::Low => DispatchQueueGlobalPriority::Low,
        };

        unsafe {
            DispatchQueue::global_queue(GlobalQueueIdentifier::Priority(queue_priority))
                .exec_async_f(context, trampoline);
        }
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        let context = runnable.into_raw().as_ptr() as *mut c_void;
        unsafe {
            DispatchQueue::main().exec_async_f(context, trampoline);
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let context = runnable.into_raw().as_ptr() as *mut c_void;
        let queue = DispatchQueue::global_queue(GlobalQueueIdentifier::Priority(
            DispatchQueueGlobalPriority::High,
        ));
        let when = DispatchTime::NOW.time(duration.as_nanos() as i64);
        unsafe {
            DispatchQueue::exec_after_f(when, &queue, context, trampoline);
        }
    }

    fn spawn_realtime(&self, function: Box<dyn FnOnce() + Send>) {
        std::thread::spawn(move || {
            function();
        });
    }
}

extern "C" fn trampoline(context: *mut c_void) {
    let runnable =
        unsafe { Runnable::<RunnableMeta>::from_raw(NonNull::new_unchecked(context as *mut ())) };
    runnable.run();
}
