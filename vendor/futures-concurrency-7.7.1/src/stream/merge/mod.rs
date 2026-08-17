use futures_core::Stream;

pub(crate) mod array;
pub(crate) mod tuple;
#[cfg(feature = "alloc")]
pub(crate) mod vec;

/// Combines multiple streams into a single stream of all their outputs.
///
/// Items are yielded as soon as they're received, and the stream continues
/// yield until both streams have been exhausted. The output ordering
/// between streams is not guaranteed.
///
/// # Examples
///
/// ```
/// use futures_concurrency::prelude::*;
/// use futures_lite::stream::{self, StreamExt};
/// use futures_lite::future::block_on;
///
/// block_on(async {
///     let a = stream::once(1);
///     let b = stream::once(2);
///     let c = stream::once(3);
///     let mut s = [a, b, c].merge();
///
///     let mut buf = vec![];
///     s.for_each(|n| buf.push(n)).await;
///     buf.sort_unstable();
///     assert_eq!(&buf, &[1, 2, 3]);
/// })
/// ```
pub trait Merge {
    /// The resulting output type.
    type Item;

    /// The stream type.
    type Stream: Stream<Item = Self::Item>;

    /// Combine multiple streams into a single stream.
    fn merge(self) -> Self::Stream;
}
