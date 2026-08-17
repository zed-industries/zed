use core::mem;
use core::marker::PhantomData;

use {IntoFuture, Future, Poll, Async, StartSend, AsyncSink};
use sink::Sink;
use stream::Stream;

/// Sink for the `Sink::with` combinator, chaining a computation to run *prior*
/// to pushing a value into the underlying sink.
#[derive(Clone, Debug)]
#[must_use = "sinks do nothing unless polled"]
pub struct With<S, U, F, Fut>
    where S: Sink,
          F: FnMut(U) -> Fut,
          Fut: IntoFuture,
{
    sink: S,
    f: F,
    state: State<Fut::Future, S::SinkItem>,
    _phantom: PhantomData<fn(U)>,
}

#[derive(Clone, Debug)]
enum State<Fut, T> {
    Empty,
    Process(Fut),
    Buffered(T),
}

impl<Fut, T> State<Fut, T> {
    fn is_empty(&self) -> bool {
        if let State::Empty = *self {
            true
        } else {
            false
        }
    }
}

pub fn new<S, U, F, Fut>(sink: S, f: F) -> With<S, U, F, Fut>
    where S: Sink,
          F: FnMut(U) -> Fut,
          Fut: IntoFuture<Item = S::SinkItem>,
          Fut::Error: From<S::SinkError>,
{
    With {
        state: State::Empty,
        sink: sink,
        f: f,
        _phantom: PhantomData,
    }
}

// Forwarding impl of Stream from the underlying sink
impl<S, U, F, Fut> Stream for With<S, U, F, Fut>
    where S: Stream + Sink,
          F: FnMut(U) -> Fut,
          Fut: IntoFuture
{
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<S::Item>, S::Error> {
        self.sink.poll()
    }
}

impl<S, U, F, Fut> With<S, U, F, Fut>
    where S: Sink,
          F: FnMut(U) -> Fut,
          Fut: IntoFuture<Item = S::SinkItem>,
          Fut::Error: From<S::SinkError>,
{
    /// Get a shared reference to the inner sink.
    pub fn get_ref(&self) -> &S {
        &self.sink
    }

    /// Get a mutable reference to the inner sink.
    pub fn get_mut(&mut self) -> &mut S {
        &mut self.sink
    }

    /// Consumes this combinator, returning the underlying sink.
    ///
    /// Note that this may discard intermediate state of this combinator, so
    /// care should be taken to avoid losing resources when this is called.
    pub fn into_inner(self) -> S {
        self.sink
    }

    fn poll(&mut self) -> Poll<(), Fut::Error> {
        loop {
            match mem::replace(&mut self.state, State::Empty) {
                State::Empty => break,
                State::Process(mut fut) => {
                    match fut.poll()? {
                        Async::Ready(item) => {
                            self.state = State::Buffered(item);
                        }
                        Async::NotReady => {
                            self.state = State::Process(fut);
                            break
                        }
                    }
                }
                State::Buffered(item) => {
                    if let AsyncSink::NotReady(item) = self.sink.start_send(item)? {
                        self.state = State::Buffered(item);
                        break
                    }
                }
            }
        }

        if self.state.is_empty() {
            Ok(Async::Ready(()))
        } else {
            Ok(Async::NotReady)
        }
    }
}

impl<S, U, F, Fut> Sink for With<S, U, F, Fut>
    where S: Sink,
          F: FnMut(U) -> Fut,
          Fut: IntoFuture<Item = S::SinkItem>,
          Fut::Error: From<S::SinkError>,
{
    type SinkItem = U;
    type SinkError = Fut::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Fut::Error> {
        if self.poll()?.is_not_ready() {
            return Ok(AsyncSink::NotReady(item))
        }
        self.state = State::Process((self.f)(item).into_future());
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Fut::Error> {
        // poll ourselves first, to push data downward
        let me_ready = self.poll()?;
        // always propagate `poll_complete` downward to attempt to make progress
        try_ready!(self.sink.poll_complete());
        Ok(me_ready)
    }

    fn close(&mut self) -> Poll<(), Fut::Error> {
        try_ready!(self.poll());
        Ok(self.sink.close()?)
    }
}
