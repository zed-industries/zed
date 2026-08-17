use {Async, Poll, IntoFuture, Future};
use stream::Stream;

/// A stream combinator which skips elements of a stream while a predicate
/// holds.
///
/// This structure is produced by the `Stream::skip_while` method.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct SkipWhile<S, P, R> where S: Stream, R: IntoFuture {
    stream: S,
    pred: P,
    pending: Option<(R::Future, S::Item)>,
    done_skipping: bool,
}

pub fn new<S, P, R>(s: S, p: P) -> SkipWhile<S, P, R>
    where S: Stream,
          P: FnMut(&S::Item) -> R,
          R: IntoFuture<Item=bool, Error=S::Error>,
{
    SkipWhile {
        stream: s,
        pred: p,
        pending: None,
        done_skipping: false,
    }
}

impl<S, P, R> SkipWhile<S, P, R> where S: Stream, R: IntoFuture {
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
impl<S, P, R> ::sink::Sink for SkipWhile<S, P, R>
    where S: ::sink::Sink + Stream, R: IntoFuture
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

impl<S, P, R> Stream for SkipWhile<S, P, R>
    where S: Stream,
          P: FnMut(&S::Item) -> R,
          R: IntoFuture<Item=bool, Error=S::Error>,
{
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<S::Item>, S::Error> {
        if self.done_skipping {
            return self.stream.poll();
        }

        loop {
            if self.pending.is_none() {
                let item = match try_ready!(self.stream.poll()) {
                    Some(e) => e,
                    None => return Ok(Async::Ready(None)),
                };
                self.pending = Some(((self.pred)(&item).into_future(), item));
            }

            assert!(self.pending.is_some());
            match self.pending.as_mut().unwrap().0.poll() {
                Ok(Async::Ready(true)) => self.pending = None,
                Ok(Async::Ready(false)) => {
                    let (_, item) = self.pending.take().unwrap();
                    self.done_skipping = true;
                    return Ok(Async::Ready(Some(item)))
                }
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(e) => {
                    self.pending = None;
                    return Err(e)
                }
            }
        }
    }
}
