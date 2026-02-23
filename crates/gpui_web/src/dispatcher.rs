use gpui::{PlatformDispatcher, Priority, RunnableVariant, ThreadTaskTimings};
use std::time::{Duration, Instant};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "queueMicrotask")]
    fn queue_microtask(callback: &JsValue);

    #[wasm_bindgen(js_name = "setTimeout")]
    fn set_timeout(callback: &JsValue, timeout: i32) -> i32;
}

fn schedule_runnable(runnable: RunnableVariant, priority: Priority) {
    let callback = Closure::once_into_js(move || {
        if !runnable.metadata().is_closed() {
            runnable.run();
        }
    });

    match priority {
        Priority::RealtimeAudio | Priority::High => {
            queue_microtask(&callback);
        }
        Priority::Medium | Priority::Low => {
            set_timeout(&callback, 0);
        }
    }
}

pub struct WebDispatcher {
    main_thread_id: std::thread::ThreadId,
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

    fn dispatch(&self, runnable: RunnableVariant, priority: Priority) {
        schedule_runnable(runnable, priority);
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, priority: Priority) {
        schedule_runnable(runnable, priority);
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let callback = Closure::once_into_js(move || {
            if !runnable.metadata().is_closed() {
                runnable.run();
            }
        });

        let millis = duration.as_millis().min(i32::MAX as u128) as i32;
        set_timeout(&callback, millis);
    }

    fn spawn_realtime(&self, function: Box<dyn FnOnce() + Send>) {
        let callback = Closure::once_into_js(move || {
            function();
        });
        queue_microtask(&callback);
    }

    fn now(&self) -> Instant {
        Instant::now()
    }
}
