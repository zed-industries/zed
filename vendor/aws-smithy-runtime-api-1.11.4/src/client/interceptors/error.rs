/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Errors related to Smithy interceptors

use crate::box_error::BoxError;
use std::fmt;

macro_rules! interceptor_error_fn {
    ($fn_name:ident => $error_kind:ident (with source)) => {
        #[doc = concat!("Create a new error indicating a failure with a ", stringify!($fn_name), " interceptor.")]
        pub fn $fn_name(
            interceptor_name: impl Into<String>,
            source: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
        ) -> Self {
            Self {
                kind: ErrorKind::$error_kind,
                interceptor_name: Some(interceptor_name.into()),
                source: Some(source.into()),
            }
        }
    };
    ($fn_name:ident => $error_kind:ident (invalid $thing:ident access)) => {
        #[doc = concat!("Create a new error indicating that an interceptor tried to access the ", stringify!($thing), " out of turn.")]
        pub fn $fn_name() -> Self {
            Self {
                kind: ErrorKind::$error_kind,
                interceptor_name: None,
                source: None,
            }
        }
    }
}

/// An error related to Smithy interceptors.
#[derive(Debug)]
pub struct InterceptorError {
    kind: ErrorKind,
    interceptor_name: Option<String>,
    source: Option<BoxError>,
}

impl InterceptorError {
    interceptor_error_fn!(read_before_execution => ReadBeforeExecution (with source));
    interceptor_error_fn!(modify_before_serialization => ModifyBeforeSerialization (with source));
    interceptor_error_fn!(read_before_serialization => ReadBeforeSerialization (with source));
    interceptor_error_fn!(read_after_serialization => ReadAfterSerialization (with source));
    interceptor_error_fn!(modify_before_retry_loop => ModifyBeforeRetryLoop (with source));
    interceptor_error_fn!(read_before_attempt => ReadBeforeAttempt (with source));
    interceptor_error_fn!(modify_before_signing => ModifyBeforeSigning (with source));
    interceptor_error_fn!(read_before_signing => ReadBeforeSigning (with source));
    interceptor_error_fn!(read_after_signing => ReadAfterSigning (with source));
    interceptor_error_fn!(modify_before_transmit => ModifyBeforeTransmit (with source));
    interceptor_error_fn!(read_before_transmit => ReadBeforeTransmit (with source));
    interceptor_error_fn!(read_after_transmit => ReadAfterTransmit (with source));
    interceptor_error_fn!(modify_before_deserialization => ModifyBeforeDeserialization (with source));
    interceptor_error_fn!(read_before_deserialization => ReadBeforeDeserialization (with source));
    interceptor_error_fn!(read_after_deserialization => ReadAfterDeserialization (with source));
    interceptor_error_fn!(modify_before_attempt_completion => ModifyBeforeAttemptCompletion (with source));
    interceptor_error_fn!(read_after_attempt => ReadAfterAttempt (with source));
    interceptor_error_fn!(modify_before_completion => ModifyBeforeCompletion (with source));
    interceptor_error_fn!(read_after_execution => ReadAfterExecution (with source));

    interceptor_error_fn!(modify_before_attempt_completion_failed => ModifyBeforeAttemptCompletion (with source));
    interceptor_error_fn!(read_after_attempt_failed => ReadAfterAttempt (with source));
    interceptor_error_fn!(modify_before_completion_failed => ModifyBeforeCompletion (with source));
    interceptor_error_fn!(read_after_execution_failed => ReadAfterExecution (with source));

    interceptor_error_fn!(invalid_request_access => InvalidRequestAccess (invalid request access));
    interceptor_error_fn!(invalid_response_access => InvalidResponseAccess (invalid response access));
    interceptor_error_fn!(invalid_input_access => InvalidInputAccess (invalid input access));
    interceptor_error_fn!(invalid_output_access => InvalidOutputAccess (invalid output access));
}

