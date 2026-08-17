use core::future::Future;

pub(crate) mod array;
pub(crate) mod tuple;
#[cfg(feature = "alloc")]
pub(crate) mod vec;

/// Wait for the first future to complete.
///
/// Awaits multiple future at once, returning as soon as one completes. The
/// other futures are cancelled.
pub trait Race {
    /// The resulting output type.
    type Output;

    /// Which kind of future are we turning this into?
    type Future: Future<Output = Self::Output>;

    /// Wait for the first future to complete.
    ///
    /// Awaits multiple futures at once, returning as soon as one completes. The
    /// other futures are cancelled.
    ///
    /// This function returns a new future which polls all futures concurrently.
    ///
    /// # Panics
    ///
    /// This method will panic if the collection is empty (for `Vec` and array
    /// implementations) since there would be no future to race.
    fn race(self) -> Self::Future;
}
