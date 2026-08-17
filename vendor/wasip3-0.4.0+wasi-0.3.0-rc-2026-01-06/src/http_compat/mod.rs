//! Extension types for wasip3::http

pub use body_writer::*;
mod body_writer;

pub use conversions::*;
mod conversions;

use crate::{
    http::types::{self, ErrorCode},
    wit_bindgen::{self, StreamResult},
    wit_future,
};
use bytes::Bytes;
use http_body::SizeHint;
use std::{
    pin::Pin,
    task::{self, Poll},
};

const READ_FRAME_SIZE: usize = 16 * 1024;

/// The body type used for incoming HTTP requests.
///
/// This is a type alias for [`IncomingBody`] specialized with
/// [`types::Request`], representing the structured payload of an
/// inbound request as it is received and processed.
///
/// This type is typically used by components that consume HTTP
/// requests and need to read or stream the request body.
pub type IncomingRequestBody = IncomingBody<types::Request>;

/// The body type used for incoming HTTP responses.
///
/// This is a type alias for [`IncomingBody`] specialized with
/// [`types::Response`], representing the structured payload of an
/// inbound response as it is received and processed.
///
/// This type is typically used by components that handle HTTP
/// responses and need to read or stream the response body.
pub type IncomingResponseBody = IncomingBody<types::Response>;

/// A type alias for an HTTP request with a customizable body type.
///
/// This is a convenience wrapper around [`http::Request`], parameterized
/// by the body type `T`. By default, it uses [`IncomingRequestBody`],
/// which represents the standard incoming body used by this runtime.
///
/// # See also
/// - [`IncomingRequestBody`]: The body type for inbound HTTP requests.
/// - [`http::Request`]: The standard HTTP request type from the `http` crate.
pub type Request<T = IncomingRequestBody> = http::Request<T>;

/// A type alias for an HTTP response with a customizable body type.
///
/// This is a convenience wrapper around [`http::Response`], parameterized
/// by the body type `T`. By default, it uses [`IncomingResponseBody`],
/// which represents the standard incoming body type used by this runtime.
///
/// # See also
/// - [`IncomingResponseBody`]: The body type for inbound HTTP responses.
/// - [`http::Response`]: The standard HTTP response type from the `http` crate.
pub type Response<T = IncomingResponseBody> = http::Response<T>;

/// A wrapper around [`types::RequestOptions`] that implements [`Clone`]
pub struct RequestOptionsExtension(pub types::RequestOptions);

impl Clone for RequestOptionsExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// Internal trait representing a readable inbound HTTP message.
///
/// Implemented by types that expose request or response headers
/// and provide mechanisms to consume the message body.
pub trait IncomingMessage: Unpin {
    fn get_headers(&self) -> types::Headers;

    fn consume_body(
        self,
        res: wit_bindgen::FutureReader<Result<(), ErrorCode>>,
    ) -> (
        wit_bindgen::StreamReader<u8>,
        wit_bindgen::FutureReader<Result<Option<types::Trailers>, ErrorCode>>,
    );
}

impl IncomingMessage for types::Request {
    fn get_headers(&self) -> types::Headers {
        self.get_headers()
    }

    fn consume_body(
        self,
        res: wit_bindgen::FutureReader<Result<(), ErrorCode>>,
    ) -> (
        wit_bindgen::StreamReader<u8>,
        wit_bindgen::FutureReader<Result<Option<types::Trailers>, ErrorCode>>,
    ) {
        Self::consume_body(self, res)
    }
}

impl IncomingMessage for types::Response {
    fn get_headers(&self) -> types::Headers {
        self.get_headers()
    }

    fn consume_body(
        self,
        res: wit_bindgen::FutureReader<Result<(), ErrorCode>>,
    ) -> (
        wit_bindgen::StreamReader<u8>,
        wit_bindgen::FutureReader<Result<Option<types::Trailers>, ErrorCode>>,
    ) {
        Self::consume_body(self, res)
    }
}

/// A stream of Bytes, used when receiving bodies from the network.
pub struct IncomingBody<T> {
    state: StartedState<T>,
    content_length: Option<u64>,
}

enum StartedState<T> {
    Unstarted(T),
    Started {
        #[allow(dead_code)]
        result: wit_bindgen::FutureWriter<Result<(), ErrorCode>>,
        state: IncomingState,
    },
    Empty,
}

impl<T: IncomingMessage> IncomingBody<T> {
    /// Creates a new [`IncomingBody`] from the given incoming message.
    ///
    /// This initializes the body in an *unstarted* state and extracts the
    /// `Content-Length` header if present. The resulting instance can later
    /// transition into a streaming or consumed state.
    ///
    /// Returns an [`ErrorCode`] if the content length is invalid or cannot
    /// be determined.
    pub fn new(msg: T) -> Result<Self, ErrorCode> {
        let content_length = get_content_length(msg.get_headers())?;
        Ok(Self {
            state: StartedState::Unstarted(msg),
            content_length,
        })
    }

