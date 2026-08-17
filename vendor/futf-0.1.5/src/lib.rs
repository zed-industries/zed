// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(test, feature(test))]

#[macro_use]
extern crate debug_unreachable;

#[macro_use]
extern crate mac;

#[cfg(test)]
extern crate test as std_test;

use std::{slice, char};

/// Meaning of a complete or partial UTF-8 codepoint.
///
/// Not all checking is performed eagerly. That is, a codepoint `Prefix` or
/// `Suffix` may in reality have no valid completion.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Meaning {
    /// We found a whole codepoint.
    Whole(char),

    /// We found something that isn't a valid Unicode codepoint, but
    /// it *would* correspond to a UTF-16 leading surrogate code unit,
    /// i.e. a value in the range `U+D800` - `U+DBFF`.
    ///
    /// The argument is the code unit's 10-bit index within that range.
    ///
    /// These are found in UTF-8 variants such as CESU-8 and WTF-8.
    LeadSurrogate(u16),

    /// We found something that isn't a valid Unicode codepoint, but
    /// it *would* correspond to a UTF-16 trailing surrogate code unit,
    /// i.e. a value in the range `U+DC00` - `U+DFFF`.
    ///
    /// The argument is the code unit's 10-bit index within that range.
    ///
    /// These are found in UTF-8 variants such as CESU-8 and WTF-8.
    TrailSurrogate(u16),

    /// We found only a prefix of a codepoint before the buffer ended.
    ///
    /// Includes the number of additional bytes needed.
    Prefix(usize),

    /// We found only a suffix of a codepoint before running off the
    /// start of the buffer.
    ///
    /// Up to 3 more bytes may be needed.
    Suffix,
}

/// Represents a complete or partial UTF-8 codepoint.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Codepoint<'a> {
    /// The bytes that make up the partial or full codepoint.
    ///
    /// For a `Suffix` this depends on `idx`. We don't scan forward
    /// for additional continuation bytes after the reverse scan
    /// failed to locate a multibyte sequence start.
    pub bytes: &'a [u8],

    /// Start of the codepoint in the buffer, expressed as an offset
    /// back from `idx`.
    pub rewind: usize,

    /// Meaning of the partial or full codepoint.
    pub meaning: Meaning,
}

#[derive(Debug, PartialEq, Eq)]
enum Byte {
    Ascii,
    Start(usize),
    Cont,
}

impl Byte {
    #[inline(always)]
    fn classify(x: u8) -> Option<Byte> {
        match x & 0xC0 {
            0xC0 => match x {
                x if x & 0b11111_000 == 0b11110_000 => Some(Byte::Start(4)),
                x if x & 0b1111_0000 == 0b1110_0000 => Some(Byte::Start(3)),
                x if x & 0b111_00000 == 0b110_00000 => Some(Byte::Start(2)),
                _ => None,
            },
            0x80 => Some(Byte::Cont),
            _ => Some(Byte::Ascii),
        }
    }
}

#[inline(always)]
fn all_cont(buf: &[u8]) -> bool {
    buf.iter().all(|&b| matches!(Byte::classify(b), Some(Byte::Cont)))
}

// NOTE: Assumes the buffer is a syntactically valid multi-byte UTF-8 sequence:
// a starting byte followed by the correct number of continuation bytes.
#[inline(always)]
unsafe fn decode(buf: &[u8]) -> Option<Meaning> {
    debug_assert!(buf.len() >= 2);
    debug_assert!(buf.len() <= 4);
    let n;
    match buf.len() {
        2 => {
            n = ((*buf.get_unchecked(0) & 0b11111) as u32) << 6
                | ((*buf.get_unchecked(1) & 0x3F) as u32);
            if n < 0x80 { return None }  // Overlong
        }
        3 => {
            n = ((*buf.get_unchecked(0) & 0b1111) as u32) << 12
                | ((*buf.get_unchecked(1) & 0x3F) as u32) << 6
                | ((*buf.get_unchecked(2) & 0x3F) as u32);
            match n {
                0x0000 ... 0x07FF => return None,  // Overlong
                0xD800 ... 0xDBFF => return Some(Meaning::LeadSurrogate(n as u16 - 0xD800)),
                0xDC00 ... 0xDFFF => return Some(Meaning::TrailSurrogate(n as u16 - 0xDC00)),
                _ => {}
            }
        }
        4 => {
            n = ((*buf.get_unchecked(0) & 0b111) as u32) << 18
                | ((*buf.get_unchecked(1) & 0x3F) as u32) << 12
                | ((*buf.get_unchecked(2) & 0x3F) as u32) << 6
                | ((*buf.get_unchecked(3) & 0x3F) as u32);
            if n < 0x1_0000 { return None }  // Overlong
        }
        _ => debug_unreachable!(),
    }

    char::from_u32(n).map(Meaning::Whole)
}

