//! Callback-style timer APIs.

use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "setTimeout", catch)]
    fn set_timeout(handler: &Function, timeout: i32) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = "setInterval", catch)]
    fn set_interval(handler: &Function, timeout: i32) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = "clearTimeout")]
    fn clear_timeout(handle: JsValue) -> JsValue;

    #[wasm_bindgen(js_name = "clearInterval")]
    fn clear_interval(handle: JsValue) -> JsValue;
}

/// A scheduled timeout.
///
/// See `Timeout::new` for scheduling new timeouts.
///
/// Once scheduled, you can [`drop`] the [`Timeout`] to clear it or [`forget`](Timeout::forget) to leak it. Once forgotten, the interval will keep running forever.
/// This pattern is known as Resource Acquisition Is Initialization (RAII).
#[derive(Debug)]
#[must_use = "timeouts cancel on drop; either call `forget` or `drop` explicitly"]
pub struct Timeout {
    id: Option<JsValue>,
    closure: Option<Closure<dyn FnMut()>>,
}

impl Drop for Timeout {
    /// Disposes of the timeout, dually cancelling this timeout by calling
    /// `clearTimeout` directly.
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            clear_timeout(id);
        }
    }
}

impl Timeout {
    /// Schedule a timeout to invoke `callback` in `millis` milliseconds from
    /// now.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_timers::callback::Timeout;
    ///
    /// let timeout = Timeout::new(1_000, move || {
    ///     // Do something...
    /// });
    /// ```
    pub fn new<F>(millis: u32, callback: F) -> Timeout
    where
        F: 'static + FnOnce(),
    {
        let closure = Closure::once(callback);

        let id = set_timeout(
            closure.as_ref().unchecked_ref::<js_sys::Function>(),
            millis as i32,
        )
        .unwrap_throw();

        Timeout {
            id: Some(id),
            closure: Some(closure),
        }
    }

    /// Forgets this resource without clearing the timeout.
    ///
    /// Returns the identifier returned by the original `setTimeout` call, and
    /// therefore you can still cancel the timeout by calling `clearTimeout`
    /// directly (perhaps via `web_sys::clear_timeout_with_handle`).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_timers::callback::Timeout;
    ///
    /// // We definitely want to do stuff, and aren't going to ever cancel this
    /// // timeout.
    /// Timeout::new(1_000, || {
    ///     // Do stuff...
    /// }).forget();
    /// ```
    pub fn forget(mut self) -> JsValue {
        let id = self.id.take().unwrap_throw();
        self.closure.take().unwrap_throw().forget();
        id
    }

    /// Cancel this timeout so that the callback is not invoked after the time
    /// is up.
    ///
    /// The scheduled callback is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_timers::callback::Timeout;
    ///
    /// let timeout = Timeout::new(1_000, || {
    ///     // Do stuff...
    /// });
    ///
    /// // If actually we didn't want to set a timer, then cancel it.
    /// if nevermind() {
    ///     timeout.cancel();
    /// }
    /// # fn nevermind() -> bool { true }
    /// ```
    pub fn cancel(mut self) -> Closure<dyn FnMut()> {
        self.closure.take().unwrap_throw()
    }
}

/// A scheduled interval.
///
/// See `Interval::new` for scheduling new intervals.
///
/// Once scheduled, you can [`drop`] the [`Interval`] to clear it or [`forget`](Interval::forget) to leak it. Once forgotten, the interval will keep running forever.
/// This pattern is known as Resource Acquisition Is Initialization (RAII).
#[derive(Debug)]
#[must_use = "intervals cancel on drop; either call `forget` or `drop` explicitly"]
pub struct Interval {
    id: Option<JsValue>,
    closure: Option<Closure<dyn FnMut()>>,
}

impl Drop for Interval {
    /// Disposes of the interval, dually cancelling this interval by calling
    /// `clearInterval` directly.
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            clear_interval(id);
        }
    }
}

impl Interval {
    /// Schedule an interval to invoke `callback` every `millis` milliseconds.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_timers::callback::Interval;
    ///
    /// let interval = Interval::new(1_000, move || {
    ///     // Do something...
    /// });
    /// ```
    pub fn new<F>(millis: u32, callback: F) -> Interval
    where
        F: 'static + FnMut(),
    {
        let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);

        let id = set_interval(
            closure.as_ref().unchecked_ref::<js_sys::Function>(),
            millis as i32,
        )
        .unwrap_throw();

        Interval {
            id: Some(id),
            closure: Some(closure),
        }
    }

    /// Forget this resource without clearing the interval.
    ///
    /// Returns the identifier returned by the original `setInterval` call, and
    /// therefore you can still cancel the interval by calling `clearInterval`
    /// directly (perhaps via `web_sys::clear_interval_with_handle`).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_timers::callback::Interval;
    ///
    /// // We want to do stuff every second, indefinitely.
    /// Interval::new(1_000, || {
    ///     // Do stuff...
    /// }).forget();
    /// ```
    pub fn forget(mut self) -> JsValue {
        let id = self.id.take().unwrap_throw();
        self.closure.take().unwrap_throw().forget();
        id
    }

    /// Cancel this interval so that the callback is no longer periodically
    /// invoked.
    ///
    /// The scheduled callback is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_timers::callback::Interval;
    ///
    /// let interval = Interval::new(1_000, || {
    ///     // Do stuff...
    /// });
    ///
    /// // If we don't want this interval to run anymore, then cancel it.
    /// if nevermind() {
    ///     interval.cancel();
    /// }
    /// # fn nevermind() -> bool { true }
    /// ```
    pub fn cancel(mut self) -> Closure<dyn FnMut()> {
        self.closure.take().unwrap_throw()
    }
}
