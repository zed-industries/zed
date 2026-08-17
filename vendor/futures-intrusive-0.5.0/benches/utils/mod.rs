use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// A Future which yields to the executor for a given amount of iterations
/// and resolves after this
pub struct Yield {
    iter: usize,
}

impl Yield {
    pub fn new(iter: usize) -> Yield {
        Yield { iter }
    }
}

impl Future for Yield {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.iter == 0 {
            Poll::Ready(())
        } else {
            self.iter -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
