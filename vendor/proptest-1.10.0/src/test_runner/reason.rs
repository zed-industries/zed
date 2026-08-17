//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::std_facade::{fmt, Box, Cow, String};

/// The reason for why something, such as a generated value, was rejected.
///
/// Currently this is merely a wrapper around a message, but more properties
/// may be added in the future.
///
/// This is constructed via `.into()` on a `String`, `&'static str`, or
/// `Box<str>`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Reason(Cow<'static, str>);

impl Reason {
    /// Return the message for this `Reason`.
    ///
    /// The message is intended for human consumption, and is not guaranteed to
    /// have any format in particular.
    pub fn message(&self) -> &str {
        &*self.0
    }
}

impl From<&'static str> for Reason {
    fn from(s: &'static str) -> Self {
        Reason(s.into())
    }
}

impl From<String> for Reason {
    fn from(s: String) -> Self {
        Reason(s.into())
    }
}

impl From<Box<str>> for Reason {
    fn from(s: Box<str>) -> Self {
        Reason(String::from(s).into())
    }
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.message(), f)
    }
}
