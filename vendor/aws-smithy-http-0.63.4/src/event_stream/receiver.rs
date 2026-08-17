/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_eventstream::frame::{
    DecodedFrame, MessageFrameDecoder, UnmarshallMessage, UnmarshalledMessage,
};
use aws_smithy_runtime_api::client::result::{ConnectorError, ResponseError, SdkError};
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::event_stream::{Message, RawMessage};
use bytes::Buf;
use bytes::Bytes;
use bytes_utils::SegmentedBuf;
use std::error::Error as StdError;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use tracing::trace;

/// Wrapper around SegmentedBuf that tracks the state of the stream.
#[derive(Debug)]
enum RecvBuf {
    /// Nothing has been buffered yet.
    Empty,
    /// Some data has been buffered.
    /// The SegmentedBuf will automatically purge when it reads off the end of a chunk boundary.
    Partial(SegmentedBuf<Bytes>),
    /// The end of the stream has been reached, but there may still be some buffered data.
    EosPartial(SegmentedBuf<Bytes>),
    /// An exception terminated this stream.
    Terminated,
}

impl RecvBuf {
    /// Returns true if there's more buffered data.
    fn has_data(&self) -> bool {
        match self {
            RecvBuf::Empty | RecvBuf::Terminated => false,
            RecvBuf::Partial(segments) | RecvBuf::EosPartial(segments) => segments.remaining() > 0,
        }
    }

    /// Returns true if the stream has ended.
    fn is_eos(&self) -> bool {
        matches!(self, RecvBuf::EosPartial(_) | RecvBuf::Terminated)
    }

    /// Returns a mutable reference to the underlying buffered data.
    fn buffered(&mut self) -> &mut SegmentedBuf<Bytes> {
        match self {
            RecvBuf::Empty => panic!("buffer must be populated before reading; this is a bug"),
            RecvBuf::Partial(segmented) => segmented,
            RecvBuf::EosPartial(segmented) => segmented,
            RecvBuf::Terminated => panic!("buffer has been terminated; this is a bug"),
        }
    }

    /// Returns a new `RecvBuf` with additional data buffered. This will only allocate
    /// if the `RecvBuf` was previously empty.
    fn with_partial(self, partial: Bytes) -> Self {
        match self {
            RecvBuf::Empty => {
                let mut segmented = SegmentedBuf::new();
                segmented.push(partial);
                RecvBuf::Partial(segmented)
            }
            RecvBuf::Partial(mut segmented) => {
                segmented.push(partial);
                RecvBuf::Partial(segmented)
            }
            RecvBuf::EosPartial(_) | RecvBuf::Terminated => {
                panic!("cannot buffer more data after the stream has ended or been terminated; this is a bug")
            }
        }
    }

    /// Returns a `RecvBuf` that has reached end of stream.
    fn ended(self) -> Self {
        match self {
            RecvBuf::Empty => RecvBuf::EosPartial(SegmentedBuf::new()),
            RecvBuf::Partial(segmented) => RecvBuf::EosPartial(segmented),
            RecvBuf::EosPartial(_) => panic!("already end of stream; this is a bug"),
            RecvBuf::Terminated => panic!("stream terminated; this is a bug"),
        }
    }
}

#[derive(Debug)]
enum ReceiverErrorKind {
    /// The stream ended before a complete message frame was received.
    UnexpectedEndOfStream,
}

/// An error that occurs within an event stream receiver.
#[derive(Debug)]
pub struct ReceiverError {
    kind: ReceiverErrorKind,
}

impl fmt::Display for ReceiverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ReceiverErrorKind::UnexpectedEndOfStream => write!(f, "unexpected end of stream"),
        }
    }
}

impl StdError for ReceiverError {}

/// Receives Smithy-modeled messages out of an Event Stream.
#[derive(Debug)]
pub struct Receiver<T, E> {
    unmarshaller: Box<dyn UnmarshallMessage<Output = T, Error = E> + Send + Sync>,
    decoder: MessageFrameDecoder,
    buffer: RecvBuf,
    body: SdkBody,
    /// Event Stream has optional initial response frames an with `:message-type` of
    /// `initial-response`. If `try_recv_initial()` is called and the next message isn't an
    /// initial response, then the message will be stored in `buffered_message` so that it can
    /// be returned with the next call of `recv()`.
    buffered_message: Option<Message>,
    _phantom: PhantomData<E>,
}

