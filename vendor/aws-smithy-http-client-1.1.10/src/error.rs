/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */
use aws_smithy_runtime_api::box_error::BoxError;
use std::fmt;

/// HTTP client errors
///
/// This is normally due to configuration issues, internal SDK bugs, or other user error.
#[derive(Debug)]
pub struct HttpClientError {
    source: Option<BoxError>,
}

impl fmt::Display for HttpClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown HTTP client error")
    }
}

impl std::error::Error for HttpClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|err| err.as_ref() as _)
    }
}
