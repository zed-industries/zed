//! A multi-producer, single-consumer, futures-aware, FIFO queue with back pressure.
//!
//! A channel can be used as a communication primitive between tasks running on
//! `futures-rs` executors. Channel creation provides `Receiver` and `Sender`
//! handles. `Receiver` implements `Stream` and allows a task to read values
//! out of the channel. If there is no message to read from the channel, the
//! current task will be notified when a new value is sent. `Sender` implements
//! the `Sink` trait and allows a task to send messages into the channel. If
//! the channel is at capacity, then send will be rejected and the task will be
//! notified when additional capacity is available.
//!
//! # Disconnection
//!
//! When all `Sender` handles have been dropped, it is no longer possible to
//! send values into the channel. This is considered the termination event of
//! the stream. As such, `Sender::poll` will return `Ok(Ready(None))`.
//!
//! If the receiver handle is dropped, then messages can no longer be read out
//! of the channel. In this case, a `send` will result in an error.
//!
//! # Clean Shutdown
//!
//! If the `Receiver` is simply dropped, then it is possible for there to be
//! messages still in the channel that will not be processed. As such, it is
//! usually desirable to perform a "clean" shutdown. To do this, the receiver
//! will first call `close`, which will prevent any further messages to be sent
//! into the channel. Then, the receiver consumes the channel to completion, at
//! which point the receiver can be dropped.

// At the core, the channel uses an atomic FIFO queue for message passing. This
// queue is used as the primary coordination primitive. In order to enforce
// capacity limits and handle back pressure, a secondary FIFO queue is used to
// send parked task handles.
//
// The general idea is that the channel is created with a `buffer` size of `n`.
// The channel capacity is `n + num-senders`. Each sender gets one "guaranteed"
// slot to hold a message. This allows `Sender` to know for a fact that a send
// will succeed *before* starting to do the actual work of sending the value.
// Since most of this work is lock-free, once the work starts, it is impossible
// to safely revert.
//
// If the sender is unable to process a send operation, then the current
// task is parked and the handle is sent on the parked task queue.
//
// Note that the implementation guarantees that the channel capacity will never
// exceed the configured limit, however there is no *strict* guarantee that the
// receiver will wake up a parked task *immediately* when a slot becomes
// available. However, it will almost always unpark a task when a slot becomes
// available and it is *guaranteed* that a sender will be unparked when the
// message that caused the sender to become parked is read out of the channel.
//
// The steps for sending a message are roughly:
//
// 1) Increment the channel message count
// 2) If the channel is at capacity, push the task handle onto the wait queue
// 3) Push the message onto the message queue.
//
// The steps for receiving a message are roughly:
//
// 1) Pop a message from the message queue
// 2) Pop a task handle from the wait queue
// 3) Decrement the channel message count.
//
// It's important for the order of operations on lock-free structures to happen
// in reverse order between the sender and receiver. This makes the message
// queue the primary coordination structure and establishes the necessary
// happens-before semantics required for the acquire / release semantics used
// by the queue structure.

use std::fmt;
use std::error::Error;
use std::any::Any;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, Mutex};
use std::thread;
use std::usize;

use sync::mpsc::queue::{Queue, PopResult};
use sync::oneshot;
use task::{self, Task};
use future::Executor;
use sink::SendAll;
use resultstream::{self, Results};
use {Async, AsyncSink, Future, Poll, StartSend, Sink, Stream};

mod queue;

/// The transmission end of a channel which is used to send values.
///
/// This is created by the `channel` method.
#[derive(Debug)]
pub struct Sender<T> {
    // Channel state shared between the sender and receiver.
    inner: Arc<Inner<T>>,

    // Handle to the task that is blocked on this sender. This handle is sent
    // to the receiver half in order to be notified when the sender becomes
    // unblocked.
    sender_task: Arc<Mutex<SenderTask>>,

    // True if the sender might be blocked. This is an optimization to avoid
    // having to lock the mutex most of the time.
    maybe_parked: bool,
}

/// The transmission end of a channel which is used to send values.
///
/// This is created by the `unbounded` method.
#[derive(Debug)]
pub struct UnboundedSender<T>(Sender<T>);

