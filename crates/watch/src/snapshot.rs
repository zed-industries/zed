//! Provides a latest-value channel with independently owned snapshots.
//!
//! Channel synchronization uses atomic publication rather than reader/writer
//! locks. Retaining a snapshot never prevents publication.
//!
//! ```
//! let (mut sender, mut receiver) = watch::snapshot::channel(1);
//! let previous = receiver.snapshot();
//! assert_eq!(sender.send(2), Ok(()));
//! assert_eq!(*previous, 1);
//! assert_eq!(*receiver.snapshot(), 2);
//! ```

use crate::{NoReceiverError, NoSenderError};
use arc_swap::ArcSwap;
use futures::task::AtomicWaker;
use std::{
    ops,
    pin::Pin,
    sync::{
        Arc, Weak,
        atomic::{AtomicBool, Ordering::SeqCst},
    },
    task::{Context, Poll},
};

struct Versioned<T> {
    value: T,
    version: usize,
}

struct State<T> {
    latest: ArcSwap<Versioned<T>>,
    receivers: ArcSwap<Vec<Weak<AtomicWaker>>>,
    closed: AtomicBool,
}

impl<T> State<T> {
    fn subscribe(self: &Arc<Self>, version: usize) -> Receiver<T> {
        let waker = Arc::new(AtomicWaker::new());
        self.receivers.rcu(|receivers| {
            let mut receivers = receivers.as_ref().clone();
            receivers.push(Arc::downgrade(&waker));
            receivers
        });
        Receiver {
            state: self.clone(),
            waker,
            version,
        }
    }

    fn wake_receivers(&self) -> Result<(), NoReceiverError> {
        let mut result = Err(NoReceiverError);
        for receiver in self.receivers.load().iter() {
            if let Some(waker) = receiver.upgrade() {
                result = Ok(());
                waker.wake();
            }
        }
        result
    }
}

/// Creates a channel whose initial value is already considered seen.
///
/// Use [`Receiver::snapshot`] to read the initial value. Each send counts as a
/// change, even when the value is equal; slow receivers skip intermediate values.
pub fn channel<T>(value: T) -> (Sender<T>, Receiver<T>) {
    let receiver = Receiver::constant(value);
    let sender = Sender {
        state: receiver.state.clone(),
    };
    (sender, receiver)
}

/// Publishes values to all receivers without waiting for snapshots to be released.
pub struct Sender<T> {
    state: Arc<State<T>>,
}

impl<T> Sender<T> {
    /// Creates a receiver with the current version already considered seen.
    pub fn receiver(&self) -> Receiver<T> {
        let version = self.state.latest.load().version;
        self.state.subscribe(version)
    }

    /// Replaces the latest value and wakes receivers.
    ///
    /// # Errors
    ///
    /// Returns [`NoReceiverError`] if no receivers remain. The value is still
    /// stored and can be read by a subsequently created receiver.
    pub fn send(&mut self, value: T) -> Result<(), NoReceiverError> {
        let version = self.state.latest.load().version.wrapping_add(1);
        self.state
            .latest
            .store(Arc::new(Versioned { value, version }));
        self.state.wake_receivers()
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.state.closed.store(true, SeqCst);
        match self.state.wake_receivers() {
            Ok(()) | Err(NoReceiverError) => {}
        }
    }
}

/// Observes publications independently of other receivers.
///
/// Cloning copies the last-seen version but creates an independent wake
/// registration. A snapshot does not require the value to implement `Clone`.
pub struct Receiver<T> {
    state: Arc<State<T>>,
    waker: Arc<AtomicWaker>,
    version: usize,
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        self.state.subscribe(self.version)
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        let waker = Arc::downgrade(&self.waker);
        self.state.receivers.rcu(|receivers| {
            receivers
                .iter()
                .filter(|receiver| !receiver.ptr_eq(&waker))
                .cloned()
                .collect::<Vec<_>>()
        });
    }
}

impl<T> Receiver<T> {
    /// Returns an owned snapshot and marks its version as seen.
    ///
    /// The snapshot remains valid after further sends or after the channel is
    /// dropped, and does not prevent either operation.
    pub fn snapshot(&mut self) -> Snapshot<T> {
        let snapshot = self.state.latest.load_full();
        self.version = snapshot.version;
        Snapshot(snapshot)
    }

    /// Waits for an unseen publication and marks that version as seen.
    ///
    /// Dropping a pending future unregisters its waker without marking any
    /// publication as seen.
    ///
    /// # Errors
    ///
    /// Returns [`NoSenderError`] after the sender is dropped and the final
    /// publication has been seen.
    pub fn changed(&mut self) -> impl Future<Output = Result<(), NoSenderError>> {
        Changed { receiver: self }
    }

    /// Creates a fixed-value receiver whose change futures remain pending forever.
    pub fn constant(value: T) -> Self {
        Arc::new(State {
            latest: ArcSwap::from_pointee(Versioned { value, version: 0 }),
            receivers: ArcSwap::from_pointee(Vec::new()),
            closed: AtomicBool::new(false),
        })
        .subscribe(0)
    }
}

impl<T: Clone> Receiver<T> {
    /// Waits for an unseen publication and clones the latest value.
    ///
    /// # Errors
    ///
    /// Returns [`NoSenderError`] after the sender is dropped and the final
    /// publication has been seen.
    pub async fn recv(&mut self) -> Result<T, NoSenderError> {
        self.changed().await?;
        Ok((*self.snapshot()).clone())
    }
}

/// Owns an immutable channel value independently of its receiver.
pub struct Snapshot<T>(Arc<Versioned<T>>);

impl<T> Clone for Snapshot<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> ops::Deref for Snapshot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.value
    }
}

struct Changed<'a, T> {
    receiver: &'a mut Receiver<T>,
}

impl<T> Future for Changed<'_, T> {
    type Output = Result<(), NoSenderError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let receiver = &mut self.get_mut().receiver;
        // Register before checking so a racing publication cannot go unnoticed.
        receiver.waker.register(cx.waker());
        // Observing closure must precede loading the value, so closure cannot
        // hide a final publication that raced with this poll.
        let closed = receiver.state.closed.load(SeqCst);
        let snapshot = receiver.state.latest.load();
        if snapshot.version != receiver.version {
            receiver.version = snapshot.version;
            Poll::Ready(Ok(()))
        } else if closed {
            Poll::Ready(Err(NoSenderError))
        } else {
            Poll::Pending
        }
    }
}

impl<T> Drop for Changed<'_, T> {
    fn drop(&mut self) {
        self.receiver.waker.take();
    }
}

#[cfg(test)]
mod tests;
