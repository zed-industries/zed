/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Utilities for parsing information from headers

use aws_smithy_types::date_time::Format;
use aws_smithy_types::primitive::Parse;
use aws_smithy_types::DateTime;
use http_1x::header::{HeaderMap, HeaderName, HeaderValue};
use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// An error was encountered while parsing a header
#[derive(Debug)]
pub struct ParseError {
    message: Cow<'static, str>,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl ParseError {
    /// Create a new parse error with the given `message`
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// Attach a source to this error.
    pub fn with_source(self, source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
        Self {
            source: Some(source.into()),
            ..self
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "output failed to parse in headers: {}", self.message)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|err| err.as_ref() as _)
    }
}

/// Read all the dates from the header map at `key` according the `format`
///
/// This is separate from `read_many` below because we need to invoke `DateTime::read` to take advantage
/// of comma-aware parsing
pub fn many_dates<'a>(
    values: impl Iterator<Item = &'a str>,
    format: Format,
) -> Result<Vec<DateTime>, ParseError> {
    let mut out = vec![];
    for header in values {
        let mut header = header;
        while !header.is_empty() {
            let (v, next) = DateTime::read(header, format, ',').map_err(|err| {
                ParseError::new(format!("header could not be parsed as date: {err}"))
            })?;
            out.push(v);
            header = next;
        }
    }
    Ok(out)
}

/// Returns an iterator over pairs where the first element is the unprefixed header name that
/// starts with the input `key` prefix, and the second element is the full header name.
pub fn headers_for_prefix<'a>(
    header_names: impl Iterator<Item = &'a str>,
    key: &'a str,
) -> impl Iterator<Item = (&'a str, &'a str)> {
    let lower_key = key.to_ascii_lowercase();
    header_names
        .filter(move |k| k.starts_with(&lower_key))
        .map(move |k| (&k[key.len()..], k))
}

/// Convert a `HeaderValue` into a `Vec<T>` where `T: FromStr`
pub fn read_many_from_str<'a, T: FromStr>(
    values: impl Iterator<Item = &'a str>,
) -> Result<Vec<T>, ParseError>
where
    T::Err: Error + Send + Sync + 'static,
{
    read_many(values, |v: &str| {
        v.parse().map_err(|err| {
            ParseError::new("failed during `FromString` conversion").with_source(err)
        })
    })
}

/// Convert a `HeaderValue` into a `Vec<T>` where `T: Parse`
pub fn read_many_primitive<'a, T: Parse>(
    values: impl Iterator<Item = &'a str>,
) -> Result<Vec<T>, ParseError> {
    read_many(values, |v: &str| {
        T::parse_smithy_primitive(v)
            .map_err(|err| ParseError::new("failed reading a list of primitives").with_source(err))
    })
}

/// Read many comma / header delimited values from HTTP headers for `FromStr` types
fn read_many<'a, T>(
    values: impl Iterator<Item = &'a str>,
    f: impl Fn(&str) -> Result<T, ParseError>,
) -> Result<Vec<T>, ParseError> {
    let mut out = vec![];
    for header in values {
        let mut header = header.as_bytes();
        while !header.is_empty() {
            let (v, next) = read_one(header, &f)?;
            out.push(v);
            header = next;
        }
    }
    Ok(out)
}

/// Read exactly one or none from a headers iterator
///
/// This function does not perform comma splitting like `read_many`
pub fn one_or_none<'a, T: FromStr>(
    mut values: impl Iterator<Item = &'a str>,
) -> Result<Option<T>, ParseError>
where
    T::Err: Error + Send + Sync + 'static,
{
    let first = match values.next() {
        Some(v) => v,
        None => return Ok(None),
    };
    match values.next() {
        None => T::from_str(first.trim())
            .map_err(|err| ParseError::new("failed to parse string").with_source(err))
            .map(Some),
        Some(_) => Err(ParseError::new(
            "expected a single value but found multiple",
        )),
    }
}

/// Given an HTTP request, set a request header if that header was not already set.
pub fn set_request_header_if_absent<V>(
    request: http_1x::request::Builder,
    key: HeaderName,
    value: V,
) -> http_1x::request::Builder
where
    HeaderValue: TryFrom<V>,
    <HeaderValue as TryFrom<V>>::Error: Into<http_1x::Error>,
{
    if !request
        .headers_ref()
        .map(|map| map.contains_key(&key))
        .unwrap_or(false)
    {
        request.header(key, value)
    } else {
        request
    }
}

