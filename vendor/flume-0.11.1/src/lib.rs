//! # Flume
//!
//! A blazingly fast multi-producer, multi-consumer channel.
//!
//! *"Do not communicate by sharing memory; instead, share memory by communicating."*
//!
//! ## Why Flume?
//!
//! - **Featureful**: Unbounded, bounded and rendezvous queues
//! - **Fast**: Always faster than `std::sync::mpsc` and sometimes `crossbeam-channel`
//! - **Safe**: No `unsafe` code anywhere in the codebase!
//! - **Flexible**: `Sender` and `Receiver` both implement `Send + Sync + Clone`
//! - **Familiar**: Drop-in replacement for `std::sync::mpsc`
//! - **Capable**: Additional features like MPMC support and send timeouts/deadlines
//! - **Simple**: Few dependencies, minimal codebase, fast to compile
//! - **Asynchronous**: `async` support, including mix 'n match with sync code
//! - **Ergonomic**: Powerful `select`-like interface
//!
//! ## Example
//!
//! ```
//! let (tx, rx) = flume::unbounded();
//!
//! tx.send(42).unwrap();
//! assert_eq!(rx.recv().unwrap(), 42);
//! ```

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs)]

#[cfg(feature = "select")]
pub mod select;
#[cfg(feature = "async")]
pub mod r#async;

mod signal;

// Reexports
#[cfg(feature = "select")]
pub use select::Selector;

use std::{
    collections::VecDeque,
    sync::{Arc, atomic::{AtomicUsize, AtomicBool, Ordering}, Weak},
    time::{Duration, Instant},
    marker::PhantomData,
    thread,
    fmt,
};
use std::fmt::Formatter;
#[cfg(feature = "spin")]
use spin1::{Mutex as Spinlock, MutexGuard as SpinlockGuard};
use crate::signal::{Signal, SyncSignal};

/// An error that may be emitted when attempting to send a value into a channel on a sender when
/// all receivers are dropped.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SendError<T>(pub T);

impl<T> SendError<T> {
    /// Consume the error, yielding the message that failed to send.
    pub fn into_inner(self) -> T { self.0 }
}

impl<T> fmt::Debug for SendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "SendError(..)".fmt(f)
    }
}

impl<T> fmt::Display for SendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "sending on a closed channel".fmt(f)
    }
}

impl<T> std::error::Error for SendError<T> {}

/// An error that may be emitted when attempting to send a value into a channel on a sender when
/// the channel is full or all receivers are dropped.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TrySendError<T> {
    /// The channel the message is sent on has a finite capacity and was full when the send was attempted.
    Full(T),
    /// All channel receivers were dropped and so the message has nobody to receive it.
    Disconnected(T),
}

impl<T> TrySendError<T> {
    /// Consume the error, yielding the message that failed to send.
    pub fn into_inner(self) -> T {
        match self {
            Self::Full(msg) | Self::Disconnected(msg) => msg,
        }
    }
}

impl<T> fmt::Debug for TrySendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TrySendError::Full(..) => "Full(..)".fmt(f),
            TrySendError::Disconnected(..) => "Disconnected(..)".fmt(f),
        }
    }
}

impl<T> fmt::Display for TrySendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrySendError::Full(..) => "sending on a full channel".fmt(f),
            TrySendError::Disconnected(..) => "sending on a closed channel".fmt(f),
        }
    }
}

impl<T> std::error::Error for TrySendError<T> {}

impl<T> From<SendError<T>> for TrySendError<T> {
    fn from(err: SendError<T>) -> Self {
        match err {
            SendError(item) => Self::Disconnected(item),
        }
    }
}

/// An error that may be emitted when sending a value into a channel on a sender with a timeout when
/// the send operation times out or all receivers are dropped.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SendTimeoutError<T> {
    /// A timeout occurred when attempting to send the message.
    Timeout(T),
    /// All channel receivers were dropped and so the message has nobody to receive it.
    Disconnected(T),
}

impl<T> SendTimeoutError<T> {
    /// Consume the error, yielding the message that failed to send.
    pub fn into_inner(self) -> T {
        match self {
            Self::Timeout(msg) | Self::Disconnected(msg) => msg,
        }
    }
}

impl<T> fmt::Debug for SendTimeoutError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "SendTimeoutError(..)".fmt(f)
    }
}

impl<T> fmt::Display for SendTimeoutError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SendTimeoutError::Timeout(..) => "timed out sending on a full channel".fmt(f),
            SendTimeoutError::Disconnected(..) => "sending on a closed channel".fmt(f),
        }
    }
}

