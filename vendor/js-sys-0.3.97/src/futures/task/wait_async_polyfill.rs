//! Polyfill for `Atomics.waitAsync` using a dedicated `Worker` per wait.
//! Workers are pooled and reused. The worker communicates the result back via
//! `postMessage`, which resolves a `Promise` on the calling thread.

#![allow(clippy::incompatible_msrv)]

use crate::{Array, Function, Promise};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::sync::atomic::AtomicI32;
use wasm_bindgen::prelude::*;

// Inline bindings for the web Worker and MessageEvent APIs.
// These replace the web-sys dependency, which would create a circular
// dependency since web-sys depends on js-sys.
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    type Worker;
    type MessageEvent;

    #[wasm_bindgen(constructor)]
    fn new_worker(url: &str) -> Worker;

    #[wasm_bindgen(method, js_name = postMessage, catch)]
    fn post_message(this: &Worker, msg: &JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, setter, js_name = onmessage)]
    fn set_onmessage(this: &Worker, cb: Option<&Function>);

    #[wasm_bindgen(method, getter)]
    fn data(this: &MessageEvent) -> JsValue;
}

#[thread_local]
static HELPERS: RefCell<Vec<Worker>> = RefCell::new(vec![]);

fn alloc_helper() -> Worker {
    if let Some(helper) = HELPERS.borrow_mut().pop() {
        return helper;
    }

    let worker_url = wasm_bindgen::link_to!(module = "/src/futures/task/worker.js");
    Worker::new_worker(&worker_url)
}

fn free_helper(helper: Worker) {
    let mut helpers = HELPERS.borrow_mut();
    helpers.push(helper.clone());
    helpers.truncate(10); // random arbitrary limit chosen here
}

pub fn wait_async(ptr: &AtomicI32, value: i32) -> Promise {
    Promise::new(&mut |resolve, _reject| {
        let helper = alloc_helper();
        let helper_ref = helper.clone();

        let onmessage_callback = Closure::once_into_js(move |e: MessageEvent| {
            // Our helper is done waiting so it's available to wait on a
            // different location, so return it to the free list.
            free_helper(helper_ref);
            drop(resolve.call1(&JsValue::NULL, &e.data()));
        });
        helper.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));

        let data = Array::of3(
            &wasm_bindgen::memory(),
            &JsValue::from(ptr.as_ptr() as u32 / 4),
            &JsValue::from(value),
        );

        helper
            .post_message(&data)
            .unwrap_or_else(|js| wasm_bindgen::throw_val(js));
    })
}
