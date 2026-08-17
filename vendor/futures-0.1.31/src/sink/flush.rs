use {Poll, Async, Future};
use sink::Sink;

/// Future for the `Sink::flush` combinator, which polls the sink until all data
/// has been flushed.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct Flush<S> {
    sink: Option<S>,
}

pub fn new<S: Sink>(sink: S) -> Flush<S> {
    Flush { sink: Some(sink) }
}

impl<S: Sink> Flush<S> {
    /// Get a shared reference to the inner sink.
    pub fn get_ref(&self) -> &S {
        self.sink.as_ref().expect("Attempted `Flush::get_ref` after the flush completed")
    }

    /// Get a mutable reference to the inner sink.
    pub fn get_mut(&mut self) -> &mut S {
        self.sink.as_mut().expect("Attempted `Flush::get_mut` after the flush completed")
    }

    /// Consume the `Flush` and return the inner sink.
    pub fn into_inner(self) -> S {
        self.sink.expect("Attempted `Flush::into_inner` after the flush completed")
    }
}

impl<S: Sink> Future for Flush<S> {
    type Item = S;
    type Error = S::SinkError;

    fn poll(&mut self) -> Poll<S, S::SinkError> {
        let mut sink = self.sink.take().expect("Attempted to poll Flush after it completed");
        if sink.poll_complete()?.is_ready() {
            Ok(Async::Ready(sink))
        } else {
            self.sink = Some(sink);
            Ok(Async::NotReady)
        }
    }
}