impl<T> std::error::Error for SendTimeoutError<T> {}

impl<T> From<SendError<T>> for SendTimeoutError<T> {
    fn from(err: SendError<T>) -> Self {
        match err {
            SendError(item) => Self::Disconnected(item),
        }
    }
}

enum TrySendTimeoutError<T> {
    Full(T),
    Disconnected(T),
    Timeout(T),
}

/// An error that may be emitted when attempting to wait for a value on a receiver when all senders
/// are dropped and there are no more messages in the channel.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RecvError {
    /// All senders were dropped and no messages are waiting in the channel, so no further messages can be received.
    Disconnected,
}

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecvError::Disconnected => "receiving on a closed channel".fmt(f),
        }
    }
}

impl std::error::Error for RecvError {}

/// An error that may be emitted when attempting to fetch a value on a receiver when there are no
/// messages in the channel. If there are no messages in the channel and all senders are dropped,
/// then `TryRecvError::Disconnected` will be returned.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TryRecvError {
    /// The channel was empty when the receive was attempted.
    Empty,
    /// All senders were dropped and no messages are waiting in the channel, so no further messages can be received.
    Disconnected,
}

impl fmt::Display for TryRecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TryRecvError::Empty => "receiving on an empty channel".fmt(f),
            TryRecvError::Disconnected => "channel is empty and closed".fmt(f),
        }
    }
}

impl std::error::Error for TryRecvError {}

impl From<RecvError> for TryRecvError {
    fn from(err: RecvError) -> Self {
        match err {
            RecvError::Disconnected => Self::Disconnected,
        }
    }
}

/// An error that may be emitted when attempting to wait for a value on a receiver with a timeout
/// when the receive operation times out or all senders are dropped and there are no values left
/// in the channel.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RecvTimeoutError {
    /// A timeout occurred when attempting to receive a message.
    Timeout,
    /// All senders were dropped and no messages are waiting in the channel, so no further messages can be received.
    Disconnected,
}

impl fmt::Display for RecvTimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecvTimeoutError::Timeout => "timed out waiting on a channel".fmt(f),
            RecvTimeoutError::Disconnected => "channel is empty and closed".fmt(f),
        }
    }
}

impl std::error::Error for RecvTimeoutError {}

impl From<RecvError> for RecvTimeoutError {
    fn from(err: RecvError) -> Self {
        match err {
            RecvError::Disconnected => Self::Disconnected,
        }
    }
}

enum TryRecvTimeoutError {
    Empty,
    Timeout,
    Disconnected,
}

// TODO: Investigate some sort of invalidation flag for timeouts
#[cfg(feature = "spin")]
struct Hook<T, S: ?Sized>(Option<Spinlock<Option<T>>>, S);

#[cfg(not(feature = "spin"))]
struct Hook<T, S: ?Sized>(Option<Mutex<Option<T>>>, S);

#[cfg(feature = "spin")]
impl<T, S: ?Sized + Signal> Hook<T, S> {
    pub fn slot(msg: Option<T>, signal: S) -> Arc<Self>
    where
        S: Sized,
    {
        Arc::new(Self(Some(Spinlock::new(msg)), signal))
    }

    fn lock(&self) -> Option<SpinlockGuard<'_, Option<T>>> {
        self.0.as_ref().map(|s| s.lock())
    }
}

#[cfg(not(feature = "spin"))]
impl<T, S: ?Sized + Signal> Hook<T, S> {
    pub fn slot(msg: Option<T>, signal: S) -> Arc<Self>
    where
        S: Sized,
    {
        Arc::new(Self(Some(Mutex::new(msg)), signal))
    }

    fn lock(&self) -> Option<MutexGuard<'_, Option<T>>> {
        self.0.as_ref().map(|s| s.lock().unwrap())
    }
}

impl<T, S: ?Sized + Signal> Hook<T, S> {
    pub fn fire_recv(&self) -> (T, &S) {
        let msg = self.lock().unwrap().take().unwrap();
        (msg, self.signal())
    }

    pub fn fire_send(&self, msg: T) -> (Option<T>, &S) {
        let ret = match self.lock() {
            Some(mut lock) => {
                *lock = Some(msg);
                None
            }
            None => Some(msg),
        };
        (ret, self.signal())
    }

    pub fn is_empty(&self) -> bool {
        self.lock().map(|s| s.is_none()).unwrap_or(true)
    }