/// Given an HTTP response, set a response header if that header was not already set.
pub fn set_response_header_if_absent<V>(
    response: http_1x::response::Builder,
    key: HeaderName,
    value: V,
) -> http_1x::response::Builder
where
    HeaderValue: TryFrom<V>,
    <HeaderValue as TryFrom<V>>::Error: Into<http_1x::Error>,
{
    if !response
        .headers_ref()
        .map(|map| map.contains_key(&key))
        .unwrap_or(false)
    {
        response.header(key, value)
    } else {
        response
    }
}

/// Functions for parsing multiple comma-delimited header values out of a
/// single header. This parsing adheres to
/// [RFC-7230's specification of header values](https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6).
mod parse_multi_header {
    use super::ParseError;
    use std::borrow::Cow;

    fn trim(s: Cow<'_, str>) -> Cow<'_, str> {
        match s {
            Cow::Owned(s) => Cow::Owned(s.trim().into()),
            Cow::Borrowed(s) => Cow::Borrowed(s.trim()),
        }
    }

    fn replace<'a>(value: Cow<'a, str>, pattern: &str, replacement: &str) -> Cow<'a, str> {
        if value.contains(pattern) {
            Cow::Owned(value.replace(pattern, replacement))
        } else {
            value
        }
    }

    /// Reads a single value out of the given input, and returns a tuple containing
    /// the parsed value and the remainder of the slice that can be used to parse
    /// more values.
    pub(crate) fn read_value(input: &[u8]) -> Result<(Cow<'_, str>, &[u8]), ParseError> {
        for (index, &byte) in input.iter().enumerate() {
            let current_slice = &input[index..];
            match byte {
                b' ' | b'\t' => { /* skip whitespace */ }
                b'"' => return read_quoted_value(&current_slice[1..]),
                _ => {
                    let (value, rest) = read_unquoted_value(current_slice)?;
                    return Ok((trim(value), rest));
                }
            }
        }

        // We only end up here if the entire header value was whitespace or empty
        Ok((Cow::Borrowed(""), &[]))
    }

    fn read_unquoted_value(input: &[u8]) -> Result<(Cow<'_, str>, &[u8]), ParseError> {
        let next_delim = input.iter().position(|&b| b == b',').unwrap_or(input.len());
        let (first, next) = input.split_at(next_delim);
        let first = std::str::from_utf8(first)
            .map_err(|_| ParseError::new("header was not valid utf-8"))?;
        Ok((Cow::Borrowed(first), then_comma(next).unwrap()))
    }

    /// Reads a header value that is surrounded by quotation marks and may have escaped
    /// quotes inside of it.
    fn read_quoted_value(input: &[u8]) -> Result<(Cow<'_, str>, &[u8]), ParseError> {
        for index in 0..input.len() {
            match input[index] {
                b'"' if index == 0 || input[index - 1] != b'\\' => {
                    let mut inner = Cow::Borrowed(
                        std::str::from_utf8(&input[0..index])
                            .map_err(|_| ParseError::new("header was not valid utf-8"))?,
                    );
                    inner = replace(inner, "\\\"", "\"");
                    inner = replace(inner, "\\\\", "\\");
                    let rest = then_comma(&input[(index + 1)..])?;
                    return Ok((inner, rest));
                }
                _ => {}
            }
        }
        Err(ParseError::new(
            "header value had quoted value without end quote",
        ))
    }

    fn then_comma(s: &[u8]) -> Result<&[u8], ParseError> {
        if s.is_empty() {
            Ok(s)
        } else if s.starts_with(b",") {
            Ok(&s[1..])
        } else {
            Err(ParseError::new("expected delimiter `,`"))
        }
    }
}

/// Read one comma delimited value for `FromStr` types
fn read_one<'a, T>(
    s: &'a [u8],
    f: &impl Fn(&str) -> Result<T, ParseError>,
) -> Result<(T, &'a [u8]), ParseError> {
    let (value, rest) = parse_multi_header::read_value(s)?;
    Ok((f(&value)?, rest))
}

/// Conditionally quotes and escapes a header value if the header value contains a comma or quote.
pub fn quote_header_value<'a>(value: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
    let value = value.into();
    if value.trim().len() != value.len()
        || value.contains('"')
        || value.contains(',')
        || value.contains('(')
        || value.contains(')')
    {
        Cow::Owned(format!(
            "\"{}\"",
            value.replace('\\', "\\\\").replace('"', "\\\"")
        ))
    } else {
        value
    }
}

