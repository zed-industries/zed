/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Adapters to use http-body 1.0 bodies with SdkBody & ByteStream

use std::pin::Pin;
use std::task::{ready, Context, Poll};

use bytes::Bytes;
use http_body_util::BodyExt;
use pin_project_lite::pin_project;

use crate::body::{Error, SdkBody};

impl SdkBody {
    /// Construct an `SdkBody` from a type that implements [`http_body_1_0::Body<Data = Bytes>`](http_body_1_0::Body).
    pub fn from_body_1_x<T, E>(body: T) -> Self
    where
        T: http_body_1_0::Body<Data = Bytes, Error = E> + Send + Sync + 'static,
        E: Into<Error> + 'static,
    {
        SdkBody::from_body_1_x_internal(body.map_err(Into::into))
    }

    pub(crate) fn poll_data_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<http_body_1_0::Frame<Bytes>, Error>>> {
        match ready!(self.as_mut().poll_next(cx)) {
            // if there's no more data, try to return trailers
            None => match ready!(self.poll_next_trailers(cx)) {
                Ok(Some(trailers)) => {
                    Poll::Ready(Some(Ok(http_body_1_0::Frame::trailers(trailers))))
                }
                Ok(None) => Poll::Ready(None),
                Err(e) => Poll::Ready(Some(Err(e))),
            },
            Some(result) => match result {
                Err(err) => Poll::Ready(Some(Err(err))),
                Ok(bytes) => Poll::Ready(Some(Ok(http_body_1_0::Frame::data(bytes)))),
            },
        }
    }
}

#[cfg(feature = "http-body-1-x")]
impl http_body_1_0::Body for SdkBody {
    type Data = Bytes;
    type Error = Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<http_body_1_0::Frame<Self::Data>, Self::Error>>> {
        self.poll_data_frame(cx)
    }

    fn is_end_stream(&self) -> bool {
        self.is_end_stream()
    }

    fn size_hint(&self) -> http_body_1_0::SizeHint {
        let mut hint = http_body_1_0::SizeHint::default();
        let (lower, upper) = self.bounds_on_remaining_length();
        hint.set_lower(lower);
        if let Some(upper) = upper {
            hint.set_upper(upper);
        }
        hint
    }
}

pin_project! {
    struct Http1toHttp04<B> {
        #[pin]
        inner: B,
        trailers: Option<http_1x::HeaderMap>,
    }
}

impl<B> Http1toHttp04<B> {
    #[allow(dead_code)]
    fn new(inner: B) -> Self {
        Self {
            inner,
            trailers: None,
        }
    }
}

