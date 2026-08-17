use core::mem;
use core::pin::Pin;

use crate::stream::Stream;
use crate::task::{Context, Poll};

use pin_project_lite::pin_project;

/// Creates a new stream where to produce each new element a closure is called with the previous
/// value.
///
/// # Examples
///
/// ```
/// # fn main() { async_std::task::block_on(async {
/// #
/// use async_std::prelude::*;
/// use async_std::stream;
///
/// let mut s = stream::successors(Some(22), |&val| Some(val + 1));
///
/// assert_eq!(s.next().await, Some(22));
/// assert_eq!(s.next().await, Some(23));
/// assert_eq!(s.next().await, Some(24));
/// assert_eq!(s.next().await, Some(25));
///
/// #
/// # }) }
/// ```
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
pub fn successors<F, T>(first: Option<T>, succ: F) -> Successors<F, T>
where
    F: FnMut(&T) -> Option<T>,
{
    Successors { succ, slot: first }
}

pin_project! {
    /// A stream that yields elements by calling an async closure with the previous value as an
    /// argument
    ///
    /// This stream is constructed by [`successors`] function
    ///
    /// [`successors`]: fn.successors.html
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    #[derive(Debug)]
    pub struct Successors<F, T>
    where
        F: FnMut(&T) -> Option<T>
    {
        succ: F,
        slot: Option<T>,
    }
}

impl<F, T> Stream for Successors<F, T>
where
    F: FnMut(&T) -> Option<T>,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        if this.slot.is_none() {
            return Poll::Ready(None);
        }

        let mut next = (this.succ)(&this.slot.as_ref().unwrap());

        // 'swapping' here means 'slot' will hold the next value and next will be th one from the previous iteration
        mem::swap(this.slot, &mut next);
        Poll::Ready(next)
    }
}