#[inline(always)]
unsafe fn unsafe_slice<'a>(buf: &'a [u8], start: usize, new_len: usize) -> &'a [u8] {
    debug_assert!(start <= buf.len());
    debug_assert!(new_len <= (buf.len() - start));
    slice::from_raw_parts(buf.as_ptr().offset(start as isize), new_len)
}

macro_rules! otry {
    ($x:expr) => { unwrap_or_return!($x, None) }
}

/// Describes the UTF-8 codepoint containing the byte at index `idx` within
/// `buf`.
///
/// Returns `None` if `idx` is out of range, or if `buf` contains invalid UTF-8
/// in the vicinity of `idx`.
#[inline]
pub fn classify<'a>(buf: &'a [u8], idx: usize) -> Option<Codepoint<'a>> {
    if idx >= buf.len() {
        return None;
    }

    unsafe {
        let x = *buf.get_unchecked(idx);
        match otry!(Byte::classify(x)) {
            Byte::Ascii => Some(Codepoint {
                bytes: unsafe_slice(buf, idx, 1),
                rewind: 0,
                meaning: Meaning::Whole(x as char),
            }),
            Byte::Start(n) => {
                let avail = buf.len() - idx;
                if avail >= n {
                    let bytes = unsafe_slice(buf, idx, n);
                    if !all_cont(unsafe_slice(bytes, 1, n-1)) {
                        return None;
                    }
                    let meaning = otry!(decode(bytes));
                    Some(Codepoint {
                        bytes: bytes,
                        rewind: 0,
                        meaning: meaning,
                    })
                } else {
                    Some(Codepoint {
                        bytes: unsafe_slice(buf, idx, avail),
                        rewind: 0,
                        meaning: Meaning::Prefix(n - avail),
                    })
                }
            },
            Byte::Cont => {
                let mut start = idx;
                let mut checked = 0;
                loop {
                    if start == 0 {
                        // Whoops, fell off the beginning.
                        return Some(Codepoint {
                            bytes: unsafe_slice(buf, 0, idx + 1),
                            rewind: idx,
                            meaning: Meaning::Suffix,
                        });
                    }

                    start -= 1;
                    checked += 1;
                    match otry!(Byte::classify(*buf.get_unchecked(start))) {
                        Byte::Cont => (),
                        Byte::Start(n) => {
                            let avail = buf.len() - start;
                            if avail >= n {
                                let bytes = unsafe_slice(buf, start, n);
                                if checked < n {
                                    if !all_cont(unsafe_slice(bytes, checked, n-checked)) {
                                        return None;
                                    }
                                }
                                let meaning = otry!(decode(bytes));
                                return Some(Codepoint {
                                    bytes: bytes,
                                    rewind: idx - start,
                                    meaning: meaning,
                                });
                            } else {
                                return Some(Codepoint {
                                    bytes: unsafe_slice(buf, start, avail),
                                    rewind: idx - start,
                                    meaning: Meaning::Prefix(n - avail),
                                });
                            }
                        }
                        _ => return None,
                    }

                    if idx - start >= 3 {
                        // We looked at 3 bytes before a continuation byte
                        // and didn't find a start byte.
                        return None;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test;