    pub fn try_take(&self) -> Option<T> {
        self.lock().unwrap().take()
    }

    pub fn trigger(signal: S) -> Arc<Self>
    where
        S: Sized,
    {
        Arc::new(Self(None, signal))
    }

    pub fn signal(&self) -> &S {
        &self.1
    }

    pub fn fire_nothing(&self) -> bool {
        self.signal().fire()
    }
}

impl<T> Hook<T, SyncSignal> {
    pub fn wait_recv(&self, abort: &AtomicBool) -> Option<T> {
        loop {
            let disconnected = abort.load(Ordering::SeqCst); // Check disconnect *before* msg
            let msg = self.lock().unwrap().take();
            if let Some(msg) = msg {
                break Some(msg);
            } else if disconnected {
                break None;
            } else {
                self.signal().wait()
            }
        }
    }

    // Err(true) if timeout
    pub fn wait_deadline_recv(&self, abort: &AtomicBool, deadline: Instant) -> Result<T, bool> {
        loop {
            let disconnected = abort.load(Ordering::SeqCst); // Check disconnect *before* msg
            let msg = self.lock().unwrap().take();
            if let Some(msg) = msg {
                break Ok(msg);
            } else if disconnected {
                break Err(false);
            } else if let Some(dur) = deadline.checked_duration_since(Instant::now()) {
                self.signal().wait_timeout(dur);
            } else {
                break Err(true);
            }
        }
    }

    pub fn wait_send(&self, abort: &AtomicBool) {
        loop {
            let disconnected = abort.load(Ordering::SeqCst); // Check disconnect *before* msg
            if disconnected || self.lock().unwrap().is_none() {
                break;
            }

            self.signal().wait();
        }
    }

    // Err(true) if timeout
    pub fn wait_deadline_send(&self, abort: &AtomicBool, deadline: Instant) -> Result<(), bool> {
        loop {
            let disconnected = abort.load(Ordering::SeqCst); // Check disconnect *before* msg
            if self.lock().unwrap().is_none() {
                break Ok(());
            } else if disconnected {
                break Err(false);
            } else if let Some(dur) = deadline.checked_duration_since(Instant::now()) {
                self.signal().wait_timeout(dur);
            } else {
                break Err(true);
            }
        }
    }
}

#[cfg(feature = "spin")]
#[inline]
fn wait_lock<T>(lock: &Spinlock<T>) -> SpinlockGuard<T> {
    // Some targets don't support `thread::sleep` (e.g. the `wasm32-unknown-unknown` target when
    // running in the main thread of a web browser) so we only use it on targets where we know it
    // will work
    #[cfg(any(target_family = "unix", target_family = "windows"))]
    {
        let mut i = 4;
        loop {
            for _ in 0..10 {
                if let Some(guard) = lock.try_lock() {
                    return guard;
                }
                thread::yield_now();
            }
            // Sleep for at most ~1 ms
            thread::sleep(Duration::from_nanos(1 << i.min(20)));
            i += 1;
        }
    }
    #[cfg(not(any(target_family = "unix", target_family = "windows")))]
    lock.lock()
}

#[cfg(not(feature = "spin"))]
#[inline]
fn wait_lock<'a, T>(lock: &'a Mutex<T>) -> MutexGuard<'a, T> {
    lock.lock().unwrap()
}

#[cfg(not(feature = "spin"))]
use std::sync::{Mutex, MutexGuard};

#[cfg(feature = "spin")]
type ChanLock<T> = Spinlock<T>;
#[cfg(not(feature = "spin"))]
type ChanLock<T> = Mutex<T>;


type SignalVec<T> = VecDeque<Arc<Hook<T, dyn signal::Signal>>>;
struct Chan<T> {
    sending: Option<(usize, SignalVec<T>)>,
    queue: VecDeque<T>,
    waiting: SignalVec<T>,
}

impl<T> Chan<T> {
    fn pull_pending(&mut self, pull_extra: bool) {
        if let Some((cap, sending)) = &mut self.sending {
            let effective_cap = *cap + pull_extra as usize;

            while self.queue.len() < effective_cap {
                if let Some(s) = sending.pop_front() {
                    let (msg, signal) = s.fire_recv();
                    signal.fire();
                    self.queue.push_back(msg);
                } else {
                    break;
                }
            }
        }
    }

    fn try_wake_receiver_if_pending(&mut self) {
        if !self.queue.is_empty() {
            while Some(false) == self.waiting.pop_front().map(|s| s.fire_nothing()) {}
        }
    }
}

