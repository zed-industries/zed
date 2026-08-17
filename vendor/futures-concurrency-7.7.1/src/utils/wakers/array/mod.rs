#[cfg(not(feature = "std"))]
mod no_std;
#[cfg(feature = "std")]
mod readiness_array;
#[cfg(feature = "std")]
mod waker;
#[cfg(feature = "std")]
mod waker_array;

#[cfg(not(feature = "std"))]
pub(crate) use no_std::WakerArray;
#[cfg(feature = "std")]
pub(crate) use readiness_array::ReadinessArray;
#[cfg(feature = "std")]
pub(crate) use waker::InlineWakerArray;
#[cfg(feature = "std")]
pub(crate) use waker_array::WakerArray;
