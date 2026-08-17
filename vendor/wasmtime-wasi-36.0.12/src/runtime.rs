//! This module provides an "ambient Tokio runtime"
//! [`with_ambient_tokio_runtime`]. Embedders of wasmtime-wasi may do so from
//! synchronous Rust, and not use tokio directly. The implementation of
//! wasmtime-wasi requires a tokio executor in a way that is [deeply tied to
//! its
//! design](https://github.com/bytecodealliance/wasmtime/issues/7973#issuecomment-1960513214).
//! When used from a synchronous wasmtime context, this module provides the
//! wrapper function [`in_tokio`] used throughout the shim implementations of
//! synchronous component binding `Host` traits in terms of the async ones.
//!
//! This module also provides a thin wrapper on tokio's tasks.
//! [`AbortOnDropJoinHandle`], which is exactly like a
//! [`tokio::task::JoinHandle`] except for the obvious behavioral change. This
//! whole crate, and any child crates which spawn tasks as part of their
//! implementations, should please use this crate's [`spawn`] and
//! [`spawn_blocking`] over tokio's. so we wanted the type name to stick out
//! if someone misses it.
//!
//! Each of these facilities should be used by dependencies of wasmtime-wasi
//! which when implementing component bindings.

use std::future::Future;
use std::pin::Pin;
use std::sync::LazyLock;
use std::task::{Context, Poll, Waker};

pub(crate) static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap()
});

/// Exactly like a [`tokio::task::JoinHandle`], except that it aborts the task when
/// the handle is dropped.
///
/// This behavior makes it easier to tie a worker task to the lifetime of a Resource
/// by keeping this handle owned by the Resource.
#[derive(Debug)]
pub struct AbortOnDropJoinHandle<T>(tokio::task::JoinHandle<T>);
impl<T> AbortOnDropJoinHandle<T> {
    /// Abort the task and wait for it to finish. Optionally returns the result
    /// of the task if it ran to completion prior to being aborted.
    pub async fn cancel(mut self) -> Option<T> {
        self.0.abort();

        match (&mut self.0).await {
            Ok(value) => Some(value),
            Err(err) if err.is_cancelled() => None,
            Err(err) => std::panic::resume_unwind(err.into_panic()),
        }
    }
}
impl<T> Drop for AbortOnDropJoinHandle<T> {
    fn drop(&mut self) {
        self.0.abort()
    }
}
impl<T> std::ops::Deref for AbortOnDropJoinHandle<T> {
    type Target = tokio::task::JoinHandle<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> std::ops::DerefMut for AbortOnDropJoinHandle<T> {
    fn deref_mut(&mut self) -> &mut tokio::task::JoinHandle<T> {
        &mut self.0
    }
}
impl<T> From<tokio::task::JoinHandle<T>> for AbortOnDropJoinHandle<T> {
    fn from(jh: tokio::task::JoinHandle<T>) -> Self {
        AbortOnDropJoinHandle(jh)
    }
}
impl<T> Future for AbortOnDropJoinHandle<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.as_mut().0).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(r) => Poll::Ready(r.expect("child task panicked")),
        }
    }
}

pub fn spawn<F>(f: F) -> AbortOnDropJoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let j = with_ambient_tokio_runtime(|| tokio::task::spawn(f));
    AbortOnDropJoinHandle(j)
}

pub fn spawn_blocking<F, R>(f: F) -> AbortOnDropJoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let j = with_ambient_tokio_runtime(|| tokio::task::spawn_blocking(f));
    AbortOnDropJoinHandle(j)
}

pub fn in_tokio<F: Future>(f: F) -> F::Output {
    match tokio::runtime::Handle::try_current() {
        Ok(h) => {
            let _enter = h.enter();
            h.block_on(f)
        }
        // The `yield_now` here is non-obvious and if you're reading this
        // you're likely curious about why it's here. This is currently required
        // to get some features of "sync mode" working correctly, such as with
        // the CLI. To illustrate why this is required, consider a program
        // organized as:
        //
        // * A program has a `pollable` that it's waiting on.
        // * This `pollable` is always ready .
        // * Actually making the corresponding operation ready, however,
        //   requires some background work on Tokio's part.
        // * The program is looping on "wait for readiness" coupled with
        //   performing the operation.
        //
        // In this situation this program ends up infinitely looping in waiting
        // for pollables. The reason appears to be that when we enter the tokio
        // runtime here it doesn't necessary yield to background work because
        // the provided future `f` is ready immediately. The future `f` will run
        // through the list of pollables and determine one of them is ready.
        //
        // Historically this happened with UDP sockets. A test send a datagram
        // from one socket to another and the other socket infinitely didn't
        // receive the data. This appeared to be because the server socket was
        // waiting on `READABLE | WRITABLE` (which is itself a bug but ignore
        // that) and the socket was currently in the "writable" state but never
        // ended up receiving a notification for the "readable" state. Moving
        // the socket to "readable" would require Tokio to perform some
        // background work via epoll/kqueue/handle events but if the future
        // provided here is always ready, then that never happened.
        //
        // Thus the `yield_now()` is an attempt to force Tokio to go do some
        // background work eventually and look at new interest masks for
        // example. This is a bit of a kludge but everything's already a bit
        // wonky in synchronous mode anyway. Note that this is hypothesized to
        // not be an issue in async mode because async mode typically has the
        // Tokio runtime in a separate thread or otherwise participating in a
        // larger application, it's only here in synchronous mode where we
        // effectively own the runtime that we need some special care.
        Err(_) => {
            let _enter = RUNTIME.enter();
            RUNTIME.block_on(async move {
                tokio::task::yield_now().await;
                f.await
            })
        }
    }
}

/// Executes the closure `f` with an "ambient Tokio runtime" which basically
/// means that if code in `f` tries to get a runtime `Handle` it'll succeed.
///
/// If a `Handle` is already available, e.g. in async contexts, then `f` is run
/// immediately. Otherwise for synchronous contexts this crate's fallback
/// runtime is configured and then `f` is executed.
pub fn with_ambient_tokio_runtime<R>(f: impl FnOnce() -> R) -> R {
    match tokio::runtime::Handle::try_current() {
        Ok(_) => f(),
        Err(_) => {
            let _enter = RUNTIME.enter();
            f()
        }
    }
}

/// Attempts to get the result of a `future`.
///
/// This function does not block and will poll the provided future once. If the
/// result is here then `Some` is returned, otherwise `None` is returned.
///
/// Note that by polling `future` this means that `future` must be re-polled
/// later if it's to wake up a task.
pub fn poll_noop<F>(future: Pin<&mut F>) -> Option<F::Output>
where
    F: Future,
{
    let mut task = Context::from_waker(Waker::noop());
    match future.poll(&mut task) {
        Poll::Ready(result) => Some(result),
        Poll::Pending => None,
    }
}