// Used by `Receiver::try_recv_initial`, hence this enum is also doc hidden
#[doc(hidden)]
#[non_exhaustive]
pub enum InitialMessageType {
    Request,
    Response,
}

impl InitialMessageType {
    fn as_str(&self) -> &'static str {
        match self {
            InitialMessageType::Request => "initial-request",
            InitialMessageType::Response => "initial-response",
        }
    }
}

impl<T, E> Receiver<T, E> {
    /// Creates a new `Receiver` with the given message unmarshaller and SDK body.
    pub fn new(
        unmarshaller: impl UnmarshallMessage<Output = T, Error = E> + Send + Sync + 'static,
        body: SdkBody,
    ) -> Self {
        Receiver {
            unmarshaller: Box::new(unmarshaller),
            decoder: MessageFrameDecoder::new(),
            buffer: RecvBuf::Empty,
            body,
            buffered_message: None,
            _phantom: Default::default(),
        }
    }

    fn unmarshall(&self, message: Message) -> Result<Option<T>, SdkError<E, RawMessage>> {
        match self.unmarshaller.unmarshall(&message) {
            Ok(unmarshalled) => match unmarshalled {
                UnmarshalledMessage::Event(event) => Ok(Some(event)),
                UnmarshalledMessage::Error(err) => {
                    Err(SdkError::service_error(err, RawMessage::Decoded(message)))
                }
            },
            Err(err) => Err(SdkError::response_error(err, RawMessage::Decoded(message))),
        }
    }

    async fn buffer_next_chunk(&mut self) -> Result<(), SdkError<E, RawMessage>> {
        use http_body_util::BodyExt;

        if !self.buffer.is_eos() {
            let next_chunk = self
                .body
                .frame()
                .await
                .transpose()
                .map_err(|err| SdkError::dispatch_failure(ConnectorError::io(err)))?;
            let buffer = mem::replace(&mut self.buffer, RecvBuf::Empty);
            if let Some(chunk) = next_chunk {
                // Ignoring the possibility of trailers here since event_streams don't have them
                if let Ok(data) = chunk.into_data() {
                    self.buffer = buffer.with_partial(data);
                }
            } else {
                self.buffer = buffer.ended();
            }
        }
        Ok(())
    }

    async fn next_message(&mut self) -> Result<Option<Message>, SdkError<E, RawMessage>> {
        while !self.buffer.is_eos() {
            if self.buffer.has_data() {
                if let DecodedFrame::Complete(message) = self
                    .decoder
                    .decode_frame(self.buffer.buffered())
                    .map_err(|err| {
                        SdkError::response_error(
                            err,
                            // the buffer has been consumed
                            RawMessage::Invalid(None),
                        )
                    })?
                {
                    trace!(message = ?message, "received complete event stream message");
                    return Ok(Some(message));
                }
            }

            self.buffer_next_chunk().await?;
        }
        if self.buffer.has_data() {
            trace!(remaining_data = ?self.buffer, "data left over in the event stream response stream");
            let buf = self.buffer.buffered();
            return Err(SdkError::response_error(
                ReceiverError {
                    kind: ReceiverErrorKind::UnexpectedEndOfStream,
                },
                RawMessage::invalid(Some(buf.copy_to_bytes(buf.remaining()))),
            ));
        }
        Ok(None)
    }

    /// Tries to receive the initial response message that has `:event-type` of a given `message_type`.
    /// If a different event type is received, then it is buffered and `Ok(None)` is returned.
    #[doc(hidden)]
    pub async fn try_recv_initial(
        &mut self,
        message_type: InitialMessageType,
    ) -> Result<Option<Message>, SdkError<E, RawMessage>> {
        self.try_recv_initial_with_preprocessor(message_type, |msg| Ok((msg, ())))
            .await
            .map(|opt| opt.map(|(msg, _)| msg))
    }

