//! [`Pager`] allows the user to remotely wait for a desired resource.

#![deny(unsafe_code)]

use std::cell::UnsafeCell;
use std::marker::{PhantomData, PhantomPinned};
use std::pin::Pin;

use crate::wait_queue::{Entry, WaitQueue};

/// Tasks holding a [`Pager`] can remotely acquire a desired resource.
///
/// [`Pager`] contains a wait queue entry which forms an intrusive linked list. It is important that
/// the [`Pager`] is not moved while it is registered in a synchronization primitive, otherwise it
/// may lead to undefined behavior, therefore [`Pager`] does not implement [`Unpin`].
#[derive(Debug, Default)]
pub struct Pager<'s, S: SyncResult> {
    /// The wait queue for the [`Pager`].
    wait_queue: UnsafeCell<WaitQueue>,
    /// The [`Pager`] cannot outlive the associated synchronization primitive.
    _phantom: PhantomData<&'s S>,
    /// The [`Pager`] cannot be unpinned.
    _pinned: PhantomPinned,
}
/// Errors that can occur when using a [`Pager`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// The [`Pager`] is not registered in a synchronization primitive.
    NotRegistered,
    /// The [`Pager`] is registered in a synchronization primitive with a different mode.
    WrongMode,
    /// The result is not ready.
    NotReady,
}

/// Defines result value interpretation interfaces.
pub trait SyncResult: Sized {
    /// Operation result type.
    type Result: Clone + Copy + Eq + PartialEq;

    /// Converts a `u8` value into a `Self::Result`.
    fn to_result(value: u8, pager_error: Option<Error>) -> Self::Result;
}

impl<'s, S: SyncResult> Pager<'s, S> {
    /// Returns `true` if the [`Pager`] is registered in a synchronization primitive.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::pin::pin;
    ///
    /// use saa::{Gate, Pager};
    /// use saa::gate::State;
    ///
    /// let gate = Gate::default();
    ///
    /// let mut pinned_pager = pin!(Pager::default());
    /// assert!(!pinned_pager.is_registered());
    ///
    /// assert!(gate.register_pager(&mut pinned_pager, false));
    /// assert!(pinned_pager.is_registered());
    ///
    /// assert!(!gate.register_pager(&mut pinned_pager, false));
    /// assert!(pinned_pager.is_registered());
    /// ```
    #[inline]
    pub fn is_registered(self: &mut Pin<&mut Pager<'s, S>>) -> bool {
        self.wait_queue().is_pollable()
    }

    /// Waits for the desired resource to become available asynchronously.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::pin::pin;
    ///
    /// use saa::{Gate, Pager};
    /// use saa::gate::State;
    ///
    /// let gate = Gate::default();
    ///
    /// let mut pinned_pager = pin!(Pager::default());
    ///
    /// assert!(gate.register_pager(&mut pinned_pager, false));
    ///
    /// assert_eq!(gate.open().1, 1);
    ///
    /// async {
    ///     assert_eq!(pinned_pager.poll_async().await, Ok(State::Open));
    /// };
    /// ```
    #[inline]
    pub async fn poll_async(self: &mut Pin<&mut Pager<'s, S>>) -> S::Result {
        if !self.is_registered() {
            return S::to_result(0, Some(Error::NotRegistered));
        }
        let result = self.wait_queue().await;
        if result == Entry::ERROR_WRONG_MODE {
            return S::to_result(result, Some(Error::WrongMode));
        }
        S::to_result(result, None)
    }

    /// Waits for the desired resource to become available synchronously.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::pin::pin;
    ///
    /// use saa::{Gate, Pager};
    /// use saa::gate::State;
    ///
    /// let gate = Gate::default();
    ///
    /// let mut pinned_pager = pin!(Pager::default());
    ///
    /// assert!(gate.register_pager(&mut pinned_pager, true));
    ///
    /// assert_eq!(gate.open().1, 1);
    ///
    /// assert_eq!(pinned_pager.poll_sync(), Ok(State::Open));
    /// ```
    #[inline]
    pub fn poll_sync(self: &mut Pin<&mut Pager<'s, S>>) -> S::Result {
        if !self.is_registered() {
            return S::to_result(0, Some(Error::NotRegistered));
        }
        let result = Entry::entry_ref(self.wait_queue().entry_ptr()).poll_result_sync();
        if result == Entry::ERROR_WRONG_MODE {
            return S::to_result(result, Some(Error::WrongMode));
        }
        S::to_result(result, None)
    }

    /// Tries to get the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::pin::pin;
    ///
    /// use saa::{Gate, Pager};
    /// use saa::gate::{Error, State};
    ///
    /// let gate = Gate::default();
    ///
    /// let mut pinned_pager = pin!(Pager::default());
    ///
    /// assert_eq!(pinned_pager.try_poll(), Err(Error::NotRegistered));
    ///
    /// assert!(gate.register_pager(&mut pinned_pager, true));
    ///
    /// assert_eq!(pinned_pager.try_poll(), Err(Error::NotReady));
    /// assert_eq!(gate.open().1, 1);
    ///
    /// assert_eq!(pinned_pager.try_poll(), Ok(State::Open));
    /// ```
    #[inline]
    pub fn try_poll(self: &mut Pin<&mut Pager<'s, S>>) -> S::Result {
        if !self.is_registered() {
            return S::to_result(0, Some(Error::NotRegistered));
        }
        if let Some(result) = Entry::entry_ref(self.wait_queue().entry_ptr()).try_consume_result() {
            S::to_result(result, None)
        } else {
            S::to_result(0, Some(Error::NotReady))
        }
    }

    /// Returns a reference to the wait queue entry.
    #[inline]
    pub(crate) fn wait_queue(&self) -> Pin<&WaitQueue> {
        WaitQueue::pin_ptr(self.wait_queue.get())
    }
}