impl<B> http_body_0_4::Body for Http1toHttp04<B>
where
    B: http_body_1_0::Body,
{
    type Data = B::Data;
    type Error = B::Error;

    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        loop {
            let this = self.as_mut().project();
            match ready!(this.inner.poll_frame(cx)) {
                Some(Ok(frame)) => {
                    let frame = match frame.into_data() {
                        Ok(data) => return Poll::Ready(Some(Ok(data))),
                        Err(frame) => frame,
                    };
                    // when we get a trailers frame, store the trailers for the next poll
                    if let Ok(trailers) = frame.into_trailers() {
                        if let Some(trailer_map) = this.trailers {
                            trailer_map.extend(trailers);
                        } else {
                            this.trailers.replace(trailers);
                        }
                        return Poll::Ready(None);
                    };
                    // if the frame type was unknown, discard it. the next one might be something
                    // useful
                }
                Some(Err(e)) => return Poll::Ready(Some(Err(e))),
                None => return Poll::Ready(None),
            }
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        // all of the polling happens in poll_data, once we get to the trailers we've actually
        // already read everything
        let this = self.project();
        match this.trailers.take() {
            Some(headers) => Poll::Ready(Ok(Some(convert_headers_1x_0x(headers)))),
            None => Poll::Ready(Ok(None)),
        }
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn size_hint(&self) -> http_body_0_4::SizeHint {
        let mut size_hint = http_body_0_4::SizeHint::new();
        let inner_hint = self.inner.size_hint();
        if let Some(exact) = inner_hint.exact() {
            size_hint.set_exact(exact);
        } else {
            size_hint.set_lower(inner_hint.lower());
            if let Some(upper) = inner_hint.upper() {
                size_hint.set_upper(upper);
            }
        }
        size_hint
    }
}

pub(crate) fn convert_headers_1x_0x(input: http_1x::HeaderMap) -> http::HeaderMap {
    let mut map = http::HeaderMap::with_capacity(input.capacity());
    let mut mem: Option<http_1x::HeaderName> = None;
    for (k, v) in input.into_iter() {
        let name = k.or_else(|| mem.clone()).unwrap();
        map.append(
            http::HeaderName::from_bytes(name.as_str().as_bytes()).expect("already validated"),
            http::HeaderValue::from_bytes(v.as_bytes()).expect("already validated"),
        );
        mem = Some(name);
    }
    map
}

pub(crate) fn convert_headers_0x_1x(input: http::HeaderMap) -> http_1x::HeaderMap {
    let mut map = http_1x::HeaderMap::with_capacity(input.capacity());
    let mut mem: Option<http::HeaderName> = None;
    for (k, v) in input.into_iter() {
        let name = k.or_else(|| mem.clone()).unwrap();
        map.append(
            http_1x::HeaderName::from_bytes(name.as_str().as_bytes()).expect("already validated"),
            http_1x::HeaderValue::from_bytes(v.as_bytes()).expect("already validated"),
        );
        mem = Some(name);
    }
    map
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use bytes::Bytes;
    use http::header::{CONTENT_LENGTH as CL0, CONTENT_TYPE as CT0};
    use http_1x::header::{CONTENT_LENGTH as CL1, CONTENT_TYPE as CT1};
    use http_1x::{HeaderMap, HeaderName, HeaderValue};
    use http_body_1_0::Frame;
    use http_body_util::BodyExt;

    use crate::body::http_body_1_x::{convert_headers_1x_0x, Http1toHttp04};
    use crate::body::{Error, SdkBody};
    use crate::byte_stream::ByteStream;

    struct TestBody {
        chunks: VecDeque<Chunk>,
    }

    enum Chunk {
        Data(&'static str),
        Error(&'static str),
        Trailers(HeaderMap),
    }

    impl http_body_1_0::Body for TestBody {
        type Data = Bytes;
        type Error = Error;

        fn poll_frame(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
            let next = self.chunks.pop_front();
            let mk = |v: Frame<Bytes>| Poll::Ready(Some(Ok(v)));

            match next {
                Some(Chunk::Data(s)) => mk(Frame::data(Bytes::from_static(s.as_bytes()))),
                Some(Chunk::Trailers(headers)) => mk(Frame::trailers(headers)),
                Some(Chunk::Error(err)) => Poll::Ready(Some(Err(err.into()))),
                None => Poll::Ready(None),
            }
        }
    }

    fn trailers() -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(
            HeaderName::from_static("x-test"),
            HeaderValue::from_static("x-test-value"),
        );
        map.append(
            HeaderName::from_static("x-test"),
            HeaderValue::from_static("x-test-value-2"),
        );
        map.append(
            HeaderName::from_static("y-test"),
            HeaderValue::from_static("y-test-value-2"),
        );
        map
    }

    #[tokio::test]
    async fn test_body_with_trailers() {
        let body = TestBody {
            chunks: vec![
                Chunk::Data("123"),
                Chunk::Data("456"),
                Chunk::Data("789"),
                Chunk::Trailers(trailers()),
            ]
            .into(),
        };
        let body = SdkBody::from_body_1_x(body);
        let data = ByteStream::new(body);
        assert_eq!(data.collect().await.unwrap().to_vec(), b"123456789");
    }

    #[tokio::test]
    async fn test_read_trailers() {
        let body = TestBody {
            chunks: vec![
                Chunk::Data("123"),
                Chunk::Data("456"),
                Chunk::Data("789"),
                Chunk::Trailers(trailers()),
            ]
            .into(),
        };
        let body = SdkBody::from_body_1_x(body);
        let collected = body.collect().await.unwrap();
        let collected_trailers = collected.trailers();

        assert_eq!(collected_trailers, Some(&trailers()));
    }

    #[tokio::test]
    async fn test_read_multiple_trailers() {
        let mut second_trailers = HeaderMap::new();
        second_trailers.insert(
            HeaderName::from_static("second-trailer"),
            HeaderValue::from_static("second"),
        );

        let mut merged_trailers = HeaderMap::new();
        merged_trailers.extend(second_trailers.clone());
        merged_trailers.extend(trailers());

        let body = TestBody {
            chunks: vec![
                Chunk::Data("123"),
                Chunk::Data("456"),
                Chunk::Data("789"),
                Chunk::Trailers(trailers()),
                Chunk::Trailers(second_trailers),
            ]
            .into(),
        };
        let body = SdkBody::from_body_1_x(body);
        let collected = body.collect().await.unwrap();
        let collected_trailers = collected.trailers().unwrap();

        assert_eq!(collected_trailers, &merged_trailers);
    }

    #[tokio::test]
    async fn test_trailers_04x_to_1x() {
        let body = TestBody {
            chunks: vec![
                Chunk::Data("123"),
                Chunk::Data("456"),
                Chunk::Data("789"),
                Chunk::Trailers(trailers()),
            ]
            .into(),
        };
        let body = SdkBody::from_body_0_4(Http1toHttp04::new(body));

        let collected = BodyExt::collect(body).await.expect("should succeed");
        assert_eq!(collected.trailers(), Some(&trailers()));
        assert_eq!(collected.to_bytes().as_ref(), b"123456789");
    }

    #[tokio::test]
    async fn test_multiple_trailers_04x_to_1x() {
        let mut second_trailers = HeaderMap::new();
        second_trailers.insert(
            HeaderName::from_static("second-trailer"),
            HeaderValue::from_static("second"),
        );

        let mut merged_trailers = HeaderMap::new();
        merged_trailers.extend(second_trailers.clone());
        merged_trailers.extend(trailers());

        let body = TestBody {
            chunks: vec![
                Chunk::Data("123"),
                Chunk::Data("456"),
                Chunk::Data("789"),
                Chunk::Trailers(trailers()),
                Chunk::Trailers(second_trailers),
            ]
            .into(),
        };
        let body = SdkBody::from_body_0_4(Http1toHttp04::new(body));

        let collected = BodyExt::collect(body).await.expect("should succeed");
        assert_eq!(collected.trailers().unwrap(), &merged_trailers);
        assert_eq!(collected.to_bytes().as_ref(), b"123456789");
    }

    #[tokio::test]
    async fn test_errors() {
        let body = TestBody {
            chunks: vec![
                Chunk::Data("123"),
                Chunk::Data("456"),
                Chunk::Data("789"),
                Chunk::Error("errors!"),
            ]
            .into(),
        };

        let body = SdkBody::from_body_1_x(body);
        let body = ByteStream::new(body);
        body.collect().await.expect_err("body returned an error");
    }

    #[tokio::test]
    async fn test_no_trailers() {
        let body = TestBody {
            chunks: vec![Chunk::Data("123"), Chunk::Data("456"), Chunk::Data("789")].into(),
        };

        let body = SdkBody::from_body_1_x(body);
        let collected = BodyExt::collect(body).await.expect("should succeed");
        assert_eq!(collected.trailers(), None);
        assert_eq!(collected.to_bytes().as_ref(), b"123456789");
    }

    #[test]
    fn test_convert_headers() {
        let mut http1_headermap = http_1x::HeaderMap::new();
        http1_headermap.append(CT1, HeaderValue::from_static("a"));
        http1_headermap.append(CT1, HeaderValue::from_static("b"));
        http1_headermap.append(CT1, HeaderValue::from_static("c"));

        http1_headermap.insert(CL1, HeaderValue::from_static("1234"));

        let mut expect = http::HeaderMap::new();
        expect.append(CT0, http::HeaderValue::from_static("a"));
        expect.append(CT0, http::HeaderValue::from_static("b"));
        expect.append(CT0, http::HeaderValue::from_static("c"));

        expect.insert(CL0, http::HeaderValue::from_static("1234"));

        assert_eq!(convert_headers_1x_0x(http1_headermap), expect);
    }

    #[test]
    fn sdkbody_debug_dyn() {
        let body = TestBody {
            chunks: vec![
                Chunk::Data("123"),
                Chunk::Data("456"),
                Chunk::Data("789"),
                Chunk::Trailers(trailers()),
            ]
            .into(),
        };
        let body = SdkBody::from_body_1_x(body);
        assert!(format!("{body:?}").contains("BoxBody"));
    }
}
