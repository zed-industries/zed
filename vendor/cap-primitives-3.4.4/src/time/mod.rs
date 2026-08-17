//! Time utilities.

mod instant;
mod monotonic_clock;
mod system_clock;
mod system_time;

pub use instant::Instant;
pub use monotonic_clock::MonotonicClock;
pub use system_clock::SystemClock;
pub use system_time::SystemTime;

pub use std::time::{Duration, SystemTimeError};
