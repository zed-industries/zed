// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Error types

use core::fmt;

#[cfg(feature="std")]
use std::error::Error as stdError;
#[cfg(feature="std")]
use std::io;

/// Error kind which can be matched over.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ErrorKind {
    /// Feature is not available; not recoverable.
    /// 
    /// This is the most permanent failure type and implies the error cannot be
    /// resolved simply by retrying (e.g. the feature may not exist in this
    /// build of the application or on the current platform).
    Unavailable,
    /// General failure; there may be a chance of recovery on retry.
    /// 
    /// This is the catch-all kind for errors from known and unknown sources
    /// which do not have a more specific kind / handling method.
    /// 
    /// It is suggested to retry a couple of times or retry later when
    /// handling; some error sources may be able to resolve themselves,
    /// although this is not likely.
    Unexpected,
    /// A transient failure which likely can be resolved or worked around.
    /// 
    /// This error kind exists for a few specific cases where it is known that
    /// the error likely can be resolved internally, but is reported anyway.
    Transient,
    /// Not ready yet: recommended to try again a little later.
    /// 
    /// This error kind implies the generator needs more time or needs some
    /// other part of the application to do something else first before it is
    /// ready for use; for example this may be used by external generators
    /// which require time for initialization.
    NotReady,
    #[doc(hidden)]
    __Nonexhaustive,
}

impl ErrorKind {
    /// True if this kind of error may resolve itself on retry.
    /// 
    /// See also `should_wait()`.
    pub fn should_retry(self) -> bool {
        self != ErrorKind::Unavailable
    }
    
    /// True if we should retry but wait before retrying
    /// 
    /// This implies `should_retry()` is true.
    pub fn should_wait(self) -> bool {
        self == ErrorKind::NotReady
    }
    
    /// A description of this error kind
    pub fn description(self) -> &'static str {
        match self {
            ErrorKind::Unavailable => "permanently unavailable",
            ErrorKind::Unexpected => "unexpected failure",
            ErrorKind::Transient => "transient failure",
            ErrorKind::NotReady => "not ready yet",
            ErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}


/// Error type of random number generators
/// 
/// This is a relatively simple error type, designed for compatibility with and
/// without the Rust `std` library. It embeds a "kind" code, a message (static
/// string only), and an optional chained cause (`std` only). The `kind` and
/// `msg` fields can be accessed directly; cause can be accessed via
/// `std::error::Error::cause` or `Error::take_cause`. Construction can only be
/// done via `Error::new` or `Error::with_cause`.
#[derive(Debug)]
pub struct Error {
    /// The error kind
    pub kind: ErrorKind,
    /// The error message
    pub msg: &'static str,
    #[cfg(feature="std")]
    cause: Option<Box<stdError + Send + Sync>>,
}

impl Error {
    /// Create a new instance, with specified kind and a message.
    pub fn new(kind: ErrorKind, msg: &'static str) -> Self {
        #[cfg(feature="std")] {
            Error { kind, msg, cause: None }
        }
        #[cfg(not(feature="std"))] {
            Error { kind, msg }
        }
    }
    
    /// Create a new instance, with specified kind, message, and a
    /// chained cause.
    /// 
    /// Note: `stdError` is an alias for `std::error::Error`.
    /// 
    /// If not targetting `std` (i.e. `no_std`), this function is replaced by
    /// another with the same prototype, except that there are no bounds on the
    /// type `E` (because both `Box` and `stdError` are unavailable), and the
    /// `cause` is ignored.
    #[cfg(feature="std")]
    pub fn with_cause<E>(kind: ErrorKind, msg: &'static str, cause: E) -> Self
        where E: Into<Box<stdError + Send + Sync>>
    {
        Error { kind, msg, cause: Some(cause.into()) }
    }
    
    /// Create a new instance, with specified kind, message, and a
    /// chained cause.
    /// 
    /// In `no_std` mode the *cause* is ignored.
    #[cfg(not(feature="std"))]
    pub fn with_cause<E>(kind: ErrorKind, msg: &'static str, _cause: E) -> Self {
        Error { kind, msg }
    }
    
    /// Take the cause, if any. This allows the embedded cause to be extracted.
    /// This uses `Option::take`, leaving `self` with no cause.
    #[cfg(feature="std")]
    pub fn take_cause(&mut self) -> Option<Box<stdError + Send + Sync>> {
        self.cause.take()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature="std")] {
            if let Some(ref cause) = self.cause {
                return write!(f, "{} ({}); cause: {}",
                        self.msg, self.kind.description(), cause);
            }
        }
        write!(f, "{} ({})", self.msg, self.kind.description())
    }
}

#[cfg(feature="std")]
impl stdError for Error {
    fn description(&self) -> &str {
        self.msg
    }

    fn cause(&self) -> Option<&stdError> {
        self.cause.as_ref().map(|e| e.as_ref() as &stdError)
    }
}

#[cfg(feature="std")]
impl From<Error> for io::Error {
    fn from(error: Error) -> Self {
        use std::io::ErrorKind::*;
        match error.kind {
            ErrorKind::Unavailable => io::Error::new(NotFound, error),
            ErrorKind::Unexpected |
            ErrorKind::Transient => io::Error::new(Other, error),
            ErrorKind::NotReady => io::Error::new(WouldBlock, error),
            ErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}
