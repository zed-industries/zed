//! # Errors of nvim-rs.
//!
//! Nvim-rs reports very detailed errors, to facilitate debugging by logging
//! even in rare edge cases, and to enable clients to handle errors according to
//! their needs. Errors are boxed to not overly burden the size of the
//! `Result`s.
//!
//! ### Overview
//!
//! Errors can originate in three ways:
//!
//!   1. Failure of a request to neovim is communicated by a
//!      [`CallError`](crate::error::CallError).
//!   2. A failure in the io loop is communicated by a
//!      [`LoopError`](crate::error::LoopError).
//!   3. A failure to connect to neovim when starting up via one of the
//!      [`new_*`](crate::create) functions  is communicated by an
//!      [`io::Error`](std::io::Error).
//!
//! Most errors should probably be treated as fatal, and the application should
//! just exit.
//!
//!
//! ### Special errors
//!
//! Use [`is_reader_error`](crate::error::LoopError::is_reader_error)
//! to check if it might sense to try to show an error message to the neovim
//! user (see [this example](crate::examples::scorched_earth)).
//!
//! Use
//! [`CallError::is_channel_closed`](crate::error::CallError::is_channel_closed)
//! or
//! [`LoopError::is_channel_closed`](crate::error::LoopError::is_channel_closed)
//! to determine if the error originates from a closed channel. This means
//! either neovim closed the channel actively, or neovim was closed. Often, this
//! is not seen as a real error, but the signal for the plugin to quit. Again,
//! see the [example](crate::examples::scorched_earth).
use std::{
  error::Error, fmt, fmt::Display, io, io::ErrorKind, ops::RangeInclusive,
  sync::Arc,
};

use futures::channel::oneshot;
use rmpv::{
  decode::Error as RmpvDecodeError, encode::Error as RmpvEncodeError, Value,
};

/// A message from neovim had an invalid format
///
/// This should be very basically non-existent, since it would indicate a bug in
/// neovim.
#[derive(Debug, PartialEq, Clone)]
pub enum InvalidMessage {
  /// The value read was not an array
  NotAnArray(Value),
  /// WrongArrayLength(should, is) means that the array should have length in
  /// the range `should`, but has length `is`
  WrongArrayLength(RangeInclusive<u64>, u64),
  /// The first array element (=the message type) was not decodable into a u64
  InvalidType(Value),
  /// The first array element (=the message type) was decodable into a u64
  /// larger than 2
  UnknownMessageType(u64),
  /// The params of a request or notification weren't an array
  InvalidParams(Value, String),
  /// The method name of a notification was not decodable into a String
  InvalidNotificationName(Value),
  /// The method name of a request was not decodable into a String
  InvalidRequestName(u64, Value),
  /// The msgid of a request or response was not decodable into a u64
  InvalidMsgid(Value),
}

impl Error for InvalidMessage {}

impl Display for InvalidMessage {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    use InvalidMessage::*;

    match self {
      NotAnArray(val) => write!(fmt, "Value not an Array: '{val}'"),
      WrongArrayLength(should, is) => write!(
        fmt,
        "Array should have length {:?}, has length {}",
        should, is
      ),
      InvalidType(val) => {
        write!(fmt, "Message type not decodable into u64: {val}")
      }
      UnknownMessageType(m) => {
        write!(fmt, "Message type {m} is not 0, 1 or 2")
      }
      InvalidParams(val, s) => {
        write!(fmt, "Params of method '{s}' not an Array: '{val}'")
      }
      InvalidNotificationName(val) => write!(
        fmt,
        "Notification name not a
        string: '{}'",
        val
      ),
      InvalidRequestName(id, val) => {
        write!(fmt, "Request id {id}: name not valid String: '{val}'")
      }
      InvalidMsgid(val) => {
        write!(fmt, "Msgid of message not decodable into u64: '{val}'")
      }
    }
  }
}

/// Receiving a message from neovim failed
#[derive(Debug)]
pub enum DecodeError {
  /// Reading from the internal buffer failed.
  BufferError(RmpvDecodeError),
  /// Reading from the stream failed. This is probably unrecoverable from, but
  /// might also mean that neovim closed the stream and wants the plugin to
  /// finish. See examples/quitting.rs on how this might be caught.
  ReaderError(io::Error),
  /// Neovim sent a message that's not valid.
  InvalidMessage(InvalidMessage),
}

impl Error for DecodeError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      DecodeError::BufferError(ref e) => Some(e),
      DecodeError::InvalidMessage(ref e) => Some(e),
      DecodeError::ReaderError(ref e) => Some(e),
    }
  }
}

impl Display for DecodeError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let s = match *self {
      DecodeError::BufferError(_) => "Error while reading from buffer",
      DecodeError::InvalidMessage(_) => "Error while decoding",
      DecodeError::ReaderError(_) => "Error while reading from Reader",
    };

    fmt.write_str(s)
  }
}

impl From<RmpvDecodeError> for Box<DecodeError> {
  fn from(err: RmpvDecodeError) -> Box<DecodeError> {
    Box::new(DecodeError::BufferError(err))
  }
}

