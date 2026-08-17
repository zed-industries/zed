//! Definition of the `LoopFn` combinator, implementing `Future` loops.

use {Async, Future, IntoFuture, Poll};

/// The status of a `loop_fn` loop.
#[derive(Debug)]
pub enum Loop<T, S> {
    /// Indicates that the loop has completed with output `T`.
    Break(T),

    /// Indicates that the loop function should be called again with input
    /// state `S`.
    Continue(S),
}

/// A future implementing a tail-recursive loop.
///
/// Created by the `loop_fn` function.
#[derive(Debug)]
pub struct LoopFn<A, F> where A: IntoFuture {
    future: A::Future,
    func: F,
}

/// Creates a new future implementing a tail-recursive loop.
///
/// The loop function is immediately called with `initial_state` and should
/// return a value that can be converted to a future. On successful completion,
/// this future should output a `Loop<T, S>` to indicate the status of the
/// loop.
///
/// `Loop::Break(T)` halts the loop and completes the future with output `T`.
///
/// `Loop::Continue(S)` reinvokes the loop function with state `S`. The returned
/// future will be subsequently polled for a new `Loop<T, S>` value.
///
/// # Examples
///
/// ```
/// use futures::future::{ok, loop_fn, Future, FutureResult, Loop};
/// use std::io::Error;
///
/// struct Client {
///     ping_count: u8,
/// }
///
/// impl Client {
///     fn new() -> Self {
///         Client { ping_count: 0 }
///     }
///
///     fn send_ping(self) -> FutureResult<Self, Error> {
///         ok(Client { ping_count: self.ping_count + 1 })
///     }
///
///     fn receive_pong(self) -> FutureResult<(Self, bool), Error> {
///         let done = self.ping_count >= 5;
///         ok((self, done))
///     }
/// }
///
/// let ping_til_done = loop_fn(Client::new(), |client| {
///     client.send_ping()
///         .and_then(|client| client.receive_pong())
///         .and_then(|(client, done)| {
///             if done {
///                 Ok(Loop::Break(client))
///             } else {
///                 Ok(Loop::Continue(client))
///             }
///         })
/// });
/// ```
pub fn loop_fn<S, T, A, F>(initial_state: S, mut func: F) -> LoopFn<A, F>
    where F: FnMut(S) -> A,
          A: IntoFuture<Item = Loop<T, S>>,
{
    LoopFn {
        future: func(initial_state).into_future(),
        func: func,
    }
}

impl<S, T, A, F> Future for LoopFn<A, F>
    where F: FnMut(S) -> A,
          A: IntoFuture<Item = Loop<T, S>>,
{
    type Item = T;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match try_ready!(self.future.poll()) {
                Loop::Break(x) => return Ok(Async::Ready(x)),
                Loop::Continue(s) => self.future = (self.func)(s).into_future(),
            }
        }
    }
}
