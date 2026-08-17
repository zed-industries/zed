use {Future, Poll, Async};

/// A future which "fuses" a future once it's been resolved.
///
/// Normally futures can behave unpredictable once they're used after a future
/// has been resolved, but `Fuse` is always defined to return `Async::NotReady`
/// from `poll` after it has resolved successfully or returned an error.
///
/// This is created by the `Future::fuse` method.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct Fuse<A: Future> {
    future: Option<A>,
}

pub fn new<A: Future>(f: A) -> Fuse<A> {
    Fuse {
        future: Some(f),
    }
}

impl<A: Future> Fuse<A> {
    /// Returns whether the underlying future has finished or not.
    /// 
    /// If this method returns `true`, then all future calls to `poll`
    /// are guaranteed to return `Ok(Async::NotReady)`. If this returns
    /// false, then the underlying future has not been driven to
    /// completion.
    pub fn is_done(&self) -> bool {
        self.future.is_none()
    }
}

impl<A: Future> Future for Fuse<A> {
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<A::Item, A::Error> {
        let res = self.future.as_mut().map(|f| f.poll());
        match res.unwrap_or(Ok(Async::NotReady)) {
            res @ Ok(Async::Ready(_)) |
            res @ Err(_) => {
                self.future = None;
                res
            }
            Ok(Async::NotReady) => Ok(Async::NotReady)
        }
    }
}
