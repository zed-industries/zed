//! Error and Result module.

#[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
use crate::client::connect::Connected;
use std::error::Error as StdError;
use std::fmt;

/// Result type often returned from methods that can have hyper `Error`s.
pub type Result<T> = std::result::Result<T, Error>;

type Cause = Box<dyn StdError + Send + Sync>;

/// Represents errors that can occur handling HTTP streams.
pub struct Error {
    inner: Box<ErrorImpl>,
}

struct ErrorImpl {
    kind: Kind,
    cause: Option<Cause>,
    #[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
    connect_info: Option<Connected>,
}

#[derive(Debug)]
pub(super) enum Kind {
    Parse(Parse),
    User(User),
    /// A message reached EOF, but is not complete.
    #[allow(unused)]
    IncompleteMessage,
    /// A connection received a message (or bytes) when not waiting for one.
    #[cfg(feature = "http1")]
    UnexpectedMessage,
    /// A pending item was dropped before ever being processed.
    Canceled,
    /// Indicates a channel (client or body sender) is closed.
    ChannelClosed,
    /// An `io::Error` that occurred while trying to read or write to a network stream.
    #[cfg(any(feature = "http1", feature = "http2"))]
    Io,
    /// Error occurred while connecting.
    #[allow(unused)]
    Connect,
    /// Error creating a TcpListener.
    #[cfg(all(feature = "tcp", feature = "server"))]
    Listen,
    /// Error accepting on an Incoming stream.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    Accept,
    /// User took too long to send headers
    #[cfg(all(feature = "http1", feature = "server", feature = "runtime"))]
    HeaderTimeout,
    /// Error while reading a body from connection.
    #[cfg(any(feature = "http1", feature = "http2", feature = "stream"))]
    Body,
    /// Error while writing a body to connection.
    #[cfg(any(feature = "http1", feature = "http2"))]
    BodyWrite,
    /// Error calling AsyncWrite::shutdown()
    #[cfg(feature = "http1")]
    Shutdown,

    /// A general error from h2.
    #[cfg(feature = "http2")]
    Http2,
}

#[derive(Debug)]
pub(super) enum Parse {
    Method,
    Version,
    #[cfg(feature = "http1")]
    VersionH2,
    Uri,
    #[cfg_attr(not(all(feature = "http1", feature = "server")), allow(unused))]
    UriTooLong,
    Header(Header),
    TooLarge,
    Status,
    #[cfg_attr(debug_assertions, allow(unused))]
    Internal,
}

#[derive(Debug)]
pub(super) enum Header {
    Token,
    #[cfg(feature = "http1")]
    ContentLengthInvalid,
    #[cfg(all(feature = "http1", feature = "server"))]
    TransferEncodingInvalid,
    #[cfg(feature = "http1")]
    TransferEncodingUnexpected,
}

#[derive(Debug)]
pub(super) enum User {
    /// Error calling user's HttpBody::poll_data().
    #[cfg(any(feature = "http1", feature = "http2"))]
    Body,
    /// The user aborted writing of the outgoing body.
    BodyWriteAborted,
    /// Error calling user's MakeService.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    MakeService,
    /// Error from future of user's Service.
    #[cfg(any(feature = "http1", feature = "http2"))]
    Service,
    /// User tried to send a certain header in an unexpected context.
    ///
    /// For example, sending both `content-length` and `transfer-encoding`.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    UnexpectedHeader,
    /// User tried to create a Request with bad version.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    UnsupportedVersion,
    /// User tried to create a CONNECT Request with the Client.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    UnsupportedRequestMethod,
    /// User tried to respond with a 1xx (not 101) response code.
    #[cfg(feature = "http1")]
    #[cfg(feature = "server")]
    UnsupportedStatusCode,
    /// User tried to send a Request with Client with non-absolute URI.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    AbsoluteUriRequired,

    /// User tried polling for an upgrade that doesn't exist.
    NoUpgrade,

    /// User polled for an upgrade, but low-level API is not using upgrades.
    #[cfg(feature = "http1")]
    ManualUpgrade,

    /// User called `server::Connection::without_shutdown()` on an HTTP/2 conn.
    #[cfg(feature = "server")]
    WithoutShutdownNonHttp1,

    /// The dispatch task is gone.
    #[cfg(feature = "client")]
    DispatchGone,

