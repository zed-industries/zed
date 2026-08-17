pub use crate::stream::Stream;

/// A stream that knows its exact length.
///
/// Many [`Stream`]s don't know how many times they will iterate, but some do.
/// If a stream knows how many times it can iterate, providing access to
/// that information can be useful. For example, if you want to iterate
/// backwards, a good start is to know where the end is.
///
/// When implementing an `ExactSizeStream`, you must also implement
/// [`Stream`]. When doing so, the implementation of [`size_hint`] *must*
/// return the exact size of the stream.
///
/// [`Stream`]: trait.Stream.html
/// [`size_hint`]: trait.Stream.html#method.size_hint
///
/// The [`len`] method has a default implementation, so you usually shouldn't
/// implement it. However, you may be able to provide a more performant
/// implementation than the default, so overriding it in this case makes sense.
///
/// [`len`]: #method.len
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// // a finite range knows exactly how many times it will iterate
/// let five = 0..5;
///
/// assert_eq!(5, five.len());
/// ```
///
/// In the [module level docs][moddocs], we implemented an [`Stream`],
/// `Counter`. Let's implement `ExactSizeStream` for it as well:
///
/// [moddocs]: index.html
///
/// ```
/// # use std::task::{Context, Poll};
/// # use std::pin::Pin;
/// # use async_std::prelude::*;
/// # struct Counter {
/// #     count: usize,
/// # }
/// # impl Counter {
/// #     fn new() -> Counter {
/// #         Counter { count: 0 }
/// #     }
/// # }
/// # impl Stream for Counter {
/// #     type Item = usize;
/// #     fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
/// #         self.count += 1;
/// #         if self.count < 6 {
/// #             Poll::Ready(Some(self.count))
/// #         } else {
/// #             Poll::Ready(None)
/// #         }
/// #     }
/// # }
/// # async_std::task::block_on(async {
/// #
/// impl ExactSizeStream for Counter {
///     // We can easily calculate the remaining number of iterations.
///     fn len(&self) -> usize {
///         5 - self.count
///     }
/// }
///
/// // And now we can use it!
///
/// let counter = Counter::new();
///
/// assert_eq!(5, counter.len());
/// # });
/// ```
#[allow(clippy::len_without_is_empty)] // ExactSizeIterator::is_empty is unstable
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
pub trait ExactSizeStream: Stream {
    /// Returns the exact number of times the stream will iterate.
    ///
    /// This method has a default implementation, so you usually should not
    /// implement it directly. However, if you can provide a more efficient
    /// implementation, you can do so. See the [trait-level] docs for an
    /// example.
    ///
    /// This function has the same safety guarantees as the [`size_hint`]
    /// function.
    ///
    /// [trait-level]: trait.ExactSizeStream.html
    /// [`size_hint`]: trait.Stream.html#method.size_hint
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// // a finite range knows exactly how many times it will iterate
    /// let five = 0..5;
    ///
    /// assert_eq!(5, five.len());
    /// ```
    fn len(&self) -> usize {
        let (lower, upper) = self.size_hint();
        // Note: This assertion is overly defensive, but it checks the invariant
        // guaranteed by the trait. If this trait were rust-internal,
        // we could use debug_assert!; assert_eq! will check all Rust user
        // implementations too.
        assert_eq!(upper, Some(lower));
        lower
    }
}

impl<I: ExactSizeStream + ?Sized + Unpin> ExactSizeStream for &mut I {
    fn len(&self) -> usize {
        (**self).len()
    }
}