/// Given two http-02x [`HeaderMap`]s, merge them together and return the merged `HeaderMap`. If the
/// two `HeaderMap`s share any keys, values from the right `HeaderMap` be appended to the left `HeaderMap`.
pub fn append_merge_header_maps(
    mut lhs: HeaderMap<HeaderValue>,
    rhs: HeaderMap<HeaderValue>,
) -> HeaderMap<HeaderValue> {
    let mut last_header_name_seen = None;
    for (header_name, header_value) in rhs.into_iter() {
        // For each yielded item that has None provided for the `HeaderName`,
        // then the associated header name is the same as that of the previously
        // yielded item. The first yielded item will have `HeaderName` set.
        // https://docs.rs/http/latest/http/header/struct.HeaderMap.html#method.into_iter-2
        match (&mut last_header_name_seen, header_name) {
            (_, Some(header_name)) => {
                lhs.append(header_name.clone(), header_value);
                last_header_name_seen = Some(header_name);
            }
            (Some(header_name), None) => {
                lhs.append(header_name.clone(), header_value);
            }
            (None, None) => unreachable!(),
        };
    }

    lhs
}

/// Given two http-1x [`HeaderMap`]s, merge them together and return the merged `HeaderMap`. If the
/// two `HeaderMap`s share any keys, values from the right `HeaderMap` be appended to the left `HeaderMap`.
pub fn append_merge_header_maps_http_1x(
    mut lhs: http_1x::HeaderMap<http_1x::HeaderValue>,
    rhs: http_1x::HeaderMap<http_1x::HeaderValue>,
) -> http_1x::HeaderMap<http_1x::HeaderValue> {
    let mut last_header_name_seen = None;
    for (header_name, header_value) in rhs.into_iter() {
        // For each yielded item that has None provided for the `HeaderName`,
        // then the associated header name is the same as that of the previously
        // yielded item. The first yielded item will have `HeaderName` set.
        // https://docs.rs/http/latest/http/header/struct.HeaderMap.html#method.into_iter-2
        match (&mut last_header_name_seen, header_name) {
            (_, Some(header_name)) => {
                lhs.append(header_name.clone(), header_value);
                last_header_name_seen = Some(header_name);
            }
            (Some(header_name), None) => {
                lhs.append(header_name.clone(), header_value);
            }
            (None, None) => unreachable!(),
        };
    }

    lhs
}

#[cfg(test)]
mod test {
    use super::quote_header_value;
    use crate::header::{
        append_merge_header_maps, headers_for_prefix, many_dates, read_many_from_str,
        read_many_primitive, set_request_header_if_absent, set_response_header_if_absent,
        ParseError,
    };
    use aws_smithy_runtime_api::http::Request;
    use aws_smithy_types::error::display::DisplayErrorContext;
    use aws_smithy_types::{date_time::Format, DateTime};
    use http_1x::header::{HeaderMap, HeaderName, HeaderValue};
    use std::collections::HashMap;

    #[test]
    fn put_on_request_if_absent() {
        let builder = http_1x::Request::builder().header("foo", "bar");
        let builder = set_request_header_if_absent(builder, HeaderName::from_static("foo"), "baz");
        let builder =
            set_request_header_if_absent(builder, HeaderName::from_static("other"), "value");
        let req = builder.body(()).expect("valid request");
        assert_eq!(
            req.headers().get_all("foo").iter().collect::<Vec<_>>(),
            vec!["bar"]
        );
        assert_eq!(
            req.headers().get_all("other").iter().collect::<Vec<_>>(),
            vec!["value"]
        );
    }

    #[test]
    fn put_on_response_if_absent() {
        let builder = http_1x::Response::builder().header("foo", "bar");
        let builder = set_response_header_if_absent(builder, HeaderName::from_static("foo"), "baz");
        let builder =
            set_response_header_if_absent(builder, HeaderName::from_static("other"), "value");
        let response = builder.body(()).expect("valid response");
        assert_eq!(
            response.headers().get_all("foo").iter().collect::<Vec<_>>(),
            vec!["bar"]
        );
        assert_eq!(
            response
                .headers()
                .get_all("other")
                .iter()
                .collect::<Vec<_>>(),
            vec!["value"]
        );
    }