struct Shared<T> {
    chan: ChanLock<Chan<T>>,
    disconnected: AtomicBool,
    sender_count: AtomicUsize,
    receiver_count: AtomicUsize,
}

impl<T> Shared<T> {
    fn new(cap: Option<usize>) -> Self {
        Self {
            chan: ChanLock::new(Chan {
                sending: cap.map(|cap| (cap, VecDeque::new())),
                queue: VecDeque::new(),
                waiting: VecDeque::new(),
            }),
            disconnected: AtomicBool::new(false),
            sender_count: AtomicUsize::new(1),
            receiver_count: AtomicUsize::new(1),
        }
    }

    fn send<S: Signal, R: From<Result<(), TrySendTimeoutError<T>>>>(
        &self,
        msg: T,
        should_block: bool,
        make_signal: impl FnOnce(T) -> Arc<Hook<T, S>>,
        do_block: impl FnOnce(Arc<Hook<T, S>>) -> R,
    ) -> R {
        let mut chan = wait_lock(&self.chan);

        if self.is_disconnected() {
            Err(TrySendTimeoutError::Disconnected(msg)).into()
        } else if !chan.waiting.is_empty() {
            let mut msg = Some(msg);

            loop {
                let slot = chan.waiting.pop_front();
                match slot.as_ref().map(|r| r.fire_send(msg.take().unwrap())) {
                    // No more waiting receivers and msg in queue, so break out of the loop
                    None if msg.is_none() => break,
                    // No more waiting receivers, so add msg to queue and break out of the loop
                    None => {
                        chan.queue.push_back(msg.unwrap());
                        break;
                    }
                    Some((Some(m), signal)) => {
                        if signal.fire() {
                            // Was async and a stream, so didn't acquire the message. Wake another
                            // receiver, and do not yet push the message.
                            msg.replace(m);
                            continue;
                        } else {
                            // Was async and not a stream, so it did acquire the message. Push the
                            // message to the queue for it to be received.
                            chan.queue.push_back(m);
                            drop(chan);
                            break;
                        }
                    },
                    Some((None, signal)) => {
                        drop(chan);
                        signal.fire();
                        break; // Was sync, so it has acquired the message
                    },
                }
            }

            Ok(()).into()
        } else if chan.sending.as_ref().map(|(cap, _)| chan.queue.len() < *cap).unwrap_or(true) {
            chan.queue.push_back(msg);
            Ok(()).into()
        } else if should_block { // Only bounded from here on
            let hook = make_signal(msg);
            chan.sending.as_mut().unwrap().1.push_back(hook.clone());
            drop(chan);

            do_block(hook)
        } else {
            Err(TrySendTimeoutError::Full(msg)).into()
        }
    }

    fn send_sync(
        &self,
        msg: T,
        block: Option<Option<Instant>>,
    ) -> Result<(), TrySendTimeoutError<T>> {
        self.send(
            // msg
            msg,
            // should_block
            block.is_some(),
            // make_signal
            |msg| Hook::slot(Some(msg), SyncSignal::default()),
            // do_block
            |hook| if let Some(deadline) = block.unwrap() {
                hook.wait_deadline_send(&self.disconnected, deadline)
                    .or_else(|timed_out| {
                        if timed_out { // Remove our signal
                            let hook: Arc<Hook<T, dyn signal::Signal>> = hook.clone();
                            wait_lock(&self.chan).sending
                                .as_mut()
                                .unwrap().1
                                .retain(|s| s.signal().as_ptr() != hook.signal().as_ptr());
                        }
                        hook.try_take().map(|msg| if self.is_disconnected() {
                            Err(TrySendTimeoutError::Disconnected(msg))
                        } else {
                            Err(TrySendTimeoutError::Timeout(msg))
                        })
                        .unwrap_or(Ok(()))
                    })
            } else {
                hook.wait_send(&self.disconnected);

                match hook.try_take() {
                    Some(msg) => Err(TrySendTimeoutError::Disconnected(msg)),
                    None => Ok(()),
                }
            },
        )
    }

    fn recv<S: Signal, R: From<Result<T, TryRecvTimeoutError>>>(
        &self,
        should_block: bool,
        make_signal: impl FnOnce() -> Arc<Hook<T, S>>,
        do_block: impl FnOnce(Arc<Hook<T, S>>) -> R,
    ) -> R {
        let mut chan = wait_lock(&self.chan);
        chan.pull_pending(true);

        if let Some(msg) = chan.queue.pop_front() {
            drop(chan);
            Ok(msg).into()
        } else if self.is_disconnected() {
            drop(chan);
            Err(TryRecvTimeoutError::Disconnected).into()
        } else if should_block {
            let hook = make_signal();
            chan.waiting.push_back(hook.clone());
            drop(chan);

            do_block(hook)
        } else {
            drop(chan);
            Err(TryRecvTimeoutError::Empty).into()
        }
    }