#[derive(Debug)]
enum ErrorKind {
    /// An error occurred within the read_before_execution interceptor
    ReadBeforeExecution,
    /// An error occurred within the modify_before_serialization interceptor
    ModifyBeforeSerialization,
    /// An error occurred within the read_before_serialization interceptor
    ReadBeforeSerialization,
    /// An error occurred within the read_after_serialization interceptor
    ReadAfterSerialization,
    /// An error occurred within the modify_before_retry_loop interceptor
    ModifyBeforeRetryLoop,
    /// An error occurred within the read_before_attempt interceptor
    ReadBeforeAttempt,
    /// An error occurred within the modify_before_signing interceptor
    ModifyBeforeSigning,
    /// An error occurred within the read_before_signing interceptor
    ReadBeforeSigning,
    /// An error occurred within the read_after_signing interceptor
    ReadAfterSigning,
    /// An error occurred within the modify_before_transmit interceptor
    ModifyBeforeTransmit,
    /// An error occurred within the read_before_transmit interceptor
    ReadBeforeTransmit,
    /// An error occurred within the read_after_transmit interceptor
    ReadAfterTransmit,
    /// An error occurred within the modify_before_deserialization interceptor
    ModifyBeforeDeserialization,
    /// An error occurred within the read_before_deserialization interceptor
    ReadBeforeDeserialization,
    /// An error occurred within the read_after_deserialization interceptor
    ReadAfterDeserialization,
    /// An error occurred within the modify_before_attempt_completion interceptor
    ModifyBeforeAttemptCompletion,
    /// An error occurred within the read_after_attempt interceptor
    ReadAfterAttempt,
    /// An error occurred within the modify_before_completion interceptor
    ModifyBeforeCompletion,
    /// An error occurred within the read_after_execution interceptor
    ReadAfterExecution,
    /// An interceptor tried to access the request out of turn
    InvalidRequestAccess,
    /// An interceptor tried to access the response out of turn
    InvalidResponseAccess,
    /// An interceptor tried to access the input out of turn
    InvalidInputAccess,
    /// An interceptor tried to access the output out of turn
    InvalidOutputAccess,
}

macro_rules! display_interceptor_err {
    ($self:ident, $f:ident, $(($error_kind:ident => $fn_name:ident ($($option:tt)+)),)+) => {
        {
        use ErrorKind::*;
        match &$self.kind {
            $($error_kind => display_interceptor_err!($self, $f, $fn_name, ($($option)+)),)+
        }
    }
    };
    ($self:ident, $f:ident, $fn_name:ident, (interceptor error)) => {{
        $f.write_str($self.interceptor_name.as_deref().unwrap_or_default())?;
        $f.write_str(concat!(" ", stringify!($fn_name), " interceptor encountered an error"))
    }};
    ($self:ident, $f:ident, $fn_name:ident, (invalid access $name:ident $message:literal)) => {
        $f.write_str(concat!("tried to access the ", stringify!($name), " ", $message))
    };
}

impl fmt::Display for InterceptorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_interceptor_err!(self, f,
            (ReadBeforeExecution => read_before_execution (interceptor error)),
            (ModifyBeforeSerialization => modify_before_serialization (interceptor error)),
            (ReadBeforeSerialization => read_before_serialization (interceptor error)),
            (ReadAfterSerialization => read_after_serialization (interceptor error)),
            (ModifyBeforeRetryLoop => modify_before_retry_loop (interceptor error)),
            (ReadBeforeAttempt => read_Before_attempt (interceptor error)),
            (ModifyBeforeSigning => modify_before_signing (interceptor error)),
            (ReadBeforeSigning => read_before_signing (interceptor error)),
            (ReadAfterSigning => read_after_signing (interceptor error)),
            (ModifyBeforeTransmit => modify_before_transmit (interceptor error)),
            (ReadBeforeTransmit => read_before_transmit (interceptor error)),
            (ReadAfterTransmit => read_after_transmit (interceptor error)),
            (ModifyBeforeDeserialization => modify_before_deserialization (interceptor error)),
            (ReadBeforeDeserialization => read_before_deserialization (interceptor error)),
            (ReadAfterDeserialization => read_after_deserialization (interceptor error)),
            (ModifyBeforeAttemptCompletion => modify_before_attempt_completion (interceptor error)),
            (ReadAfterAttempt => read_after_attempt (interceptor error)),
            (ModifyBeforeCompletion => modify_before_completion (interceptor error)),
            (ReadAfterExecution => read_after_execution (interceptor error)),
            (InvalidRequestAccess => invalid_request_access (invalid access request "before request serialization")),
            (InvalidResponseAccess => invalid_response_access (invalid access response "before transmitting a request")),
            (InvalidInputAccess => invalid_input_access (invalid access input "after request serialization")),
            (InvalidOutputAccess => invalid_output_access (invalid access output "before response deserialization")),
        )
    }
}

impl std::error::Error for InterceptorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|err| err.as_ref() as _)
    }
}

/// A convenience error that allows for adding additional `context` to `source`
#[derive(Debug)]
pub struct ContextAttachedError {
    context: String,
    source: Option<BoxError>,
}

impl ContextAttachedError {
    /// Creates a new `ContextAttachedError` with the given `context` and `source`.
    pub fn new(context: impl Into<String>, source: impl Into<BoxError>) -> Self {
        Self {
            context: context.into(),
            source: Some(source.into()),
        }
    }
}

impl fmt::Display for ContextAttachedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.context)
    }
}

impl std::error::Error for ContextAttachedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|err| err.as_ref() as _)
    }
}
