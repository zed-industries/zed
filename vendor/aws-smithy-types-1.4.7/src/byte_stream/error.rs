/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Errors related to bytestreams.

use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub(super) enum ErrorKind {
    #[cfg(feature = "rt-tokio")]
    OffsetLargerThanFileSize,
    #[cfg(feature = "rt-tokio")]
    LengthLargerThanFileSizeMinusReadOffset,
    IoError(IoError),
    StreamingError(Box<dyn StdError + Send + Sync + 'static>),
}

/// An error occurred in the byte stream
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(super) fn streaming(err: impl Into<Box<dyn StdError + Send + Sync + 'static>>) -> Self {
        ErrorKind::StreamingError(err.into()).into()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        ErrorKind::IoError(err).into()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            #[cfg(feature = "rt-tokio")]
            ErrorKind::OffsetLargerThanFileSize => write!(
                f,
                "offset must be less than or equal to file size but was greater than"
            ),
            #[cfg(feature = "rt-tokio")]
            ErrorKind::LengthLargerThanFileSizeMinusReadOffset => write!(
                f,
                "`Length::Exact` was larger than file size minus read offset"
            ),
            ErrorKind::IoError(_) => write!(f, "IO error"),
            ErrorKind::StreamingError(_) => write!(f, "streaming error"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self.kind {
            ErrorKind::IoError(err) => Some(err as _),
            ErrorKind::StreamingError(err) => Some(err.as_ref() as _),
            #[cfg(feature = "rt-tokio")]
            ErrorKind::OffsetLargerThanFileSize
            | ErrorKind::LengthLargerThanFileSizeMinusReadOffset => None,
        }
    }
}

impl From<Error> for IoError {
    fn from(err: Error) -> Self {
        IoError::other(err)
    }
}