trait AssertKinds: Send + Sync + Clone {}
impl AssertKinds for UnboundedSender<u32> {}


/// The receiving end of a channel which implements the `Stream` trait.
///
/// This is a concrete implementation of a stream which can be used to represent
/// a stream of values being computed elsewhere. This is created by the
/// `channel` method.
#[derive(Debug)]
pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

/// The receiving end of a channel which implements the `Stream` trait.
///
/// This is a concrete implementation of a stream which can be used to represent
/// a stream of values being computed elsewhere. This is created by the
/// `unbounded` method.
#[derive(Debug)]
pub struct UnboundedReceiver<T>(Receiver<T>);

/// Error type for sending, used when the receiving end of a channel is
/// dropped
#[derive(Clone, PartialEq, Eq)]
pub struct SendError<T>(T);

/// Error type returned from `try_send`
#[derive(Clone, PartialEq, Eq)]
pub struct TrySendError<T> {
    kind: TrySendErrorKind<T>,
}

#[derive(Clone, PartialEq, Eq)]
enum TrySendErrorKind<T> {
    Full(T),
    Disconnected(T),
}

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

impl<T: Any> Error for SendError<T>
{
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

impl<T> fmt::Debug for TrySendError<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("TrySendError")
            .field(&"...")
            .finish()
    }
}

impl<T> fmt::Display for TrySendError<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.is_full() {
            write!(fmt, "send failed because channel is full")
        } else {
            write!(fmt, "send failed because receiver is gone")
        }
    }
}

impl<T: Any> Error for TrySendError<T> {
    fn description(&self) -> &str {
        if self.is_full() {
            "send failed because channel is full"
        } else {
            "send failed because receiver is gone"
        }
    }
}

impl<T> TrySendError<T> {
    /// Returns true if this error is a result of the channel being full
    pub fn is_full(&self) -> bool {
        use self::TrySendErrorKind::*;

        match self.kind {
            Full(_) => true,
            _ => false,
        }
    }

    /// Returns true if this error is a result of the receiver being dropped
    pub fn is_disconnected(&self) -> bool {
        use self::TrySendErrorKind::*;

        match self.kind {
            Disconnected(_) => true,
            _ => false,
        }
    }

    /// Returns the message that was attempted to be sent but failed.
    pub fn into_inner(self) -> T {
        use self::TrySendErrorKind::*;

        match self.kind {
            Full(v) | Disconnected(v) => v,
        }
    }
}

#[derive(Debug)]
struct Inner<T> {
    // Max buffer size of the channel. If `None` then the channel is unbounded.
    buffer: Option<usize>,

    // Internal channel state. Consists of the number of messages stored in the
    // channel as well as a flag signalling that the channel is closed.
    state: AtomicUsize,

    // Atomic, FIFO queue used to send messages to the receiver
    message_queue: Queue<Option<T>>,

    // Atomic, FIFO queue used to send parked task handles to the receiver.
    parked_queue: Queue<Arc<Mutex<SenderTask>>>,

    // Number of senders in existence
    num_senders: AtomicUsize,

    // Handle to the receiver's task.
    recv_task: Mutex<ReceiverTask>,
}

// Struct representation of `Inner::state`.
#[derive(Debug, Clone, Copy)]
struct State {
    // `true` when the channel is open
    is_open: bool,

    // Number of messages in the channel
    num_messages: usize,
}

#[derive(Debug)]
struct ReceiverTask {
    unparked: bool,
    task: Option<Task>,
}

// Returned from Receiver::try_park()
enum TryPark {
    Parked,
    Closed,
    NotEmpty,
}

// The `is_open` flag is stored in the left-most bit of `Inner::state`
const OPEN_MASK: usize = usize::MAX - (usize::MAX >> 1);

// When a new channel is created, it is created in the open state with no
// pending messages.
const INIT_STATE: usize = OPEN_MASK;

// The maximum number of messages that a channel can track is `usize::MAX >> 1`
const MAX_CAPACITY: usize = !(OPEN_MASK);

// The maximum requested buffer size must be less than the maximum capacity of
// a channel. This is because each sender gets a guaranteed slot.
const MAX_BUFFER: usize = MAX_CAPACITY >> 1;

