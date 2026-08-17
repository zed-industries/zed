use futures_core::Stream;

pub(crate) mod array;
pub(crate) mod tuple;
#[cfg(feature = "alloc")]
pub(crate) mod vec;

/// Takes multiple streams and creates a new stream over all in sequence.
pub trait Chain {
    /// What's the return type of our stream?
    type Item;

    /// What stream do we return?
    type Stream: Stream<Item = Self::Item>;

    /// Combine multiple streams into a single stream.
    fn chain(self) -> Self::Stream;
}