    fn recv_sync(&self, block: Option<Option<Instant>>) -> Result<T, TryRecvTimeoutError> {
        self.recv(
            // should_block
            block.is_some(),
            // make_signal
            || Hook::slot(None, SyncSignal::default()),
            // do_block
            |hook| if let Some(deadline) = block.unwrap() {
                hook.wait_deadline_recv(&self.disconnected, deadline)
                    .or_else(|timed_out| {
                        if timed_out { // Remove our signal
                            let hook: Arc<Hook<T, dyn Signal>> = hook.clone();
                            wait_lock(&self.chan).waiting
                                .retain(|s| s.signal().as_ptr() != hook.signal().as_ptr());
                        }
                        match hook.try_take() {
                            Some(msg) => Ok(msg),
                            None => {
                                let disconnected = self.is_disconnected(); // Check disconnect *before* msg
                                if let Some(msg) = wait_lock(&self.chan).queue.pop_front() {
                                    Ok(msg)
                                } else if disconnected {
                                    Err(TryRecvTimeoutError::Disconnected)
                                } else {
                                    Err(TryRecvTimeoutError::Timeout)
                                }
                            },
                        }
                    })
            } else {
                hook.wait_recv(&self.disconnected)
                    .or_else(|| wait_lock(&self.chan).queue.pop_front())
                    .ok_or(TryRecvTimeoutError::Disconnected)
            },
        )
    }

    /// Disconnect anything listening on this channel (this will not prevent receivers receiving
    /// msgs that have already been sent)
    fn disconnect_all(&self) {
        self.disconnected.store(true, Ordering::Relaxed);

        let mut chan = wait_lock(&self.chan);
        chan.pull_pending(false);
        if let Some((_, sending)) = chan.sending.as_ref() {
            sending.iter().for_each(|hook| {
                hook.signal().fire();
            })
        }
        chan.waiting.iter().for_each(|hook| {
            hook.signal().fire();
        });
    }

    fn is_disconnected(&self) -> bool {
        self.disconnected.load(Ordering::SeqCst)
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_full(&self) -> bool {
        self.capacity().map(|cap| cap == self.len()).unwrap_or(false)
    }

    fn len(&self) -> usize {
        let mut chan = wait_lock(&self.chan);
        chan.pull_pending(false);
        chan.queue.len()
    }

    fn capacity(&self) -> Option<usize> {
        wait_lock(&self.chan).sending.as_ref().map(|(cap, _)| *cap)
    }

    fn sender_count(&self) -> usize {
        self.sender_count.load(Ordering::Relaxed)
    }

    fn receiver_count(&self) -> usize {
        self.receiver_count.load(Ordering::Relaxed)
    }
}

/// A transmitting end of a channel.
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    /// Attempt to send a value into the channel. If the channel is bounded and full, or all
    /// receivers have been dropped, an error is returned. If the channel associated with this
    /// sender is unbounded, this method has the same behaviour as [`Sender::send`].
    pub fn try_send(&self, msg: T) -> Result<(), TrySendError<T>> {
        self.shared.send_sync(msg, None).map_err(|err| match err {
            TrySendTimeoutError::Full(msg) => TrySendError::Full(msg),
            TrySendTimeoutError::Disconnected(msg) => TrySendError::Disconnected(msg),
            _ => unreachable!(),
        })
    }

    /// Send a value into the channel, returning an error if all receivers have been dropped.
    /// If the channel is bounded and is full, this method will block until space is available
    /// or all receivers have been dropped. If the channel is unbounded, this method will not
    /// block.
    pub fn send(&self, msg: T) -> Result<(), SendError<T>> {
        self.shared.send_sync(msg, Some(None)).map_err(|err| match err {
            TrySendTimeoutError::Disconnected(msg) => SendError(msg),
            _ => unreachable!(),
        })
    }

