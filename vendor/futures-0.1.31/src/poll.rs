/// A macro for extracting the successful type of a `Poll<T, E>`.
///
/// This macro bakes propagation of both errors and `NotReady` signals by
/// returning early.
#[macro_export]
macro_rules! try_ready {
    ($e:expr) => (match $e {
        Ok($crate::Async::Ready(t)) => t,
        Ok($crate::Async::NotReady) => return Ok($crate::Async::NotReady),
        Err(e) => return Err(From::from(e)),
    })
}

/// Return type of the `Future::poll` method, indicates whether a future's value
/// is ready or not.
///
/// * `Ok(Async::Ready(t))` means that a future has successfully resolved
/// * `Ok(Async::NotReady)` means that a future is not ready to complete yet
/// * `Err(e)` means that a future has completed with the given failure
pub type Poll<T, E> = Result<Async<T>, E>;

/// Return type of future, indicating whether a value is ready or not.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Async<T> {
    /// Represents that a value is immediately ready.
    Ready(T),

    /// Represents that a value is not ready yet, but may be so later.
    NotReady,
}

impl<T> Async<T> {
    /// Change the success value of this `Async` with the closure provided
    pub fn map<F, U>(self, f: F) -> Async<U>
        where F: FnOnce(T) -> U
    {
        match self {
            Async::Ready(t) => Async::Ready(f(t)),
            Async::NotReady => Async::NotReady,
        }
    }

    /// Returns whether this is `Async::Ready`
    pub fn is_ready(&self) -> bool {
        match *self {
            Async::Ready(_) => true,
            Async::NotReady => false,
        }
    }

    /// Returns whether this is `Async::NotReady`
    pub fn is_not_ready(&self) -> bool {
        !self.is_ready()
    }
}

impl<T> From<T> for Async<T> {
    fn from(t: T) -> Async<T> {
        Async::Ready(t)
    }
}

/// The result of an asynchronous attempt to send a value to a sink.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AsyncSink<T> {
    /// The `start_send` attempt succeeded, so the sending process has
    /// *started*; you must use `Sink::poll_complete` to drive the send
    /// to completion.
    Ready,

    /// The `start_send` attempt failed due to the sink being full. The value
    /// being sent is returned, and the current `Task` will be automatically
    /// notified again once the sink has room.
    NotReady(T),
}

impl<T> AsyncSink<T> {
    /// Change the NotReady value of this `AsyncSink` with the closure provided
    pub fn map<F, U>(self, f: F) -> AsyncSink<U>
        where F: FnOnce(T) -> U,
    {
        match self {
            AsyncSink::Ready => AsyncSink::Ready,
            AsyncSink::NotReady(t) => AsyncSink::NotReady(f(t)),
        }
    }

    /// Returns whether this is `AsyncSink::Ready`
    pub fn is_ready(&self) -> bool {
        match *self {
            AsyncSink::Ready => true,
            AsyncSink::NotReady(_) => false,
        }
    }

    /// Returns whether this is `AsyncSink::NotReady`
    pub fn is_not_ready(&self) -> bool {
        !self.is_ready()
    }
}


/// Return type of the `Sink::start_send` method, indicating the outcome of a
/// send attempt. See `AsyncSink` for more details.
pub type StartSend<T, E> = Result<AsyncSink<T>, E>;