    #[test]
    fn parse_floats() {
        let test_request = http_1x::Request::builder()
            .header("X-Float-Multi", "0.0,Infinity,-Infinity,5555.5")
            .header("X-Float-Error", "notafloat")
            .body(())
            .unwrap();
        assert_eq!(
            read_many_primitive::<f32>(
                test_request
                    .headers()
                    .get_all("X-Float-Multi")
                    .iter()
                    .map(|v| v.to_str().unwrap())
            )
            .expect("valid"),
            vec![0.0, f32::INFINITY, f32::NEG_INFINITY, 5555.5]
        );
        let message = format!(
            "{}",
            DisplayErrorContext(
                read_many_primitive::<f32>(
                    test_request
                        .headers()
                        .get_all("X-Float-Error")
                        .iter()
                        .map(|v| v.to_str().unwrap())
                )
                .expect_err("invalid")
            )
        );
        let expected = "output failed to parse in headers: failed reading a list of primitives: failed to parse input as f32";
        assert!(
            message.starts_with(expected),
            "expected '{message}' to start with '{expected}'"
        );
    }

    #[test]
    fn test_many_dates() {
        let test_request = http_1x::Request::builder()
            .header("Empty", "")
            .header("SingleHttpDate", "Wed, 21 Oct 2015 07:28:00 GMT")
            .header(
                "MultipleHttpDates",
                "Wed, 21 Oct 2015 07:28:00 GMT,Thu, 22 Oct 2015 07:28:00 GMT",
            )
            .header("SingleEpochSeconds", "1234.5678")
            .header("MultipleEpochSeconds", "1234.5678,9012.3456")
            .body(())
            .unwrap();
        let read = |name: &str, format: Format| {
            many_dates(
                test_request
                    .headers()
                    .get_all(name)
                    .iter()
                    .map(|v| v.to_str().unwrap()),
                format,
            )
        };
        let read_valid = |name: &str, format: Format| read(name, format).expect("valid");
        assert_eq!(
            read_valid("Empty", Format::DateTime),
            Vec::<DateTime>::new()
        );
        assert_eq!(
            read_valid("SingleHttpDate", Format::HttpDate),
            vec![DateTime::from_secs_and_nanos(1445412480, 0)]
        );
        assert_eq!(
            read_valid("MultipleHttpDates", Format::HttpDate),
            vec![
                DateTime::from_secs_and_nanos(1445412480, 0),
                DateTime::from_secs_and_nanos(1445498880, 0)
            ]
        );
        assert_eq!(
            read_valid("SingleEpochSeconds", Format::EpochSeconds),
            vec![DateTime::from_secs_and_nanos(1234, 567_800_000)]
        );
        assert_eq!(
            read_valid("MultipleEpochSeconds", Format::EpochSeconds),
            vec![
                DateTime::from_secs_and_nanos(1234, 567_800_000),
                DateTime::from_secs_and_nanos(9012, 345_600_000)
            ]
        );
    }

    #[test]
    fn read_many_strings() {
        let test_request = http_1x::Request::builder()
            .header("Empty", "")
            .header("Foo", "  foo")
            .header("FooTrailing", "foo   ")
            .header("FooInQuotes", "\"  foo  \"")
            .header("CommaInQuotes", "\"foo,bar\",baz")
            .header("CommaInQuotesTrailing", "\"foo,bar\",baz  ")
            .header("QuoteInQuotes", "\"foo\\\",bar\",\"\\\"asdf\\\"\",baz")
            .header(
                "QuoteInQuotesWithSpaces",
                "\"foo\\\",bar\", \"\\\"asdf\\\"\", baz",
            )
            .header("JunkFollowingQuotes", "\"\\\"asdf\\\"\"baz")
            .header("EmptyQuotes", "\"\",baz")
            .header("EscapedSlashesInQuotes", "foo, \"(foo\\\\bar)\"")
            .body(())
            .unwrap();
        let read = |name: &str| {
            read_many_from_str::<String>(
                test_request
                    .headers()
                    .get_all(name)
                    .iter()
                    .map(|v| v.to_str().unwrap()),
            )
        };
        let read_valid = |name: &str| read(name).expect("valid");
        assert_eq!(read_valid("Empty"), Vec::<String>::new());
        assert_eq!(read_valid("Foo"), vec!["foo"]);
        assert_eq!(read_valid("FooTrailing"), vec!["foo"]);
        assert_eq!(read_valid("FooInQuotes"), vec!["  foo  "]);
        assert_eq!(read_valid("CommaInQuotes"), vec!["foo,bar", "baz"]);
        assert_eq!(read_valid("CommaInQuotesTrailing"), vec!["foo,bar", "baz"]);
        assert_eq!(
            read_valid("QuoteInQuotes"),
            vec!["foo\",bar", "\"asdf\"", "baz"]
        );
        assert_eq!(
            read_valid("QuoteInQuotesWithSpaces"),
            vec!["foo\",bar", "\"asdf\"", "baz"]
        );
        assert!(read("JunkFollowingQuotes").is_err());
        assert_eq!(read_valid("EmptyQuotes"), vec!["", "baz"]);
        assert_eq!(
            read_valid("EscapedSlashesInQuotes"),
            vec!["foo", "(foo\\bar)"]
        );
    }