    /// Send a value into the channel, returning an error if all receivers have been dropped
    /// or the deadline has passed. If the channel is bounded and is full, this method will
    /// block until space is available, the deadline is reached, or all receivers have been
    /// dropped.
    pub fn send_deadline(&self, msg: T, deadline: Instant) -> Result<(), SendTimeoutError<T>> {
        self.shared.send_sync(msg, Some(Some(deadline))).map_err(|err| match err {
            TrySendTimeoutError::Disconnected(msg) => SendTimeoutError::Disconnected(msg),
            TrySendTimeoutError::Timeout(msg) => SendTimeoutError::Timeout(msg),
            _ => unreachable!(),
        })
    }

    /// Send a value into the channel, returning an error if all receivers have been dropped
    /// or the timeout has expired. If the channel is bounded and is full, this method will
    /// block until space is available, the timeout has expired, or all receivers have been
    /// dropped.
    pub fn send_timeout(&self, msg: T, dur: Duration) -> Result<(), SendTimeoutError<T>> {
        self.send_deadline(msg, Instant::now().checked_add(dur).unwrap())
    }

    /// Returns true if all receivers for this channel have been dropped.
    pub fn is_disconnected(&self) -> bool {
        self.shared.is_disconnected()
    }

    /// Returns true if the channel is empty.
    /// Note: Zero-capacity channels are always empty.
    pub fn is_empty(&self) -> bool {
        self.shared.is_empty()
    }

    /// Returns true if the channel is full.
    /// Note: Zero-capacity channels are always full.
    pub fn is_full(&self) -> bool {
        self.shared.is_full()
    }

    /// Returns the number of messages in the channel
    pub fn len(&self) -> usize {
        self.shared.len()
    }

    /// If the channel is bounded, returns its capacity.
    pub fn capacity(&self) -> Option<usize> {
        self.shared.capacity()
    }

    /// Get the number of senders that currently exist, including this one.
    pub fn sender_count(&self) -> usize {
        self.shared.sender_count()
    }

    /// Get the number of receivers that currently exist.
    ///
    /// Note that this method makes no guarantees that a subsequent send will succeed; it's
    /// possible that between `receiver_count()` being called and a `send()`, all open receivers
    /// could drop.
    pub fn receiver_count(&self) -> usize {
        self.shared.receiver_count()
    }

    /// Creates a [`WeakSender`] that does not keep the channel open.
    ///
    /// The channel is closed once all `Sender`s are dropped, even if there
    /// are still active `WeakSender`s.
    pub fn downgrade(&self) -> WeakSender<T> {
        WeakSender {
            shared: Arc::downgrade(&self.shared),
        }
    }

    /// Returns whether the senders are belong to the same channel.
    pub fn same_channel(&self, other: &Sender<T>) -> bool {
        Arc::ptr_eq(&self.shared, &other.shared)
    }
}

impl<T> Clone for Sender<T> {
    /// Clone this sender. [`Sender`] acts as a handle to the ending a channel. Remaining channel
    /// contents will only be cleaned up when all senders and the receiver have been dropped.
    fn clone(&self) -> Self {
        self.shared.sender_count.fetch_add(1, Ordering::Relaxed);
        Self { shared: self.shared.clone() }
    }
}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Sender").finish()
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // Notify receivers that all senders have been dropped if the number of senders drops to 0.
        if self.shared.sender_count.fetch_sub(1, Ordering::Relaxed) == 1 {
            self.shared.disconnect_all();
        }
    }
}

/// A sender that does not prevent the channel from being closed.
///
/// Weak senders do not count towards the number of active senders on the channel. As soon as
/// all normal [`Sender`]s are dropped, the channel is closed, even if there is still a
/// `WeakSender`.
///
/// To send messages, a `WeakSender` must first be upgraded to a `Sender` using the [`upgrade`]
/// method.
pub struct WeakSender<T> {
    shared: Weak<Shared<T>>,
}

impl<T> WeakSender<T> {
    /// Tries to upgrade the `WeakSender` to a [`Sender`], in order to send messages.
    ///
    /// Returns `None` if the channel was closed already. Note that a `Some` return value
    /// does not guarantee that the channel is still open.
    pub fn upgrade(&self) -> Option<Sender<T>> {
        self.shared
            .upgrade()
            // check that there are still live senders
            .filter(|shared| {
                shared
                    .sender_count
                    .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |count| {
                        if count == 0 {
                            // all senders are closed already -> don't increase the sender count
                            None
                        } else {
                            // there is still at least one active sender
                            Some(count + 1)
                        }
                    })
                    .is_ok()
            })
            .map(|shared| Sender { shared })
    }
}

impl<T> fmt::Debug for WeakSender<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("WeakSender").finish()
    }
}

