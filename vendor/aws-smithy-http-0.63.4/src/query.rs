/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Utilities for writing Smithy values into a query string.
//!
//! Formatting values into the query string as specified in
//! [httpQuery](https://smithy.io/2.0/spec/http-bindings.html#httpquery-trait)

use crate::urlencode::BASE_SET;
use aws_smithy_types::date_time::{DateTimeFormatError, Format};
use aws_smithy_types::DateTime;
use percent_encoding::utf8_percent_encode;

/// Format a given string as a query string.
pub fn fmt_string<T: AsRef<str>>(t: T) -> String {
    utf8_percent_encode(t.as_ref(), BASE_SET).to_string()
}

/// Format a given [`DateTime`] as a query string.
pub fn fmt_timestamp(t: &DateTime, format: Format) -> Result<String, DateTimeFormatError> {
    Ok(fmt_string(t.fmt(format)?))
}

/// Simple abstraction to enable appending params to a string as query params.
///
/// ```rust
/// use aws_smithy_http::query::Writer;
/// let mut s = String::from("www.example.com");
/// let mut q = Writer::new(&mut s);
/// q.push_kv("key", "value");
/// q.push_v("another_value");
/// assert_eq!(s, "www.example.com?key=value&another_value");
/// ```
#[allow(missing_debug_implementations)]
pub struct Writer<'a> {
    out: &'a mut String,
    prefix: char,
}

impl<'a> Writer<'a> {
    /// Create a new query string writer.
    pub fn new(out: &'a mut String) -> Self {
        Writer { out, prefix: '?' }
    }

    /// Add a new key and value pair to this writer.
    pub fn push_kv(&mut self, k: &str, v: &str) {
        self.out.push(self.prefix);
        self.out.push_str(k);
        self.out.push('=');
        self.out.push_str(v);
        self.prefix = '&';
    }

    /// Add a new value (which is its own key) to this writer.
    pub fn push_v(&mut self, v: &str) {
        self.out.push(self.prefix);
        self.out.push_str(v);
        self.prefix = '&';
    }
}

#[cfg(test)]
mod test {
    use crate::query::{fmt_string, Writer};
    use http_1x::Uri;
    use proptest::proptest;

    #[test]
    fn url_encode() {
        assert_eq!(fmt_string("y̆").as_str(), "y%CC%86");
        assert_eq!(fmt_string(" ").as_str(), "%20");
        assert_eq!(fmt_string("foo/baz%20").as_str(), "foo%2Fbaz%2520");
        assert_eq!(fmt_string("&=").as_str(), "%26%3D");
        assert_eq!(fmt_string("🐱").as_str(), "%F0%9F%90%B1");
        // `:` needs to be encoded, but only for AWS services
        assert_eq!(fmt_string("a:b"), "a%3Ab")
    }

    #[test]
    fn writer_sets_prefix_properly() {
        let mut out = String::new();
        let mut writer = Writer::new(&mut out);
        writer.push_v("a");
        writer.push_kv("b", "c");
        assert_eq!(out, "?a&b=c");
    }

    proptest! {
        #[test]
        fn test_encode_request(s: String) {
            let _: Uri = format!("http://host.example.com/?{}", fmt_string(s)).parse().expect("all strings should be encoded properly");
        }
    }
}
