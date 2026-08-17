#[cfg(not(feature = "std"))]
mod no_std;
#[cfg(feature = "std")]
mod readiness_vec;
#[cfg(feature = "std")]
mod waker;
#[cfg(feature = "std")]
mod waker_vec;

#[cfg(not(feature = "std"))]
pub(crate) use no_std::WakerVec;
#[cfg(feature = "std")]
pub(crate) use readiness_vec::ReadinessVec;
#[cfg(feature = "std")]
pub(crate) use waker::InlineWakerVec;
#[cfg(feature = "std")]
pub(crate) use waker_vec::WakerVec;
