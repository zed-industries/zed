// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `errors` module defines the common error type.

use std::error;
use std::fmt;
use std::io;
use std::result;

/// `SeekErrorKind` is a list of generic reasons why a seek may fail.
#[derive(Debug)]
pub enum SeekErrorKind {
    /// The stream is not seekable at all.
    Unseekable,
    /// The stream can only be seeked forward.
    ForwardOnly,
    /// The timestamp to seek to is out of range.
    OutOfRange,
    /// The track ID provided is invalid.
    InvalidTrack,
}

impl SeekErrorKind {
    fn as_str(&self) -> &'static str {
        match *self {
            SeekErrorKind::Unseekable => "stream is not seekable",
            SeekErrorKind::ForwardOnly => "stream can only be seeked forward",
            SeekErrorKind::OutOfRange => "requested seek timestamp is out-of-range for stream",
            SeekErrorKind::InvalidTrack => "invalid track id",
        }
    }
}

/// `Error` provides an enumeration of all possible errors reported by Symphonia.
#[derive(Debug)]
pub enum Error {
    /// An IO error occured while reading, writing, or seeking the stream.
    IoError(std::io::Error),
    /// The stream contained malformed data and could not be decoded or demuxed.
    DecodeError(&'static str),
    /// The stream could not be seeked.
    SeekError(SeekErrorKind),
    /// An unsupported container or codec feature was encounted.
    Unsupported(&'static str),
    /// A default or user-defined limit was reached while decoding or demuxing the stream. Limits
    /// are used to prevent denial-of-service attacks from malicious streams.
    LimitError(&'static str),
    /// The demuxer or decoder needs to be reset before continuing.
    ResetRequired,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref err) => err.fmt(f),
            Error::DecodeError(msg) => {
                write!(f, "malformed stream: {}", msg)
            }
            Error::SeekError(ref kind) => {
                write!(f, "seek error: {}", kind.as_str())
            }
            Error::Unsupported(feature) => {
                write!(f, "unsupported feature: {}", feature)
            }
            Error::LimitError(constraint) => {
                write!(f, "limit reached: {}", constraint)
            }
            Error::ResetRequired => {
                write!(f, "decoder needs to be reset")
            }
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::DecodeError(_) => None,
            Error::SeekError(_) => None,
            Error::Unsupported(_) => None,
            Error::LimitError(_) => None,
            Error::ResetRequired => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;

/// Convenience function to create a decode error.
pub fn decode_error<T>(desc: &'static str) -> Result<T> {
    Err(Error::DecodeError(desc))
}

/// Convenience function to create a seek error.
pub fn seek_error<T>(kind: SeekErrorKind) -> Result<T> {
    Err(Error::SeekError(kind))
}

/// Convenience function to create an unsupport feature error.
pub fn unsupported_error<T>(feature: &'static str) -> Result<T> {
    Err(Error::Unsupported(feature))
}

/// Convenience function to create a limit error.
pub fn limit_error<T>(constraint: &'static str) -> Result<T> {
    Err(Error::LimitError(constraint))
}

/// Convenience function to create a reset required error.
pub fn reset_error<T>() -> Result<T> {
    Err(Error::ResetRequired)
}

/// Convenience function to create an end-of-stream error.
pub fn end_of_stream_error<T>() -> Result<T> {
    Err(Error::IoError(io::Error::new(io::ErrorKind::UnexpectedEof, "end of stream")))
}