impl<T> Clone for WeakSender<T> {
    /// Clones this [`WeakSender`].
    fn clone(&self) -> Self {
        Self { shared: self.shared.clone() }
    }
}

/// The receiving end of a channel.
///
/// Note: Cloning the receiver *does not* turn this channel into a broadcast channel.
/// Each message will only be received by a single receiver. This is useful for
/// implementing work stealing for concurrent programs.
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    /// Attempt to fetch an incoming value from the channel associated with this receiver,
    /// returning an error if the channel is empty or if all senders have been dropped.
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.shared.recv_sync(None).map_err(|err| match err {
            TryRecvTimeoutError::Disconnected => TryRecvError::Disconnected,
            TryRecvTimeoutError::Empty => TryRecvError::Empty,
            _ => unreachable!(),
        })
    }

    /// Wait for an incoming value from the channel associated with this receiver, returning an
    /// error if all senders have been dropped.
    pub fn recv(&self) -> Result<T, RecvError> {
        self.shared.recv_sync(Some(None)).map_err(|err| match err {
            TryRecvTimeoutError::Disconnected => RecvError::Disconnected,
            _ => unreachable!(),
        })
    }

    /// Wait for an incoming value from the channel associated with this receiver, returning an
    /// error if all senders have been dropped or the deadline has passed.
    pub fn recv_deadline(&self, deadline: Instant) -> Result<T, RecvTimeoutError> {
        self.shared.recv_sync(Some(Some(deadline))).map_err(|err| match err {
            TryRecvTimeoutError::Disconnected => RecvTimeoutError::Disconnected,
            TryRecvTimeoutError::Timeout => RecvTimeoutError::Timeout,
            _ => unreachable!(),
        })
    }

    /// Wait for an incoming value from the channel associated with this receiver, returning an
    /// error if all senders have been dropped or the timeout has expired.
    pub fn recv_timeout(&self, dur: Duration) -> Result<T, RecvTimeoutError> {
        self.recv_deadline(Instant::now().checked_add(dur).unwrap())
    }

    /// Create a blocking iterator over the values received on the channel that finishes iteration
    /// when all senders have been dropped.
    ///
    /// You can also create a self-owned iterator with [`Receiver::into_iter`].
    pub fn iter(&self) -> Iter<T> {
        Iter { receiver: &self }
    }

    /// A non-blocking iterator over the values received on the channel that finishes iteration
    /// when all senders have been dropped or the channel is empty.
    pub fn try_iter(&self) -> TryIter<T> {
        TryIter { receiver: &self }
    }

    /// Take all msgs currently sitting in the channel and produce an iterator over them. Unlike
    /// `try_iter`, the iterator will not attempt to fetch any more values from the channel once
    /// the function has been called.
    pub fn drain(&self) -> Drain<T> {
        let mut chan = wait_lock(&self.shared.chan);
        chan.pull_pending(false);
        let queue = std::mem::take(&mut chan.queue);

        Drain { queue, _phantom: PhantomData }
    }

    /// Returns true if all senders for this channel have been dropped.
    pub fn is_disconnected(&self) -> bool {
        self.shared.is_disconnected()
    }

    /// Returns true if the channel is empty.
    /// Note: Zero-capacity channels are always empty.
    pub fn is_empty(&self) -> bool {
        self.shared.is_empty()
    }

    /// Returns true if the channel is full.
    /// Note: Zero-capacity channels are always full.
    pub fn is_full(&self) -> bool {
        self.shared.is_full()
    }

    /// Returns the number of messages in the channel.
    pub fn len(&self) -> usize {
        self.shared.len()
    }

    /// If the channel is bounded, returns its capacity.
    pub fn capacity(&self) -> Option<usize> {
        self.shared.capacity()
    }

    /// Get the number of senders that currently exist.
    pub fn sender_count(&self) -> usize {
        self.shared.sender_count()
    }

    /// Get the number of receivers that currently exist, including this one.
    pub fn receiver_count(&self) -> usize {
        self.shared.receiver_count()
    }

    /// Returns whether the receivers are belong to the same channel.
    pub fn same_channel(&self, other: &Receiver<T>) -> bool {
        Arc::ptr_eq(&self.shared, &other.shared)
    }
}