// Sent to the consumer to wake up blocked producers
#[derive(Debug)]
struct SenderTask {
    task: Option<Task>,
    is_parked: bool,
}

impl SenderTask {
    fn new() -> Self {
        SenderTask {
            task: None,
            is_parked: false,
        }
    }

    fn notify(&mut self) {
        self.is_parked = false;

        if let Some(task) = self.task.take() {
            task.notify();
        }
    }
}

/// Creates an in-memory channel implementation of the `Stream` trait with
/// bounded capacity.
///
/// This method creates a concrete implementation of the `Stream` trait which
/// can be used to send values across threads in a streaming fashion. This
/// channel is unique in that it implements back pressure to ensure that the
/// sender never outpaces the receiver. The channel capacity is equal to
/// `buffer + num-senders`. In other words, each sender gets a guaranteed slot
/// in the channel capacity, and on top of that there are `buffer` "first come,
/// first serve" slots available to all senders.
///
/// The `Receiver` returned implements the `Stream` trait and has access to any
/// number of the associated combinators for transforming the result.
pub fn channel<T>(buffer: usize) -> (Sender<T>, Receiver<T>) {
    // Check that the requested buffer size does not exceed the maximum buffer
    // size permitted by the system.
    assert!(buffer < MAX_BUFFER, "requested buffer size too large");
    channel2(Some(buffer))
}

/// Creates an in-memory channel implementation of the `Stream` trait with
/// unbounded capacity.
///
/// This method creates a concrete implementation of the `Stream` trait which
/// can be used to send values across threads in a streaming fashion. A `send`
/// on this channel will always succeed as long as the receive half has not
/// been closed. If the receiver falls behind, messages will be buffered
/// internally.
///
/// **Note** that the amount of available system memory is an implicit bound to
/// the channel. Using an `unbounded` channel has the ability of causing the
/// process to run out of memory. In this case, the process will be aborted.
pub fn unbounded<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) {
    let (tx, rx) = channel2(None);
    (UnboundedSender(tx), UnboundedReceiver(rx))
}

fn channel2<T>(buffer: Option<usize>) -> (Sender<T>, Receiver<T>) {
    let inner = Arc::new(Inner {
        buffer: buffer,
        state: AtomicUsize::new(INIT_STATE),
        message_queue: Queue::new(),
        parked_queue: Queue::new(),
        num_senders: AtomicUsize::new(1),
        recv_task: Mutex::new(ReceiverTask {
            unparked: false,
            task: None,
        }),
    });

    let tx = Sender {
        inner: inner.clone(),
        sender_task: Arc::new(Mutex::new(SenderTask::new())),
        maybe_parked: false,
    };

    let rx = Receiver {
        inner: inner,
    };

    (tx, rx)
}

/*
 *
 * ===== impl Sender =====
 *
 */

impl<T> Sender<T> {
    /// Attempts to send a message on this `Sender<T>` without blocking.
    ///
    /// This function, unlike `start_send`, is safe to call whether it's being
    /// called on a task or not. Note that this function, however, will *not*
    /// attempt to block the current task if the message cannot be sent.
    ///
    /// It is not recommended to call this function from inside of a future,
    /// only from an external thread where you've otherwise arranged to be
    /// notified when the channel is no longer full.
    pub fn try_send(&mut self, msg: T) -> Result<(), TrySendError<T>> {
        // If the sender is currently blocked, reject the message
        if !self.poll_unparked(false).is_ready() {
            return Err(TrySendError {
                kind: TrySendErrorKind::Full(msg),
            });
        }

        // The channel has capacity to accept the message, so send it
        self.do_send(Some(msg), false)
            .map_err(|SendError(v)| {
                TrySendError {
                    kind: TrySendErrorKind::Disconnected(v),
                }
            })
    }

