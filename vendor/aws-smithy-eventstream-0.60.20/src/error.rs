/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_types::DateTime;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub(crate) enum ErrorKind {
    HeadersTooLong,
    HeaderValueTooLong,
    InvalidHeaderNameLength,
    InvalidHeaderValue,
    InvalidHeaderValueType(u8),
    InvalidHeadersLength,
    InvalidMessageLength,
    InvalidUtf8String,
    MessageChecksumMismatch(u32, u32),
    MessageTooLong,
    PayloadTooLong,
    PreludeChecksumMismatch(u32, u32),
    TimestampValueTooLarge(DateTime),
    Marshalling(String),
    Unmarshalling(String),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    // Used in tests to match on the underlying error kind
    #[cfg(test)]
    pub(crate) fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Create an `Error` for failure to marshall a message from a Smithy shape
    pub fn marshalling(message: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::Marshalling(message.into()),
        }
    }

    /// Create an `Error` for failure to unmarshall a message into a Smithy shape
    pub fn unmarshalling(message: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::Unmarshalling(message.into()),
        }
    }

    /// Returns true if the error is one generated during serialization
    pub fn is_invalid_message(&self) -> bool {
        use ErrorKind::*;
        matches!(
            self.kind,
            HeadersTooLong
                | PayloadTooLong
                | MessageTooLong
                | InvalidHeaderNameLength
                | TimestampValueTooLarge(_)
                | Marshalling(_)
        )
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        match &self.kind {
            HeadersTooLong => write!(f, "headers too long to fit in event stream frame"),
            HeaderValueTooLong => write!(f, "header value too long to fit in event stream frame"),
            InvalidHeaderNameLength => write!(f, "invalid header name length"),
            InvalidHeaderValue => write!(f, "invalid header value"),
            InvalidHeaderValueType(val) => write!(f, "invalid header value type: {val}"),
            InvalidHeadersLength => write!(f, "invalid headers length"),
            InvalidMessageLength => write!(f, "invalid message length"),
            InvalidUtf8String => write!(f, "encountered invalid UTF-8 string"),
            MessageChecksumMismatch(expected, actual) => write!(
                f,
                "message checksum 0x{actual:X} didn't match expected checksum 0x{expected:X}"
            ),
            MessageTooLong => write!(f, "message too long to fit in event stream frame"),
            PayloadTooLong => write!(f, "message payload too long to fit in event stream frame"),
            PreludeChecksumMismatch(expected, actual) => write!(
                f,
                "prelude checksum 0x{actual:X} didn't match expected checksum 0x{expected:X}"
            ),
            TimestampValueTooLarge(time) => write!(
                f,
                "timestamp value {time:?} is too large to fit into an i64"
            ),
            Marshalling(error) => write!(f, "failed to marshall message: {error}"),
            Unmarshalling(error) => write!(f, "failed to unmarshall message: {error}"),
        }
    }
}
