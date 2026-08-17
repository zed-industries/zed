// This crate comprises hacks and glue required to test private functions from tests/
//
// Keep this as slim as possible.
//
// If you're caught using this outside this crates tests/, you get to clean up the mess.

#[cfg(not(feature = "std"))]
use alloc::string::String;

use crate::stream_safe::StreamSafe;

pub fn stream_safe(s: &str) -> String {
    StreamSafe::new(s.chars()).collect()
}

pub mod quick_check {
    pub use crate::quick_check::*;
}
