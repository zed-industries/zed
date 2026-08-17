use std::pin::Pin;
use std::future::Future;

use crate::task::{Context, Poll};

/// Creates a new future wrapping around a function returning [`Poll`].
///
/// Polling the returned future delegates to the wrapped function.
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::future;
/// use async_std::task::{Context, Poll};
///
/// fn poll_greeting(_: &mut Context<'_>) -> Poll<String> {
///     Poll::Ready("hello world".to_string())
/// }
///
/// assert_eq!(future::poll_fn(poll_greeting).await, "hello world");
/// #
/// # })
/// ```
pub async fn poll_fn<F, T>(f: F) -> T
where
    F: FnMut(&mut Context<'_>) -> Poll<T>,
{
    let fut = PollFn { f };
    fut.await
}

struct PollFn<F> {
    f: F,
}

impl<F> Unpin for PollFn<F> {}

impl<T, F> Future for PollFn<F>
where
    F: FnMut(&mut Context<'_>) -> Poll<T>,
{
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        (&mut self.f)(cx)
    }
}
