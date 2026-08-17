/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use bytes::Bytes;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::content_encoding::body::{AwsChunkedBody, AwsChunkedBodyError, AwsChunkedBodyState};
use crate::content_encoding::{CHUNK_TERMINATOR, CRLF, TRAILER_SEPARATOR};

impl<Inner> http_body_04x::Body for AwsChunkedBody<Inner>
where
    Inner: http_body_04x::Body<Data = Bytes, Error = aws_smithy_types::body::Error>,
{
    type Data = Bytes;
    type Error = aws_smithy_types::body::Error;

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        tracing::trace!(state = ?self.state, "polling AwsChunkedBody");
        let mut this = self.project();

        use AwsChunkedBodyState::*;
        match *this.state {
            WritingChunk => {
                if this.options.stream_length == 0 {
                    // If the stream is empty, we skip to writing trailers after writing the CHUNK_TERMINATOR.
                    *this.state = WritingTrailers;
                    tracing::trace!("stream is empty, writing chunk terminator");
                    Poll::Ready(Some(Ok(Bytes::from([CHUNK_TERMINATOR].concat()))))
                } else {
                    *this.state = WritingChunkData;
                    // A chunk must be prefixed by chunk size in hexadecimal
                    let chunk_size = format!("{:X?}{CRLF}", this.options.stream_length);
                    tracing::trace!(%chunk_size, "writing chunk size");
                    let chunk_size = Bytes::from(chunk_size);
                    Poll::Ready(Some(Ok(chunk_size)))
                }
            }
            WritingChunkData => match this.inner.poll_data(cx) {
                Poll::Ready(Some(Ok(data))) => {
                    tracing::trace!(len = data.len(), "writing chunk data");
                    *this.inner_body_bytes_read_so_far += data.len();
                    Poll::Ready(Some(Ok(data)))
                }
                Poll::Ready(None) => {
                    let actual_stream_length = *this.inner_body_bytes_read_so_far as u64;
                    let expected_stream_length = this.options.stream_length;
                    if actual_stream_length != expected_stream_length {
                        let err = Box::new(AwsChunkedBodyError::StreamLengthMismatch {
                            actual: actual_stream_length,
                            expected: expected_stream_length,
                        });
                        return Poll::Ready(Some(Err(err)));
                    };

                    tracing::trace!("no more chunk data, writing CRLF and chunk terminator");
                    *this.state = WritingTrailers;
                    // Since we wrote chunk data, we end it with a CRLF and since we only write
                    // a single chunk, we write the CHUNK_TERMINATOR immediately after
                    Poll::Ready(Some(Ok(Bytes::from([CRLF, CHUNK_TERMINATOR].concat()))))
                }
                Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
                Poll::Pending => Poll::Pending,
            },
            WritingTrailers => {
                return match this.inner.poll_trailers(cx) {
                    Poll::Ready(Ok(trailers)) => {
                        *this.state = Closed;
                        let expected_length = total_rendered_length_of_trailers(trailers.as_ref());
                        let actual_length = this.options.total_trailer_length();

                        if expected_length != actual_length {
                            let err = AwsChunkedBodyError::ReportedTrailerLengthMismatch {
                                actual: actual_length,
                                expected: expected_length,
                            };
                            return Poll::Ready(Some(Err(err.into())));
                        }

                        let mut trailers =
                            trailers_as_aws_chunked_bytes(trailers, actual_length + 1);
                        // Insert the final CRLF to close the body
                        trailers.extend_from_slice(CRLF.as_bytes());

                        Poll::Ready(Some(Ok(trailers.into())))
                    }
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
                };
            }
            Closed => Poll::Ready(None),
            ref otherwise => {
                unreachable!(
                    "invalid state {otherwise:?} for `poll_data` in http-02x; this is a bug"
                )
            }
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<Option<http_02x::HeaderMap<http_02x::HeaderValue>>, Self::Error>> {
        // Trailers were already appended to the body because of the content encoding scheme
        Poll::Ready(Ok(None))
    }

    fn is_end_stream(&self) -> bool {
        self.state == AwsChunkedBodyState::Closed
    }

    fn size_hint(&self) -> http_body_04x::SizeHint {
        http_body_04x::SizeHint::with_exact(self.options.encoded_length())
    }
}

/// Writes trailers out into a `string` and then converts that `String` to a `Bytes` before
/// returning.
///
/// - Trailer names are separated by a single colon only, no space.
/// - Trailer names with multiple values will be written out one line per value, with the name
///   appearing on each line.
fn trailers_as_aws_chunked_bytes(
    trailer_map: Option<http_02x::HeaderMap>,
    estimated_length: u64,
) -> bytes::BytesMut {
    if let Some(trailer_map) = trailer_map {
        let mut current_header_name = None;
        let mut trailers =
            bytes::BytesMut::with_capacity(estimated_length.try_into().unwrap_or_default());

        for (header_name, header_value) in trailer_map.into_iter() {
            // When a header has multiple values, the name only comes up in iteration the first time
            // we see it. Therefore, we need to keep track of the last name we saw and fall back to
            // it when `header_name == None`.
            current_header_name = header_name.or(current_header_name);

            // In practice, this will always exist, but `if let` is nicer than unwrap
            if let Some(header_name) = current_header_name.as_ref() {
                trailers.extend_from_slice(header_name.as_ref());
                trailers.extend_from_slice(TRAILER_SEPARATOR);
                trailers.extend_from_slice(header_value.as_bytes());
                trailers.extend_from_slice(CRLF.as_bytes());
            }
        }

        trailers
    } else {
        bytes::BytesMut::new()
    }
}

/// Given an optional `HeaderMap`, calculate the total number of bytes required to represent the
/// `HeaderMap`. If no `HeaderMap` is given as input, return 0.
///
/// - Trailer names are separated by a single colon only, no space.
/// - Trailer names with multiple values will be written out one line per value, with the name
///   appearing on each line.
fn total_rendered_length_of_trailers(trailer_map: Option<&http_02x::HeaderMap>) -> u64 {
    match trailer_map {
        Some(trailer_map) => trailer_map
            .iter()
            .map(|(trailer_name, trailer_value)| {
                trailer_name.as_str().len()
                    + TRAILER_SEPARATOR.len()
                    + trailer_value.len()
                    + CRLF.len()
            })
            .sum::<usize>() as u64,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::{total_rendered_length_of_trailers, trailers_as_aws_chunked_bytes};
    use crate::content_encoding::{AwsChunkedBody, AwsChunkedBodyOptions, CHUNK_TERMINATOR, CRLF};

    use aws_smithy_types::body::SdkBody;
    use bytes::{Buf, Bytes};
    use bytes_utils::SegmentedBuf;
    use http_02x::{HeaderMap, HeaderValue};
    use http_body_04x::{Body, SizeHint};
    use pin_project_lite::pin_project;

    use std::io::Read;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::time::Duration;

    pin_project! {
        struct SputteringBody {
            parts: Vec<Option<Bytes>>,
            cursor: usize,
            delay_in_millis: u64,
        }
    }

    impl SputteringBody {
        fn len(&self) -> usize {
            self.parts.iter().flatten().map(|b| b.len()).sum()
        }
    }

    impl Body for SputteringBody {
        type Data = Bytes;
        type Error = aws_smithy_types::body::Error;

        fn poll_data(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
            if self.cursor == self.parts.len() {
                return Poll::Ready(None);
            }

            let this = self.project();
            let delay_in_millis = *this.delay_in_millis;
            let next_part = this.parts.get_mut(*this.cursor).unwrap().take();

            match next_part {
                None => {
                    *this.cursor += 1;
                    let waker = cx.waker().clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_millis(delay_in_millis)).await;
                        waker.wake();
                    });
                    Poll::Pending
                }
                Some(data) => {
                    *this.cursor += 1;
                    Poll::Ready(Some(Ok(data)))
                }
            }
        }

        fn poll_trailers(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<Option<HeaderMap<HeaderValue>>, Self::Error>> {
            Poll::Ready(Ok(None))
        }

        fn is_end_stream(&self) -> bool {
            false
        }

        fn size_hint(&self) -> SizeHint {
            SizeHint::new()
        }
    }

    #[tokio::test]
    async fn test_aws_chunked_encoding() {
        let test_fut = async {
            let input_str = "Hello world";
            let opts = AwsChunkedBodyOptions::new(input_str.len() as u64, Vec::new());
            let mut body = AwsChunkedBody::new(SdkBody::from(input_str), opts);

            let mut output = SegmentedBuf::new();
            while let Some(buf) = body.data().await {
                output.push(buf.unwrap());
            }

            let mut actual_output = String::new();
            output
                .reader()
                .read_to_string(&mut actual_output)
                .expect("Doesn't cause IO errors");

            let expected_output = "B\r\nHello world\r\n0\r\n\r\n";

            assert_eq!(expected_output, actual_output);
            assert!(
                body.trailers()
                    .await
                    .expect("no errors occurred during trailer polling")
                    .is_none(),
                "aws-chunked encoded bodies don't have normal HTTP trailers"
            );

            // You can insert a `tokio::time::sleep` here to verify the timeout works as intended
        };

        let timeout_duration = Duration::from_secs(3);
        if tokio::time::timeout(timeout_duration, test_fut)
            .await
            .is_err()
        {
            panic!("test_aws_chunked_encoding timed out after {timeout_duration:?}");
        }
    }

    #[tokio::test]
    async fn test_aws_chunked_encoding_sputtering_body() {
        let test_fut = async {
            let input = SputteringBody {
                parts: vec![
                    Some(Bytes::from_static(b"chunk 1, ")),
                    None,
                    Some(Bytes::from_static(b"chunk 2, ")),
                    Some(Bytes::from_static(b"chunk 3, ")),
                    None,
                    None,
                    Some(Bytes::from_static(b"chunk 4, ")),
                    Some(Bytes::from_static(b"chunk 5, ")),
                    Some(Bytes::from_static(b"chunk 6")),
                ],
                cursor: 0,
                delay_in_millis: 500,
            };
            let opts = AwsChunkedBodyOptions::new(input.len() as u64, Vec::new());
            let mut body = AwsChunkedBody::new(input, opts);

            let mut output = SegmentedBuf::new();
            while let Some(buf) = body.data().await {
                output.push(buf.unwrap());
            }

            let mut actual_output = String::new();
            output
                .reader()
                .read_to_string(&mut actual_output)
                .expect("Doesn't cause IO errors");

            let expected_output =
                "34\r\nchunk 1, chunk 2, chunk 3, chunk 4, chunk 5, chunk 6\r\n0\r\n\r\n";

            assert_eq!(expected_output, actual_output);
            assert!(
                body.trailers()
                    .await
                    .expect("no errors occurred during trailer polling")
                    .is_none(),
                "aws-chunked encoded bodies don't have normal HTTP trailers"
            );
        };

        let timeout_duration = Duration::from_secs(3);
        if tokio::time::timeout(timeout_duration, test_fut)
            .await
            .is_err()
        {
            panic!(
                "test_aws_chunked_encoding_sputtering_body timed out after {timeout_duration:?}"
            );
        }
    }

    #[tokio::test]
    #[should_panic = "called `Result::unwrap()` on an `Err` value: ReportedTrailerLengthMismatch { actual: 44, expected: 0 }"]
    async fn test_aws_chunked_encoding_incorrect_trailer_length_panic() {
        let input_str = "Hello world";
        // Test body has no trailers, so this length is incorrect and will trigger an assert panic
        // When the panic occurs, it will actually expect a length of 44. This is because, when using
        // aws-chunked encoding, each trailer will end with a CRLF which is 2 bytes long.
        let wrong_trailer_len = 42;
        let opts = AwsChunkedBodyOptions::new(input_str.len() as u64, vec![wrong_trailer_len]);
        let mut body = AwsChunkedBody::new(SdkBody::from(input_str), opts);

        // We don't care about the body contents but we have to read it all before checking for trailers
        while let Some(buf) = body.data().await {
            drop(buf.unwrap());
        }

        assert!(
            body.trailers()
                .await
                .expect("no errors occurred during trailer polling")
                .is_none(),
            "aws-chunked encoded bodies don't have normal HTTP trailers"
        );
    }

    #[tokio::test]
    async fn test_aws_chunked_encoding_empty_body() {
        let input_str = "";
        let opts = AwsChunkedBodyOptions::new(input_str.len() as u64, Vec::new());
        let mut body = AwsChunkedBody::new(SdkBody::from(input_str), opts);

        let mut output = SegmentedBuf::new();
        while let Some(buf) = body.data().await {
            output.push(buf.unwrap());
        }

        let mut actual_output = String::new();
        output
            .reader()
            .read_to_string(&mut actual_output)
            .expect("Doesn't cause IO errors");

        let expected_output = [CHUNK_TERMINATOR, CRLF].concat();

        assert_eq!(expected_output, actual_output);
        assert!(
            body.trailers()
                .await
                .expect("no errors occurred during trailer polling")
                .is_none(),
            "aws-chunked encoded bodies don't have normal HTTP trailers"
        );
    }

    #[tokio::test]
    async fn test_total_rendered_length_of_trailers() {
        let mut trailers = HeaderMap::new();

        trailers.insert("empty_value", HeaderValue::from_static(""));

        trailers.insert("single_value", HeaderValue::from_static("value 1"));

        trailers.insert("two_values", HeaderValue::from_static("value 1"));
        trailers.append("two_values", HeaderValue::from_static("value 2"));

        trailers.insert("three_values", HeaderValue::from_static("value 1"));
        trailers.append("three_values", HeaderValue::from_static("value 2"));
        trailers.append("three_values", HeaderValue::from_static("value 3"));

        let trailers = Some(trailers);
        let actual_length = total_rendered_length_of_trailers(trailers.as_ref());
        let expected_length = (trailers_as_aws_chunked_bytes(trailers, actual_length).len()) as u64;

        assert_eq!(expected_length, actual_length);
    }

    #[tokio::test]
    async fn test_total_rendered_length_of_empty_trailers() {
        let trailers = Some(HeaderMap::new());
        let actual_length = total_rendered_length_of_trailers(trailers.as_ref());
        let expected_length = (trailers_as_aws_chunked_bytes(trailers, actual_length).len()) as u64;

        assert_eq!(expected_length, actual_length);
    }
}