    // Do the send without failing
    // None means close
    fn do_send(&mut self, msg: Option<T>, do_park: bool) -> Result<(), SendError<T>> {
        // First, increment the number of messages contained by the channel.
        // This operation will also atomically determine if the sender task
        // should be parked.
        //
        // None is returned in the case that the channel has been closed by the
        // receiver. This happens when `Receiver::close` is called or the
        // receiver is dropped.
        let park_self = match self.inc_num_messages(msg.is_none()) {
            Some(park_self) => park_self,
            None => {
                // The receiver has closed the channel. Only abort if actually
                // sending a message. It is important that the stream
                // termination (None) is always sent. This technically means
                // that it is possible for the queue to contain the following
                // number of messages:
                //
                //     num-senders + buffer + 1
                //
                if let Some(msg) = msg {
                    return Err(SendError(msg));
                } else {
                    return Ok(());
                }
            }
        };

        // If the channel has reached capacity, then the sender task needs to
        // be parked. This will send the task handle on the parked task queue.
        //
        // However, when `do_send` is called while dropping the `Sender`,
        // `task::current()` can't be called safely. In this case, in order to
        // maintain internal consistency, a blank message is pushed onto the
        // parked task queue.
        if park_self {
            self.park(do_park);
        }

        self.queue_push_and_signal(msg);

        Ok(())
    }

    // Do the send without parking current task.
    //
    // To be called from unbounded sender.
    fn do_send_nb(&self, msg: T) -> Result<(), SendError<T>> {
        match self.inc_num_messages(false) {
            Some(park_self) => assert!(!park_self),
            None => return Err(SendError(msg)),
        };

        self.queue_push_and_signal(Some(msg));

        Ok(())
    }

    // Push message to the queue and signal to the receiver
    fn queue_push_and_signal(&self, msg: Option<T>) {
        // Push the message onto the message queue
        self.inner.message_queue.push(msg);

        // Signal to the receiver that a message has been enqueued. If the
        // receiver is parked, this will unpark the task.
        self.signal();
    }

    // Increment the number of queued messages. Returns if the sender should
    // block.
    fn inc_num_messages(&self, close: bool) -> Option<bool> {
        let mut curr = self.inner.state.load(SeqCst);

        loop {
            let mut state = decode_state(curr);

            // The receiver end closed the channel.
            if !state.is_open {
                return None;
            }

            // This probably is never hit? Odds are the process will run out of
            // memory first. It may be worth to return something else in this
            // case?
            assert!(state.num_messages < MAX_CAPACITY, "buffer space exhausted; \
                    sending this messages would overflow the state");

            state.num_messages += 1;

            // The channel is closed by all sender handles being dropped.
            if close {
                state.is_open = false;
            }

            let next = encode_state(&state);
            match self.inner.state.compare_exchange(curr, next, SeqCst, SeqCst) {
                Ok(_) => {
                    // Block if the current number of pending messages has exceeded
                    // the configured buffer size
                    let park_self = match self.inner.buffer {
                        Some(buffer) => state.num_messages > buffer,
                        None => false,
                    };

                    return Some(park_self)
                }
                Err(actual) => curr = actual,
            }
        }
    }

    // Signal to the receiver task that a message has been enqueued
    fn signal(&self) {
        // TODO
        // This logic can probably be improved by guarding the lock with an
        // atomic.
        //
        // Do this step first so that the lock is dropped when
        // `unpark` is called
        let task = {
            let mut recv_task = self.inner.recv_task.lock().unwrap();

            // If the receiver has already been unparked, then there is nothing
            // more to do
            if recv_task.unparked {
                return;
            }

            // Setting this flag enables the receiving end to detect that
            // an unpark event happened in order to avoid unnecessarily
            // parking.
            recv_task.unparked = true;
            recv_task.task.take()
        };

        if let Some(task) = task {
            task.notify();
        }
    }

    fn park(&mut self, can_park: bool) {
        // TODO: clean up internal state if the task::current will fail

        let task = if can_park {
            Some(task::current())
        } else {
            None
        };

        {
            let mut sender = self.sender_task.lock().unwrap();
            sender.task = task;
            sender.is_parked = true;
        }

        // Send handle over queue
        let t = self.sender_task.clone();
        self.inner.parked_queue.push(t);

        // Check to make sure we weren't closed after we sent our task on the
        // queue
        let state = decode_state(self.inner.state.load(SeqCst));
        self.maybe_parked = state.is_open;
    }

