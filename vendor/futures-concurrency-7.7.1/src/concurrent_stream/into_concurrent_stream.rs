use super::ConcurrentStream;

/// Conversion into a [`ConcurrentStream`]
pub trait IntoConcurrentStream {
    /// The type of the elements being iterated over.
    type Item;
    /// Which kind of iterator are we turning this into?
    type IntoConcurrentStream: ConcurrentStream<Item = Self::Item>;

    /// Convert `self` into a concurrent iterator.
    fn into_co_stream(self) -> Self::IntoConcurrentStream;
}

impl<S: ConcurrentStream> IntoConcurrentStream for S {
    type Item = S::Item;
    type IntoConcurrentStream = S;

    fn into_co_stream(self) -> Self::IntoConcurrentStream {
        self
    }
}
