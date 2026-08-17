// Copyright 2015-2016 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

//! Conversions into the library's time type.

/// The time type.
///
/// Internally this is merely a UNIX timestamp: a count of non-leap
/// seconds since the start of 1970.  This type exists to assist
/// unit-of-measure correctness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Time(u64);

impl Time {
    /// Create a `webpki::Time` from a unix timestamp.
    ///
    /// It is usually better to use the less error-prone
    /// `webpki::Time::try_from(time: std::time::SystemTime)` instead when
    /// `std::time::SystemTime` is available (when `#![no_std]` isn't being
    /// used).
    #[allow(clippy::must_use_candidate)]
    pub fn from_seconds_since_unix_epoch(secs: u64) -> Self {
        Self(secs)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl TryFrom<std::time::SystemTime> for Time {
    type Error = std::time::SystemTimeError;

    /// Create a `webpki::Time` from a `std::time::SystemTime`.
    ///
    /// # Example:
    ///
    /// Construct a `webpki::Time` from the current system time:
    ///
    /// ```
    /// # extern crate ring;
    /// # extern crate webpki;
    /// #
    /// #![cfg(feature = "std")]
    /// use std::time::SystemTime;
    ///
    /// # fn foo() -> Result<(), std::time::SystemTimeError> {
    /// let time = webpki::Time::try_from(SystemTime::now())?;
    /// # Ok(())
    /// # }
    /// ```
    fn try_from(value: std::time::SystemTime) -> Result<Self, Self::Error> {
        value
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| Self::from_seconds_since_unix_epoch(d.as_secs()))
    }
}