    /// User aborted in an FFI callback.
    #[cfg(feature = "ffi")]
    AbortedByCallback,
}

// Sentinel type to indicate the error was caused by a timeout.
#[derive(Debug)]
pub(super) struct TimedOut;

impl Error {
    /// Returns true if this was an HTTP parse error.
    pub fn is_parse(&self) -> bool {
        matches!(self.inner.kind, Kind::Parse(_))
    }

    /// Returns true if this was an HTTP parse error caused by a message that was too large.
    pub fn is_parse_too_large(&self) -> bool {
        matches!(
            self.inner.kind,
            Kind::Parse(Parse::TooLarge) | Kind::Parse(Parse::UriTooLong)
        )
    }

    /// Returns true if this was an HTTP parse error caused by an invalid response status code or
    /// reason phrase.
    pub fn is_parse_status(&self) -> bool {
        matches!(self.inner.kind, Kind::Parse(Parse::Status))
    }

    /// Returns true if this error was caused by user code.
    pub fn is_user(&self) -> bool {
        matches!(self.inner.kind, Kind::User(_))
    }

    /// Returns true if this was about a `Request` that was canceled.
    pub fn is_canceled(&self) -> bool {
        matches!(self.inner.kind, Kind::Canceled)
    }

    /// Returns true if a sender's channel is closed.
    pub fn is_closed(&self) -> bool {
        matches!(self.inner.kind, Kind::ChannelClosed)
    }

    /// Returns true if this was an error from `Connect`.
    pub fn is_connect(&self) -> bool {
        matches!(self.inner.kind, Kind::Connect)
    }

    /// Returns true if the connection closed before a message could complete.
    pub fn is_incomplete_message(&self) -> bool {
        matches!(self.inner.kind, Kind::IncompleteMessage)
    }

    /// Returns true if the body write was aborted.
    pub fn is_body_write_aborted(&self) -> bool {
        matches!(self.inner.kind, Kind::User(User::BodyWriteAborted))
    }

    /// Returns true if the error was caused by a timeout.
    pub fn is_timeout(&self) -> bool {
        self.find_source::<TimedOut>().is_some()
    }

    /// Consumes the error, returning its cause.
    pub fn into_cause(self) -> Option<Box<dyn StdError + Send + Sync>> {
        self.inner.cause
    }

    /// Returns the info of the client connection on which this error occurred.
    #[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
    pub fn client_connect_info(&self) -> Option<&Connected> {
        self.inner.connect_info.as_ref()
    }

    pub(super) fn new(kind: Kind) -> Error {
        Error {
            inner: Box::new(ErrorImpl {
                kind,
                cause: None,
                #[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
                connect_info: None,
            }),
        }
    }

    pub(super) fn with<C: Into<Cause>>(mut self, cause: C) -> Error {
        self.inner.cause = Some(cause.into());
        self
    }

    #[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
    pub(super) fn with_client_connect_info(mut self, connect_info: Connected) -> Error {
        self.inner.connect_info = Some(connect_info);
        self
    }

    #[cfg(any(all(feature = "http1", feature = "server"), feature = "ffi"))]
    pub(super) fn kind(&self) -> &Kind {
        &self.inner.kind
    }

    pub(crate) fn find_source<E: StdError + 'static>(&self) -> Option<&E> {
        let mut cause = self.source();
        while let Some(err) = cause {
            if let Some(ref typed) = err.downcast_ref() {
                return Some(typed);
            }
            cause = err.source();
        }