    /// Polls the channel to determine if there is guaranteed to be capacity to send at least one
    /// item without waiting.
    ///
    /// Returns `Ok(Async::Ready(_))` if there is sufficient capacity, or returns
    /// `Ok(Async::NotReady)` if the channel is not guaranteed to have capacity. Returns
    /// `Err(SendError(_))` if the receiver has been dropped.
    ///
    /// # Panics
    ///
    /// This method will panic if called from outside the context of a task or future.
    pub fn poll_ready(&mut self) -> Poll<(), SendError<()>> {
        let state = decode_state(self.inner.state.load(SeqCst));
        if !state.is_open {
            return Err(SendError(()));
        }

        Ok(self.poll_unparked(true))
    }

    /// Returns whether this channel is closed without needing a context.
    pub fn is_closed(&self) -> bool {
        !decode_state(self.inner.state.load(SeqCst)).is_open
    }

    fn poll_unparked(&mut self, do_park: bool) -> Async<()> {
        // First check the `maybe_parked` variable. This avoids acquiring the
        // lock in most cases
        if self.maybe_parked {
            // Get a lock on the task handle
            let mut task = self.sender_task.lock().unwrap();

            if !task.is_parked {
                self.maybe_parked = false;
                return Async::Ready(())
            }

            // At this point, an unpark request is pending, so there will be an
            // unpark sometime in the future. We just need to make sure that
            // the correct task will be notified.
            //
            // Update the task in case the `Sender` has been moved to another
            // task
            task.task = if do_park {
                Some(task::current())
            } else {
                None
            };

            Async::NotReady
        } else {
            Async::Ready(())
        }
    }
}

impl<T> Sink for Sender<T> {
    type SinkItem = T;
    type SinkError = SendError<T>;

    fn start_send(&mut self, msg: T) -> StartSend<T, SendError<T>> {
        // If the sender is currently blocked, reject the message before doing
        // any work.
        if !self.poll_unparked(true).is_ready() {
            return Ok(AsyncSink::NotReady(msg));
        }

        // The channel has capacity to accept the message, so send it.
        self.do_send(Some(msg), true)?;

        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), SendError<T>> {
        self.poll_ready()
            // At this point, the value cannot be returned and `SendError`
            // cannot be created with a `T` without breaking backwards
            // comptibility. This means we cannot return an error.
            //
            // That said, there is also no guarantee that a `poll_complete`
            // returning `Ok` implies the receiver sees the message.
            .or_else(|_| Ok(().into()))
    }

    fn close(&mut self) -> Poll<(), SendError<T>> {
        Ok(Async::Ready(()))
    }
}

impl<T> UnboundedSender<T> {
    /// Returns whether this channel is closed without needing a context.
    pub fn is_closed(&self) -> bool {
        self.0.is_closed()
    }

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
        self.0.do_send_nb(msg)
    }
}

impl<T> Sink for UnboundedSender<T> {
    type SinkItem = T;
    type SinkError = SendError<T>;

    fn start_send(&mut self, msg: T) -> StartSend<T, SendError<T>> {
        self.0.start_send(msg)
    }

    fn poll_complete(&mut self) -> Poll<(), SendError<T>> {
        self.0.poll_complete()
    }

    fn close(&mut self) -> Poll<(), SendError<T>> {
        Ok(Async::Ready(()))
    }
}

impl<'a, T> Sink for &'a UnboundedSender<T> {
    type SinkItem = T;
    type SinkError = SendError<T>;

    fn start_send(&mut self, msg: T) -> StartSend<T, SendError<T>> {
        self.0.do_send_nb(msg)?;
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), SendError<T>> {
        Ok(Async::Ready(()))
    }

    fn close(&mut self) -> Poll<(), SendError<T>> {
        Ok(Async::Ready(()))
    }
}

impl<T> Clone for UnboundedSender<T> {
    fn clone(&self) -> UnboundedSender<T> {
        UnboundedSender(self.0.clone())
    }
}


