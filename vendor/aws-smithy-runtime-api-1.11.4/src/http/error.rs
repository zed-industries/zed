/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Error types for HTTP requests/responses.

use crate::box_error::BoxError;
use http_02x::header::{InvalidHeaderName, InvalidHeaderValue};
use http_02x::uri::InvalidUri;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::Utf8Error;

#[derive(Debug)]
/// An error occurred constructing an Http Request.
///
/// This is normally due to configuration issues, internal SDK bugs, or other user error.
pub struct HttpError {
    kind: Kind,
    source: Option<BoxError>,
}

#[derive(Debug)]
enum Kind {
    InvalidExtensions,
    InvalidHeaderName,
    InvalidHeaderValue,
    InvalidStatusCode,
    InvalidUri,
    InvalidUriParts,
    MissingAuthority,
    MissingScheme,
    NonUtf8Header(NonUtf8Header),
}

#[derive(Debug)]
pub(super) struct NonUtf8Header {
    error: Utf8Error,
    value: Vec<u8>,
    name: Option<String>,
}

impl NonUtf8Header {
    #[cfg(any(feature = "http-1x", feature = "http-02x"))]
    pub(super) fn new(name: String, value: Vec<u8>, error: Utf8Error) -> Self {
        Self {
            error,
            value,
            name: Some(name),
        }
    }

    pub(super) fn new_missing_name(value: Vec<u8>, error: Utf8Error) -> Self {
        Self {
            error,
            value,
            name: None,
        }
    }
}

impl HttpError {
    pub(super) fn invalid_extensions() -> Self {
        Self {
            kind: Kind::InvalidExtensions,
            source: None,
        }
    }

    pub(super) fn invalid_header_name(err: InvalidHeaderName) -> Self {
        Self {
            kind: Kind::InvalidHeaderName,
            source: Some(Box::new(err)),
        }
    }

    pub(super) fn invalid_header_value(err: InvalidHeaderValue) -> Self {
        Self {
            kind: Kind::InvalidHeaderValue,
            source: Some(Box::new(err)),
        }
    }

    pub(super) fn invalid_status_code() -> Self {
        Self {
            kind: Kind::InvalidStatusCode,
            source: None,
        }
    }

    pub(super) fn invalid_uri(err: InvalidUri) -> Self {
        Self {
            kind: Kind::InvalidUri,
            source: Some(Box::new(err)),
        }
    }

    pub(super) fn invalid_uri_parts(err: http_02x::Error) -> Self {
        Self {
            kind: Kind::InvalidUriParts,
            source: Some(Box::new(err)),
        }
    }

    pub(super) fn missing_authority() -> Self {
        Self {
            kind: Kind::MissingAuthority,
            source: None,
        }
    }

    pub(super) fn missing_scheme() -> Self {
        Self {
            kind: Kind::MissingScheme,
            source: None,
        }
    }

    pub(super) fn non_utf8_header(non_utf8_header: NonUtf8Header) -> Self {
        Self {
            kind: Kind::NonUtf8Header(non_utf8_header),
            source: None,
        }
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Kind::*;
        match &self.kind {
            InvalidExtensions => write!(f, "Extensions were provided during initialization. This prevents the request format from being converted."),
            InvalidHeaderName => write!(f, "invalid header name"),
            InvalidHeaderValue => write!(f, "invalid header value"),
            InvalidStatusCode => write!(f, "invalid HTTP status code"),
            InvalidUri => write!(f, "endpoint is not a valid URI"),
            InvalidUriParts => write!(f, "endpoint parts are not valid"),
            MissingAuthority => write!(f, "endpoint must contain authority"),
            MissingScheme => write!(f, "endpoint must contain scheme"),
            NonUtf8Header(hv) => {
                // In some cases, we won't know the key so we default to "<unknown>".
                let key = hv.name.as_deref().unwrap_or("<unknown>");
                let value = String::from_utf8_lossy(&hv.value);
                let index = hv.error.valid_up_to();
                write!(f, "header `{key}={value}` contains non-UTF8 octet at index {index}")
            },
        }
    }
}

impl Error for HttpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|err| err.as_ref() as _)
    }
}
