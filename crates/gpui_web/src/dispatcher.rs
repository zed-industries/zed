use gpui::{PlatformDispatcher, Priority, RunnableVariant, ThreadTaskTimings};
use std::{
    thread::ThreadId,
    time::{Duration, Instant},
};

pub struct WebDispatcher {
    main_thread_id: ThreadId,
}

impl WebDispatcher {
    pub fn new() -> Self {
        Self {
            main_thread_id: std::thread::current().id(),
        }
    }
}

impl PlatformDispatcher for WebDispatcher {
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
        std::thread::current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: RunnableVariant, _priority: Priority) {
        // Stub: run inline. Real implementation will use setTimeout/microtasks.
        runnable.run();
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, _priority: Priority) {
        // Stub: run inline. In wasm there is only one thread anyway.
        runnable.run();
    }

    fn dispatch_after(&self, _duration: Duration, runnable: RunnableVariant) {
        log::warn!("WebDispatcher::dispatch_after: delay ignored in stub implementation");
        runnable.run();
    }

    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>) {
        // Stub: just call directly. No real-time thread spawning in wasm.
        f();
    }

    fn now(&self) -> Instant {
        Instant::now()
    }
}