impl<T> Clone for Sender<T> {
    fn clone(&self) -> Sender<T> {
        // Since this atomic op isn't actually guarding any memory and we don't
        // care about any orderings besides the ordering on the single atomic
        // variable, a relaxed ordering is acceptable.
        let mut curr = self.inner.num_senders.load(SeqCst);

        loop {
            // If the maximum number of senders has been reached, then fail
            if curr == self.inner.max_senders() {
                panic!("cannot clone `Sender` -- too many outstanding senders");
            }

            debug_assert!(curr < self.inner.max_senders());

            let next = curr + 1;
            let actual = self.inner.num_senders.compare_and_swap(curr, next, SeqCst);

            // The ABA problem doesn't matter here. We only care that the
            // number of senders never exceeds the maximum.
            if actual == curr {
                return Sender {
                    inner: self.inner.clone(),
                    sender_task: Arc::new(Mutex::new(SenderTask::new())),
                    maybe_parked: false,
                };
            }

            curr = actual;
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // Ordering between variables don't matter here
        let prev = self.inner.num_senders.fetch_sub(1, SeqCst);

        if prev == 1 {
            let _ = self.do_send(None, false);
        }
    }
}

/*
 *
 * ===== impl Receiver =====
 *
 */

impl<T> Receiver<T> {
    /// Closes the receiving half
    ///
    /// This prevents any further messages from being sent on the channel while
    /// still enabling the receiver to drain messages that are buffered.
    pub fn close(&mut self) {
        let mut curr = self.inner.state.load(SeqCst);

        loop {
            let mut state = decode_state(curr);

            if !state.is_open {
                break
            }

            state.is_open = false;

            let next = encode_state(&state);
            match self.inner.state.compare_exchange(curr, next, SeqCst, SeqCst) {
                Ok(_) => break,
                Err(actual) => curr = actual,
            }
        }

        // Wake up any threads waiting as they'll see that we've closed the
        // channel and will continue on their merry way.
        loop {
            match unsafe { self.inner.parked_queue.pop() } {
                PopResult::Data(task) => {
                    task.lock().unwrap().notify();
                }
                PopResult::Empty => break,
                PopResult::Inconsistent => thread::yield_now(),
            }
        }
    }

    fn next_message(&mut self) -> Async<Option<T>> {
        // Pop off a message
        loop {
            match unsafe { self.inner.message_queue.pop() } {
                PopResult::Data(msg) => {
                    // If there are any parked task handles in the parked queue,
                    // pop one and unpark it.
                    self.unpark_one();
                    // Decrement number of messages
                    self.dec_num_messages();

                    return Async::Ready(msg);
                }
                PopResult::Empty => {
                    // The queue is empty, return NotReady
                    return Async::NotReady;
                }
                PopResult::Inconsistent => {
                    // Inconsistent means that there will be a message to pop
                    // in a short time. This branch can only be reached if
                    // values are being produced from another thread, so there
                    // are a few ways that we can deal with this:
                    //
                    // 1) Spin
                    // 2) thread::yield_now()
                    // 3) task::current().unwrap() & return NotReady
                    //
                    // For now, thread::yield_now() is used, but it would
                    // probably be better to spin a few times then yield.
                    thread::yield_now();
                }
            }
        }
    }

    // Unpark a single task handle if there is one pending in the parked queue
    fn unpark_one(&mut self) {
        loop {
            match unsafe { self.inner.parked_queue.pop() } {
                PopResult::Data(task) => {
                    task.lock().unwrap().notify();
                    return;
                }
                PopResult::Empty => {
                    // Queue empty, no task to wake up.
                    return;
                }
                PopResult::Inconsistent => {
                    // Same as above
                    thread::yield_now();
                }
            }
        }
    }

    // Try to park the receiver task
    fn try_park(&self) -> TryPark {
        let curr = self.inner.state.load(SeqCst);
        let state = decode_state(curr);

        // If the channel is closed, then there is no need to park.
        if state.is_closed() {
            return TryPark::Closed;
        }

        // First, track the task in the `recv_task` slot
        let mut recv_task = self.inner.recv_task.lock().unwrap();

        if recv_task.unparked {
            // Consume the `unpark` signal without actually parking
            recv_task.unparked = false;
            return TryPark::NotEmpty;
        }

        recv_task.task = Some(task::current());
        TryPark::Parked
    }

