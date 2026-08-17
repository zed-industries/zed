use std::fmt;
use std::ops::{Bound, RangeBounds};

use {util, HeaderValue};

/// Content-Range, described in [RFC7233](https://tools.ietf.org/html/rfc7233#section-4.2)
///
/// # ABNF
///
/// ```text
/// Content-Range       = byte-content-range
///                     / other-content-range
///
/// byte-content-range  = bytes-unit SP
///                       ( byte-range-resp / unsatisfied-range )
///
/// byte-range-resp     = byte-range "/" ( complete-length / "*" )
/// byte-range          = first-byte-pos "-" last-byte-pos
/// unsatisfied-range   = "*/" complete-length
///
/// complete-length     = 1*DIGIT
///
/// other-content-range = other-range-unit SP other-range-resp
/// other-range-resp    = *CHAR
/// ```
///
/// # Example
///
/// ```
/// # extern crate headers;
/// use headers::ContentRange;
///
/// // 100 bytes (included byte 199), with a full length of 3,400
/// let cr = ContentRange::bytes(100..200, 3400).unwrap();
/// ```
//NOTE: only supporting bytes-content-range, YAGNI the extension
#[derive(Clone, Debug, PartialEq)]
pub struct ContentRange {
    /// First and last bytes of the range, omitted if request could not be
    /// satisfied
    range: Option<(u64, u64)>,

    /// Total length of the instance, can be omitted if unknown
    complete_length: Option<u64>,
}

error_type!(InvalidContentRange);

impl ContentRange {
    /// Construct a new `Content-Range: bytes ..` header.
    pub fn bytes(
        range: impl RangeBounds<u64>,
        complete_length: impl Into<Option<u64>>,
    ) -> Result<ContentRange, InvalidContentRange> {
        let complete_length = complete_length.into();

        let start = match range.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&e) => e,
            Bound::Excluded(&e) => e - 1,
            Bound::Unbounded => match complete_length {
                Some(max) => max - 1,
                None => return Err(InvalidContentRange { _inner: () }),
            },
        };

        Ok(ContentRange {
            range: Some((start, end)),
            complete_length,
        })
    }

    /// Create a new `ContentRange` stating the range could not be satisfied.
    ///
    /// The passed argument is the complete length of the entity.
    pub fn unsatisfied_bytes(complete_length: u64) -> Self {
        ContentRange {
            range: None,
            complete_length: Some(complete_length),
        }
    }

    /// Get the byte range if satisified.
    ///
    /// Note that these byte ranges are inclusive on both ends.
    pub fn bytes_range(&self) -> Option<(u64, u64)> {
        self.range
    }

    /// Get the bytes complete length if available.
    pub fn bytes_len(&self) -> Option<u64> {
        self.complete_length
    }
}

impl ::Header for ContentRange {
    fn name() -> &'static ::HeaderName {
        &::http::header::CONTENT_RANGE
    }

    fn decode<'i, I: Iterator<Item = &'i HeaderValue>>(values: &mut I) -> Result<Self, ::Error> {
        values
            .next()
            .and_then(|v| v.to_str().ok())
            .and_then(|s| split_in_two(s, ' '))
            .and_then(|(unit, spec)| {
                if unit != "bytes" {
                    // For now, this only supports bytes-content-range. nani?
                    return None;
                }

                let (range, complete_length) = split_in_two(spec, '/')?;

                let complete_length = if complete_length == "*" {
                    None
                } else {
                    Some(complete_length.parse().ok()?)
                };

                let range = if range == "*" {
                    None
                } else {
                    let (first_byte, last_byte) = split_in_two(range, '-')?;
                    let first_byte = first_byte.parse().ok()?;
                    let last_byte = last_byte.parse().ok()?;
                    if last_byte < first_byte {
                        return None;
                    }
                    Some((first_byte, last_byte))
                };

                Some(ContentRange {
                    range,
                    complete_length,
                })
            })
            .ok_or_else(::Error::invalid)
    }

    fn encode<E: Extend<::HeaderValue>>(&self, values: &mut E) {
        struct Adapter<'a>(&'a ContentRange);

        impl<'a> fmt::Display for Adapter<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("bytes ")?;

                if let Some((first_byte, last_byte)) = self.0.range {
                    write!(f, "{}-{}", first_byte, last_byte)?;
                } else {
                    f.write_str("*")?;
                }

                f.write_str("/")?;

                if let Some(v) = self.0.complete_length {
                    write!(f, "{}", v)
                } else {
                    f.write_str("*")
                }
            }
        }

        values.extend(::std::iter::once(util::fmt(Adapter(self))));
    }
}

fn split_in_two(s: &str, separator: char) -> Option<(&str, &str)> {
    let mut iter = s.splitn(2, separator);
    match (iter.next(), iter.next()) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

/*
test_header!(test_bytes,
    vec![b"bytes 0-499/500"],
    Some(ContentRange(ContentRangeSpec::Bytes {
        range: Some((0, 499)),
        complete_length: Some(500)
    })));

test_header!(test_bytes_unknown_len,
    vec![b"bytes 0-499/*"],
    Some(ContentRange(ContentRangeSpec::Bytes {
        range: Some((0, 499)),
        complete_length: None
    })));

test_header!(test_bytes_unknown_range,
    vec![b"bytes */
500"],
            Some(ContentRange(ContentRangeSpec::Bytes {
                range: None,
                complete_length: Some(500)
            })));

        test_header!(test_unregistered,
            vec![b"seconds 1-2"],
            Some(ContentRange(ContentRangeSpec::Unregistered {
                unit: "seconds".to_owned(),
                resp: "1-2".to_owned()
            })));

        test_header!(test_no_len,
            vec![b"bytes 0-499"],
            None::<ContentRange>);

        test_header!(test_only_unit,
            vec![b"bytes"],
            None::<ContentRange>);

        test_header!(test_end_less_than_start,
            vec![b"bytes 499-0/500"],
            None::<ContentRange>);

        test_header!(test_blank,
            vec![b""],
            None::<ContentRange>);

        test_header!(test_bytes_many_spaces,
            vec![b"bytes 1-2/500 3"],
            None::<ContentRange>);

        test_header!(test_bytes_many_slashes,
            vec![b"bytes 1-2/500/600"],
            None::<ContentRange>);

        test_header!(test_bytes_many_dashes,
            vec![b"bytes 1-2-3/500"],
            None::<ContentRange>);
*/