    /// Tries to receive the initial response message with preprocessing support.
    ///
    /// The preprocessor function can transform the raw message (e.g., unwrap envelopes)
    /// and return metadata alongside the transformed message. If the transformed message
    /// matches the expected `message_type`, both the message and metadata are returned.
    /// Otherwise, the transformed message is buffered and `Ok(None)` is returned.
    #[doc(hidden)]
    pub async fn try_recv_initial_with_preprocessor<F, M>(
        &mut self,
        message_type: InitialMessageType,
        preprocessor: F,
    ) -> Result<Option<(Message, M)>, SdkError<E, RawMessage>>
    where
        F: FnOnce(Message) -> Result<(Message, M), ResponseError<RawMessage>>,
    {
        if let Some(message) = self.next_message().await? {
            let (processed_message, metadata) =
                preprocessor(message.clone()).map_err(|err| SdkError::ResponseError(err))?;

            if let Some(event_type) = processed_message
                .headers()
                .iter()
                .find(|h| h.name().as_str() == ":event-type")
            {
                if event_type
                    .value()
                    .as_string()
                    .map(|s| s.as_str() == message_type.as_str())
                    .unwrap_or(false)
                {
                    return Ok(Some((processed_message, metadata)));
                }
            }
            // Buffer the processed message so that it can be returned by the next call to `recv()`
            self.buffered_message = Some(message);
        }
        Ok(None)
    }

