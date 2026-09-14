//! iOS task dispatcher using Grand Central Dispatch (GCD).
//!
//! iOS shares the same GCD infrastructure as macOS, so this implementation
//! is nearly identical to the macOS dispatcher.

use dispatch2::{DispatchQueue, DispatchQueueGlobalPriority, DispatchTime, GlobalQueueIdentifier};
use gpui::{PlatformDispatcher, Priority, RunnableVariant};
use std::thread;

use objc2_foundation::NSThread;
use std::{ffi::c_void, ptr::NonNull, time::Duration};

fn priority_to_gcd(priority: Priority) -> GlobalQueueIdentifier {
    GlobalQueueIdentifier::Priority(match priority {
        Priority::High => DispatchQueueGlobalPriority::High,
        Priority::Low => DispatchQueueGlobalPriority::Low,
        _ => DispatchQueueGlobalPriority::Default,
    })
}

pub(crate) struct IosDispatcher;

impl PlatformDispatcher for IosDispatcher {
    fn is_main_thread(&self) -> bool {
        NSThread::isMainThread_class()
    }

    fn dispatch(&self, runnable: RunnableVariant, priority: Priority) {
        let context = runnable.into_raw().as_ptr() as *mut c_void;
        unsafe {
            DispatchQueue::exec_async_f(
                &DispatchQueue::global_queue(priority_to_gcd(priority)),
                context,
                trampoline,
            );
        }
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        let context = runnable.into_raw().as_ptr() as *mut c_void;
        unsafe {
            DispatchQueue::exec_async_f(DispatchQueue::main(), context, trampoline);
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let context = runnable.into_raw().as_ptr() as *mut c_void;
        unsafe {
            let queue = DispatchQueue::global_queue(priority_to_gcd(Priority::High));
            let when = DispatchTime::try_from(duration).unwrap_or(DispatchTime::FOREVER);
            DispatchQueue::exec_after_f(when, &queue, context, trampoline);
        }
    }

    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>) {
        // On iOS, we don't have direct realtime thread control like macOS.
        // Use a high-priority GCD queue as an approximation.
        thread::Builder::new()
            .name("gpui-ios-realtime".into())
            .spawn(move || {
                f();
            })
            .ok();
    }
}

extern "C" fn trampoline(runnable: *mut c_void) {
    // Every submission transfers exactly one RunnableVariant to GCD. Foreground
    // runnables use only the main queue; reconstructing them elsewhere is unsound.
    let task = unsafe { RunnableVariant::from_raw(NonNull::new_unchecked(runnable as *mut ())) };
    task.run();
}
