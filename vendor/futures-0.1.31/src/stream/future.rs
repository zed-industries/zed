use {Future, Poll, Async};
use stream::Stream;

/// A combinator used to temporarily convert a stream into a future.
///
/// This future is returned by the `Stream::into_future` method.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct StreamFuture<S> {
    stream: Option<S>,
}

pub fn new<S: Stream>(s: S) -> StreamFuture<S> {
    StreamFuture { stream: Some(s) }
}

impl<S> StreamFuture<S> {
    /// Acquires a reference to the underlying stream that this combinator is
    /// pulling from.
    ///
    /// This method returns an `Option` to account for the fact that `StreamFuture`'s
    /// implementation of `Future::poll` consumes the underlying stream during polling 
    /// in order to return it to the caller of `Future::poll` if the stream yielded
    /// an element.
    pub fn get_ref(&self) -> Option<&S> {
        self.stream.as_ref()
    }

    /// Acquires a mutable reference to the underlying stream that this
    /// combinator is pulling from.
    ///
    /// Note that care must be taken to avoid tampering with the state of the
    /// stream which may otherwise confuse this combinator.
    ///
    /// This method returns an `Option` to account for the fact that `StreamFuture`'s
    /// implementation of `Future::poll` consumes the underlying stream during polling 
    /// in order to return it to the caller of `Future::poll` if the stream yielded
    /// an element.
    pub fn get_mut(&mut self) -> Option<&mut S> {
        self.stream.as_mut()
    }

    /// Consumes this combinator, returning the underlying stream.
    ///
    /// Note that this may discard intermediate state of this combinator, so
    /// care should be taken to avoid losing resources when this is called.
    ///
    /// This method returns an `Option` to account for the fact that `StreamFuture`'s
    /// implementation of `Future::poll` consumes the underlying stream during polling 
    /// in order to return it to the caller of `Future::poll` if the stream yielded
    /// an element.
    pub fn into_inner(self) -> Option<S> {
        self.stream
    }
}

impl<S: Stream> Future for StreamFuture<S> {
    type Item = (Option<S::Item>, S);
    type Error = (S::Error, S);

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let item = {
            let s = self.stream.as_mut().expect("polling StreamFuture twice");
            match s.poll() {
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Ok(Async::Ready(e)) => Ok(e),
                Err(e) => Err(e),
            }
        };
        let stream = self.stream.take().unwrap();
        match item {
            Ok(e) => Ok(Async::Ready((e, stream))),
            Err(e) => Err((e, stream)),
        }
    }
}
