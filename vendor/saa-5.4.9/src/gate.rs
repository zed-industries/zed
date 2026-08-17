//! [`Gate`] is a synchronization primitive that blocks tasks from entering a critical section until
//! they are allowed to do so.

#![deny(unsafe_code)]

use std::pin::{Pin, pin};
use std::ptr::{null_mut, without_provenance_mut};
#[cfg(not(feature = "loom"))]
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{self, AcqRel, Acquire, Relaxed};

#[cfg(feature = "loom")]
use loom::sync::atomic::AtomicPtr;

use crate::opcode::Opcode;
use crate::pager::SyncResult;
use crate::sync_primitive::SyncPrimitive;
use crate::wait_queue::{Entry, WaitQueue};
use crate::{Pager, pager};

/// [`Gate`] is a synchronization primitive that blocks tasks from entering a critical section until
/// they are allowed to do so.
#[derive(Debug, Default)]
pub struct Gate {
    /// [`Gate`] state.
    state: AtomicPtr<()>,
}

/// The state of a [`Gate`].
///
/// [`Gate`] can be in one of three states.
///
/// * `Controlled` - The default state where tasks can enter the [`Gate`] if permitted.
/// * `Sealed` - The [`Gate`] is sealed and tasks immediately get rejected when they attempt to enter.
/// * `Open` - The [`Gate`] is open and tasks can immediately enter it.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum State {
    /// The default state where tasks can enter the [`Gate`] if permitted.
    Controlled = 0_u8,
    /// The [`Gate`] is sealed and tasks immediately get rejected when they attempt to enter it.
    Sealed = 1_u8,
    /// The [`Gate`] is open and tasks can immediately enter it.
    Open = 2_u8,
}

/// Errors that can occur when accessing a [`Gate`].
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Error {
    /// The [`Gate`] rejected the task.
    Rejected = 4_u8,
    /// The [`Gate`] has been sealed.
    Sealed = 8_u8,
    /// Spurious failure to enter a [`Gate`] in a [`Controlled`](State::Controlled) state.
    ///
    /// This can happen if a task holding a [`Pager`] gets cancelled or drops the [`Pager`] before
    /// the [`Gate`] has permitted or rejected the task; the task causes all other waiting tasks
    /// of the [`Gate`] to get this error.
    SpuriousFailure = 12_u8,
    /// The [`Pager`] is not registered in any [`Gate`].
    NotRegistered = 16_u8,
    /// The wrong asynchronous/synchronous mode was used for a [`Pager`].
    WrongMode = 20_u8,
    /// The result is not ready.
    NotReady = 24_u8,
}

impl Gate {
    /// Mask to get the state value from `u8`.
    const STATE_MASK: u8 = 0b11;