    #[test]
    fn read_many_bools() {
        let test_request = http_1x::Request::builder()
            .header("X-Bool-Multi", "true,false")
            .header("X-Bool-Multi", "true")
            .header("X-Bool", "true")
            .header("X-Bool-Invalid", "truth,falsy")
            .header("X-Bool-Single", "true,false,true,true")
            .header("X-Bool-Quoted", "true,\"false\",true,true")
            .body(())
            .unwrap();
        assert_eq!(
            read_many_primitive::<bool>(
                test_request
                    .headers()
                    .get_all("X-Bool-Multi")
                    .iter()
                    .map(|v| v.to_str().unwrap())
            )
            .expect("valid"),
            vec![true, false, true]
        );

        assert_eq!(
            read_many_primitive::<bool>(
                test_request
                    .headers()
                    .get_all("X-Bool")
                    .iter()
                    .map(|v| v.to_str().unwrap())
            )
            .unwrap(),
            vec![true]
        );
        assert_eq!(
            read_many_primitive::<bool>(
                test_request
                    .headers()
                    .get_all("X-Bool-Single")
                    .iter()
                    .map(|v| v.to_str().unwrap())
            )
            .unwrap(),
            vec![true, false, true, true]
        );
        assert_eq!(
            read_many_primitive::<bool>(
                test_request
                    .headers()
                    .get_all("X-Bool-Quoted")
                    .iter()
                    .map(|v| v.to_str().unwrap())
            )
            .unwrap(),
            vec![true, false, true, true]
        );
        read_many_primitive::<bool>(
            test_request
                .headers()
                .get_all("X-Bool-Invalid")
                .iter()
                .map(|v| v.to_str().unwrap()),
        )
        .expect_err("invalid");
    }

    #[test]
    fn check_read_many_i16() {
        let test_request = http_1x::Request::builder()
            .header("X-Multi", "123,456")
            .header("X-Multi", "789")
            .header("X-Num", "777")
            .header("X-Num-Invalid", "12ef3")
            .header("X-Num-Single", "1,2,3,-4,5")
            .header("X-Num-Quoted", "1, \"2\",3,\"-4\",5")
            .body(())
            .unwrap();
        assert_eq!(
            read_many_primitive::<i16>(
                test_request
                    .headers()
                    .get_all("X-Multi")
                    .iter()
                    .map(|v| v.to_str().unwrap())
            )
            .expect("valid"),
            vec![123, 456, 789]
        );

        assert_eq!(
            read_many_primitive::<i16>(
                test_request
                    .headers()
                    .get_all("X-Num")
                    .iter()
                    .map(|v| v.to_str().unwrap())
            )
            .unwrap(),
            vec![777]
        );
        assert_eq!(
            read_many_primitive::<i16>(
                test_request
                    .headers()
                    .get_all("X-Num-Single")
                    .iter()
                    .map(|v| v.to_str().unwrap())
            )
            .unwrap(),
            vec![1, 2, 3, -4, 5]
        );
        assert_eq!(
            read_many_primitive::<i16>(
                test_request
                    .headers()
                    .get_all("X-Num-Quoted")
                    .iter()
                    .map(|v| v.to_str().unwrap())
            )
            .unwrap(),
            vec![1, 2, 3, -4, 5]
        );
        read_many_primitive::<i16>(
            test_request
                .headers()
                .get_all("X-Num-Invalid")
                .iter()
                .map(|v| v.to_str().unwrap()),
        )
        .expect_err("invalid");
    }

