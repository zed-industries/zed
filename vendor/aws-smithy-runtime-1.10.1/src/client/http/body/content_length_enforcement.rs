/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! RuntimePlugin to ensure that the amount of data received matches the `Content-Length` header

use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::context::{
    BeforeDeserializationInterceptorContextMut, BeforeTransmitInterceptorContextRef,
};
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::runtime_components::{
    RuntimeComponents, RuntimeComponentsBuilder,
};
use aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin;
use aws_smithy_runtime_api::http::Response;
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::config_bag::{ConfigBag, Storable, StoreReplace};
use bytes::Buf;
use http_body_1x::{Frame, SizeHint};
use pin_project_lite::pin_project;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::pin::Pin;
use std::task::{ready, Context, Poll};
pin_project! {
    /// A body-wrapper that will calculate the `InnerBody`'s checksum and emit it as a trailer.
    struct ContentLengthEnforcingBody<InnerBody> {
            #[pin]
            body: InnerBody,
            expected_length: u64,
            bytes_received: u64,
    }
}

/// An error returned when a body did not have the expected content length
#[derive(Debug)]
pub struct ContentLengthError {
    expected: u64,
    received: u64,
}

impl Error for ContentLengthError {}

impl Display for ContentLengthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid Content-Length: Expected {} bytes but {} bytes were received",
            self.expected, self.received
        )
    }
}

impl ContentLengthEnforcingBody<SdkBody> {
    /// Wraps an existing [`SdkBody`] in a content-length enforcement layer
    fn wrap(body: SdkBody, content_length: u64) -> SdkBody {
        body.map_preserve_contents(move |b| {
            SdkBody::from_body_1_x(ContentLengthEnforcingBody {
                body: b,
                expected_length: content_length,
                bytes_received: 0,
            })
        })
    }
}

impl<
        E: Into<aws_smithy_types::body::Error>,
        Data: Buf,
        InnerBody: http_body_1x::Body<Error = E, Data = Data>,
    > http_body_1x::Body for ContentLengthEnforcingBody<InnerBody>
{
    type Data = Data;
    type Error = aws_smithy_types::body::Error;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.as_mut().project();
        match ready!(this.body.poll_frame(cx)) {
            None => {
                if *this.expected_length == *this.bytes_received {
                    Poll::Ready(None)
                } else {
                    Poll::Ready(Some(Err(ContentLengthError {
                        expected: *this.expected_length,
                        received: *this.bytes_received,
                    }
                    .into())))
                }
            }
            Some(Err(e)) => Poll::Ready(Some(Err(e.into()))),
            Some(Ok(frame)) => {
                if let Some(data) = frame.data_ref() {
                    *this.bytes_received += data.remaining() as u64;
                }
                Poll::Ready(Some(Ok(frame)))
            }
        }
    }

    fn is_end_stream(&self) -> bool {
        self.body.is_end_stream()
    }

    fn size_hint(&self) -> SizeHint {
        self.body.size_hint()
    }
}

#[derive(Debug, Default)]
struct EnforceContentLengthInterceptor {}

#[derive(Debug)]
struct EnableContentLengthEnforcement;
impl Storable for EnableContentLengthEnforcement {
    type Storer = StoreReplace<EnableContentLengthEnforcement>;
}

impl Intercept for EnforceContentLengthInterceptor {
    fn name(&self) -> &'static str {
        "EnforceContentLength"
    }

    fn read_before_transmit(
        &self,
        context: &BeforeTransmitInterceptorContextRef<'_>,
        _runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        if context.request().method() == "GET" {
            cfg.interceptor_state()
                .store_put(EnableContentLengthEnforcement);
        }
        Ok(())
    }
    fn modify_before_deserialization(
        &self,
        context: &mut BeforeDeserializationInterceptorContextMut<'_>,
        _runtime_components: &RuntimeComponents,
        cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        // if we didn't enable it for this request, bail out
        if cfg.load::<EnableContentLengthEnforcement>().is_none() {
            return Ok(());
        }
        let content_length = match extract_content_length(context.response()) {
            Err(err) => {
                tracing::warn!(err = ?err, "could not parse content length from content-length header. This header will be ignored");
                return Ok(());
            }
            Ok(Some(content_length)) => content_length,
            Ok(None) => return Ok(()),
        };

        tracing::trace!(
            expected_length = content_length,
            "Wrapping response body in content-length enforcement."
        );

        let body = context.response_mut().take_body();
        let wrapped = body.map_preserve_contents(move |body| {
            ContentLengthEnforcingBody::wrap(body, content_length)
        });
        *context.response_mut().body_mut() = wrapped;
        Ok(())
    }
}