    /// Creates a new [`Gate`].
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Gate;
    ///
    /// let gate = Gate::new();
    /// ```
    #[cfg(not(feature = "loom"))]
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: AtomicPtr::new(null_mut()),
        }
    }

    /// Creates a new [`Gate`].
    #[cfg(feature = "loom")]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: AtomicPtr::new(null_mut()),
        }
    }

    /// Returns the current state of the [`Gate`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// use saa::Gate;
    /// use saa::gate::State;
    ///
    /// let gate = Gate::default();
    ///
    /// assert_eq!(gate.state(Relaxed), State::Controlled);
    /// ```
    #[inline]
    pub fn state(&self, mo: Ordering) -> State {
        State::from(self.state.load(mo).addr() & WaitQueue::DATA_MASK)
    }

    /// Resets the [`Gate`] to its initial state if it is not in a [`Controlled`](State::Controlled)
    /// state.
    ///
    /// Returns the previous state of the [`Gate`].
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Gate;
    /// use saa::gate::State;
    ///
    /// let gate = Gate::default();
    ///
    /// assert_eq!(gate.reset(), None);
    ///
    /// gate.seal();
    ///
    /// assert_eq!(gate.reset(), Some(State::Sealed));
    /// ```
    #[inline]
    pub fn reset(&self) -> Option<State> {
        match self.state.fetch_update(Relaxed, Relaxed, |value| {
            let state = State::from(value.addr() & WaitQueue::DATA_MASK);
            if state == State::Controlled {
                None
            } else {
                debug_assert_eq!(value.addr() & WaitQueue::ADDR_MASK, 0);
                Some(
                    value.map_addr(|addr| (addr & WaitQueue::ADDR_MASK) | u8::from(state) as usize),
                )
            }
        }) {
            Ok(state) => Some(State::from(state.addr() & WaitQueue::DATA_MASK)),
            Err(_) => None,
        }
    }

    /// Permits waiting tasks to enter the [`Gate`] if the [`Gate`] is in a
    /// [`Controlled`](State::Controlled) state.
    ///
    /// Returns the number of permitted tasks.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the [`Gate`] is not in a [`Controlled`](State::Controlled) state.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use std::thread;
    ///
    /// use saa::Gate;
    /// use saa::gate::State;
    ///
    /// let gate = Arc::new(Gate::default());
    ///
    /// let gate_clone = gate.clone();
    ///
    /// let thread = thread::spawn(move || {
    ///     assert_eq!(gate_clone.enter_sync(), Ok(State::Controlled));
    /// });
    ///
    /// loop {
    ///     if gate.permit() == Ok(1) {
    ///         break;
    ///     }
    /// }
    ///
    /// thread.join().unwrap();
    /// ```
    #[inline]
    pub fn permit(&self) -> Result<usize, State> {
        let (state, count) = self.wake_all(None, None);
        if state == State::Controlled {
            Ok(count)
        } else {
            debug_assert_eq!(count, 0);
            Err(state)
        }
    }

    /// Rejects waiting tasks from entering the [`Gate`] if the [`Gate`] is in a
    /// [`Controlled`](State::Controlled) state.
    ///
    /// Returns the number of rejected tasks.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the [`Gate`] is not in a [`Controlled`](State::Controlled) state.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use std::thread;
    ///
    /// use saa::Gate;
    /// use saa::gate::Error;
    ///
    /// let gate = Arc::new(Gate::default());
    ///
    /// let gate_clone = gate.clone();
    ///
    /// let thread = thread::spawn(move || {
    ///     assert_eq!(gate_clone.enter_sync(), Err(Error::Rejected));
    /// });
    ///
    /// loop {
    ///     if gate.reject() == Ok(1) {
    ///         break;
    ///     }
    /// }
    ///
    /// thread.join().unwrap();
    /// ```
    #[inline]
    pub fn reject(&self) -> Result<usize, State> {
        let (state, count) = self.wake_all(None, Some(Error::Rejected));
        if state == State::Controlled {
            Ok(count)
        } else {
            debug_assert_eq!(count, 0);
            Err(state)
        }
    }

    /// Opens the [`Gate`] to allow any tasks to enter it.
    ///
    /// Returns the number of tasks that were waiting to enter it.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// use saa::Gate;
    /// use saa::gate::State;
    ///
    /// let gate = Gate::default();
    /// assert_eq!(gate.state(Relaxed), State::Controlled);
    ///
    /// let (prev_state, count) = gate.open();
    ///
    /// assert_eq!(prev_state, State::Controlled);
    /// assert_eq!(count, 0);
    /// assert_eq!(gate.state(Relaxed), State::Open);
    /// ```
    #[inline]
    pub fn open(&self) -> (State, usize) {
        self.wake_all(Some(State::Open), None)
    }

    /// Seals the [`Gate`] to disallow tasks from entering.
    ///
    /// Returns the previous state of the [`Gate`] and the number of tasks that were waiting to
    /// enter the Gate.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// use saa::Gate;
    /// use saa::gate::State;
    ///
    /// let gate = Gate::default();
    ///
    /// let (prev_state, count) = gate.seal();
    ///
    /// assert_eq!(prev_state, State::Controlled);
    /// assert_eq!(count, 0);
    /// assert_eq!(gate.state(Relaxed), State::Sealed);
    /// ```
    #[inline]
    pub fn seal(&self) -> (State, usize) {
        self.wake_all(Some(State::Sealed), Some(Error::Sealed))
    }

    /// Enters the [`Gate`] asynchronously.
    ///
    /// Returns the current state of the [`Gate`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if it failed to enter the [`Gate`].
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::future;
    /// use saa::Gate;
    ///
    /// let gate = Gate::default();
    ///
    /// let a = async {
    ///     assert!(gate.enter_async().await.is_ok());
    /// };
    /// let b = async {
    ///     gate.permit();
    /// };
    /// future::join(a, b);
    /// ```
    #[inline]
    pub async fn enter_async(&self) -> Result<State, Error> {
        let mut pinned_pager = pin!(Pager::default());
        pinned_pager
            .wait_queue()
            .construct(self, Opcode::Wait(0), false);
        self.push_wait_queue_entry(&mut pinned_pager, || {});
        pinned_pager.poll_async().await
    }

    /// Enters the [`Gate`] synchronously.
    ///
    /// Returns the current state of the [`Gate`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if it failed to enter the [`Gate`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use std::thread;
    ///
    /// use saa::Gate;
    /// use saa::gate::State;
    ///
    /// let gate = Arc::new(Gate::default());
    ///
    /// let gate_clone = gate.clone();
    /// let thread_1 = thread::spawn(move || {
    ///     assert_eq!(gate_clone.enter_sync(), Ok(State::Controlled));
    /// });
    ///
    /// let gate_clone = gate.clone();
    /// let thread_2 = thread::spawn(move || {
    ///     assert_eq!(gate_clone.enter_sync(), Ok(State::Controlled));
    /// });
    ///
    /// let mut count = 0;
    /// while count != 2 {
    ///     if let Ok(n) = gate.permit() {
    ///         count += n;
    ///     }
    /// }
    ///
    /// thread_1.join().unwrap();
    /// thread_2.join().unwrap();
    /// ```
    #[inline]
    pub fn enter_sync(&self) -> Result<State, Error> {
        self.enter_sync_with(|| ())
    }

    /// Enters the [`Gate`] asynchronously with a wait callback.
    ///
    /// Returns the current state of the [`Gate`]. The callback is invoked when the task starts
    /// waiting.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if it failed to enter the [`Gate`].
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::future;
    /// use saa::Gate;
    ///
    /// let gate = Gate::default();
    ///
    /// let a = async {
    ///     let mut wait = false;
    ///     assert!(gate.enter_async_with(|| wait = true).await.is_ok());
    /// };
    /// let b = async {
    ///    gate.permit();
    /// };
    /// future::join(a, b);
    /// ```
    #[inline]
    pub async fn enter_async_with<F: FnOnce()>(&self, begin_wait: F) -> Result<State, Error> {
        let mut pinned_pager = pin!(Pager::default());
        pinned_pager
            .wait_queue()
            .construct(self, Opcode::Wait(0), false);
        self.push_wait_queue_entry(&mut pinned_pager, begin_wait);
        pinned_pager.poll_async().await
    }

    /// Enters the [`Gate`] synchronously with a wait callback.
    ///
    /// Returns the current state of the [`Gate`]. The callback is invoked when the task starts
    /// waiting.
    /// # Errors
    ///
    /// Returns an [`Error`] if it failed to enter the [`Gate`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use std::thread;
    ///
    /// use saa::Gate;
    /// use saa::gate::State;
    ///
    /// let gate = Arc::new(Gate::default());
    ///
    /// let gate_clone = gate.clone();
    /// let thread_1 = thread::spawn(move || {
    ///     let mut wait = false;
    ///     assert_eq!(gate_clone.enter_sync_with(|| wait = true), Ok(State::Controlled));
    /// });
    ///
    /// let gate_clone = gate.clone();
    /// let thread_2 = thread::spawn(move || {
    ///     let mut wait = false;
    ///     assert_eq!(gate_clone.enter_sync_with(|| wait = true), Ok(State::Controlled));
    /// });
    ///
    /// let mut count = 0;
    /// while count != 2 {
    ///     if let Ok(n) = gate.permit() {
    ///         count += n;
    ///     }
    /// }
    ///
    /// thread_1.join().unwrap();
    /// thread_2.join().unwrap();
    /// ```
    #[inline]
    pub fn enter_sync_with<F: FnOnce()>(&self, begin_wait: F) -> Result<State, Error> {
        let mut pinned_pager = pin!(Pager::default());
        pinned_pager
            .wait_queue()
            .construct(self, Opcode::Wait(0), true);
        self.push_wait_queue_entry(&mut pinned_pager, begin_wait);
        pinned_pager.poll_sync()
    }

    /// Registers a [`Pager`] to allow it to get a permit to enter the [`Gate`] remotely.
    ///
    /// `is_sync` indicates whether the [`Pager`] will be polled asynchronously (`false`) or
    /// synchronously (`true`).
    ///
    /// Returns `false` if the [`Pager`] was already registered.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::pin::pin;
    /// use std::sync::Arc;
    /// use std::thread;
    ///
    /// use saa::{Gate, Pager};
    /// use saa::gate::State;
    ///
    /// let gate = Arc::new(Gate::default());
    ///
    /// let mut pinned_pager = pin!(Pager::default());
    ///
    /// assert!(gate.register_pager(&mut pinned_pager, true));
    /// assert!(!gate.register_pager(&mut pinned_pager, true));
    ///
    /// let gate_clone = gate.clone();
    /// let thread = thread::spawn(move || {
    ///     assert_eq!(gate_clone.permit(), Ok(1));
    /// });
    ///
    /// thread.join().unwrap();
    ///
    /// assert_eq!(pinned_pager.poll_sync(), Ok(State::Controlled));
    /// ```
    #[inline]
    pub fn register_pager<'g>(
        &'g self,
        pager: &mut Pin<&mut Pager<'g, Self>>,
        is_sync: bool,
    ) -> bool {
        if pager.is_registered() {
            return false;
        }
        pager.wait_queue().construct(self, Opcode::Wait(0), is_sync);
        self.push_wait_queue_entry(pager, || ());
        true
    }

    /// Wakes up all waiting tasks and updates the state.
    ///
    /// Returns `(prev_state, count)` where `prev_state` is the previous state of the Gate and
    /// `count` is the number of tasks that were woken up.
    fn wake_all(&self, next_state: Option<State>, error: Option<Error>) -> (State, usize) {
        match self.state.fetch_update(AcqRel, Acquire, |value| {
            if let Some(new_value) = next_state {
                Some(without_provenance_mut(u8::from(new_value) as usize))
            } else {
                Some(value.map_addr(|addr| addr & WaitQueue::DATA_MASK))
            }
        }) {
            Ok(value) | Err(value) => {
                let mut count = 0;
                let prev_state = State::from(value.addr() & WaitQueue::DATA_MASK);
                let next_state = next_state.unwrap_or(prev_state);
                let result = Self::into_u8(next_state, error);
                let anchor_ptr = WaitQueue::to_anchor_ptr(value);
                if !anchor_ptr.is_null() {
                    let tail_entry_ptr = WaitQueue::to_entry_ptr(anchor_ptr);
                    Entry::iter_forward(tail_entry_ptr, false, |entry_ptr, _| {
                        Entry::set_result(entry_ptr, result);
                        count += 1;
                        false
                    });
                }
                (prev_state, count)
            }
        }
    }

    /// Pushes the wait queue entry.
    #[inline]
    fn push_wait_queue_entry<F: FnOnce()>(&self, pager: &mut Pin<&mut Pager<Self>>, begin_wait: F) {
        loop {
            let state = self.state.load(Acquire);
            match State::from(state.addr() & WaitQueue::DATA_MASK) {
                State::Controlled => {
                    if !self.try_push_wait_queue_entry(pager.wait_queue(), state) {
                        continue;
                    }
                    begin_wait();
                }
                State::Sealed => {
                    Entry::set_result(
                        pager.wait_queue().entry_ptr(),
                        Self::into_u8(State::Sealed, Some(Error::Sealed)),
                    );
                }
                State::Open => {
                    Entry::set_result(
                        pager.wait_queue().entry_ptr(),
                        Self::into_u8(State::Open, None),
                    );
                }
            }
            break;
        }
    }

    /// Converts `(State, Error)` into `u8`.
    #[inline]
    fn into_u8(state: State, error: Option<Error>) -> u8 {
        u8::from(state) | error.map_or(0_u8, u8::from)
    }
}