        // else
        None
    }

    #[cfg(feature = "http2")]
    pub(super) fn h2_reason(&self) -> h2::Reason {
        // Find an h2::Reason somewhere in the cause stack, if it exists,
        // otherwise assume an INTERNAL_ERROR.
        self.find_source::<h2::Error>()
            .and_then(|h2_err| h2_err.reason())
            .unwrap_or(h2::Reason::INTERNAL_ERROR)
    }

    pub(super) fn new_canceled() -> Error {
        Error::new(Kind::Canceled)
    }

    #[cfg(feature = "http1")]
    pub(super) fn new_incomplete() -> Error {
        Error::new(Kind::IncompleteMessage)
    }

    #[cfg(feature = "http1")]
    pub(super) fn new_too_large() -> Error {
        Error::new(Kind::Parse(Parse::TooLarge))
    }

    #[cfg(feature = "http1")]
    pub(super) fn new_version_h2() -> Error {
        Error::new(Kind::Parse(Parse::VersionH2))
    }

    #[cfg(feature = "http1")]
    pub(super) fn new_unexpected_message() -> Error {
        Error::new(Kind::UnexpectedMessage)
    }

    #[cfg(any(feature = "http1", feature = "http2"))]
    pub(super) fn new_io(cause: std::io::Error) -> Error {
        Error::new(Kind::Io).with(cause)
    }

    #[cfg(all(feature = "server", feature = "tcp"))]
    pub(super) fn new_listen<E: Into<Cause>>(cause: E) -> Error {
        Error::new(Kind::Listen).with(cause)
    }

    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    pub(super) fn new_accept<E: Into<Cause>>(cause: E) -> Error {
        Error::new(Kind::Accept).with(cause)
    }

    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    pub(super) fn new_connect<E: Into<Cause>>(cause: E) -> Error {
        Error::new(Kind::Connect).with(cause)
    }

    pub(super) fn new_closed() -> Error {
        Error::new(Kind::ChannelClosed)
    }

    #[cfg(any(feature = "http1", feature = "http2", feature = "stream"))]
    pub(super) fn new_body<E: Into<Cause>>(cause: E) -> Error {
        Error::new(Kind::Body).with(cause)
    }

    #[cfg(any(feature = "http1", feature = "http2"))]
    pub(super) fn new_body_write<E: Into<Cause>>(cause: E) -> Error {
        Error::new(Kind::BodyWrite).with(cause)
    }

    pub(super) fn new_body_write_aborted() -> Error {
        Error::new(Kind::User(User::BodyWriteAborted))
    }

    fn new_user(user: User) -> Error {
        Error::new(Kind::User(user))
    }

    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    pub(super) fn new_user_header() -> Error {
        Error::new_user(User::UnexpectedHeader)
    }

    #[cfg(all(feature = "http1", feature = "server", feature = "runtime"))]
    pub(super) fn new_header_timeout() -> Error {
        Error::new(Kind::HeaderTimeout)
    }

    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    pub(super) fn new_user_unsupported_version() -> Error {
        Error::new_user(User::UnsupportedVersion)
    }

    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    pub(super) fn new_user_unsupported_request_method() -> Error {
        Error::new_user(User::UnsupportedRequestMethod)
    }

    #[cfg(feature = "http1")]
    #[cfg(feature = "server")]
    pub(super) fn new_user_unsupported_status_code() -> Error {
        Error::new_user(User::UnsupportedStatusCode)
    }

    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    pub(super) fn new_user_absolute_uri_required() -> Error {
        Error::new_user(User::AbsoluteUriRequired)
    }

    pub(super) fn new_user_no_upgrade() -> Error {
        Error::new_user(User::NoUpgrade)
    }

    #[cfg(feature = "http1")]
    pub(super) fn new_user_manual_upgrade() -> Error {
        Error::new_user(User::ManualUpgrade)
    }

    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    pub(super) fn new_user_make_service<E: Into<Cause>>(cause: E) -> Error {
        Error::new_user(User::MakeService).with(cause)
    }

    #[cfg(any(feature = "http1", feature = "http2"))]
    pub(super) fn new_user_service<E: Into<Cause>>(cause: E) -> Error {
        Error::new_user(User::Service).with(cause)
    }

    #[cfg(any(feature = "http1", feature = "http2"))]
    pub(super) fn new_user_body<E: Into<Cause>>(cause: E) -> Error {
        Error::new_user(User::Body).with(cause)
    }

    #[cfg(feature = "server")]
    pub(super) fn new_without_shutdown_not_h1() -> Error {
        Error::new(Kind::User(User::WithoutShutdownNonHttp1))
    }

    #[cfg(feature = "http1")]
    pub(super) fn new_shutdown(cause: std::io::Error) -> Error {
        Error::new(Kind::Shutdown).with(cause)
    }

    #[cfg(feature = "ffi")]
    pub(super) fn new_user_aborted_by_callback() -> Error {
        Error::new_user(User::AbortedByCallback)
    }

    #[cfg(feature = "client")]
    pub(super) fn new_user_dispatch_gone() -> Error {
        Error::new(Kind::User(User::DispatchGone))
    }

    #[cfg(feature = "http2")]
    pub(super) fn new_h2(cause: ::h2::Error) -> Error {
        if cause.is_io() {
            Error::new_io(cause.into_io().expect("h2::Error::is_io"))
        } else {
            Error::new(Kind::Http2).with(cause)
        }
    }

    /// The error's standalone message, without the message from the source.
    pub fn message(&self) -> impl fmt::Display + '_ {
        self.description()
    }

    fn description(&self) -> &str {
        match self.inner.kind {
            Kind::Parse(Parse::Method) => "invalid HTTP method parsed",
            Kind::Parse(Parse::Version) => "invalid HTTP version parsed",
            #[cfg(feature = "http1")]
            Kind::Parse(Parse::VersionH2) => "invalid HTTP version parsed (found HTTP2 preface)",
            Kind::Parse(Parse::Uri) => "invalid URI",
            Kind::Parse(Parse::UriTooLong) => "URI too long",
            Kind::Parse(Parse::Header(Header::Token)) => "invalid HTTP header parsed",
            #[cfg(feature = "http1")]
            Kind::Parse(Parse::Header(Header::ContentLengthInvalid)) => {
                "invalid content-length parsed"
            }
            #[cfg(all(feature = "http1", feature = "server"))]
            Kind::Parse(Parse::Header(Header::TransferEncodingInvalid)) => {
                "invalid transfer-encoding parsed"
            }
            #[cfg(feature = "http1")]
            Kind::Parse(Parse::Header(Header::TransferEncodingUnexpected)) => {
                "unexpected transfer-encoding parsed"
            }
            Kind::Parse(Parse::TooLarge) => "message head is too large",
            Kind::Parse(Parse::Status) => "invalid HTTP status-code parsed",
            Kind::Parse(Parse::Internal) => {
                "internal error inside Hyper and/or its dependencies, please report"
            }
            Kind::IncompleteMessage => "connection closed before message completed",
            #[cfg(feature = "http1")]
            Kind::UnexpectedMessage => "received unexpected message from connection",
            Kind::ChannelClosed => "channel closed",
            Kind::Connect => "error trying to connect",
            Kind::Canceled => "operation was canceled",
            #[cfg(all(feature = "server", feature = "tcp"))]
            Kind::Listen => "error creating server listener",
            #[cfg(any(feature = "http1", feature = "http2"))]
            #[cfg(feature = "server")]
            Kind::Accept => "error accepting connection",
            #[cfg(all(feature = "http1", feature = "server", feature = "runtime"))]
            Kind::HeaderTimeout => "read header from client timeout",
            #[cfg(any(feature = "http1", feature = "http2", feature = "stream"))]
            Kind::Body => "error reading a body from connection",
            #[cfg(any(feature = "http1", feature = "http2"))]
            Kind::BodyWrite => "error writing a body to connection",
            #[cfg(feature = "http1")]
            Kind::Shutdown => "error shutting down connection",
            #[cfg(feature = "http2")]
            Kind::Http2 => "http2 error",
            #[cfg(any(feature = "http1", feature = "http2"))]
            Kind::Io => "connection error",

            #[cfg(any(feature = "http1", feature = "http2"))]
            Kind::User(User::Body) => "error from user's HttpBody stream",
            Kind::User(User::BodyWriteAborted) => "user body write aborted",
            #[cfg(any(feature = "http1", feature = "http2"))]
            #[cfg(feature = "server")]
            Kind::User(User::MakeService) => "error from user's MakeService",
            #[cfg(any(feature = "http1", feature = "http2"))]
            Kind::User(User::Service) => "error from user's Service",
            #[cfg(any(feature = "http1", feature = "http2"))]
            #[cfg(feature = "server")]
            Kind::User(User::UnexpectedHeader) => "user sent unexpected header",
            #[cfg(any(feature = "http1", feature = "http2"))]
            #[cfg(feature = "client")]
            Kind::User(User::UnsupportedVersion) => "request has unsupported HTTP version",
            #[cfg(any(feature = "http1", feature = "http2"))]
            #[cfg(feature = "client")]
            Kind::User(User::UnsupportedRequestMethod) => "request has unsupported HTTP method",
            #[cfg(feature = "http1")]
            #[cfg(feature = "server")]
            Kind::User(User::UnsupportedStatusCode) => {
                "response has 1xx status code, not supported by server"
            }
            #[cfg(any(feature = "http1", feature = "http2"))]
            #[cfg(feature = "client")]
            Kind::User(User::AbsoluteUriRequired) => "client requires absolute-form URIs",
            Kind::User(User::NoUpgrade) => "no upgrade available",
            #[cfg(feature = "http1")]
            Kind::User(User::ManualUpgrade) => "upgrade expected but low level API in use",
            #[cfg(feature = "server")]
            Kind::User(User::WithoutShutdownNonHttp1) => {
                "without_shutdown() called on a non-HTTP/1 connection"
            }
            #[cfg(feature = "client")]
            Kind::User(User::DispatchGone) => "dispatch task is gone",
            #[cfg(feature = "ffi")]
            Kind::User(User::AbortedByCallback) => "operation aborted by an application callback",
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_tuple("hyper::Error");
        f.field(&self.inner.kind);
        if let Some(ref cause) = self.inner.cause {
            f.field(cause);
        }
        f.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref cause) = self.inner.cause {
            write!(f, "{}: {}", self.description(), cause)
        } else {
            f.write_str(self.description())
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner
            .cause
            .as_ref()
            .map(|cause| &**cause as &(dyn StdError + 'static))
    }
}

#[doc(hidden)]
impl From<Parse> for Error {
    fn from(err: Parse) -> Error {
        Error::new(Kind::Parse(err))
    }
}

#[cfg(feature = "http1")]
impl Parse {
    pub(crate) fn content_length_invalid() -> Self {
        Parse::Header(Header::ContentLengthInvalid)
    }

    #[cfg(all(feature = "http1", feature = "server"))]
    pub(crate) fn transfer_encoding_invalid() -> Self {
        Parse::Header(Header::TransferEncodingInvalid)
    }

    pub(crate) fn transfer_encoding_unexpected() -> Self {
        Parse::Header(Header::TransferEncodingUnexpected)
    }
}

impl From<httparse::Error> for Parse {
    fn from(err: httparse::Error) -> Parse {
        match err {
            httparse::Error::HeaderName
            | httparse::Error::HeaderValue
            | httparse::Error::NewLine
            | httparse::Error::Token => Parse::Header(Header::Token),
            httparse::Error::Status => Parse::Status,
            httparse::Error::TooManyHeaders => Parse::TooLarge,
            httparse::Error::Version => Parse::Version,
        }
    }
}

impl From<http::method::InvalidMethod> for Parse {
    fn from(_: http::method::InvalidMethod) -> Parse {
        Parse::Method
    }
}

impl From<http::status::InvalidStatusCode> for Parse {
    fn from(_: http::status::InvalidStatusCode) -> Parse {
        Parse::Status
    }
}

impl From<http::uri::InvalidUri> for Parse {
    fn from(_: http::uri::InvalidUri) -> Parse {
        Parse::Uri
    }
}

impl From<http::uri::InvalidUriParts> for Parse {
    fn from(_: http::uri::InvalidUriParts) -> Parse {
        Parse::Uri
    }
}

#[doc(hidden)]
trait AssertSendSync: Send + Sync + 'static {}
#[doc(hidden)]
impl AssertSendSync for Error {}

// ===== impl TimedOut ====

impl fmt::Display for TimedOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("operation timed out")
    }
}

impl StdError for TimedOut {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn error_size_of() {
        assert_eq!(mem::size_of::<Error>(), mem::size_of::<usize>());
    }

    #[cfg(feature = "http2")]
    #[test]
    fn h2_reason_unknown() {
        let closed = Error::new_closed();
        assert_eq!(closed.h2_reason(), h2::Reason::INTERNAL_ERROR);
    }

    #[cfg(feature = "http2")]
    #[test]
    fn h2_reason_one_level() {
        let body_err = Error::new_user_body(h2::Error::from(h2::Reason::ENHANCE_YOUR_CALM));
        assert_eq!(body_err.h2_reason(), h2::Reason::ENHANCE_YOUR_CALM);
    }

    #[cfg(feature = "http2")]
    #[test]
    fn h2_reason_nested() {
        let recvd = Error::new_h2(h2::Error::from(h2::Reason::HTTP_1_1_REQUIRED));
        // Suppose a user were proxying the received error
        let svc_err = Error::new_user_service(recvd);
        assert_eq!(svc_err.h2_reason(), h2::Reason::HTTP_1_1_REQUIRED);
    }
}
