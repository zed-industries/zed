//! A multi-producer, single-consumer, futures-aware, FIFO queue with back
//! pressure, for use communicating between tasks on the same thread.
//!
//! These queues are the same as those in `futures::sync`, except they're not
//! intended to be sent across threads.

use std::any::Any;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use std::mem;
use std::rc::{Rc, Weak};

use task::{self, Task};
use future::Executor;
use sink::SendAll;
use resultstream::{self, Results};
use unsync::oneshot;
use {Async, AsyncSink, Future, Poll, StartSend, Sink, Stream};

/// Creates a bounded in-memory channel with buffered storage.
///
/// This method creates concrete implementations of the `Stream` and `Sink`
/// traits which can be used to communicate a stream of values between tasks
/// with backpressure. The channel capacity is exactly `buffer`. On average,
/// sending a message through this channel performs no dynamic allocation.
pub fn channel<T>(buffer: usize) -> (Sender<T>, Receiver<T>) {
    channel_(Some(buffer))
}

fn channel_<T>(buffer: Option<usize>) -> (Sender<T>, Receiver<T>) {
    let shared = Rc::new(RefCell::new(Shared {
        buffer: VecDeque::new(),
        capacity: buffer,
        blocked_senders: VecDeque::new(),
        blocked_recv: None,
    }));
    let sender = Sender { shared: Rc::downgrade(&shared) };
    let receiver = Receiver { state: State::Open(shared) };
    (sender, receiver)
}

#[derive(Debug)]
struct Shared<T> {
    buffer: VecDeque<T>,
    capacity: Option<usize>,
    blocked_senders: VecDeque<Task>,
    blocked_recv: Option<Task>,
}

/// The transmission end of a channel.
///
/// This is created by the `channel` function.
#[derive(Debug)]
pub struct Sender<T> {
    shared: Weak<RefCell<Shared<T>>>,
}

impl<T> Sender<T> {
    fn do_send(&self, msg: T) -> StartSend<T, SendError<T>> {
        let shared = match self.shared.upgrade() {
            Some(shared) => shared,
            None => return Err(SendError(msg)), // receiver was dropped
        };
        let mut shared = shared.borrow_mut();

        match shared.capacity {
            Some(capacity) if shared.buffer.len() == capacity => {
                shared.blocked_senders.push_back(task::current());
                Ok(AsyncSink::NotReady(msg))
            }
            _ => {
                shared.buffer.push_back(msg);
                if let Some(task) = shared.blocked_recv.take() {
                    task.notify();
                }
                Ok(AsyncSink::Ready)
            }
        }
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender { shared: self.shared.clone() }
    }
}

impl<T> Sink for Sender<T> {
    type SinkItem = T;
    type SinkError = SendError<T>;

    fn start_send(&mut self, msg: T) -> StartSend<T, SendError<T>> {
        self.do_send(msg)
    }

    fn poll_complete(&mut self) -> Poll<(), SendError<T>> {
        Ok(Async::Ready(()))
    }

    fn close(&mut self) -> Poll<(), SendError<T>> {
        Ok(Async::Ready(()))
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let shared = match self.shared.upgrade() {
            Some(shared) => shared,
            None => return,
        };
        // The number of existing `Weak` indicates if we are possibly the last
        // `Sender`. If we are the last, we possibly must notify a blocked
        // `Receiver`. `self.shared` is always one of the `Weak` to this shared
        // data. Therefore the smallest possible Rc::weak_count(&shared) is 1.
        if Rc::weak_count(&shared) == 1 {
            if let Some(task) = shared.borrow_mut().blocked_recv.take() {
                // Wake up receiver as its stream has ended
                task.notify();
            }
        }
    }
}

/// The receiving end of a channel which implements the `Stream` trait.
///
/// This is created by the `channel` function.
#[derive(Debug)]
pub struct Receiver<T> {
    state: State<T>,
}

/// Possible states of a receiver. We're either Open (can receive more messages)
/// or we're closed with a list of messages we have left to receive.
#[derive(Debug)]
enum State<T> {
    Open(Rc<RefCell<Shared<T>>>),
    Closed(VecDeque<T>),
}

impl<T> Receiver<T> {
    /// Closes the receiving half
    ///
    /// This prevents any further messages from being sent on the channel while
    /// still enabling the receiver to drain messages that are buffered.
    pub fn close(&mut self) {
        let (blockers, items) = match self.state {
            State::Open(ref state) => {
                let mut state = state.borrow_mut();
                let items = mem::replace(&mut state.buffer, VecDeque::new());
                let blockers = mem::replace(&mut state.blocked_senders, VecDeque::new());
                (blockers, items)
            }
            State::Closed(_) => return,
        };
        self.state = State::Closed(items);
        for task in blockers {
            task.notify();
        }
    }
}

