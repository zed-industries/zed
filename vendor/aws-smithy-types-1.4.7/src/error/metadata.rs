/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Error metadata

use crate::retry::{ErrorKind, ProvideErrorKind};
use std::collections::HashMap;
use std::fmt;

/// Trait to retrieve error metadata from a result
pub trait ProvideErrorMetadata {
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    fn meta(&self) -> &ErrorMetadata;

    /// Returns the error code if it's available.
    fn code(&self) -> Option<&str> {
        self.meta().code()
    }

    /// Returns the error message, if there is one.
    fn message(&self) -> Option<&str> {
        self.meta().message()
    }
}

/// Empty error metadata
pub const EMPTY_ERROR_METADATA: ErrorMetadata = ErrorMetadata {
    code: None,
    message: None,
    extras: None,
};

/// Generic Error type
///
/// For many services, Errors are modeled. However, many services only partially model errors or don't
/// model errors at all. In these cases, the SDK will return this generic error type to expose the
/// `code`, `message` and `request_id`.
#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct ErrorMetadata {
    code: Option<String>,
    message: Option<String>,
    extras: Option<HashMap<&'static str, String>>,
}

impl ProvideErrorMetadata for ErrorMetadata {
    fn meta(&self) -> &ErrorMetadata {
        self
    }
}

/// Builder for [`ErrorMetadata`].
#[derive(Debug, Default)]
pub struct Builder {
    inner: ErrorMetadata,
}

impl Builder {
    /// Sets the error message.
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.inner.message = Some(message.into());
        self
    }

    /// Sets the error code.
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.inner.code = Some(code.into());
        self
    }

    /// Set a custom field on the error metadata
    ///
    /// Typically, these will be accessed with an extension trait:
    /// ```rust
    /// use aws_smithy_types::error::ErrorMetadata;
    /// const HOST_ID: &str = "host_id";
    /// trait S3ErrorExt {
    ///     fn extended_request_id(&self) -> Option<&str>;
    /// }
    ///
    /// impl S3ErrorExt for ErrorMetadata {
    ///     fn extended_request_id(&self) -> Option<&str> {
    ///         self.extra(HOST_ID)
    ///     }
    /// }
    ///
    /// fn main() {
    ///     // Extension trait must be brought into scope
    ///     use S3ErrorExt;
    ///     let sdk_response: Result<(), ErrorMetadata> = Err(ErrorMetadata::builder().custom(HOST_ID, "x-1234").build());
    ///     if let Err(err) = sdk_response {
    ///         println!("extended request id: {:?}", err.extended_request_id());
    ///     }
    /// }
    /// ```
    pub fn custom(mut self, key: &'static str, value: impl Into<String>) -> Self {
        if self.inner.extras.is_none() {
            self.inner.extras = Some(HashMap::new());
        }
        self.inner
            .extras
            .as_mut()
            .unwrap()
            .insert(key, value.into());
        self
    }

    /// Creates the error.
    pub fn build(self) -> ErrorMetadata {
        self.inner
    }
}

impl ErrorMetadata {
    /// Returns the error code.
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
    /// Returns additional information about the error if it's present.
    pub fn extra(&self, key: &'static str) -> Option<&str> {
        self.extras
            .as_ref()
            .and_then(|extras| extras.get(key).map(|k| k.as_str()))
    }

    /// Creates an `Error` builder.
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Converts an `Error` into a builder.
    pub fn into_builder(self) -> Builder {
        Builder { inner: self }
    }
}

impl ProvideErrorKind for ErrorMetadata {
    fn retryable_error_kind(&self) -> Option<ErrorKind> {
        None
    }

    fn code(&self) -> Option<&str> {
        ErrorMetadata::code(self)
    }
}

impl fmt::Display for ErrorMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt = f.debug_struct("Error");
        if let Some(code) = &self.code {
            fmt.field("code", code);
        }
        if let Some(message) = &self.message {
            fmt.field("message", message);
        }
        if let Some(extras) = &self.extras {
            for (k, v) in extras {
                fmt.field(k, &v);
            }
        }
        fmt.finish()
    }
}

impl std::error::Error for ErrorMetadata {}
