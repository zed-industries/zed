use {Stream, Poll};

/// Do something with the error of a stream, passing it on.
///
/// This is created by the `Stream::inspect_err` method.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct InspectErr<S, F> where S: Stream {
    stream: S,
    inspect: F,
}

pub fn new<S, F>(stream: S, f: F) -> InspectErr<S, F>
    where S: Stream,
          F: FnMut(&S::Error) -> (),
{
    InspectErr {
        stream: stream,
        inspect: f,
    }
}

impl<S: Stream, F> InspectErr<S, F> {
    /// Acquires a reference to the underlying stream that this combinator is
    /// pulling from.
    pub fn get_ref(&self) -> &S {
        &self.stream
    }

    /// Acquires a mutable reference to the underlying stream that this
    /// combinator is pulling from.
    ///
    /// Note that care must be taken to avoid tampering with the state of the
    /// stream which may otherwise confuse this combinator.
    pub fn get_mut(&mut self) -> &mut S {
        &mut self.stream
    }

    /// Consumes this combinator, returning the underlying stream.
    ///
    /// Note that this may discard intermediate state of this combinator, so
    /// care should be taken to avoid losing resources when this is called.
    pub fn into_inner(self) -> S {
        self.stream
    }
}

// Forwarding impl of Sink from the underlying stream
impl<S, F> ::sink::Sink for InspectErr<S, F>
    where S: ::sink::Sink + Stream
{
    type SinkItem = S::SinkItem;
    type SinkError = S::SinkError;

    fn start_send(&mut self, item: S::SinkItem) -> ::StartSend<S::SinkItem, S::SinkError> {
        self.stream.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), S::SinkError> {
        self.stream.poll_complete()
    }

    fn close(&mut self) -> Poll<(), S::SinkError> {
        self.stream.close()
    }
}

impl<S, F> Stream for InspectErr<S, F>
    where S: Stream,
          F: FnMut(&S::Error),
{
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<S::Item>, S::Error> {
        self.stream.poll().map_err(|e| {
            (self.inspect)(&e);
            e
        })
    }
}
