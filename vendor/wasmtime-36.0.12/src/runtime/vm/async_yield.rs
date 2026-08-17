use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// A small future that yields once and then returns.
#[derive(Default)]
pub struct Yield {
    yielded: bool,
}

impl Yield {
    /// Create a new `Yield`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Future for Yield {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            // Flag ourselves as yielded to return next time, and also
            // flag the waker that we're already ready to get
            // re-enqueued for another poll.
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
