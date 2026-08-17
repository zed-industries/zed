//! WebSocket handshake control.

pub mod client;
pub mod headers;
pub mod machine;
pub mod server;

use std::{
    error::Error as ErrorTrait,
    fmt,
    io::{Read, Write},
};

use sha1::{Digest, Sha1};

use self::machine::{HandshakeMachine, RoundResult, StageResult, TryParse};
use crate::error::Error;

/// A WebSocket handshake.
#[derive(Debug)]
pub struct MidHandshake<Role: HandshakeRole> {
    role: Role,
    machine: HandshakeMachine<Role::InternalStream>,
}

impl<Role: HandshakeRole> MidHandshake<Role> {
    /// Allow access to machine
    pub fn get_ref(&self) -> &HandshakeMachine<Role::InternalStream> {
        &self.machine
    }

    /// Allow mutable access to machine
    pub fn get_mut(&mut self) -> &mut HandshakeMachine<Role::InternalStream> {
        &mut self.machine
    }

    /// Restarts the handshake process.
    pub fn handshake(mut self) -> Result<Role::FinalResult, HandshakeError<Role>> {
        let mut mach = self.machine;
        loop {
            mach = match mach.single_round()? {
                RoundResult::WouldBlock(m) => {
                    return Err(HandshakeError::Interrupted(MidHandshake { machine: m, ..self }))
                }
                RoundResult::Incomplete(m) => m,
                RoundResult::StageFinished(s) => match self.role.stage_finished(s)? {
                    ProcessingResult::Continue(m) => m,
                    ProcessingResult::Done(result) => return Ok(result),
                },
            }
        }
    }
}

/// A handshake result.
pub enum HandshakeError<Role: HandshakeRole> {
    /// Handshake was interrupted (would block).
    Interrupted(MidHandshake<Role>),
    /// Handshake failed.
    Failure(Error),
}

impl<Role: HandshakeRole> fmt::Debug for HandshakeError<Role> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HandshakeError::Interrupted(_) => write!(f, "HandshakeError::Interrupted(...)"),
            HandshakeError::Failure(ref e) => write!(f, "HandshakeError::Failure({e:?})"),
        }
    }
}

impl<Role: HandshakeRole> fmt::Display for HandshakeError<Role> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HandshakeError::Interrupted(_) => write!(f, "Interrupted handshake (WouldBlock)"),
            HandshakeError::Failure(ref e) => write!(f, "{e}"),
        }
    }
}

impl<Role: HandshakeRole> ErrorTrait for HandshakeError<Role> {}

impl<Role: HandshakeRole> From<Error> for HandshakeError<Role> {
    fn from(err: Error) -> Self {
        HandshakeError::Failure(err)
    }
}

/// Handshake role.
pub trait HandshakeRole {
    #[doc(hidden)]
    type IncomingData: TryParse;
    #[doc(hidden)]
    type InternalStream: Read + Write;
    #[doc(hidden)]
    type FinalResult;
    #[doc(hidden)]
    fn stage_finished(
        &mut self,
        finish: StageResult<Self::IncomingData, Self::InternalStream>,
    ) -> Result<ProcessingResult<Self::InternalStream, Self::FinalResult>, Error>;
}

/// Stage processing result.
#[doc(hidden)]
#[derive(Debug)]
pub enum ProcessingResult<Stream, FinalResult> {
    Continue(HandshakeMachine<Stream>),
    Done(FinalResult),
}

/// Derive the `Sec-WebSocket-Accept` response header from a `Sec-WebSocket-Key` request header.
///
/// This function can be used to perform a handshake before passing a raw TCP stream to
/// [`WebSocket::from_raw_socket`][crate::protocol::WebSocket::from_raw_socket].
pub fn derive_accept_key(request_key: &[u8]) -> String {
    // ... field is constructed by concatenating /key/ ...
    // ... with the string "258EAFA5-E914-47DA-95CA-C5AB0DC85B11" (RFC 6455)
    const WS_GUID: &[u8] = b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    let mut sha1 = Sha1::default();
    sha1.update(request_key);
    sha1.update(WS_GUID);
    data_encoding::BASE64.encode(&sha1.finalize())
}

#[cfg(test)]
mod tests {
    use super::derive_accept_key;

    #[test]
    fn key_conversion() {
        // example from RFC 6455
        assert_eq!(derive_accept_key(b"dGhlIHNhbXBsZSBub25jZQ=="), "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=");
    }
}
