//! Extension traits for `SystemClock` and `MonotonicClock`

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.ico"
)]

mod monotonic_clock;
mod system_clock;
mod timezone;

pub use monotonic_clock::MonotonicClockExt;
pub use system_clock::SystemClockExt;
pub use timezone::{Timezone, TimezoneError};
