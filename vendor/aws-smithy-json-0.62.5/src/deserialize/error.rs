/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::escape::EscapeError;
use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;
use std::str::Utf8Error;

#[derive(Debug)]
pub(in crate::deserialize) enum DeserializeErrorKind {
    Custom {
        message: Cow<'static, str>,
        source: Option<Box<dyn StdError + Send + Sync + 'static>>,
    },
    ExpectedLiteral(String),
    InvalidEscape(char),
    InvalidNumber,
    InvalidUtf8,
    UnescapeFailed(EscapeError),
    UnexpectedControlCharacter(u8),
    UnexpectedEos,
    UnexpectedToken(char, &'static str),
}

#[derive(Debug)]
pub struct DeserializeError {
    pub(in crate::deserialize) kind: DeserializeErrorKind,
    pub(in crate::deserialize) offset: Option<usize>,
}

impl DeserializeError {
    pub(in crate::deserialize) fn new(kind: DeserializeErrorKind, offset: Option<usize>) -> Self {
        Self { kind, offset }
    }

    /// Returns a custom error without an offset.
    pub fn custom(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(
            DeserializeErrorKind::Custom {
                message: message.into(),
                source: None,
            },
            None,
        )
    }

    /// Returns a custom error with an error source without an offset.
    pub fn custom_source(
        message: impl Into<Cow<'static, str>>,
        source: impl Into<Box<dyn StdError + Send + Sync + 'static>>,
    ) -> Self {
        Self::new(
            DeserializeErrorKind::Custom {
                message: message.into(),
                source: Some(source.into()),
            },
            None,
        )
    }

    /// Adds an offset to the error.
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl StdError for DeserializeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use DeserializeErrorKind::*;
        match &self.kind {
            UnescapeFailed(source) => Some(source),
            Custom {
                source: Some(source),
                ..
            } => Some(source.as_ref()),
            Custom { source: None, .. }
            | ExpectedLiteral(_)
            | InvalidEscape(_)
            | InvalidNumber
            | InvalidUtf8
            | UnexpectedControlCharacter(_)
            | UnexpectedToken(..)
            | UnexpectedEos => None,
        }
    }
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DeserializeErrorKind::*;
        if let Some(offset) = self.offset {
            write!(f, "Error at offset {offset}: ")?;
        }
        match &self.kind {
            Custom { message, .. } => write!(f, "failed to parse JSON: {message}"),
            ExpectedLiteral(literal) => write!(f, "expected literal: {literal}"),
            InvalidEscape(escape) => write!(f, "invalid JSON escape: \\{escape}"),
            InvalidNumber => write!(f, "invalid number"),
            InvalidUtf8 => write!(f, "invalid UTF-8 codepoint in JSON stream"),
            UnescapeFailed(_) => write!(f, "failed to unescape JSON string"),
            UnexpectedControlCharacter(value) => write!(
                f,
                "encountered unescaped control character in string: 0x{value:X}"
            ),
            UnexpectedToken(token, expected) => {
                write!(f, "unexpected token '{token}'. Expected one of {expected}",)
            }
            UnexpectedEos => write!(f, "unexpected end of stream"),
        }
    }
}

impl From<Utf8Error> for DeserializeErrorKind {
    fn from(_: Utf8Error) -> Self {
        DeserializeErrorKind::InvalidUtf8
    }
}

impl From<EscapeError> for DeserializeError {
    fn from(err: EscapeError) -> Self {
        Self {
            kind: DeserializeErrorKind::UnescapeFailed(err),
            offset: None,
        }
    }
}

impl From<aws_smithy_types::error::TryFromNumberError> for DeserializeError {
    fn from(_: aws_smithy_types::error::TryFromNumberError) -> Self {
        Self {
            kind: DeserializeErrorKind::InvalidNumber,
            offset: None,
        }
    }
}