impl<T> Stream for Receiver<T> {
    type Item = T;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let me = match self.state {
            State::Open(ref mut me) => me,
            State::Closed(ref mut items) => {
                return Ok(Async::Ready(items.pop_front()))
            }
        };

        if let Some(shared) = Rc::get_mut(me) {
            // All senders have been dropped, so drain the buffer and end the
            // stream.
            return Ok(Async::Ready(shared.borrow_mut().buffer.pop_front()));
        }

        let mut shared = me.borrow_mut();
        if let Some(msg) = shared.buffer.pop_front() {
            if let Some(task) = shared.blocked_senders.pop_front() {
                drop(shared);
                task.notify();
            }
            Ok(Async::Ready(Some(msg)))
        } else {
            shared.blocked_recv = Some(task::current());
            Ok(Async::NotReady)
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.close();
    }
}

/// The transmission end of an unbounded channel.
///
/// This is created by the `unbounded` function.
#[derive(Debug)]
pub struct UnboundedSender<T>(Sender<T>);

impl<T> Clone for UnboundedSender<T> {
    fn clone(&self) -> Self {
        UnboundedSender(self.0.clone())
    }
}

impl<T> Sink for UnboundedSender<T> {
    type SinkItem = T;
    type SinkError = SendError<T>;

    fn start_send(&mut self, msg: T) -> StartSend<T, SendError<T>> {
        self.0.start_send(msg)
    }
    fn poll_complete(&mut self) -> Poll<(), SendError<T>> {
        Ok(Async::Ready(()))
    }
    fn close(&mut self) -> Poll<(), SendError<T>> {
        Ok(Async::Ready(()))
    }
}

impl<'a, T> Sink for &'a UnboundedSender<T> {
    type SinkItem = T;
    type SinkError = SendError<T>;

    fn start_send(&mut self, msg: T) -> StartSend<T, SendError<T>> {
        self.0.do_send(msg)
    }

    fn poll_complete(&mut self) -> Poll<(), SendError<T>> {
        Ok(Async::Ready(()))
    }

    fn close(&mut self) -> Poll<(), SendError<T>> {
        Ok(Async::Ready(()))
    }
}

impl<T> UnboundedSender<T> {
    /// Sends the provided message along this channel.
    ///
    /// This is an unbounded sender, so this function differs from `Sink::send`
    /// by ensuring the return type reflects that the channel is always ready to
    /// receive messages.
    #[deprecated(note = "renamed to `unbounded_send`")]
    #[doc(hidden)]
    pub fn send(&self, msg: T) -> Result<(), SendError<T>> {
        self.unbounded_send(msg)
    }

    /// Sends the provided message along this channel.
    ///
    /// This is an unbounded sender, so this function differs from `Sink::send`
    /// by ensuring the return type reflects that the channel is always ready to
    /// receive messages.
    pub fn unbounded_send(&self, msg: T) -> Result<(), SendError<T>> {
        let shared = match self.0.shared.upgrade() {
            Some(shared) => shared,
            None => return Err(SendError(msg)),
        };
        let mut shared = shared.borrow_mut();
        shared.buffer.push_back(msg);
        if let Some(task) = shared.blocked_recv.take() {
            drop(shared);
            task.notify();
        }
        Ok(())
    }
}

/// The receiving end of an unbounded channel.
///
/// This is created by the `unbounded` function.
#[derive(Debug)]
pub struct UnboundedReceiver<T>(Receiver<T>);

impl<T> UnboundedReceiver<T> {
    /// Closes the receiving half
    ///
    /// This prevents any further messages from being sent on the channel while
    /// still enabling the receiver to drain messages that are buffered.
    pub fn close(&mut self) {
        self.0.close();
    }
}

impl<T> Stream for UnboundedReceiver<T> {
    type Item = T;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.0.poll()
    }
}

/// Creates an unbounded in-memory channel with buffered storage.
///
/// Identical semantics to `channel`, except with no limit to buffer size.
pub fn unbounded<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) {
    let (send, recv) = channel_(None);
    (UnboundedSender(send), UnboundedReceiver(recv))
}

/// Error type for sending, used when the receiving end of a channel is
/// dropped
pub struct SendError<T>(T);

impl<T> fmt::Debug for SendError<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("SendError")
            .field(&"...")
            .finish()
    }
}

impl<T> fmt::Display for SendError<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "send failed because receiver is gone")
    }
}

impl<T: Any> Error for SendError<T> {
    fn description(&self) -> &str {
        "send failed because receiver is gone"
    }
}

impl<T> SendError<T> {
    /// Returns the message that was attempted to be sent but failed.
    pub fn into_inner(self) -> T {
        self.0
    }
}

