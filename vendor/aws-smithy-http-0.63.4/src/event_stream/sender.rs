/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_eventstream::frame::{write_message_to, MarshallMessage, SignMessage};
use aws_smithy_eventstream::message_size_hint::MessageSizeHint;
use aws_smithy_runtime_api::client::result::SdkError;
use aws_smithy_types::error::ErrorMetadata;
use bytes::Bytes;
use futures_core::Stream;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use tracing::trace;

/// Input type for Event Streams.
pub struct EventStreamSender<T, E> {
    input_stream: Pin<Box<dyn Stream<Item = Result<T, E>> + Send + Sync>>,
}

impl<T, E> Debug for EventStreamSender<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name_t = std::any::type_name::<T>();
        let name_e = std::any::type_name::<E>();
        write!(f, "EventStreamSender<{name_t}, {name_e}>")
    }
}

impl<T: Send + Sync + 'static, E: StdError + Send + Sync + 'static> EventStreamSender<T, E> {
    /// Creates an `EventStreamSender` from a single item.
    pub fn once(item: Result<T, E>) -> Self {
        Self::from(futures_util::stream::once(async move { item }))
    }
}

impl<T: Send + Sync, E: StdError + Send + Sync + 'static> EventStreamSender<T, E> {
    #[doc(hidden)]
    pub fn into_body_stream(
        self,
        marshaller: impl MarshallMessage<Input = T> + Send + Sync + 'static,
        error_marshaller: impl MarshallMessage<Input = E> + Send + Sync + 'static,
        signer: impl SignMessage + Send + Sync + 'static,
    ) -> MessageStreamAdapter<T, E> {
        MessageStreamAdapter::new(marshaller, error_marshaller, signer, self.input_stream)
    }
}

impl<T, E, S> From<S> for EventStreamSender<T, E>
where
    S: Stream<Item = Result<T, E>> + Send + Sync + 'static,
{
    fn from(stream: S) -> Self {
        EventStreamSender {
            input_stream: Box::pin(stream),
        }
    }
}

/// An error that occurs within a message stream.
#[derive(Debug)]
pub struct MessageStreamError {
    kind: MessageStreamErrorKind,
    pub(crate) meta: ErrorMetadata,
}

#[derive(Debug)]
enum MessageStreamErrorKind {
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl MessageStreamError {
    /// Creates the `MessageStreamError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            meta: Default::default(),
            kind: MessageStreamErrorKind::Unhandled(err.into()),
        }
    }

    /// Creates the `MessageStreamError::Unhandled` variant from an [`ErrorMetadata`].
    pub fn generic(err: ErrorMetadata) -> Self {
        Self {
            meta: err.clone(),
            kind: MessageStreamErrorKind::Unhandled(err.into()),
        }
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &ErrorMetadata {
        &self.meta
    }
}

impl StdError for MessageStreamError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self.kind {
            MessageStreamErrorKind::Unhandled(source) => Some(source.as_ref() as _),
        }
    }
}

impl fmt::Display for MessageStreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            MessageStreamErrorKind::Unhandled(_) => write!(f, "message stream error"),
        }
    }
}

/// Adapts a `Stream<SmithyMessageType>` to a signed `Stream<Bytes>` by using the provided
/// message marshaller and signer implementations.
///
/// This will yield an `Err(SdkError::ConstructionFailure)` if a message can't be
/// marshalled into an Event Stream frame, (e.g., if the message payload was too large).
#[allow(missing_debug_implementations)]
pub struct MessageStreamAdapter<T: Send + Sync, E: StdError + Send + Sync + 'static> {
    marshaller: Box<dyn MarshallMessage<Input = T> + Send + Sync>,
    error_marshaller: Box<dyn MarshallMessage<Input = E> + Send + Sync>,
    signer: Box<dyn SignMessage + Send + Sync>,
    stream: Pin<Box<dyn Stream<Item = Result<T, E>> + Send + Sync>>,
    end_signal_sent: bool,
    _phantom: PhantomData<E>,
}

impl<T: Send + Sync, E: StdError + Send + Sync + 'static> Unpin for MessageStreamAdapter<T, E> {}

