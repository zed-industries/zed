//! A capability-based clock API modeled after [`std::time`].
//!
//! This corresponds to [`std::time`].
//!
//! Instead of [`std::time`]'s methods which return the current time, this
//! crate has methods on [`SystemClock`] and [`MonotonicClock`].

pub use cap_primitives::time::{
    Duration, Instant, MonotonicClock, SystemClock, SystemTime, SystemTimeError,
};
