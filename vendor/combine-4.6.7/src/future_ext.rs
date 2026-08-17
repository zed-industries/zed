use crate::lib::future::Future;
use crate::lib::marker::Unpin;
use crate::lib::pin::Pin;
use crate::lib::task::{Context, Poll};

// Replace usage of this with std::future::poll_fn once it stabilizes
pub struct PollFn<F> {
    f: F,
}

impl<F> Unpin for PollFn<F> {}

pub fn poll_fn<T, F>(f: F) -> PollFn<F>
where
    F: FnMut(&mut Context<'_>) -> Poll<T>,
{
    PollFn { f }
}

impl<T, F> Future for PollFn<F>
where
    F: FnMut(&mut Context<'_>) -> Poll<T>,
{
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        (&mut self.f)(cx)
    }
}
