use crate::codec::{CodecError, Message, ZmtpVersion};
use crate::endpoint::Endpoint;
use crate::endpoint::EndpointError;
use crate::task_handle::TaskError;
use crate::ZmqMessage;

use futures::channel::mpsc;
use thiserror::Error;

pub type ZmqResult<T> = Result<T, ZmqError>;

#[derive(Error, Debug)]
pub enum ZmqError {
    #[error("Endpoint Error: {0}")]
    Endpoint(#[from] EndpointError),
    #[error("Network Error: {0}")]
    Network(#[from] std::io::Error),
    #[error("Socket bind doesn't exist: {0}")]
    NoSuchBind(Endpoint),
    #[error("Codec Error: {0}")]
    Codec(#[from] CodecError),
    #[error("Socket Error: {0}")]
    Socket(&'static str),
    #[error("{0}")]
    BufferFull(&'static str),
    #[error("Failed to deliver message ({} frames) cause of {reason}", message.len())]
    ReturnToSender {
        reason: &'static str,
        message: ZmqMessage,
    },
    // TODO refactor this.
    // Most likely Message enum should be part of public API.
    // In such case we'll be able to use this enum to return both message and multipart message in
    // same type
    #[error("Failed to deliver messages ({} messages) cause of {reason}", messages.len())]
    ReturnToSenderMultipart {
        reason: &'static str,
        messages: Vec<ZmqMessage>,
    },
    #[error("Task Error: {0}")]
    Task(#[from] TaskError),
    #[error("{0}")]
    Other(&'static str),
    #[error("No message received")]
    NoMessage,
    #[error("Invalid peer identity: must be less than 256 bytes in length")]
    PeerIdentity,
    #[error("Unsupported ZMTP version")]
    UnsupportedVersion(ZmtpVersion),
}

impl From<mpsc::TrySendError<Message>> for ZmqError {
    fn from(_: mpsc::TrySendError<Message>) -> Self {
        ZmqError::BufferFull("Failed to send message. Send queue full/broken")
    }
}

impl From<mpsc::SendError> for ZmqError {
    fn from(_: mpsc::SendError) -> Self {
        ZmqError::BufferFull("Failed to send message. Send queue full/broken")
    }
}
