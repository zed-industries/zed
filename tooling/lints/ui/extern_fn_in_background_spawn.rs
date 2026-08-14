// Tests for the `extern_fn_in_background_spawn` lint.

#![allow(unused, async_block_without_await)]

extern crate gpui;

use gpui::*;

extern "C" fn foreign_callback() {}
fn rust_callback() {}

struct SendOnlyData;

struct WorkerData;
impl WorkerSend for WorkerData {}

struct WorkerWrapper<T>(T);
impl<T: WorkerSend> WorkerSend for WorkerWrapper<T> {}

trait WorkerObject: WorkerSend + Send {}
impl WorkerObject for WorkerData {}

// SHOULD WARN

fn foreign_function_pointer(cx: &mut App) {
    let callback: extern "C" fn() = foreign_callback;
    cx.background_spawn(async move {
        callback();
    });
}

fn rust_function_pointer(cx: &mut App) {
    let callback: fn() = rust_callback;
    cx.background_spawn(async move {
        callback();
    });
}

fn send_only_data(cx: &mut App) {
    let data = SendOnlyData;
    cx.background_spawn(async move {
        drop(data);
    });
}

fn nested_closure_with_send_only_data(cx: &mut App) {
    let data = SendOnlyData;
    let closure = move || drop(data);
    cx.background_spawn(async move {
        closure();
    });
}

fn nested_future_with_send_only_data(cx: &mut App) {
    let data = SendOnlyData;
    let future = async move {
        drop(data);
    };
    cx.background_spawn(async move {
        future.await;
    });
}

// SHOULD NOT WARN

fn worker_send_data(cx: &mut App) {
    let data = WorkerData;
    cx.background_spawn(async move {
        drop(data);
    });
}

fn generic_worker_send_data(cx: &mut App) {
    let data = WorkerWrapper(WorkerData);
    cx.background_spawn(async move {
        drop(data);
    });
}

fn dyn_worker_send_data(cx: &mut App) {
    let data: Box<dyn WorkerObject> = Box::new(WorkerData);
    cx.background_spawn(async move {
        drop(data);
    });
}

fn nested_closure_with_worker_send_data(cx: &mut App) {
    let data = WorkerData;
    let closure = move || drop(data);
    cx.background_spawn(async move {
        closure();
    });
}

fn nested_future_with_worker_send_data(cx: &mut App) {
    let data = WorkerData;
    let future = async move {
        drop(data);
    };
    cx.background_spawn(async move {
        future.await;
    });
}

async fn worker_future(data: WorkerData) {
    drop(data);
}

fn opaque_worker_future(cx: &mut App) {
    cx.background_spawn(worker_future(WorkerData));
}

#[cfg(not(target_arch = "wasm32"))]
fn wasm_excluded(cx: &mut App) {
    let callback: extern "C" fn() = foreign_callback;
    cx.background_spawn(async move {
        callback();
    });
}

struct OtherContext;

impl OtherContext {
    fn background_spawn(&self, _future: impl std::future::Future<Output = ()>) {}
}

fn unrelated_method_is_allowed(cx: &OtherContext) {
    let data = SendOnlyData;
    cx.background_spawn(async move {
        drop(data);
    });
}

fn main() {}
