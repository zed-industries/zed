use gpui::{
    PlatformDispatcher, Priority, PriorityQueueReceiver, PriorityQueueSender, RunnableVariant,
};
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::sync::atomic::AtomicI32;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_time::Instant;

#[cfg(feature = "multithreaded")]
const MIN_BACKGROUND_THREADS: usize = 2;

fn shared_memory_supported() -> bool {
    let global = js_sys::global();
    let has_shared_array_buffer =
        js_sys::Reflect::has(&global, &JsValue::from_str("SharedArrayBuffer")).unwrap_or(false);
    let has_atomics = js_sys::Reflect::has(&global, &JsValue::from_str("Atomics")).unwrap_or(false);
    let memory = js_sys::WebAssembly::Memory::from(wasm_bindgen::memory());
    let buffer = memory.buffer();
    let is_shared_buffer = buffer.is_instance_of::<js_sys::SharedArrayBuffer>();
    has_shared_array_buffer && has_atomics && is_shared_buffer
}

fn wait_async_supported() -> bool {
    let global = js_sys::global();
    let Ok(atomics) = js_sys::Reflect::get(&global, &JsValue::from_str("Atomics")) else {
        return false;
    };
    let Ok(wait_async) = js_sys::Reflect::get(&atomics, &JsValue::from_str("waitAsync")) else {
        return false;
    };

    wait_async.is_function()
}

enum MainThreadItem {
    Runnable(RunnableVariant),
    Delayed {
        runnable: RunnableVariant,
        millis: i32,
    },
    Idle {
        runnable: RunnableVariant,
        timeout: Option<Duration>,
    },
    Function(Box<dyn FnOnce() + Send>),
    // TODO-Wasm: Shouldn't these run on their own dedicated thread?
    RealtimeFunction(Box<dyn FnOnce() + Send>),
}

struct MainThreadMailbox {
    sender: PriorityQueueSender<MainThreadItem>,
    receiver: parking_lot::Mutex<PriorityQueueReceiver<MainThreadItem>>,
    signal: AtomicI32,
}

impl MainThreadMailbox {
    fn new() -> Self {
        let (sender, receiver) = PriorityQueueReceiver::new();
        Self {
            sender,
            receiver: parking_lot::Mutex::new(receiver),
            signal: AtomicI32::new(0),
        }
    }

    fn post(&self, priority: Priority, item: MainThreadItem) {
        if self.sender.spin_send(priority, item).is_err() {
            log::error!("MainThreadMailbox::send failed: receiver disconnected");
        }

        // TODO-Wasm: Verify this lock-free protocol
        let view = self.signal_view();
        js_sys::Atomics::store(&view, 0, 1).ok();
        js_sys::Atomics::notify(&view, 0).ok();
    }

    fn drain(&self, window: &web_sys::Window) {
        let mut receiver = self.receiver.lock();
        loop {
            // We need these `spin` variants because we can't acquire a lock on the main thread.
            // TODO-WASM: Should we do something different?
            match receiver.spin_try_pop() {
                Ok(Some(item)) => execute_on_main_thread(window, item),
                Ok(None) => break,
                Err(_) => break,
            }
        }
    }

    fn signal_view(&self) -> js_sys::Int32Array {
        let byte_offset = self.signal.as_ptr() as u32;
        let memory = js_sys::WebAssembly::Memory::from(wasm_bindgen::memory());
        js_sys::Int32Array::new_with_byte_offset_and_length(&memory.buffer(), byte_offset, 1)
    }

    fn run_waker_loop(self: &Arc<Self>, window: web_sys::Window) {
        if !shared_memory_supported() {
            log::warn!("SharedArrayBuffer not available; main thread mailbox waker loop disabled");
            return;
        }

        let mailbox = Arc::clone(self);
        wasm_bindgen_futures::spawn_local(async move {
            let view = mailbox.signal_view();
            loop {
                js_sys::Atomics::store(&view, 0, 0).expect("Atomics.store failed");

                // Items posted between the previous drain and the store above
                // set the signal we just cleared, so their notify is lost.
                // Drain again after re-arming to avoid missing them.
                mailbox.drain(&window);

                let result = match js_sys::Atomics::wait_async(&view, 0, 0) {
                    Ok(result) => result,
                    Err(error) => {
                        log::error!("Atomics.waitAsync failed: {error:?}");
                        break;
                    }
                };

                let is_async = js_sys::Reflect::get(&result, &JsValue::from_str("async"))
                    .ok()
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                // `async: false` means the signal changed between the store and
                // the wait ("not-equal"): work has already arrived, so skip
                // waiting and drain immediately.
                if is_async {
                    let promise: js_sys::Promise =
                        js_sys::Reflect::get(&result, &JsValue::from_str("value"))
                            .expect("waitAsync result missing 'value'")
                            .unchecked_into();

                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                }

                mailbox.drain(&window);
            }
        });
    }
}

