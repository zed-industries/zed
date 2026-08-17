//! Converting between JavaScript `Promise`s to Rust `Future`s.
//!
//! This module provides a bridge for working with JavaScript `Promise` types as
//! a Rust `Future`, and similarly contains utilities to turn a rust `Future`
//! into a JavaScript `Promise`. This can be useful when working with
//! asynchronous or otherwise blocking work in Rust (wasm), and provides the
//! ability to interoperate with JavaScript events and JavaScript I/O
//! primitives.
//!
//! There are three main interfaces in this module currently:
//!
//! 1. [**`JsFuture`**](./struct.JsFuture.html)
//!
//!    A type that is constructed with a `Promise` and can then be used as a
//!    `Future<Output = Result<JsValue, JsValue>>`. This Rust future will resolve
//!    or reject with the value coming out of the `Promise`.
//!
//! 2. [**`future_to_promise`**](./fn.future_to_promise.html)
//!
//!    Converts a Rust `Future<Output = Result<JsValue, JsValue>>` into a
//!    JavaScript `Promise`. The future's result will translate to either a
//!    resolved or rejected `Promise` in JavaScript.
//!
//! 3. [**`spawn_local`**](./fn.spawn_local.html)
//!
//!    Spawns a `Future<Output = ()>` on the current thread. This is the
//!    best way to run a `Future` in Rust without sending it to JavaScript.
//!
//! These three items should provide enough of a bridge to interoperate the two
//! systems and make sure that Rust/JavaScript can work together with
//! asynchronous and I/O work.

extern crate alloc;

use crate::Promise;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt;
use core::future::{Future, IntoFuture};
use core::panic::AssertUnwindSafe;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
#[cfg(all(target_family = "wasm", feature = "std", panic = "unwind"))]
use futures_util::FutureExt;
use wasm_bindgen::__rt::marker::ErasableGeneric;
#[cfg(all(target_family = "wasm", feature = "std", panic = "unwind"))]
use wasm_bindgen::__rt::panic_to_panic_error;
use wasm_bindgen::convert::{FromWasmAbi, Upcast};
use wasm_bindgen::sys::Promising;
use wasm_bindgen::{prelude::*, JsError, JsGeneric};

#[cfg_attr(docsrs, doc(cfg(feature = "futures-core-03-stream")))]
#[cfg(feature = "futures-core-03-stream")]
pub mod stream;

mod queue;

mod task {
    use cfg_if::cfg_if;

    cfg_if! {
        if #[cfg(target_feature = "atomics")] {
            mod wait_async_polyfill;
            mod multithread;
            pub(crate) use multithread::*;

        } else {
            mod singlethread;
            pub(crate) use singlethread::*;
         }
    }
}

/// Runs a Rust `Future` on the current thread.
///
/// The `future` must be `'static` because it will be scheduled
/// to run in the background and cannot contain any stack references.
///
/// The `future` will always be run on the next microtask tick even if it
/// immediately returns `Poll::Ready`.
///
/// # Panics
///
/// This function has the same panic behavior as `future_to_promise`.
#[inline]
pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    task::Task::spawn(future);
}

struct Inner<T = JsValue> {
    result: Option<Result<T, JsValue>>,
    task: Option<Waker>,
    callbacks: Option<(
        Closure<dyn FnMut(T) -> Result<(), JsError>>,
        Closure<dyn FnMut(JsValue) -> Result<(), JsError>>,
    )>,
}

/// A Rust `Future` backed by a JavaScript `Promise`.
///
/// This type is constructed with a JavaScript `Promise` object and translates
/// it to a Rust `Future`. This type implements the `Future` trait from the
/// `futures` crate and will either succeed or fail depending on what happens
/// with the JavaScript `Promise`.
///
/// Currently this type is constructed with `JsFuture::from`.
pub struct JsFuture<T = JsValue> {
    inner: Rc<RefCell<Inner<T>>>,
}

impl core::panic::UnwindSafe for JsFuture {}

unsafe impl<T> ErasableGeneric for JsFuture<T> {
    type Repr = JsFuture<JsValue>;
}