impl<T> Clone for Receiver<T> {
    /// Clone this receiver. [`Receiver`] acts as a handle to the ending a channel. Remaining
    /// channel contents will only be cleaned up when all senders and the receiver have been
    /// dropped.
    ///
    /// Note: Cloning the receiver *does not* turn this channel into a broadcast channel.
    /// Each message will only be received by a single receiver. This is useful for
    /// implementing work stealing for concurrent programs.
    fn clone(&self) -> Self {
        self.shared.receiver_count.fetch_add(1, Ordering::Relaxed);
        Self { shared: self.shared.clone() }
    }
}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Receiver").finish()
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // Notify senders that all receivers have been dropped if the number of receivers drops
        // to 0.
        if self.shared.receiver_count.fetch_sub(1, Ordering::Relaxed) == 1 {
            self.shared.disconnect_all();
        }
    }
}

/// This exists as a shorthand for [`Receiver::iter`].
impl<'a, T> IntoIterator for &'a Receiver<T> {
    type Item = T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { receiver: self }
    }
}

impl<T> IntoIterator for Receiver<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a self-owned but semantically equivalent alternative to [`Receiver::iter`].
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { receiver: self }
    }
}

/// An iterator over the msgs received from a channel.
pub struct Iter<'a, T> {
    receiver: &'a Receiver<T>,
}

impl<'a, T> fmt::Debug for Iter<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Iter").field("receiver", &self.receiver).finish()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.recv().ok()
    }
}

/// An non-blocking iterator over the msgs received from a channel.
pub struct TryIter<'a, T> {
    receiver: &'a Receiver<T>,
}

impl<'a, T> fmt::Debug for TryIter<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("TryIter").field("receiver", &self.receiver).finish()
    }
}

impl<'a, T> Iterator for TryIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.try_recv().ok()
    }
}

/// An fixed-sized iterator over the msgs drained from a channel.
#[derive(Debug)]
pub struct Drain<'a, T> {
    queue: VecDeque<T>,
    /// A phantom field used to constrain the lifetime of this iterator. We do this because the
    /// implementation may change and we don't want to unintentionally constrain it. Removing this
    /// lifetime later is a possibility.
    _phantom: PhantomData<&'a ()>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

impl<'a, T> ExactSizeIterator for Drain<'a, T> {
    fn len(&self) -> usize {
        self.queue.len()
    }
}

/// An owned iterator over the msgs received from a channel.
pub struct IntoIter<T> {
    receiver: Receiver<T>,
}

impl<T> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntoIter").field("receiver", &self.receiver).finish()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.recv().ok()
    }
}

/// Create a channel with no maximum capacity.
///
/// Create an unbounded channel with a [`Sender`] and [`Receiver`] connected to each end respectively. Values sent in
/// one end of the channel will be received on the other end. The channel is thread-safe, and both [`Sender`] and
/// [`Receiver`] may be sent to or shared between threads as necessary. In addition, both [`Sender`] and [`Receiver`]
/// may be cloned.
///
/// # Examples
/// ```
/// let (tx, rx) = flume::unbounded();
///
/// tx.send(42).unwrap();
/// assert_eq!(rx.recv().unwrap(), 42);
/// ```
pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared::new(None));
    (
        Sender { shared: shared.clone() },
        Receiver { shared },
    )
}

/// Create a channel with a maximum capacity.
///
/// Create a bounded channel with a [`Sender`] and [`Receiver`] connected to each end respectively. Values sent in one
/// end of the channel will be received on the other end. The channel is thread-safe, and both [`Sender`] and
/// [`Receiver`] may be sent to or shared between threads as necessary. In addition, both [`Sender`] and [`Receiver`]
/// may be cloned.
///
/// Unlike an [`unbounded`] channel, if there is no space left for new messages, calls to
/// [`Sender::send`] will block (unblocking once a receiver has made space). If blocking behaviour
/// is not desired, [`Sender::try_send`] may be used.
///
/// Like `std::sync::mpsc`, `flume` supports 'rendezvous' channels. A bounded queue with a maximum capacity of zero
/// will block senders until a receiver is available to take the value. You can imagine a rendezvous channel as a
/// ['Glienicke Bridge'](https://en.wikipedia.org/wiki/Glienicke_Bridge)-style location at which senders and receivers
/// perform a handshake and transfer ownership of a value.
///
/// # Examples
/// ```
/// let (tx, rx) = flume::bounded(32);
///
/// for i in 1..33 {
///     tx.send(i).unwrap();
/// }
/// assert!(tx.try_send(33).is_err());
///
/// assert_eq!(rx.try_iter().sum::<u32>(), (1..33).sum());
/// ```
pub fn bounded<T>(cap: usize) -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared::new(Some(cap)));
    (
        Sender { shared: shared.clone() },
        Receiver { shared },
    )
}