impl Drop for Gate {
    #[inline]
    fn drop(&mut self) {
        if self.state.load(Relaxed).addr() & WaitQueue::ADDR_MASK == 0 {
            return;
        }
        self.seal();
    }
}

impl SyncPrimitive for Gate {
    #[inline]
    fn state(&self) -> &AtomicPtr<()> {
        &self.state
    }

    #[inline]
    fn max_shared_owners() -> usize {
        usize::MAX
    }

    #[inline]
    fn drop_wait_queue_entry(entry_ptr: *const Entry) {
        let entry_ref = Entry::entry_ref(entry_ptr);
        if entry_ref.try_consume_result().is_none() {
            let this_ref: &Self = Entry::entry_ref(entry_ptr).sync_primitive_ref();
            this_ref.wake_all(None, Some(Error::SpuriousFailure));
            entry_ref.acknowledge_result_sync();
        }
    }
}

impl SyncResult for Gate {
    type Result = Result<State, Error>;

    #[inline]
    fn to_result(value: u8, pager_error: Option<pager::Error>) -> Self::Result {
        if let Some(pager_error) = pager_error {
            match pager_error {
                pager::Error::NotRegistered => Err(Error::NotRegistered),
                pager::Error::WrongMode => Err(Error::WrongMode),
                pager::Error::NotReady => Err(Error::NotReady),
            }
        } else {
            let state = State::from(value & Self::STATE_MASK);
            let error = value & !(Self::STATE_MASK);
            if error != 0 {
                Err(Error::from(error))
            } else {
                Ok(state)
            }
        }
    }
}

