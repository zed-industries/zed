use std::future::Future;
use std::pin::Pin;

use crate::task::{Context, Poll, Task};

/// A handle that awaits the result of a task.
///
/// Dropping a [`JoinHandle`] will detach the task, meaning that there is no longer
/// a handle to the task and no way to `join` on it.
///
/// Created when a task is [spawned].
///
/// [spawned]: fn.spawn.html
#[derive(Debug)]
pub struct JoinHandle<T> {
    handle: Option<InnerHandle<T>>,
    task: Task,
}

#[cfg(not(target_os = "unknown"))]
type InnerHandle<T> = async_global_executor::Task<T>;
#[cfg(target_arch = "wasm32")]
type InnerHandle<T> = futures_channel::oneshot::Receiver<T>;

impl<T> JoinHandle<T> {
    /// Creates a new `JoinHandle`.
    pub(crate) fn new(inner: InnerHandle<T>, task: Task) -> JoinHandle<T> {
        JoinHandle {
            handle: Some(inner),
            task,
        }
    }

    /// Returns a handle to the underlying task.
    ///
    /// # Examples
    ///
    /// ```
    /// # async_std::task::block_on(async {
    /// #
    /// use async_std::task;
    ///
    /// let handle = task::spawn(async {
    ///     1 + 2
    /// });
    /// println!("id = {}", handle.task().id());
    /// #
    /// # })
    pub fn task(&self) -> &Task {
        &self.task
    }

    /// Cancel this task.
    #[cfg(not(target_os = "unknown"))]
    pub async fn cancel(mut self) -> Option<T> {
        let handle = self.handle.take().unwrap();
        handle.cancel().await
    }

    /// Cancel this task.
    #[cfg(target_arch = "wasm32")]
    pub async fn cancel(mut self) -> Option<T> {
        let mut handle = self.handle.take().unwrap();
        handle.close();
        handle.await.ok()
    }
}

#[cfg(not(target_os = "unknown"))]
impl<T> Drop for JoinHandle<T> {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.detach();
        }
    }
}

impl<T> Future for JoinHandle<T> {
    type Output = T;

    #[cfg(not(target_os = "unknown"))]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.handle.as_mut().unwrap()).poll(cx)
    }

    #[cfg(target_arch = "wasm32")]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.handle.as_mut().unwrap()).poll(cx) {
            Poll::Ready(Ok(t)) => Poll::Ready(t),
            Poll::Ready(Err(_)) => unreachable!("channel must not be canceled"),
            Poll::Pending => Poll::Pending,
        }
    }
}