    fn dec_num_messages(&self) {
        let mut curr = self.inner.state.load(SeqCst);

        loop {
            let mut state = decode_state(curr);

            state.num_messages -= 1;

            let next = encode_state(&state);
            match self.inner.state.compare_exchange(curr, next, SeqCst, SeqCst) {
                Ok(_) => break,
                Err(actual) => curr = actual,
            }
        }
    }
}

impl<T> Stream for Receiver<T> {
    type Item = T;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<T>, ()> {
        loop {
            // Try to read a message off of the message queue.
            match self.next_message() {
                Async::Ready(msg) => return Ok(Async::Ready(msg)),
                Async::NotReady => {
                    // There are no messages to read, in this case, attempt to
                    // park. The act of parking will verify that the channel is
                    // still empty after the park operation has completed.
                    match self.try_park() {
                        TryPark::Parked => {
                            // The task was parked, and the channel is still
                            // empty, return NotReady.
                            return Ok(Async::NotReady);
                        }
                        TryPark::Closed => {
                            // The channel is closed, there will be no further
                            // messages.
                            return Ok(Async::Ready(None));
                        }
                        TryPark::NotEmpty => {
                            // A message has been sent while attempting to
                            // park. Loop again, the next iteration is
                            // guaranteed to get the message.
                            continue;
                        }
                    }
                }
            }
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // Drain the channel of all pending messages
        self.close();

        loop {
            match self.next_message() {
                Async::Ready(_) => {}
                Async::NotReady => {
                    let curr = self.inner.state.load(SeqCst);
                    let state = decode_state(curr);

                    // If the channel is closed, then there is no need to park.
                    if state.is_closed() {
                        return;
                    }

                    // TODO: Spinning isn't ideal, it might be worth
                    // investigating using a condvar or some other strategy
                    // here. That said, if this case is hit, then another thread
                    // is about to push the value into the queue and this isn't
                    // the only spinlock in the impl right now.
                    thread::yield_now();
                }
            }
        }
    }
}

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

    fn poll(&mut self) -> Poll<Option<T>, ()> {
        self.0.poll()
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
    rx: Receiver<Result<Item, Error>>,
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
        rx: rx,
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
/// potentially hog CPU resources.
///
/// # Panics
///
/// This function will panic if `executor` is unable spawn a `Future` containing
/// the entirety of the `stream`.
pub fn spawn_unbounded<S, E>(stream: S, executor: &E) -> SpawnHandle<S::Item, S::Error>
    where S: Stream,
          E: Executor<Execute<S>>
{
    let (cancel_tx, cancel_rx) = oneshot::channel();
    let (tx, rx) = channel2(None);
    executor.execute(Execute {
        inner: tx.send_all(resultstream::new(stream)),
        cancel_rx: cancel_rx,
    }).expect("failed to spawn stream");
    SpawnHandle {
        rx: rx,
        _cancel_tx: cancel_tx,
    }
}

impl<I, E> Stream for SpawnHandle<I, E> {
    type Item = I;
    type Error = E;

    fn poll(&mut self) -> Poll<Option<I>, E> {
        match self.rx.poll() {
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

/*
 *
 * ===== impl Inner =====
 *
 */

impl<T> Inner<T> {
    // The return value is such that the total number of messages that can be
    // enqueued into the channel will never exceed MAX_CAPACITY
    fn max_senders(&self) -> usize {
        match self.buffer {
            Some(buffer) => MAX_CAPACITY - buffer,
            None => MAX_BUFFER,
        }
    }
}

unsafe impl<T: Send> Send for Inner<T> {}
unsafe impl<T: Send> Sync for Inner<T> {}

impl State {
    fn is_closed(&self) -> bool {
        !self.is_open && self.num_messages == 0
    }
}

/*
 *
 * ===== Helpers =====
 *
 */

fn decode_state(num: usize) -> State {
    State {
        is_open: num & OPEN_MASK == OPEN_MASK,
        num_messages: num & MAX_CAPACITY,
    }
}

fn encode_state(state: &State) -> usize {
    let mut num = state.num_messages;

    if state.is_open {
        num |= OPEN_MASK;
    }

    num
}