impl From<InvalidMessage> for Box<DecodeError> {
  fn from(err: InvalidMessage) -> Box<DecodeError> {
    Box::new(DecodeError::InvalidMessage(err))
  }
}

impl From<io::Error> for Box<DecodeError> {
  fn from(err: io::Error) -> Box<DecodeError> {
    Box::new(DecodeError::ReaderError(err))
  }
}

/// Sending a message to neovim failed
#[derive(Debug)]
pub enum EncodeError {
  /// Encoding the message into the internal buffer has failed.
  BufferError(RmpvEncodeError),
  /// Writing the encoded message to the stream failed.
  WriterError(io::Error),
}

impl Error for EncodeError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      EncodeError::BufferError(ref e) => Some(e),
      EncodeError::WriterError(ref e) => Some(e),
    }
  }
}

impl Display for EncodeError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let s = match *self {
      Self::BufferError(_) => "Error writing to buffer",
      Self::WriterError(_) => "Error writing to the Writer",
    };

    fmt.write_str(s)
  }
}

impl From<RmpvEncodeError> for Box<EncodeError> {
  fn from(err: RmpvEncodeError) -> Box<EncodeError> {
    Box::new(EncodeError::BufferError(err))
  }
}

impl From<io::Error> for Box<EncodeError> {
  fn from(err: io::Error) -> Box<EncodeError> {
    Box::new(EncodeError::WriterError(err))
  }
}

/// A [`call`](crate::neovim::Neovim::call) to neovim failed
///
/// The API functions return this, as they are just
/// proxies for [`call`](crate::neovim::Neovim::call).
#[derive(Debug)]
pub enum CallError {
  /// Sending the request to neovim has failed.
  ///
  /// Fields:
  ///
  /// 0. The underlying error
  /// 1. The name of the called method
  SendError(EncodeError, String),
  /// The internal channel to send the response to the right task was closed.
  /// This really should not happen, unless someone manages to kill individual
  /// tasks.
  ///
  /// Fields:
  ///
  /// 0. The underlying error
  /// 1. The name of the called method
  InternalReceiveError(oneshot::Canceled, String),
  /// Decoding neovim's response failed.
  ///
  /// Fields:
  ///
  /// 0. The underlying error
  /// 1. The name of the called method
  ///
  /// *Note*: DecodeError can't be Clone, so we Arc-wrap it
  DecodeError(Arc<DecodeError>, String),
  /// Neovim encountered an error while executing the reqest.
  ///
  /// Fields:
  ///
  /// 0. Neovim's error type (see `:h api`)
  /// 1. Neovim's error message
  NeovimError(Option<i64>, String),
  /// The response from neovim contained a [`Value`](rmpv::Value) of the wrong
  /// type
  WrongValueType(Value),
}

impl Error for CallError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      CallError::SendError(ref e, _) => Some(e),
      CallError::InternalReceiveError(ref e, _) => Some(e),
      CallError::DecodeError(ref e, _) => Some(e.as_ref()),
      CallError::NeovimError(_, _) | CallError::WrongValueType(_) => None,
    }
  }
}

impl CallError {
  /// Determine if the error originated from a closed channel. This is generally
  /// used to close a plugin from neovim's side, and so most of the time should
  /// not be treated as a real error, but a signal to finish the program.
  #[must_use]
  pub fn is_channel_closed(&self) -> bool {
    match *self {
      CallError::SendError(EncodeError::WriterError(ref e), _)
        if e.kind() == ErrorKind::UnexpectedEof =>
      {
        return true
      }
      CallError::DecodeError(ref err, _) => {
        if let DecodeError::ReaderError(ref e) = err.as_ref() {
          if e.kind() == ErrorKind::UnexpectedEof {
            return true;
          }
        }
      }
      _ => {}
    }

    false
  }
}

impl Display for CallError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      Self::SendError(_, ref s) => write!(fmt, "Error sending request '{s}'"),
      Self::InternalReceiveError(_, ref s) => {
        write!(fmt, "Error receiving response for '{s}'")
      }
      Self::DecodeError(_, ref s) => {
        write!(fmt, "Error decoding response to request '{s}'")
      }
      Self::NeovimError(ref i, ref s) => match i {
        Some(i) => write!(fmt, "Error processing request: {i} - '{s}')"),
        None => write!(
          fmt,
          "Error processing request, unknown error format: '{s}'"
        ),
      },
      CallError::WrongValueType(ref val) => {
        write!(fmt, "Wrong value type: '{val}'")
      }
    }
  }
}

impl From<Value> for Box<CallError> {
  fn from(val: Value) -> Box<CallError> {
    match val {
      Value::Array(mut arr)
        if arr.len() == 2 && arr[0].is_i64() && arr[1].is_str() =>
      {
        let s = arr
          .pop()
          .expect("This was checked")
          .as_str()
          .expect("This was checked")
          .into();
        let i = arr.pop().expect("This was checked").as_i64();
        Box::new(CallError::NeovimError(i, s))
      }
      val => Box::new(CallError::NeovimError(None, format!("{val:?}"))),
    }
  }
}

