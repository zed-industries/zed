// Copyright 2024 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

// Derived from zbus.
// Copyright 2024 Zeeshan Ali Khan.
// Licensed under the MIT license (found in the LICENSE-MIT file).

#[cfg(not(feature = "tokio"))]
use async_executor::Executor as AsyncExecutor;
#[cfg(not(feature = "tokio"))]
use async_task::Task as AsyncTask;
#[cfg(feature = "tokio")]
use std::marker::PhantomData;
#[cfg(not(feature = "tokio"))]
use std::sync::Arc;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
#[cfg(feature = "tokio")]
use tokio::task::JoinHandle;

/// A wrapper around the underlying runtime/executor.
///
/// This is used to run asynchronous tasks internally and allows integration with various runtimes.
/// See [`crate::Connection::executor`] for an example of integration with external runtimes.
///
/// **Note:** You can (and should) completely ignore this type when building with `tokio` feature
/// enabled.
#[cfg(not(feature = "tokio"))]
#[derive(Debug, Clone)]
pub(crate) struct Executor<'a> {
    executor: Arc<AsyncExecutor<'a>>,
}
#[cfg(feature = "tokio")]
#[derive(Debug, Clone)]
pub(crate) struct Executor<'a> {
    phantom: PhantomData<&'a ()>,
}

impl Executor<'_> {
    /// Spawns a task onto the executor.
    pub(crate) fn spawn<T: Send + 'static>(
        &self,
        future: impl Future<Output = T> + Send + 'static,
        #[allow(unused)] name: &str,
    ) -> Task<T> {
        #[cfg(not(feature = "tokio"))]
        {
            Task(Some(self.executor.spawn(future)))
        }

        #[cfg(feature = "tokio")]
        {
            Task(Some(tokio::task::spawn(future)))
        }
    }

    /// Create a new `Executor`.
    pub(crate) fn new() -> Self {
        #[cfg(not(feature = "tokio"))]
        {
            Self {
                executor: Arc::new(AsyncExecutor::new()),
            }
        }

        #[cfg(feature = "tokio")]
        {
            Self {
                phantom: PhantomData,
            }
        }
    }

    /// Runs the executor until the given future completes.
    ///
    /// With `tokio` feature enabled, it just awaits on the `future`.
    pub(crate) async fn run<T>(&self, future: impl Future<Output = T>) -> T {
        #[cfg(not(feature = "tokio"))]
        {
            self.executor.run(future).await
        }
        #[cfg(feature = "tokio")]
        {
            future.await
        }
    }
}

/// A wrapper around the task API of the underlying runtime/executor.
///
/// This follows the semantics of `async_task::Task` on drop:
///
/// * it will be cancelled, rather than detached. For detaching, use the `detach` method.
/// * errors from the task cancellation will will be ignored. If you need to know about task errors,
///   convert the task to a `FallibleTask` using the `fallible` method.
#[cfg(not(feature = "tokio"))]
#[derive(Debug)]
pub(crate) struct Task<T>(Option<AsyncTask<T>>);
#[cfg(feature = "tokio")]
#[derive(Debug)]
pub(crate) struct Task<T>(Option<JoinHandle<T>>);

impl<T> Task<T> {
    /// Detaches the task to let it keep running in the background.
    #[allow(unused_mut)]
    #[allow(unused)]
    pub(crate) fn detach(mut self) {
        #[cfg(not(feature = "tokio"))]
        {
            self.0.take().expect("async_task::Task is none").detach()
        }

        #[cfg(feature = "tokio")]
        {
            self.0.take().expect("tokio::task::JoinHandle is none");
        }
    }
}

impl<T> Drop for Task<T> {
    fn drop(&mut self) {
        #[cfg(feature = "tokio")]
        {
            if let Some(join_handle) = self.0.take() {
                join_handle.abort();
            }
        }
    }
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        #[cfg(not(feature = "tokio"))]
        {
            Pin::new(&mut self.get_mut().0.as_mut().expect("async_task::Task is none")).poll(cx)
        }

        #[cfg(feature = "tokio")]
        {
            Pin::new(
                &mut self
                    .get_mut()
                    .0
                    .as_mut()
                    .expect("tokio::task::JoinHandle is none"),
            )
            .poll(cx)
            .map(|r| r.expect("tokio::task::JoinHandle error"))
        }
    }
}
