use core::future::Future;

pub(crate) mod array;
pub(crate) mod tuple;
#[cfg(feature = "alloc")]
pub(crate) mod vec;

/// Wait for the first successful future to complete.
///
/// Awaits multiple futures simultaneously, returning the output of the first
/// future which completes successfully. If no future completes successfully,
/// returns an aggregate error of all failed futures.
pub trait RaceOk {
    /// The resulting output type.
    type Output;

    /// The resulting error type.
    type Error;

    /// Which kind of future are we turning this into?
    type Future: Future<Output = Result<Self::Output, Self::Error>>;

    /// Waits for the first successful future to complete.
    fn race_ok(self) -> Self::Future;
}
