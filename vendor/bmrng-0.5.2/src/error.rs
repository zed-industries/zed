use std::error::Error;
use std::fmt;
use tokio::sync::mpsc::error::SendError as MpscSendError;
use tokio::sync::oneshot;

/// Error thrown when a [`RequestSender::send()`](crate::RequestSender::send()) or [`UnboundedRequestSender::send()`](crate::unbounded::UnboundedRequestSender::send())
/// call fails because the channel is closed
#[derive(Debug, PartialEq)]
pub struct SendError<T>(pub T);

impl<T> From<SendError<T>> for MpscSendError<T> {
    fn from(err: SendError<T>) -> Self {
        Self(err.0)
    }
}

impl<T> From<MpscSendError<T>> for SendError<T> {
    fn from(err: MpscSendError<T>) -> Self {
        Self(err.0)
    }
}

impl<T> fmt::Display for SendError<T> {
    #[cfg(not(tarpaulin_include))]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "channel closed")
    }
}

impl<T> Error for SendError<T> where T: fmt::Debug {}

/// Errors that can occur when a [`RequestReceiver`](crate::RequestReceiver)
/// or [`UnboundedReceiver`](crate::unbounded::UnboundedRequestReceiver) handles a request
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RequestError<T> {
    /// Error occurring when the channel from [`RequestSender`](crate::RequestSender) to [`RequestReceiver`](crate::RequestReceiver) is closed
    RecvError,
    /// Error occurring when the Responder fails to send a response before the timeout
    RecvTimeoutError,
    /// Error occurring when the channel from [`RequestReceiver`](crate::RequestReceiver) to [RequestSender](crate::RequestSender) is closed
    SendError(T),
}

/// Errors that can occur when a [`ResponseReceiver`](crate::ResponseReceiver) is
// waiting for a response
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ReceiveError {
    /// Error occurring when the channel from [`RequestSender`](crate::RequestSender) to [`RequestReceiver`](crate::RequestReceiver) is closed
    RecvError,
    /// Error occurring when the Responder fails to send a response before the timeout
    TimeoutError,
}

impl<T> From<SendError<T>> for RequestError<T> {
    fn from(err: SendError<T>) -> RequestError<T> {
        RequestError::SendError(err.0)
    }
}

impl<T> From<RespondError<T>> for RequestError<T> {
    fn from(err: RespondError<T>) -> RequestError<T> {
        RequestError::SendError(err.0)
    }
}

impl<T> From<ReceiveError> for RequestError<T> {
    fn from(err: ReceiveError) -> RequestError<T> {
        match err {
            ReceiveError::RecvError => RequestError::RecvError,
            ReceiveError::TimeoutError => RequestError::RecvTimeoutError,
        }
    }
}

impl From<oneshot::error::RecvError> for ReceiveError {
    fn from(_err: oneshot::error::RecvError) -> ReceiveError {
        ReceiveError::RecvError
    }
}

impl<T> fmt::Display for RequestError<T> {
    #[cfg(not(tarpaulin_include))]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                RequestError::RecvError => "request channel closed",
                RequestError::RecvTimeoutError => "request timed out",
                RequestError::SendError(..) => "channel closed",
            }
        )
    }
}

impl<T> Error for RequestError<T> where T: fmt::Debug {}

impl fmt::Display for ReceiveError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                ReceiveError::RecvError => "receive channel closed",
                ReceiveError::TimeoutError => "request timed out",
            }
        )
    }
}

impl Error for ReceiveError {}

/// Error thrown when a Responder fails to respond.
/// The channel was closed by the receiver, the original request sender
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RespondError<T>(pub T);

impl<T> fmt::Display for RespondError<T> {
    #[cfg(not(tarpaulin_include))]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "sender closed the response channel",)
    }
}

impl<T> Error for RespondError<T> where T: fmt::Debug {}

#[cfg(test)]
pub mod tests {
    pub use super::*;

    #[test]
    fn send_err_into_mpsc_send_error() {
        let err = SendError(42);
        let m_err: MpscSendError<u32> = err.into();
        assert_eq!(m_err.0, 42);
    }

    #[test]
    fn mpsc_send_err_into_send_error() {
        let err = MpscSendError(42);
        let m_err: SendError<u32> = err.into();
        assert_eq!(m_err.0, 42);
    }

    #[test]
    fn send_error_into_request_error() {
        let err = SendError(42);
        let r_err: RequestError<i32> = err.into();
        assert_eq!(r_err, RequestError::SendError(42));
    }

    #[test]
    fn reply_error_to_request_error() {
        let err = RespondError(21);
        let q_err: RequestError<i32> = err.into();
        assert_eq!(q_err, RequestError::SendError(21));
    }

    #[test]
    fn receive_error_display() {
        let err = ReceiveError::RecvError;
        assert_eq!("receive channel closed", err.to_string());
        let err = ReceiveError::TimeoutError;
        assert_eq!("request timed out", err.to_string());
    }
}
