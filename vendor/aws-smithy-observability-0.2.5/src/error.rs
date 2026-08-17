/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Observability Errors

use std::fmt;

use aws_smithy_runtime_api::box_error::BoxError;

/// An error in the SDKs Observability providers
#[non_exhaustive]
#[derive(Debug)]
pub struct ObservabilityError {
    kind: ErrorKind,
    source: BoxError,
}

/// The types of errors associated with [ObservabilityError]
#[non_exhaustive]
#[derive(Debug)]
pub enum ErrorKind {
    /// A custom error that does not fall under any other error kind
    Other,
}

impl ObservabilityError {
    /// Create a new [`ObservabilityError`] from an [ErrorKind] and a [BoxError]
    pub fn new<E>(kind: ErrorKind, err: E) -> Self
    where
        E: Into<BoxError>,
    {
        Self {
            kind,
            source: err.into(),
        }
    }

    /// Returns the corresponding [`ErrorKind`] for this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl fmt::Display for ObservabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Other => write!(f, "unclassified error"),
        }
    }
}

impl std::error::Error for ObservabilityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}

/// An simple error to represent issues with the global [crate::TelemetryProvider].
#[non_exhaustive]
#[derive(Debug)]
pub struct GlobalTelemetryProviderError {
    reason: &'static str,
}

impl GlobalTelemetryProviderError {
    /// Create a new [GlobalTelemetryProviderError] with a given reason for the error.
    pub fn new(reason: &'static str) -> Self {
        Self { reason }
    }
}

impl std::error::Error for GlobalTelemetryProviderError {}

impl fmt::Display for GlobalTelemetryProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GlobalTelemetryProviderError: {}", self.reason)
    }
}