impl From<State> for u8 {
    #[inline]
    fn from(value: State) -> Self {
        match value {
            State::Controlled => 0_u8,
            State::Sealed => 1_u8,
            State::Open => 2_u8,
        }
    }
}

impl From<u8> for State {
    #[inline]
    fn from(value: u8) -> Self {
        State::from(value as usize)
    }
}

impl From<usize> for State {
    #[inline]
    fn from(value: usize) -> Self {
        match value {
            0 => State::Controlled,
            1 => State::Sealed,
            _ => State::Open,
        }
    }
}

impl From<Error> for u8 {
    #[inline]
    fn from(value: Error) -> Self {
        match value {
            Error::Rejected => 4_u8,
            Error::Sealed => 8_u8,
            Error::SpuriousFailure => 12_u8,
            Error::NotRegistered => 16_u8,
            Error::WrongMode => 20_u8,
            Error::NotReady => 24_u8,
        }
    }
}

impl From<u8> for Error {
    #[inline]
    fn from(value: u8) -> Self {
        Error::from(value as usize)
    }
}

impl From<usize> for Error {
    #[inline]
    fn from(value: usize) -> Self {
        match value {
            4 => Error::Rejected,
            8 => Error::Sealed,
            12 => Error::SpuriousFailure,
            16 => Error::NotRegistered,
            20 => Error::WrongMode,
            _ => Error::NotReady,
        }
    }
}