/// A failure in the io loop
#[derive(Debug)]
pub enum LoopError {
  /// A Msgid could not be found in the request queue
  MsgidNotFound(u64),
  /// Decoding a message failed.
  ///
  /// Fields:
  ///
  /// 0. The underlying error
  /// 1. The msgids of the requests we could not send the error to.
  ///
  /// Note: DecodeError can't be clone, so we Arc-wrap it.
  DecodeError(Arc<DecodeError>, Option<Vec<u64>>),
  /// Failed to send a Response (from neovim) through the sender from the
  /// request queue
  ///
  /// Fields:
  ///
  /// 0. The msgid of the request the response was sent for
  /// 1. The response from neovim
  InternalSendResponseError(u64, Result<Value, Value>),
}

impl Error for LoopError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      LoopError::MsgidNotFound(_)
      | LoopError::InternalSendResponseError(_, _) => None,
      LoopError::DecodeError(ref e, _) => Some(e.as_ref()),
    }
  }
}

impl LoopError {
  #[must_use]
  pub fn is_channel_closed(&self) -> bool {
    if let LoopError::DecodeError(ref err, _) = *self {
      if let DecodeError::ReaderError(ref e) = err.as_ref() {
        if e.kind() == ErrorKind::UnexpectedEof {
          return true;
        }
      }
    }
    false
  }

  #[must_use]
  pub fn is_reader_error(&self) -> bool {
    if let LoopError::DecodeError(ref err, _) = *self {
      if let DecodeError::ReaderError(_) = err.as_ref() {
        return true;
      }
    }
    false
  }
}

impl Display for LoopError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      Self::MsgidNotFound(i) => {
        write!(fmt, "Could not find Msgid '{i}' in the Queue")
      }
      Self::DecodeError(_, ref o) => match o {
        None => write!(fmt, "Error reading message"),
        Some(v) => write!(
          fmt,
          "Error reading message, could not forward \
           error to the following requests: '{:?}'",
          v
        ),
      },
      Self::InternalSendResponseError(i, ref res) => write!(
        fmt,
        "Request {i}: Could not send response, which was {:?}",
        res
      ),
    }
  }
}

impl From<(u64, Result<Value, Value>)> for Box<LoopError> {
  fn from(res: (u64, Result<Value, Value>)) -> Box<LoopError> {
    Box::new(LoopError::InternalSendResponseError(res.0, res.1))
  }
}

impl From<(Arc<DecodeError>, Vec<u64>)> for Box<LoopError> {
  fn from(v: (Arc<DecodeError>, Vec<u64>)) -> Box<LoopError> {
    Box::new(LoopError::DecodeError(v.0, Some(v.1)))
  }
}

impl From<u64> for Box<LoopError> {
  fn from(i: u64) -> Box<LoopError> {
    Box::new(LoopError::MsgidNotFound(i))
  }
}

#[derive(Debug)]
pub enum HandshakeError {
  /// Sending the request to neovim has failed.
  ///
  /// Fields:
  ///
  /// 0. The underlying error
  SendError(EncodeError),
  /// Sending the request to neovim has failed.
  ///
  /// Fields:
  ///
  /// 0. The underlying error
  /// 1. The data read so far
  RecvError(io::Error, String),
  /// Unexpected response received
  ///
  /// Fields:
  ///
  /// 0. The data read so far
  UnexpectedResponse(String),
  /// The launch of Neovim failed
  ///
  /// Fields:
  ///
  /// 0. The underlying error
  LaunchError(io::Error),
}

impl From<Box<EncodeError>> for Box<HandshakeError> {
  fn from(v: Box<EncodeError>) -> Box<HandshakeError> {
    Box::new(HandshakeError::SendError(*v))
  }
}

impl From<(io::Error, String)> for Box<HandshakeError> {
  fn from(v: (io::Error, String)) -> Box<HandshakeError> {
    Box::new(HandshakeError::RecvError(v.0, v.1))
  }
}

impl From<io::Error> for Box<HandshakeError> {
  fn from(v: io::Error) -> Box<HandshakeError> {
    Box::new(HandshakeError::LaunchError(v))
  }
}

impl Error for HandshakeError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      Self::SendError(ref s) => Some(s),
      Self::RecvError(ref s, _) => Some(s),
      Self::LaunchError(ref s) => Some(s),
      Self::UnexpectedResponse(_) => None,
    }
  }
}

impl Display for HandshakeError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      Self::SendError(ref s) => write!(fmt, "Error sending handshake '{s}'"),
      Self::RecvError(ref s, ref output) => {
        write!(
          fmt,
          "Error receiving handshake response '{s}'\n\
             Unexpected output:\n{output}"
        )
      }
      Self::LaunchError(ref s) => write!(fmt, "Error launching nvim '{s}'"),
      Self::UnexpectedResponse(ref output) => write!(
        fmt,
        "Error receiving handshake response, unexpected output:\n{output}"
      ),
    }
  }
}
