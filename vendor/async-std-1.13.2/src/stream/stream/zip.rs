use core::fmt;
use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    /// A stream that takes items from two other streams simultaneously.
    ///
    /// This `struct` is created by the [`zip`] method on [`Stream`]. See its
    /// documentation for more.
    ///
    /// [`zip`]: trait.Stream.html#method.zip
    /// [`Stream`]: trait.Stream.html
    pub struct Zip<A: Stream, B> {
        item_slot: Option<A::Item>,
        #[pin]
        first: A,
        #[pin]
        second: B,
    }
}

impl<A: Stream + fmt::Debug, B: fmt::Debug> fmt::Debug for Zip<A, B> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Zip")
            .field("first", &self.first)
            .field("second", &self.second)
            .finish()
    }
}

impl<A: Stream, B> Zip<A, B> {
    pub(crate) fn new(first: A, second: B) -> Self {
        Self {
            item_slot: None,
            first,
            second,
        }
    }
}

impl<A: Stream, B: Stream> Stream for Zip<A, B> {
    type Item = (A::Item, B::Item);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        if this.item_slot.is_none() {
            match this.first.poll_next(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Ready(Some(item)) => *this.item_slot = Some(item),
            }
        }
        let second_item = futures_core::ready!(this.second.poll_next(cx));
        let first_item = this.item_slot.take().unwrap();
        Poll::Ready(second_item.map(|second_item| (first_item, second_item)))
    }
}
