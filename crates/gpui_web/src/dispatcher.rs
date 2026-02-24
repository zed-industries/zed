use gpui::{PlatformDispatcher, Priority, RunnableVariant, ThreadTaskTimings};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_time::Instant;

pub struct WebDispatcher {
    main_thread_id: std::thread::ThreadId,
    browser_window: web_sys::Window,
}

// Safety: WASM is single-threaded — there is no concurrent access to `web_sys::Window`.
// TODO-Wasm: This won't be true soon.
unsafe impl Send for WebDispatcher {}
unsafe impl Sync for WebDispatcher {}

impl WebDispatcher {
    pub fn new(browser_window: web_sys::Window) -> Self {
        Self {
            main_thread_id: std::thread::current().id(),
            browser_window,
        }
    }

    fn schedule_runnable(&self, runnable: RunnableVariant, priority: Priority) {
        let callback = Closure::once_into_js(move || {
            if !runnable.metadata().is_closed() {
                runnable.run();
            }
        });
        let callback: &js_sys::Function = callback.unchecked_ref();

        match priority {
            Priority::RealtimeAudio | Priority::High => {
                self.browser_window.queue_microtask(callback);
            }
            Priority::Medium | Priority::Low => {
                self.browser_window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(callback, 0)
                    .ok();
            }
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

    fn dispatch(&self, runnable: RunnableVariant, priority: Priority) {
        self.schedule_runnable(runnable, priority);
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, priority: Priority) {
        self.schedule_runnable(runnable, priority);
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let callback = Closure::once_into_js(move || {
            if !runnable.metadata().is_closed() {
                runnable.run();
            }
        });

        let millis = duration.as_millis().min(i32::MAX as u128) as i32;
        self.browser_window
            .set_timeout_with_callback_and_timeout_and_arguments_0(callback.unchecked_ref(), millis)
            .ok();
    }

    fn spawn_realtime(&self, function: Box<dyn FnOnce() + Send>) {
        let callback = Closure::once_into_js(move || {
            function();
        });
        self.browser_window
            .queue_microtask(callback.unchecked_ref());
    }

    fn now(&self) -> Instant {
        Instant::now()
    }
}