    /// Asynchronously tries to receive a message from the stream. If the stream has ended,
    /// it returns an `Ok(None)`. If there is a transport layer error, it will return
    /// `Err(SdkError::DispatchFailure)`. Service-modeled errors will be a part of the returned
    /// messages.
    pub async fn recv(&mut self) -> Result<Option<T>, SdkError<E, RawMessage>> {
        if let Some(buffered) = self.buffered_message.take() {
            return match self.unmarshall(buffered) {
                Ok(message) => Ok(message),
                Err(error) => {
                    self.buffer = RecvBuf::Terminated;
                    Err(error)
                }
            };
        }
        if let Some(message) = self.next_message().await? {
            match self.unmarshall(message) {
                Ok(message) => Ok(message),
                Err(error) => {
                    self.buffer = RecvBuf::Terminated;
                    Err(error)
                }
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InitialMessageType, Receiver, UnmarshallMessage};
    use aws_smithy_eventstream::error::Error as EventStreamError;
    use aws_smithy_eventstream::frame::{write_message_to, UnmarshalledMessage};
    use aws_smithy_runtime_api::client::result::SdkError;
    use aws_smithy_types::body::SdkBody;
    use aws_smithy_types::event_stream::{Header, HeaderValue, Message};
    use bytes::Bytes;
    use http_body_1x::Frame;
    use std::error::Error as StdError;
    use std::io::{Error as IOError, ErrorKind};

    fn encode_initial_response() -> Bytes {
        let mut buffer = Vec::new();
        let message = Message::new(Bytes::new())
            .add_header(Header::new(
                ":message-type",
                HeaderValue::String("event".into()),
            ))
            .add_header(Header::new(
                ":event-type",
                HeaderValue::String("initial-response".into()),
            ));
        write_message_to(&message, &mut buffer).unwrap();
        buffer.into()
    }

    fn encode_message(message: &str) -> Bytes {
        let mut buffer = Vec::new();
        let message = Message::new(Bytes::copy_from_slice(message.as_bytes()));
        write_message_to(&message, &mut buffer).unwrap();
        buffer.into()
    }

    fn map_to_frame(stream: Vec<Result<Bytes, IOError>>) -> Vec<Result<Frame<Bytes>, IOError>> {
        stream
            .into_iter()
            .map(|chunk| chunk.map(Frame::data))
            .collect()
    }

    #[derive(Debug)]
    struct FakeError;
    impl std::fmt::Display for FakeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "FakeError")
        }
    }
    impl StdError for FakeError {}

    #[derive(Debug, Eq, PartialEq)]
    struct TestMessage(String);

    #[derive(Debug)]
    struct Unmarshaller;
    impl UnmarshallMessage for Unmarshaller {
        type Output = TestMessage;
        type Error = EventStreamError;

        fn unmarshall(
            &self,
            message: &Message,
        ) -> Result<UnmarshalledMessage<Self::Output, Self::Error>, EventStreamError> {
            Ok(UnmarshalledMessage::Event(TestMessage(
                std::str::from_utf8(&message.payload()[..]).unwrap().into(),
            )))
        }
    }

    #[tokio::test]
    async fn receive_success() {
        let chunks: Vec<Result<_, IOError>> =
            map_to_frame(vec![Ok(encode_message("one")), Ok(encode_message("two"))]);
        let chunk_stream = futures_util::stream::iter(chunks);
        let stream_body = http_body_util::StreamBody::new(chunk_stream);
        let body = SdkBody::from_body_1_x(stream_body);

        let mut receiver = Receiver::<TestMessage, EventStreamError>::new(Unmarshaller, body);

        assert_eq!(
            TestMessage("one".into()),
            receiver.recv().await.unwrap().unwrap()
        );
        assert_eq!(
            TestMessage("two".into()),
            receiver.recv().await.unwrap().unwrap()
        );
        assert_eq!(None, receiver.recv().await.unwrap());
    }

    #[tokio::test]
    async fn receive_last_chunk_empty() {
        let chunks: Vec<Result<_, IOError>> = map_to_frame(vec![
            Ok(encode_message("one")),
            Ok(encode_message("two")),
            Ok(Bytes::from_static(&[])),
        ]);
        let chunk_stream = futures_util::stream::iter(chunks);
        let stream_body = http_body_util::StreamBody::new(chunk_stream);
        let body = SdkBody::from_body_1_x(stream_body);
        let mut receiver = Receiver::<TestMessage, EventStreamError>::new(Unmarshaller, body);
        assert_eq!(
            TestMessage("one".into()),
            receiver.recv().await.unwrap().unwrap()
        );
        assert_eq!(
            TestMessage("two".into()),
            receiver.recv().await.unwrap().unwrap()
        );
        assert_eq!(None, receiver.recv().await.unwrap());
    }

    #[tokio::test]
    async fn receive_last_chunk_not_full_message() {
        let chunks: Vec<Result<_, IOError>> = map_to_frame(vec![
            Ok(encode_message("one")),
            Ok(encode_message("two")),
            Ok(encode_message("three").split_to(10)),
        ]);
        let chunk_stream = futures_util::stream::iter(chunks);
        let stream_body = http_body_util::StreamBody::new(chunk_stream);
        let body = SdkBody::from_body_1_x(stream_body);
        let mut receiver = Receiver::<TestMessage, EventStreamError>::new(Unmarshaller, body);
        assert_eq!(
            TestMessage("one".into()),
            receiver.recv().await.unwrap().unwrap()
        );
        assert_eq!(
            TestMessage("two".into()),
            receiver.recv().await.unwrap().unwrap()
        );
        assert!(matches!(
            receiver.recv().await,
            Err(SdkError::ResponseError { .. }),
        ));
    }

    #[tokio::test]
    async fn receive_last_chunk_has_multiple_messages() {
        let chunks: Vec<Result<_, IOError>> = map_to_frame(vec![
            Ok(encode_message("one")),
            Ok(encode_message("two")),
            Ok(Bytes::from(
                [encode_message("three"), encode_message("four")].concat(),
            )),
        ]);
        let chunk_stream = futures_util::stream::iter(chunks);
        let stream_body = http_body_util::StreamBody::new(chunk_stream);
        let body = SdkBody::from_body_1_x(stream_body);
        let mut receiver = Receiver::<TestMessage, EventStreamError>::new(Unmarshaller, body);
        assert_eq!(
            TestMessage("one".into()),
            receiver.recv().await.unwrap().unwrap()
        );
        assert_eq!(
            TestMessage("two".into()),
            receiver.recv().await.unwrap().unwrap()
        );
        assert_eq!(
            TestMessage("three".into()),
            receiver.recv().await.unwrap().unwrap()
        );
        assert_eq!(
            TestMessage("four".into()),
            receiver.recv().await.unwrap().unwrap()
        );
        assert_eq!(None, receiver.recv().await.unwrap());
    }

    proptest::proptest! {
        #[test]
        fn receive_multiple_messages_split_unevenly_across_chunks(b1: usize, b2: usize) {
            let combined = Bytes::from([
                encode_message("one"),
                encode_message("two"),
                encode_message("three"),
                encode_message("four"),
                encode_message("five"),
                encode_message("six"),
                encode_message("seven"),
                encode_message("eight"),
            ].concat());

            let midpoint = combined.len() / 2;
            let (start, boundary1, boundary2, end) = (
                0,
                b1 % midpoint,
                midpoint + b2 % midpoint,
                combined.len()
            );
            println!("[{start}, {boundary1}], [{boundary1}, {boundary2}], [{boundary2}, {end}]");

            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let chunks: Vec<Result<_, IOError>> = map_to_frame(vec![
                    Ok(Bytes::copy_from_slice(&combined[start..boundary1])),
                    Ok(Bytes::copy_from_slice(&combined[boundary1..boundary2])),
                    Ok(Bytes::copy_from_slice(&combined[boundary2..end])),
                ]);

                let chunk_stream = futures_util::stream::iter(chunks);
                let stream_body = http_body_util::StreamBody::new(chunk_stream);
                let body = SdkBody::from_body_1_x(stream_body);
                let mut receiver = Receiver::<TestMessage, EventStreamError>::new(Unmarshaller, body);
                for payload in &["one", "two", "three", "four", "five", "six", "seven", "eight"] {
                    assert_eq!(
                        TestMessage((*payload).into()),
                        receiver.recv().await.unwrap().unwrap()
                    );
                }
                assert_eq!(None, receiver.recv().await.unwrap());
            });
        }
    }

    #[tokio::test]
    async fn receive_network_failure() {
        let chunks: Vec<Result<_, IOError>> = map_to_frame(vec![
            Ok(encode_message("one")),
            Err(IOError::new(ErrorKind::ConnectionReset, FakeError)),
        ]);
        let chunk_stream = futures_util::stream::iter(chunks);
        let stream_body = http_body_util::StreamBody::new(chunk_stream);
        let body = SdkBody::from_body_1_x(stream_body);
        let mut receiver = Receiver::<TestMessage, EventStreamError>::new(Unmarshaller, body);
        assert_eq!(
            TestMessage("one".into()),
            receiver.recv().await.unwrap().unwrap()
        );
        assert!(matches!(
            receiver.recv().await,
            Err(SdkError::DispatchFailure(_))
        ));
    }

    #[tokio::test]
    async fn receive_message_parse_failure() {
        let chunks: Vec<Result<_, IOError>> = map_to_frame(vec![
            Ok(encode_message("one")),
            // A zero length message will be invalid. We need to provide a minimum of 12 bytes
            // for the MessageFrameDecoder to actually start parsing it.
            Ok(Bytes::from_static(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])),
        ]);
        let chunk_stream = futures_util::stream::iter(chunks);
        let stream_body = http_body_util::StreamBody::new(chunk_stream);
        let body = SdkBody::from_body_1_x(stream_body);
        let mut receiver = Receiver::<TestMessage, EventStreamError>::new(Unmarshaller, body);
        assert_eq!(
            TestMessage("one".into()),
            receiver.recv().await.unwrap().unwrap()
        );
        assert!(matches!(
            receiver.recv().await,
            Err(SdkError::ResponseError { .. })
        ));
    }

    #[tokio::test]
    async fn receive_initial_response() {
        let chunks: Vec<Result<_, IOError>> = map_to_frame(vec![
            Ok(encode_initial_response()),
            Ok(encode_message("one")),
        ]);
        let chunk_stream = futures_util::stream::iter(chunks);
        let stream_body = http_body_util::StreamBody::new(chunk_stream);
        let body = SdkBody::from_body_1_x(stream_body);
        let mut receiver = Receiver::<TestMessage, EventStreamError>::new(Unmarshaller, body);
        assert!(receiver
            .try_recv_initial(InitialMessageType::Response)
            .await
            .unwrap()
            .is_some());
        assert_eq!(
            TestMessage("one".into()),
            receiver.recv().await.unwrap().unwrap()
        );
    }

    #[tokio::test]
    async fn receive_no_initial_response() {
        let chunks: Vec<Result<_, IOError>> =
            map_to_frame(vec![Ok(encode_message("one")), Ok(encode_message("two"))]);
        let chunk_stream = futures_util::stream::iter(chunks);
        let stream_body = http_body_util::StreamBody::new(chunk_stream);

        let body = SdkBody::from_body_1_x(stream_body);
        let mut receiver = Receiver::<TestMessage, EventStreamError>::new(Unmarshaller, body);
        assert!(receiver
            .try_recv_initial(InitialMessageType::Response)
            .await
            .unwrap()
            .is_none());
        assert_eq!(
            TestMessage("one".into()),
            receiver.recv().await.unwrap().unwrap()
        );
        assert_eq!(
            TestMessage("two".into()),
            receiver.recv().await.unwrap().unwrap()
        );
    }

    fn assert_send_and_sync<T: Send + Sync>() {}

    #[tokio::test]
    async fn receiver_is_send_and_sync() {
        assert_send_and_sync::<Receiver<(), ()>>();
    }
}
