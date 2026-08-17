//! A type that wraps a future to keep track of its completion status.
//!
//! This implementation was taken from the original `macro_rules` `join/try_join`
//! macros in the `futures-preview` crate.

use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::ready;

/// A future that may have completed.
#[derive(Debug)]
pub(crate) enum MaybeDone<Fut: Future> {
    /// A not-yet-completed future
    Future(Fut),

    /// The output of the completed future
    Done(Fut::Output),

    /// The empty variant after the result of a [`MaybeDone`] has been
    /// taken using the [`take`](MaybeDone::take) method.
    Gone,
}

impl<Fut: Future> MaybeDone<Fut> {
    /// Create a new instance of `MaybeDone`.
    pub(crate) fn new(future: Fut) -> MaybeDone<Fut> {
        Self::Future(future)
    }

    /// Returns an [`Option`] containing a reference to the output of the future.
    /// The output of this method will be [`Some`] if and only if the inner
    /// future has been completed and [`take`](MaybeDone::take)
    /// has not yet been called.
    #[inline]
    pub(crate) fn output(self: Pin<&Self>) -> Option<&Fut::Output> {
        let this = self.get_ref();
        match this {
            MaybeDone::Done(res) => Some(res),
            _ => None,
        }
    }

    /// Attempt to take the output of a `MaybeDone` without driving it
    /// towards completion.
    #[inline]
    pub(crate) fn take(self: Pin<&mut Self>) -> Option<Fut::Output> {
        unsafe {
            let this = self.get_unchecked_mut();
            match this {
                MaybeDone::Done(_) => {}
                MaybeDone::Future(_) | MaybeDone::Gone => return None,
            };
            if let MaybeDone::Done(output) = mem::replace(this, MaybeDone::Gone) {
                Some(output)
            } else {
                unreachable!()
            }
        }
    }
}

impl<Fut: Future> Future for MaybeDone<Fut> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = unsafe {
            match Pin::as_mut(&mut self).get_unchecked_mut() {
                MaybeDone::Future(a) => ready!(Pin::new_unchecked(a).poll(cx)),
                MaybeDone::Done(_) => return Poll::Ready(()),
                MaybeDone::Gone => panic!("MaybeDone polled after value taken"),
            }
        };
        self.set(MaybeDone::Done(res));
        Poll::Ready(())
    }
}
