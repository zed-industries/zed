use futures_core::Stream;

pub(crate) mod array;
pub(crate) mod tuple;
#[cfg(feature = "alloc")]
pub(crate) mod vec;

/// ‘Zips up’ multiple streams into a single stream of pairs.
pub trait Zip {
    /// What's the return type of our stream?
    type Item;

    /// What stream do we return?
    type Stream: Stream<Item = Self::Item>;

    /// Combine multiple streams into a single stream.
    fn zip(self) -> Self::Stream;
}