    #[test]
    fn test_prefix_headers() {
        let test_request = Request::try_from(
            http_1x::Request::builder()
                .header("X-Prefix-A", "123,456")
                .header("X-Prefix-B", "789")
                .header("X-Prefix-C", "777")
                .header("X-Prefix-C", "777")
                .body(())
                .unwrap(),
        )
        .unwrap();
        let resp: Result<HashMap<String, Vec<i16>>, ParseError> =
            headers_for_prefix(test_request.headers().iter().map(|h| h.0), "X-Prefix-")
                .map(|(key, header_name)| {
                    let values = test_request.headers().get_all(header_name);
                    read_many_primitive(values).map(|v| (key.to_string(), v))
                })
                .collect();
        let resp = resp.expect("valid");
        assert_eq!(resp.get("a"), Some(&vec![123_i16, 456_i16]));
    }

    #[test]
    fn test_quote_header_value() {
        assert_eq!("", &quote_header_value(""));
        assert_eq!("foo", &quote_header_value("foo"));
        assert_eq!("\"  foo\"", &quote_header_value("  foo"));
        assert_eq!("foo bar", &quote_header_value("foo bar"));
        assert_eq!("\"foo,bar\"", &quote_header_value("foo,bar"));
        assert_eq!("\",\"", &quote_header_value(","));
        assert_eq!("\"\\\"foo\\\"\"", &quote_header_value("\"foo\""));
        assert_eq!("\"\\\"f\\\\oo\\\"\"", &quote_header_value("\"f\\oo\""));
        assert_eq!("\"(\"", &quote_header_value("("));
        assert_eq!("\")\"", &quote_header_value(")"));
    }

    #[test]
    fn test_append_merge_header_maps_with_shared_key() {
        let header_name = HeaderName::from_static("some_key");
        let left_header_value = HeaderValue::from_static("lhs value");
        let right_header_value = HeaderValue::from_static("rhs value");

        let mut left_hand_side_headers = HeaderMap::new();
        left_hand_side_headers.insert(header_name.clone(), left_header_value.clone());

        let mut right_hand_side_headers = HeaderMap::new();
        right_hand_side_headers.insert(header_name.clone(), right_header_value.clone());

        let merged_header_map =
            append_merge_header_maps(left_hand_side_headers, right_hand_side_headers);
        let actual_merged_values: Vec<_> =
            merged_header_map.get_all(header_name).into_iter().collect();

        let expected_merged_values = vec![left_header_value, right_header_value];

        assert_eq!(actual_merged_values, expected_merged_values);
    }

    #[test]
    fn test_append_merge_header_maps_with_multiple_values_in_left_hand_map() {
        let header_name = HeaderName::from_static("some_key");
        let left_header_value_1 = HeaderValue::from_static("lhs value 1");
        let left_header_value_2 = HeaderValue::from_static("lhs_value 2");
        let right_header_value = HeaderValue::from_static("rhs value");

        let mut left_hand_side_headers = HeaderMap::new();
        left_hand_side_headers.insert(header_name.clone(), left_header_value_1.clone());
        left_hand_side_headers.append(header_name.clone(), left_header_value_2.clone());

        let mut right_hand_side_headers = HeaderMap::new();
        right_hand_side_headers.insert(header_name.clone(), right_header_value.clone());

        let merged_header_map =
            append_merge_header_maps(left_hand_side_headers, right_hand_side_headers);
        let actual_merged_values: Vec<_> =
            merged_header_map.get_all(header_name).into_iter().collect();

        let expected_merged_values =
            vec![left_header_value_1, left_header_value_2, right_header_value];

        assert_eq!(actual_merged_values, expected_merged_values);
    }

    #[test]
    fn test_append_merge_header_maps_with_empty_left_hand_map() {
        let header_name = HeaderName::from_static("some_key");
        let right_header_value_1 = HeaderValue::from_static("rhs value 1");
        let right_header_value_2 = HeaderValue::from_static("rhs_value 2");

        let left_hand_side_headers = HeaderMap::new();

        let mut right_hand_side_headers = HeaderMap::new();
        right_hand_side_headers.insert(header_name.clone(), right_header_value_1.clone());
        right_hand_side_headers.append(header_name.clone(), right_header_value_2.clone());

        let merged_header_map =
            append_merge_header_maps(left_hand_side_headers, right_hand_side_headers);
        let actual_merged_values: Vec<_> =
            merged_header_map.get_all(header_name).into_iter().collect();

        let expected_merged_values = vec![right_header_value_1, right_header_value_2];

        assert_eq!(actual_merged_values, expected_merged_values);
    }
}