// Upcast for JsFuture is covariant in T (the success type)
// JsFuture<T> can upcast to JsFuture<Target> if T: Upcast<Target>
impl<T, Target> Upcast<JsFuture<Target>> for JsFuture<T> where T: Upcast<Target> {}

impl<T> fmt::Debug for JsFuture<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JsFuture {{ ... }}")
    }
}

impl<T: JsGeneric + FromWasmAbi> From<Promise<T>> for JsFuture<T> {
    fn from(js: Promise<T>) -> JsFuture<T> {
        // Use the `then` method to schedule two callbacks, one for the
        // resolved value and one for the rejected value. We're currently
        // assuming that JS engines will unconditionally invoke precisely one of
        // these callbacks, no matter what.
        //
        // Ideally we'd have a way to cancel the callbacks getting invoked and
        // free up state ourselves when this `JsFuture` is dropped. We don't
        // have that, though, and one of the callbacks is likely always going to
        // be invoked.
        //
        // As a result we need to make sure that no matter when the callbacks
        // are invoked they are valid to be called at any time, which means they
        // have to be self-contained. Through the `Closure::once` and some
        // `Rc`-trickery we can arrange for both instances of `Closure`, and the
        // `Rc`, to all be destroyed once the first one is called.
        let state = Rc::new(RefCell::new(Inner::<T> {
            result: None,
            task: None,
            callbacks: None,
        }));

        fn finish<T>(state: &RefCell<Inner<T>>, val: Result<T, JsValue>) {
            let task = {
                let mut state = state.borrow_mut();
                assert!(
                    state.callbacks.is_some(),
                    "finish: callbacks should be Some"
                );
                assert!(state.result.is_none(), "finish: result should be None");

                // First up drop our closures as they'll never be invoked again and
                // this is our chance to clean up their state.
                drop(state.callbacks.take());

                // Next, store the value into the internal state.
                state.result = Some(val);
                state.task.take()
            };

            // And then finally if any task was waiting on the value wake it up and
            // let them know it's there.
            if let Some(task) = task {
                task.wake()
            }
        }

        let resolve = {
            let state = AssertUnwindSafe(state.clone());
            Closure::once(move |val: T| {
                finish(&*state, Ok(val));
                Ok(())
            })
        };

        let reject = {
            let state = AssertUnwindSafe(state.clone());
            Closure::once(move |val| {
                finish(&*state, Err(val));
                Ok(())
            })
        };

        let _ = js.then_with_reject(&resolve, &reject);

        state.borrow_mut().callbacks = Some((resolve, reject));

        JsFuture { inner: state }
    }
}

impl<T> Future for JsFuture<T> {
    type Output = Result<T, JsValue>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut inner = self.inner.borrow_mut();

        // If our value has come in then we return it...
        if let Some(val) = inner.result.take() {
            return Poll::Ready(val);
        }

        // ... otherwise we arrange ourselves to get woken up once the value
        // does come in
        inner.task = Some(cx.waker().clone());
        Poll::Pending
    }
}

impl<T: JsGeneric + FromWasmAbi> IntoFuture for Promise<T> {
    type Output = Result<T, JsValue>;
    type IntoFuture = JsFuture<T>;

    fn into_future(self) -> JsFuture<T> {
        JsFuture::from(self)
    }
}

/// Converts a Rust `Future` into a JavaScript `Promise`.
///
/// This function will take any future in Rust and schedule it to be executed,
/// returning a JavaScript `Promise` which can then be passed to JavaScript.
///
/// The `future` must be `'static` because it will be scheduled to run in the
/// background and cannot contain any stack references.
///
/// The returned `Promise` will be resolved or rejected when the future
/// completes, depending on whether it finishes with `Ok` or `Err`.
///
/// # Panics
///
/// Note that in Wasm panics are currently translated to aborts, but "abort" in
/// this case means that a JavaScript exception is thrown. The Wasm module is
/// still usable (likely erroneously) after Rust panics.
#[cfg(not(all(target_family = "wasm", feature = "std", panic = "unwind")))]
pub fn future_to_promise<F>(future: F) -> Promise
where
    F: Future<Output = Result<JsValue, JsValue>> + 'static,
{
    let mut future = Some(future);

    Promise::new_typed(&mut move |resolve, reject| {
        let future = future.take().unwrap_throw();

        spawn_local(async move {
            match future.await {
                Ok(val) => {
                    resolve.call(&JsValue::UNDEFINED, (&val,)).unwrap_throw();
                }
                Err(val) => {
                    reject.call(&JsValue::UNDEFINED, (&val,)).unwrap_throw();
                }
            }
        });
    })
}