impl<T: Send + Sync, E: StdError + Send + Sync + 'static> MessageStreamAdapter<T, E> {
    /// Create a new `MessageStreamAdapter`.
    pub fn new(
        marshaller: impl MarshallMessage<Input = T> + Send + Sync + 'static,
        error_marshaller: impl MarshallMessage<Input = E> + Send + Sync + 'static,
        signer: impl SignMessage + Send + Sync + 'static,
        stream: Pin<Box<dyn Stream<Item = Result<T, E>> + Send + Sync>>,
    ) -> Self {
        MessageStreamAdapter {
            marshaller: Box::new(marshaller),
            error_marshaller: Box::new(error_marshaller),
            signer: Box::new(signer),
            stream,
            end_signal_sent: false,
            _phantom: Default::default(),
        }
    }
}

impl<T: Send + Sync, E: StdError + Send + Sync + 'static> Stream for MessageStreamAdapter<T, E> {
    type Item = Result<
        http_body_1x::Frame<Bytes>,
        SdkError<E, aws_smithy_runtime_api::client::orchestrator::HttpResponse>,
    >;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.stream.as_mut().poll_next(cx) {
            Poll::Ready(message_option) => {
                if let Some(message_result) = message_option {
                    let message = match message_result {
                        Ok(message) => self
                            .marshaller
                            .marshall(message)
                            .map_err(SdkError::construction_failure)?,
                        Err(message) => self
                            .error_marshaller
                            .marshall(message)
                            .map_err(SdkError::construction_failure)?,
                    };

                    trace!(unsigned_message = ?message, "signing event stream message");
                    let message = self
                        .signer
                        .sign(message)
                        .map_err(SdkError::construction_failure)?;

                    let mut buffer = Vec::with_capacity(message.size_hint());
                    write_message_to(&message, &mut buffer)
                        .map_err(SdkError::construction_failure)?;
                    trace!(signed_message = ?buffer, "sending signed event stream message");
                    Poll::Ready(Some(Ok(http_body_1x::Frame::data(Bytes::from(buffer)))))
                } else if !self.end_signal_sent {
                    self.end_signal_sent = true;
                    match self.signer.sign_empty() {
                        Some(sign) => {
                            let message = sign.map_err(SdkError::construction_failure)?;
                            let mut buffer = Vec::with_capacity(message.size_hint());
                            write_message_to(&message, &mut buffer)
                                .map_err(SdkError::construction_failure)?;
                            trace!(signed_message = ?buffer, "sending signed empty message to terminate the event stream");
                            Poll::Ready(Some(Ok(http_body_1x::Frame::data(Bytes::from(buffer)))))
                        }
                        None => Poll::Ready(None),
                    }
                } else {
                    Poll::Ready(None)
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarshallMessage;
    use crate::event_stream::{EventStreamSender, MessageStreamAdapter};
    use async_stream::stream;
    use aws_smithy_eventstream::error::Error as EventStreamError;
    use aws_smithy_eventstream::frame::{
        read_message_from, write_message_to, NoOpSigner, SignMessage, SignMessageError,
    };
    use aws_smithy_runtime_api::client::result::SdkError;
    use aws_smithy_types::event_stream::{Header, HeaderValue, Message};
    use futures_util::stream::StreamExt;
    use std::error::Error as StdError;

    #[derive(Debug, Eq, PartialEq)]
    struct TestMessage(String);

    #[derive(Debug)]
    struct Marshaller;
    impl MarshallMessage for Marshaller {
        type Input = TestMessage;

        fn marshall(&self, input: Self::Input) -> Result<Message, EventStreamError> {
            Ok(Message::new(input.0.as_bytes().to_vec()))
        }
    }
    #[derive(Debug)]
    struct ErrorMarshaller;
    impl MarshallMessage for ErrorMarshaller {
        type Input = TestServiceError;

        fn marshall(&self, _input: Self::Input) -> Result<Message, EventStreamError> {
            Err(read_message_from(&b""[..]).expect_err("this should always fail"))
        }
    }

    #[derive(Debug)]
    struct TestServiceError;
    impl std::fmt::Display for TestServiceError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestServiceError")
        }
    }
    impl StdError for TestServiceError {}

    #[derive(Debug)]
    struct TestSigner;
    impl SignMessage for TestSigner {
        fn sign(&mut self, message: Message) -> Result<Message, SignMessageError> {
            let mut buffer = Vec::new();
            write_message_to(&message, &mut buffer).unwrap();
            Ok(Message::new(buffer).add_header(Header::new("signed", HeaderValue::Bool(true))))
        }

        fn sign_empty(&mut self) -> Option<Result<Message, SignMessageError>> {
            Some(Ok(
                Message::new(&b""[..]).add_header(Header::new("signed", HeaderValue::Bool(true)))
            ))
        }
    }

    fn check_send_sync<T: Send + Sync>(value: T) -> T {
        value
    }

    #[test]
    fn event_stream_sender_send_sync() {
        check_send_sync(EventStreamSender::from(stream! {
            yield Result::<_, SignMessageError>::Ok(TestMessage("test".into()));
        }));
    }

    #[tokio::test]
    async fn message_stream_adapter_success() {
        let stream = stream! {
            yield Ok(TestMessage("test".into()));
        };
        let mut adapter = MessageStreamAdapter::<TestMessage, TestServiceError>::new(
            Marshaller,
            ErrorMarshaller,
            TestSigner,
            Box::pin(stream),
        );

        let sent_bytes = adapter.next().await.unwrap().unwrap();
        let sent = read_message_from(&mut sent_bytes.into_data().unwrap()).unwrap();
        assert_eq!("signed", sent.headers()[0].name().as_str());
        assert_eq!(&HeaderValue::Bool(true), sent.headers()[0].value());
        let inner = read_message_from(&mut (&sent.payload()[..])).unwrap();
        assert_eq!(&b"test"[..], &inner.payload()[..]);

        let end_signal_bytes = adapter.next().await.unwrap().unwrap();
        let end_signal = read_message_from(&mut end_signal_bytes.into_data().unwrap()).unwrap();
        assert_eq!("signed", end_signal.headers()[0].name().as_str());
        assert_eq!(&HeaderValue::Bool(true), end_signal.headers()[0].value());
        assert_eq!(0, end_signal.payload().len());
    }

    #[tokio::test]
    async fn message_stream_adapter_construction_failure() {
        let stream = stream! {
            yield Err(TestServiceError);
        };
        let mut adapter = MessageStreamAdapter::<TestMessage, TestServiceError>::new(
            Marshaller,
            ErrorMarshaller,
            NoOpSigner {},
            Box::pin(stream),
        );

        let result = adapter.next().await.unwrap();
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            SdkError::ConstructionFailure(_)
        ));
    }

    #[tokio::test]
    async fn event_stream_sender_once() {
        let sender = EventStreamSender::once(Ok(TestMessage("test".into())));
        let mut adapter = MessageStreamAdapter::<TestMessage, TestServiceError>::new(
            Marshaller,
            ErrorMarshaller,
            TestSigner,
            sender.input_stream,
        );

        let sent_bytes = adapter.next().await.unwrap().unwrap();
        let sent = read_message_from(&mut sent_bytes.into_data().unwrap()).unwrap();
        assert_eq!("signed", sent.headers()[0].name().as_str());
        let inner = read_message_from(&mut (&sent.payload()[..])).unwrap();
        assert_eq!(&b"test"[..], &inner.payload()[..]);

        // Should get end signal next
        let end_signal_bytes = adapter.next().await.unwrap().unwrap();
        let end_signal = read_message_from(&mut end_signal_bytes.into_data().unwrap()).unwrap();
        assert_eq!("signed", end_signal.headers()[0].name().as_str());
        assert_eq!(0, end_signal.payload().len());

        // Stream should be exhausted
        assert!(adapter.next().await.is_none());
    }

    // Verify the developer experience for this compiles
    #[allow(unused)]
    fn event_stream_input_ergonomics() {
        fn check(input: impl Into<EventStreamSender<TestMessage, TestServiceError>>) {
            let _: EventStreamSender<TestMessage, TestServiceError> = input.into();
        }
        check(stream! {
            yield Ok(TestMessage("test".into()));
        });
        check(stream! {
            yield Err(TestServiceError);
        });
    }
}