    /// Takes ownership of the inner message if the body has not yet been started.
    ///
    /// This method replaces the internal state with [`StartedState::Empty`]
    /// and returns the contained message if it was still in the
    /// *unstarted* state. Returns `None` otherwise.
    pub fn take_unstarted(&mut self) -> Option<T> {
        match self.state {
            StartedState::Unstarted(_) => {
                let StartedState::Unstarted(msg) =
                    std::mem::replace(&mut self.state, StartedState::Empty)
                else {
                    unreachable!();
                };
                Some(msg)
            }
            _ => None,
        }
    }

    fn ensure_started(&mut self) -> Result<&mut IncomingState, ErrorCode> {
        if let StartedState::Unstarted(_) = self.state {
            let msg = self.take_unstarted().unwrap();
            let (result, reader) = wit_future::new(|| Ok(()));
            let (stream, trailers) = msg.consume_body(reader);
            self.state = StartedState::Started {
                result,
                state: IncomingState::Ready { stream, trailers },
            };
        };
        match &mut self.state {
            StartedState::Started { state, .. } => Ok(state),
            StartedState::Unstarted(_) => unreachable!(),
            StartedState::Empty => Err(to_internal_error_code(
                "cannot use IncomingBody after call to take_unstarted",
            )),
        }
    }
}

enum IncomingState {
    Ready {
        stream: wit_bindgen::StreamReader<u8>,
        trailers: wit_bindgen::FutureReader<Result<Option<types::Trailers>, ErrorCode>>,
    },
    Reading(Pin<Box<dyn std::future::Future<Output = ReadResult> + 'static + Send>>),
    Done,
}

enum ReadResult {
    Trailers(Result<Option<types::Trailers>, ErrorCode>),
    BodyChunk {
        chunk: Vec<u8>,
        stream: wit_bindgen::StreamReader<u8>,
        trailers: wit_bindgen::FutureReader<Result<Option<types::Trailers>, ErrorCode>>,
    },
}

impl<T: IncomingMessage> http_body::Body for IncomingBody<T> {
    type Data = Bytes;
    type Error = ErrorCode;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> Poll<Option<Result<http_body::Frame<Self::Data>, Self::Error>>> {
        let state = self.ensure_started()?;
        loop {
            match state {
                IncomingState::Ready { .. } => {
                    let IncomingState::Ready {
                        mut stream,
                        trailers,
                    } = std::mem::replace(state, IncomingState::Done)
                    else {
                        unreachable!();
                    };
                    *state = IncomingState::Reading(Box::pin(async move {
                        let (result, chunk) =
                            stream.read(Vec::with_capacity(READ_FRAME_SIZE)).await;
                        match result {
                            StreamResult::Complete(_n) => ReadResult::BodyChunk {
                                chunk,
                                stream,
                                trailers,
                            },
                            StreamResult::Cancelled => unreachable!(),
                            StreamResult::Dropped => ReadResult::Trailers(trailers.await),
                        }
                    }));
                }
                IncomingState::Reading(future) => {
                    match std::task::ready!(future.as_mut().poll(cx)) {
                        ReadResult::BodyChunk {
                            chunk,
                            stream,
                            trailers,
                        } => {
                            *state = IncomingState::Ready { stream, trailers };
                            break Poll::Ready(Some(Ok(http_body::Frame::data(chunk.into()))));
                        }
                        ReadResult::Trailers(trailers) => {
                            *state = IncomingState::Done;
                            match trailers {
                                Ok(Some(fields)) => {
                                    let trailers = fields.try_into()?;
                                    break Poll::Ready(Some(Ok(http_body::Frame::trailers(
                                        trailers,
                                    ))));
                                }
                                Ok(None) => {}
                                Err(e) => {
                                    break Poll::Ready(Some(Err(e)));
                                }
                            }
                        }
                    }
                }
                IncomingState::Done => break Poll::Ready(None),
            }
        }
    }

    fn is_end_stream(&self) -> bool {
        matches!(
            self.state,
            StartedState::Started {
                state: IncomingState::Done,
                ..
            }
        )
    }

    fn size_hint(&self) -> SizeHint {
        let Some(n) = self.content_length else {
            return SizeHint::default();
        };
        let mut size_hint = SizeHint::new();
        size_hint.set_lower(0);
        size_hint.set_upper(n);
        size_hint
    }
}

fn get_content_length(headers: types::Headers) -> Result<Option<u64>, ErrorCode> {
    let values = headers.get(http::header::CONTENT_LENGTH.as_str());
    if values.len() > 1 {
        return Err(to_internal_error_code("multiple content-length values"));
    }
    let Some(value_bytes) = values.into_iter().next() else {
        return Ok(None);
    };
    let value_str = std::str::from_utf8(&value_bytes).map_err(to_internal_error_code)?;
    let value_i64: i64 = value_str.parse().map_err(to_internal_error_code)?;
    let value = value_i64.try_into().map_err(to_internal_error_code)?;
    Ok(Some(value))
}

fn to_internal_error_code(e: impl ::std::fmt::Display) -> ErrorCode {
    ErrorCode::InternalError(Some(e.to_string()))
}
