#![cfg(feature = "CFString")]
use core::fmt;

use crate::CFError;

impl fmt::Display for CFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = self.description().unwrap();
        write!(f, "{desc}")
    }
}

#[cfg(feature = "std")] // use core::error::Error from Rust 1.81 once in MSRV.
impl std::error::Error for CFError {}
