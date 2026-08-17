//! Implementation of the [Bytes] type

use super::RmpRead;
use crate::decode::RmpReadErr;
use core::fmt::{Display, Formatter};

/// Indicates that an error occurred reading from [Bytes]
#[derive(Debug)]
#[non_exhaustive]
// NOTE: We can't use thiserror because of no_std :(
pub enum BytesReadError {
    /// Indicates that there were not enough bytes.
    InsufficientBytes {
        expected: usize,
        actual: usize,
        position: u64,
    },
}

impl Display for BytesReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            BytesReadError::InsufficientBytes { expected, actual, position } => {
                write!(f, "Expected at least bytes {expected}, but only got {actual} (pos {position})")
            }
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for BytesReadError {}
impl RmpReadErr for BytesReadError {}

/// A wrapper around `&[u8]` to read more efficiently.
///
/// This has a specialized implementation of `RmpWrite`
/// and has error type [Infallible](core::convert::Infallible).
///
/// This has the additional benefit of working on `#[no_std]` (unlike the builtin Read trait)
///
/// See also [serde_bytes::Bytes](https://docs.rs/serde_bytes/0.11/serde_bytes/struct.Bytes.html)
///
/// Unlike a plain `&[u8]` this also tracks an internal offset in the input (See [`Self::position`]).
///
/// This is used for (limited) compatibility with [`std::io::Cursor`]. Unlike a [Cursor](std::io::Cursor) it does
/// not support mark/reset.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Bytes<'a> {
    /// The internal position of the input buffer.
    ///
    /// This is not required for correctness.
    /// It is only used for error reporting (and to implement [`Self::position`])
    current_position: u64,
    bytes: &'a [u8],
}
impl<'a> Bytes<'a> {
    /// Wrap an existing bytes slice.
    ///
    /// This sets the internal position to zero.
    #[inline]
    #[must_use]
    pub fn new(bytes: &'a [u8]) -> Self {
        Bytes { bytes, current_position: 0 }
    }
    /// Get a reference to the remaining bytes in the buffer.
    #[inline]
    #[must_use]
    pub fn remaining_slice(&self) -> &'a [u8] {
        self.bytes
    }
    /// Return the position of the input buffer.
    ///
    /// This is not required for correctness, it only exists to help mimic
    /// [`Cursor::position`](std::io::Cursor::position)
    #[inline]
    #[must_use]
    pub fn position(&self) -> u64 {
        self.current_position
    }
}
impl<'a> From<&'a [u8]> for Bytes<'a> {
    #[inline]
    fn from(bytes: &'a [u8]) -> Self {
        Bytes { bytes, current_position: 0 }
    }
}

impl RmpRead for Bytes<'_> {
    type Error = BytesReadError;

    #[inline]
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        if let Some((&first, newly_remaining)) = self.bytes.split_first() {
            self.bytes = newly_remaining;
            self.current_position += 1;
            Ok(first)
        } else {
            Err(BytesReadError::InsufficientBytes {
                expected: 1,
                actual: 0,
                position: self.current_position,
            })
        }
    }

    #[inline]
    fn read_exact_buf(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        let to_read = buf.len();
        if to_read <= self.bytes.len() {
            let (src, newly_remaining) = self.bytes.split_at(to_read);
            self.bytes = newly_remaining;
            self.current_position += to_read as u64;
            buf.copy_from_slice(src);
            Ok(())
        } else {
            Err(BytesReadError::InsufficientBytes {
                expected: to_read,
                actual: self.bytes.len(),
                position: self.current_position,
            })
        }
    }
}

#[cfg(not(feature = "std"))]
impl<'a> RmpRead for &'a [u8] {
    type Error = BytesReadError;

    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        if let Some((&first, newly_remaining)) = self.split_first() {
            *self = newly_remaining;
            Ok(first)
        } else {
            Err(BytesReadError::InsufficientBytes {
                expected: 1,
                actual: 0,
                position: 0,
            })
        }
    }

    fn read_exact_buf(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        let to_read = buf.len();
        if to_read <= self.len() {
            let (src, newly_remaining) = self.split_at(to_read);
            *self = newly_remaining;
            buf.copy_from_slice(src);
            Ok(())
        } else {
            Err(BytesReadError::InsufficientBytes {
                expected: to_read,
                actual: self.len(),
                position: 0,
            })
        }
    }
}