/// Converts a Rust `Future` into a JavaScript `Promise`.
///
/// This function will take any future in Rust and schedule it to be executed,
/// returning a JavaScript `Promise` which can then be passed to JavaScript.
///
/// The `future` must be `'static` because it will be scheduled to run in the
/// background and cannot contain any stack references.
///
/// The returned `Promise` will be resolved or rejected when the future
/// completes, depending on whether it finishes with `Ok` or `Err`.
///
/// # Panics
///
/// If the `future` provided panics then the returned `Promise` will be rejected
/// with a PanicError.
#[cfg(all(target_family = "wasm", feature = "std", panic = "unwind"))]
pub fn future_to_promise<F>(future: F) -> Promise
where
    F: Future<Output = Result<JsValue, JsValue>> + 'static + std::panic::UnwindSafe,
{
    // Wrap `future` in AssertUnwindSafe and move it into the closure so the closure
    // satisfies MaybeUnwindSafe (required when panic=unwind). Using `move` avoids
    // capturing a `&mut` reference, which is never UnwindSafe. The Promise executor
    // is not called inside a panic-catching context, so this is always safe.
    let mut future = core::panic::AssertUnwindSafe(Some(future));
    Promise::new(&mut move |resolve, reject| {
        let future = future.take().unwrap_throw();
        spawn_local(async move {
            let res = future.catch_unwind().await;
            match res {
                Ok(Ok(val)) => {
                    resolve.call(&JsValue::UNDEFINED, (&val,)).unwrap_throw();
                }
                Ok(Err(val)) => {
                    reject.call(&JsValue::UNDEFINED, (&val,)).unwrap_throw();
                }
                Err(val) => {
                    reject
                        .call(&JsValue::UNDEFINED, (&panic_to_panic_error(val),))
                        .unwrap_throw();
                }
            }
        });
    })
}

// Note: Once we bump MSRV, we can type future_to_promise with backwards compatible inference.
/// Converts a Rust `Future` into a corresponding typed JavaScript `Promise<T>`.
///
/// This function will take any future in Rust and schedule it to be executed,
/// returning a JavaScript `Promise` which can then be passed to JavaScript.
///
/// The `future` must be `'static` because it will be scheduled to run in the
/// background and cannot contain any stack references.
///
/// The returned `Promise` will be resolved or rejected when the future completes,
/// depending on whether it finishes with `Ok` or `Err`.
///
/// # Panics
///
/// Note that in Wasm panics are currently translated to aborts, but "abort" in
/// this case means that a JavaScript exception is thrown. The Wasm module is
/// still usable (likely erroneously) after Rust panics.
///
/// If the `future` provided panics then the returned `Promise` **will not
/// resolve**. Instead it will be a leaked promise. This is an unfortunate
/// limitation of Wasm currently that's hoped to be fixed one day!
pub fn future_to_promise_typed<T, F>(future: F) -> Promise<<T as Promising>::Resolution>
where
    F: Future<Output = Result<T, JsValue>> + 'static,
    T: Promising + FromWasmAbi + JsGeneric,
    <T as Promising>::Resolution: JsGeneric,
{
    let mut future = Some(future);

    Promise::new_typed(&mut move |resolve, reject| {
        let future = future.take().unwrap_throw();
        spawn_local(async move {
            match future.await {
                Ok(val) => {
                    resolve.call(&JsValue::UNDEFINED, (&val,)).unwrap_throw();
                }
                Err(val) => {
                    reject.call(&JsValue::UNDEFINED, (&val,)).unwrap_throw();
                }
            }
        });
    })
}