/// Handle returned from the `spawn` function.
///
/// This handle is a stream that proxies a stream on a separate `Executor`.
/// Created through the `mpsc::spawn` function, this handle will produce
/// the same values as the proxied stream, as they are produced in the executor,
/// and uses a limited buffer to exert back-pressure on the remote stream.
///
/// If this handle is dropped, then the stream will no longer be polled and is
/// scheduled to be dropped.
pub struct SpawnHandle<Item, Error> {
    inner: Receiver<Result<Item, Error>>,
    _cancel_tx: oneshot::Sender<()>,
}

/// Type of future which `Executor` instances must be able to execute for `spawn`.
pub struct Execute<S: Stream> {
    inner: SendAll<Sender<Result<S::Item, S::Error>>, Results<S, SendError<Result<S::Item, S::Error>>>>,
    cancel_rx: oneshot::Receiver<()>,
}

/// Spawns a `stream` onto the instance of `Executor` provided, `executor`,
/// returning a handle representing the remote stream.
///
/// The `stream` will be canceled if the `SpawnHandle` is dropped.
///
/// The `SpawnHandle` returned is a stream that is a proxy for `stream` itself.
/// When `stream` has additional items available, then the `SpawnHandle`
/// will have those same items available.
///
/// At most `buffer + 1` elements will be buffered at a time. If the buffer
/// is full, then `stream` will stop progressing until more space is available.
/// This allows the `SpawnHandle` to exert backpressure on the `stream`.
///
/// # Panics
///
/// This function will panic if `executor` is unable spawn a `Future` containing
/// the entirety of the `stream`.
pub fn spawn<S, E>(stream: S, executor: &E, buffer: usize) -> SpawnHandle<S::Item, S::Error>
    where S: Stream,
          E: Executor<Execute<S>>
{
    let (cancel_tx, cancel_rx) = oneshot::channel();
    let (tx, rx) = channel(buffer);
    executor.execute(Execute {
        inner: tx.send_all(resultstream::new(stream)),
        cancel_rx: cancel_rx,
    }).expect("failed to spawn stream");
    SpawnHandle {
        inner: rx,
        _cancel_tx: cancel_tx,
    }
}

/// Spawns a `stream` onto the instance of `Executor` provided, `executor`,
/// returning a handle representing the remote stream, with unbounded buffering.
///
/// The `stream` will be canceled if the `SpawnHandle` is dropped.
///
/// The `SpawnHandle` returned is a stream that is a proxy for `stream` itself.
/// When `stream` has additional items available, then the `SpawnHandle`
/// will have those same items available.
///
/// An unbounded buffer is used, which means that values will be buffered as
/// fast as `stream` can produce them, without any backpressure. Therefore, if
/// `stream` is an infinite stream, it can use an unbounded amount of memory, and
/// potentially hog CPU resources. In particular, if `stream` is infinite
/// and doesn't ever yield (by returning `Async::NotReady` from `poll`), it
/// will result in an infinite loop.
///
/// # Panics
///
/// This function will panic if `executor` is unable spawn a `Future` containing
/// the entirety of the `stream`.
pub fn spawn_unbounded<S,E>(stream: S, executor: &E) -> SpawnHandle<S::Item, S::Error>
    where S: Stream,
          E: Executor<Execute<S>>
{
    let (cancel_tx, cancel_rx) = oneshot::channel();
    let (tx, rx) = channel_(None);
    executor.execute(Execute {
        inner: tx.send_all(resultstream::new(stream)),
        cancel_rx: cancel_rx,
    }).expect("failed to spawn stream");
    SpawnHandle {
        inner: rx,
        _cancel_tx: cancel_tx,
    }
}

impl<I, E> Stream for SpawnHandle<I, E> {
    type Item = I;
    type Error = E;

    fn poll(&mut self) -> Poll<Option<I>, E> {
        match self.inner.poll() {
            Ok(Async::Ready(Some(Ok(t)))) => Ok(Async::Ready(Some(t.into()))),
            Ok(Async::Ready(Some(Err(e)))) => Err(e),
            Ok(Async::Ready(None)) => Ok(Async::Ready(None)),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(_) => unreachable!("mpsc::Receiver should never return Err"),
        }
    }
}

impl<I, E> fmt::Debug for SpawnHandle<I, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SpawnHandle")
            .finish()
    }
}

impl<S: Stream> Future for Execute<S> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        match self.cancel_rx.poll() {
            Ok(Async::NotReady) => (),
            _ => return Ok(Async::Ready(())),
        }
        match self.inner.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            _ => Ok(Async::Ready(()))
        }
    }
}

impl<S: Stream> fmt::Debug for Execute<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Execute")
         .finish()
    }
}
