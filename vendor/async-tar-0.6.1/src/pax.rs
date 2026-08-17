use std::{slice, str};

#[cfg(feature = "runtime-async-std")]
use async_std::io;
#[cfg(feature = "runtime-tokio")]
use tokio::io;

use crate::other;

/// An iterator over the pax extensions in an archive entry.
///
/// This iterator yields structures which can themselves be parsed into
/// key/value pairs.
pub struct PaxExtensions<'entry> {
    data: slice::Split<'entry, u8, fn(&u8) -> bool>,
}

/// A key/value pair corresponding to a pax extension.
pub struct PaxExtension<'entry> {
    key: &'entry [u8],
    value: &'entry [u8],
}

pub fn pax_extensions(a: &[u8]) -> PaxExtensions<'_> {
    PaxExtensions {
        data: a.split(|a| *a == b'\n'),
    }
}

impl<'entry> Iterator for PaxExtensions<'entry> {
    type Item = io::Result<PaxExtension<'entry>>;

    fn next(&mut self) -> Option<io::Result<PaxExtension<'entry>>> {
        let line = match self.data.next() {
            Some(line) => {
                if line.is_empty() {
                    return None;
                } else {
                    line
                }
            }
            None => return None,
        };

        Some(
            line.iter()
                .position(|b| *b == b' ')
                .and_then(|i| {
                    str::from_utf8(&line[..i])
                        .ok()
                        .and_then(|len| len.parse::<usize>().ok().map(|j| (i + 1, j)))
                })
                .and_then(|(kvstart, reported_len)| {
                    if line.len() + 1 == reported_len {
                        line[kvstart..]
                            .iter()
                            .position(|b| *b == b'=')
                            .map(|equals| (kvstart, equals))
                    } else {
                        None
                    }
                })
                .map(|(kvstart, equals)| PaxExtension {
                    key: &line[kvstart..kvstart + equals],
                    value: &line[kvstart + equals + 1..],
                })
                .ok_or_else(|| other("malformed pax extension")),
        )
    }
}

impl<'entry> PaxExtension<'entry> {
    /// Returns the key for this key/value pair parsed as a string.
    ///
    /// May fail if the key isn't actually utf-8.
    pub fn key(&self) -> Result<&'entry str, str::Utf8Error> {
        str::from_utf8(self.key)
    }

    /// Returns the underlying raw bytes for the key of this key/value pair.
    pub fn key_bytes(&self) -> &'entry [u8] {
        self.key
    }

    /// Returns the value for this key/value pair parsed as a string.
    ///
    /// May fail if the value isn't actually utf-8.
    pub fn value(&self) -> Result<&'entry str, str::Utf8Error> {
        str::from_utf8(self.value)
    }

    /// Returns the underlying raw bytes for this value of this key/value pair.
    pub fn value_bytes(&self) -> &'entry [u8] {
        self.value
    }
}