fn extract_content_length<B>(response: &Response<B>) -> Result<Option<u64>, BoxError> {
    let Some(content_length) = response.headers().get("content-length") else {
        tracing::trace!("No content length header was set. Will not validate content length");
        return Ok(None);
    };
    if response.headers().get_all("content-length").count() != 1 {
        return Err("Found multiple content length headers. This is invalid".into());
    }

    Ok(Some(content_length.parse::<u64>()?))
}

/// Runtime plugin that enforces response bodies match their expected content length
#[derive(Debug, Default)]
pub struct EnforceContentLengthRuntimePlugin {}

impl EnforceContentLengthRuntimePlugin {
    /// Creates a runtime plugin which installs Content-Length enforcement middleware for response bodies
    pub fn new() -> Self {
        Self {}
    }
}

impl RuntimePlugin for EnforceContentLengthRuntimePlugin {
    fn runtime_components(
        &self,
        _current_components: &RuntimeComponentsBuilder,
    ) -> Cow<'_, RuntimeComponentsBuilder> {
        Cow::Owned(
            RuntimeComponentsBuilder::new("EnforceContentLength")
                .with_interceptor(EnforceContentLengthInterceptor {}),
        )
    }
}

#[cfg(all(test, any(feature = "test-util", feature = "legacy-test-util")))]
mod test {
    use crate::assert_str_contains;
    use crate::client::http::body::content_length_enforcement::{
        extract_content_length, ContentLengthEnforcingBody,
    };
    use aws_smithy_runtime_api::http::Response;
    use aws_smithy_types::body::SdkBody;
    use aws_smithy_types::byte_stream::ByteStream;
    use aws_smithy_types::error::display::DisplayErrorContext;
    use bytes::Bytes;
    use http_1x::header::CONTENT_LENGTH;
    use http_body_1x::Frame;
    use std::error::Error;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    /// Body for tests so we ensure our code works on a body split across multiple frames
    struct ManyFrameBody {
        data: Vec<u8>,
    }

    impl ManyFrameBody {
        #[allow(clippy::new_ret_no_self)]
        fn new(input: impl Into<String>) -> SdkBody {
            let mut data = input.into().as_bytes().to_vec();
            data.reverse();
            SdkBody::from_body_1_x(Self { data })
        }
    }

    impl http_body_1x::Body for ManyFrameBody {
        type Data = Bytes;
        type Error = <SdkBody as http_body_1x::Body>::Error;

        fn poll_frame(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
            match self.data.pop() {
                Some(next) => Poll::Ready(Some(Ok(Frame::data(Bytes::from(vec![next]))))),
                None => Poll::Ready(None),
            }
        }
    }

    #[tokio::test]
    async fn stream_too_short() {
        let body = ManyFrameBody::new("123");
        let enforced = ContentLengthEnforcingBody::wrap(body, 10);
        let err = expect_body_error(enforced).await;
        assert_str_contains!(
            format!("{}", DisplayErrorContext(err)),
            "Expected 10 bytes but 3 bytes were received"
        );
    }

    #[tokio::test]
    async fn stream_too_long() {
        let body = ManyFrameBody::new("abcdefghijk");
        let enforced = ContentLengthEnforcingBody::wrap(body, 5);
        let err = expect_body_error(enforced).await;
        assert_str_contains!(
            format!("{}", DisplayErrorContext(err)),
            "Expected 5 bytes but 11 bytes were received"
        );
    }

    #[tokio::test]
    async fn stream_just_right() {
        use http_body_util::BodyExt;
        let body = ManyFrameBody::new("abcdefghijk");
        let enforced = ContentLengthEnforcingBody::wrap(body, 11);
        let data = enforced.collect().await.unwrap().to_bytes();
        assert_eq!(b"abcdefghijk", data.as_ref());
    }

    async fn expect_body_error(body: SdkBody) -> impl Error {
        ByteStream::new(body)
            .collect()
            .await
            .expect_err("body should have failed")
    }

    #[test]
    fn extract_header() {
        let mut resp1 = Response::new(200.try_into().unwrap(), ());
        resp1.headers_mut().insert(CONTENT_LENGTH, "123");
        assert_eq!(extract_content_length(&resp1).unwrap(), Some(123));
        resp1.headers_mut().append(CONTENT_LENGTH, "124");
        // duplicate content length header
        extract_content_length(&resp1).expect_err("duplicate headers");

        // not an integer
        resp1.headers_mut().insert(CONTENT_LENGTH, "-123.5");
        extract_content_length(&resp1).expect_err("not an integer");

        // not an integer
        resp1.headers_mut().insert(CONTENT_LENGTH, "");
        extract_content_length(&resp1).expect_err("empty");

        resp1.headers_mut().remove(CONTENT_LENGTH);
        assert_eq!(extract_content_length(&resp1).unwrap(), None);
    }
}
