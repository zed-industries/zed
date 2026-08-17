use core::mem;

use {Future, Poll, IntoFuture, Async};
use stream::Stream;

/// A future used to collect all the results of a stream into one generic type.
///
/// This future is returned by the `Stream::fold` method.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Fold<S, F, Fut, T> where Fut: IntoFuture {
    stream: S,
    f: F,
    state: State<T, Fut::Future>,
}

#[derive(Debug)]
enum State<T, F> where F: Future {
    /// Placeholder state when doing work
    Empty,

    /// Ready to process the next stream item; current accumulator is the `T`
    Ready(T),

    /// Working on a future the process the previous stream item
    Processing(F),
}

pub fn new<S, F, Fut, T>(s: S, f: F, t: T) -> Fold<S, F, Fut, T>
    where S: Stream,
          F: FnMut(T, S::Item) -> Fut,
          Fut: IntoFuture<Item = T>,
          S::Error: From<Fut::Error>,
{
    Fold {
        stream: s,
        f: f,
        state: State::Ready(t),
    }
}

impl<S, F, Fut, T> Future for Fold<S, F, Fut, T>
    where S: Stream,
          F: FnMut(T, S::Item) -> Fut,
          Fut: IntoFuture<Item = T>,
          S::Error: From<Fut::Error>,
{
    type Item = T;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<T, S::Error> {
        loop {
            match mem::replace(&mut self.state, State::Empty) {
                State::Empty => panic!("cannot poll Fold twice"),
                State::Ready(state) => {
                    match self.stream.poll()? {
                        Async::Ready(Some(e)) => {
                            let future = (self.f)(state, e);
                            let future = future.into_future();
                            self.state = State::Processing(future);
                        }
                        Async::Ready(None) => return Ok(Async::Ready(state)),
                        Async::NotReady => {
                            self.state = State::Ready(state);
                            return Ok(Async::NotReady)
                        }
                    }
                }
                State::Processing(mut fut) => {
                    match fut.poll()? {
                        Async::Ready(state) => self.state = State::Ready(state),
                        Async::NotReady => {
                            self.state = State::Processing(fut);
                            return Ok(Async::NotReady)
                        }
                    }
                }
            }
        }
    }
}