pub struct WebDispatcher {
    main_thread_id: std::thread::ThreadId,
    background_sender: PriorityQueueSender<RunnableVariant>,
    main_thread_mailbox: Arc<MainThreadMailbox>,
    supports_threads: bool,
    #[cfg(feature = "multithreaded")]
    _background_threads: Vec<wasm_thread::JoinHandle<()>>,
}

impl WebDispatcher {
    pub fn new(browser_window: web_sys::Window, allow_threads: bool) -> Self {
        #[cfg(feature = "multithreaded")]
        let (background_sender, background_receiver) = PriorityQueueReceiver::new();
        #[cfg(not(feature = "multithreaded"))]
        let (background_sender, _) = PriorityQueueReceiver::new();

        let main_thread_mailbox = Arc::new(MainThreadMailbox::new());

        let supports_threads = cfg!(feature = "multithreaded")
            && allow_threads
            && shared_memory_supported()
            && wait_async_supported();

        if supports_threads {
            main_thread_mailbox.run_waker_loop(browser_window.clone());
        } else if cfg!(feature = "multithreaded") && allow_threads {
            log::warn!(
                "Required WebAssembly threading APIs are unavailable; falling back to single-threaded dispatcher"
            );
        }

        #[cfg(feature = "multithreaded")]
        let background_threads = if supports_threads {
            let thread_count = browser_window
                .navigator()
                .hardware_concurrency()
                .max(MIN_BACKGROUND_THREADS as f64) as usize;

            // TODO-Wasm: Is it bad to have web workers blocking for a long time like this?
            (0..thread_count)
                .map(|i| {
                    let mut receiver = background_receiver.clone();
                    wasm_thread::Builder::new()
                        .name(format!("background-worker-{i}"))
                        .spawn(move || {
                            loop {
                                let runnable: RunnableVariant = match receiver.pop() {
                                    Ok(runnable) => runnable,
                                    Err(_) => {
                                        log::info!(
                                            "background-worker-{i}: channel disconnected, exiting"
                                        );
                                        break;
                                    }
                                };

                                runnable.run();
                            }
                        })
                        .expect("failed to spawn background worker thread")
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        Self {
            main_thread_id: std::thread::current().id(),
            background_sender,
            main_thread_mailbox,
            supports_threads,
            #[cfg(feature = "multithreaded")]
            _background_threads: background_threads,
        }
    }

    fn on_main_thread(&self) -> bool {
        std::thread::current().id() == self.main_thread_id
    }

    pub(crate) fn dispatch_function_on_main_thread(
        &self,
        function: impl FnOnce() + Send + 'static,
    ) {
        if self.on_main_thread() {
            let callback = Closure::once_into_js(function);
            browser_window().queue_microtask(callback.unchecked_ref());
        } else {
            self.main_thread_mailbox
                .post(Priority::High, MainThreadItem::Function(Box::new(function)));
        }
    }
}

impl PlatformDispatcher for WebDispatcher {
    fn is_main_thread(&self) -> bool {
        self.on_main_thread()
    }

    fn dispatch(&self, runnable: RunnableVariant, priority: Priority) {
        if !self.supports_threads {
            self.dispatch_on_main_thread(runnable, priority);
            return;
        }

        let result = if self.on_main_thread() {
            self.background_sender.spin_send(priority, runnable)
        } else {
            self.background_sender.send(priority, runnable)
        };

        if let Err(error) = result {
            log::error!("dispatch: failed to send to background queue: {error:?}");
        }
    }

    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, priority: Priority) {
        if self.on_main_thread() {
            schedule_runnable(&browser_window(), runnable, priority);
        } else {
            self.main_thread_mailbox
                .post(priority, MainThreadItem::Runnable(runnable));
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant) {
        let millis = duration.as_millis().min(i32::MAX as u128) as i32;
        if self.on_main_thread() {
            let callback = Closure::once_into_js(move || {
                runnable.run();
            });
            browser_window()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.unchecked_ref(),
                    millis,
                )
                .ok();
        } else {
            self.main_thread_mailbox
                .post(Priority::High, MainThreadItem::Delayed { runnable, millis });
        }
    }

    fn spawn_realtime(&self, function: Box<dyn FnOnce() + Send>) {
        if self.on_main_thread() {
            let callback = Closure::once_into_js(move || {
                function();
            });
            browser_window().queue_microtask(callback.unchecked_ref());
        } else {
            self.main_thread_mailbox
                .post(Priority::High, MainThreadItem::RealtimeFunction(function));
        }
    }

    fn dispatch_on_main_thread_when_idle(
        &self,
        runnable: RunnableVariant,
        timeout: Option<Duration>,
    ) {
        if self.on_main_thread() {
            schedule_idle_runnable(&browser_window(), runnable, timeout);
        } else {
            self.main_thread_mailbox
                .post(Priority::Low, MainThreadItem::Idle { runnable, timeout });
        }
    }

    fn idle_time_remaining(&self) -> Option<Duration> {
        if !self.on_main_thread() {
            return None;
        }
        IDLE_DEADLINE.with(|deadline| {
            deadline
                .borrow()
                .as_ref()
                .map(|deadline| Duration::from_secs_f64(deadline.time_remaining() / 1000.0))
        })
    }

    fn now(&self) -> Instant {
        Instant::now()
    }
}

fn browser_window() -> web_sys::Window {
    web_sys::window().expect("must be running in a browser window context")
}

fn execute_on_main_thread(window: &web_sys::Window, item: MainThreadItem) {
    match item {
        MainThreadItem::Runnable(runnable) => {
            runnable.run();
        }
        MainThreadItem::Idle { runnable, timeout } => {
            schedule_idle_runnable(window, runnable, timeout);
        }
        MainThreadItem::Delayed { runnable, millis } => {
            let callback = Closure::once_into_js(move || {
                runnable.run();
            });
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.unchecked_ref(),
                    millis,
                )
                .ok();
        }
        MainThreadItem::Function(function) | MainThreadItem::RealtimeFunction(function) => {
            function();
        }
    }
}

thread_local! {
    /// The deadline of the idle callback currently running; read by
    /// [`PlatformDispatcher::idle_time_remaining`] from inside the runnable.
    static IDLE_DEADLINE: RefCell<Option<web_sys::IdleDeadline>> = const { RefCell::new(None) };
    /// Whether `requestIdleCallback` exists (it is absent on Safari), probed
    /// on first use.
    static IDLE_CALLBACK_SUPPORTED: Cell<Option<bool>> = const { Cell::new(None) };
}

/// Registers one `requestIdleCallback` per runnable, mirroring how
/// `dispatch_after` maps each timer to its own platform alarm. The browser
/// already provides the queue semantics a central pump would rebuild: idle
/// callbacks run in registration order, an idle period drains as many as its
/// deadline allows, and a callback whose `timeout` expires is posted as an
/// ordinary task instead.
fn schedule_idle_runnable(
    window: &web_sys::Window,
    runnable: RunnableVariant,
    timeout: Option<Duration>,
) {
    if !idle_callback_supported(window) {
        // Safari: run idle work as ordinary macrotasks. With no metered
        // deadline, `idle_time_remaining` stays `None` and idle tasks bound
        // their own slices.
        schedule_runnable(window, runnable, Priority::Low);
        return;
    }
    let callback = Closure::once_into_js(move |deadline: web_sys::IdleDeadline| {
        IDLE_DEADLINE.with(|current| *current.borrow_mut() = Some(deadline));
        runnable.run();
        IDLE_DEADLINE.with(|current| *current.borrow_mut() = None);
    });
    let result = match timeout {
        Some(timeout) => {
            let options = web_sys::IdleRequestOptions::new();
            options.set_timeout(timeout.as_millis().min(u32::MAX as u128) as u32);
            window.request_idle_callback_with_options(callback.unchecked_ref(), &options)
        }
        None => window.request_idle_callback(callback.unchecked_ref()),
    };
    if let Err(error) = result {
        log::error!("requestIdleCallback failed: {error:?}");
    }
}

fn idle_callback_supported(window: &web_sys::Window) -> bool {
    IDLE_CALLBACK_SUPPORTED.with(|supported| {
        if let Some(supported) = supported.get() {
            return supported;
        }
        let probed =
            js_sys::Reflect::has(window.as_ref(), &JsValue::from_str("requestIdleCallback"))
                .unwrap_or(false);
        supported.set(Some(probed));
        probed
    })
}

fn schedule_runnable(window: &web_sys::Window, runnable: RunnableVariant, priority: Priority) {
    let callback = Closure::once_into_js(move || {
        runnable.run();
    });
    let callback: &js_sys::Function = callback.unchecked_ref();

    match priority {
        Priority::RealtimeAudio => {
            window.queue_microtask(callback);
        }
        _ => {
            // TODO-Wasm: this ought to enqueue so we can dequeue with proper priority
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(callback, 0)
                .ok();
        }
    }
}
