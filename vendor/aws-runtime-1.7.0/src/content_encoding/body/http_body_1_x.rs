/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use bytes::{Buf, Bytes, BytesMut};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::content_encoding::body::{AwsChunkedBody, AwsChunkedBodyError, AwsChunkedBodyState};
use crate::content_encoding::{
    header, SignChunk, CHUNK_SIGNATURE_BEGIN_RAW, CHUNK_TERMINATOR_RAW, CRLF_RAW, TRAILER_SEPARATOR,
};
use aws_sigv4::http_request::SigningError;
use aws_smithy_runtime_api::http::Headers;

macro_rules! signer_mut {
    ($this:expr) => {
        $this
            .signer
            .as_mut()
            .get_mut()
            .as_mut()
            .expect("signer must be set")
            .0
            .as_mut()
    };
}

impl<Inner> http_body_1x::Body for AwsChunkedBody<Inner>
where
    Inner: http_body_1x::Body<Data = Bytes, Error = aws_smithy_types::body::Error>,
{
    type Data = Bytes;
    type Error = aws_smithy_types::body::Error;

    fn is_end_stream(&self) -> bool {
        self.state == AwsChunkedBodyState::Closed
    }

    fn size_hint(&self) -> http_body_1x::SizeHint {
        http_body_1x::SizeHint::with_exact(self.options.encoded_length())
    }

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<http_body_1x::Frame<Self::Data>, Self::Error>>> {
        tracing::trace!(state = ?self.state, "polling AwsChunkedBody");
        let mut this = self.project();
        let chunk_size = this.options.chunk_size();

        use AwsChunkedBodyState::*;
        match *this.state {
            WritingChunk => {
                while !this.chunk_buffer.is_eos() {
                    if this.chunk_buffer.remaining() >= chunk_size {
                        let buf = this.chunk_buffer.buffered();
                        let chunk_bytes = buf.copy_to_bytes(chunk_size);
                        let chunk = if this.options.is_signed {
                            let signer = signer_mut!(this);
                            signed_encoded_chunk(signer, chunk_bytes).map_err(|e| {
                                Box::new(AwsChunkedBodyError::FailedToSign { source: e })
                            })?
                        } else {
                            unsigned_encoded_chunk(chunk_bytes)
                        };
                        *this.inner_body_bytes_read_so_far += chunk_size;
                        tracing::trace!("writing chunk data: {:#?}", chunk);
                        return Poll::Ready(Some(Ok(http_body_1x::Frame::data(chunk))));
                    }

                    match Self::buffer_next_chunk(
                        this.inner.as_mut(),
                        this.chunk_buffer.as_mut(),
                        this.buffered_trailing_headers.as_mut(),
                        cx,
                    ) {
                        Poll::Ready(Ok(true)) => continue,
                        Poll::Ready(Ok(false)) => break,
                        Poll::Ready(Err(e)) => return Poll::Ready(Some(Err(e))),
                        Poll::Pending => return Poll::Pending,
                    }
                }

                if this.chunk_buffer.remaining() > 0 {
                    let bytes_len_to_read =
                        std::cmp::min(this.chunk_buffer.remaining(), chunk_size);
                    let buf = this.chunk_buffer.buffered();
                    let chunk_bytes = buf.copy_to_bytes(bytes_len_to_read);
                    let chunk = if this.options.is_signed {
                        let signer = signer_mut!(this);
                        signed_encoded_chunk(signer, chunk_bytes).map_err(|e| {
                            Box::new(AwsChunkedBodyError::FailedToSign { source: e })
                        })?
                    } else {
                        unsigned_encoded_chunk(chunk_bytes)
                    };
                    *this.inner_body_bytes_read_so_far += bytes_len_to_read;
                    tracing::trace!("remaining chunk data: {:#?}", chunk);
                    return Poll::Ready(Some(Ok(http_body_1x::Frame::data(chunk))));
                }

                debug_assert!(this.chunk_buffer.remaining() == 0);

                // We exhausted the body data, now check if the length is correct
                if let Err(poll_stream_len_err) = check_for_stream_length_mismatch(
                    *this.inner_body_bytes_read_so_far as u64,
                    this.options.stream_length,
                ) {
                    return poll_stream_len_err;
                }

                if this.options.is_signed {
                    *this.state = WritingZeroSizedSignedChunk;
                } else {
                    *this.state = PollingTrailers;
                }
                // Inner future has already returned `Ready` - no active waker.
                // Wake explicitly to ensure the task gets polled again.
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            WritingZeroSizedSignedChunk => {
                let signer = signer_mut!(this);
                let zero_sized_chunk = signed_encoded_chunk(signer, Bytes::new())
                    .map_err(|e| Box::new(AwsChunkedBodyError::FailedToSign { source: e }))?;
                if this.buffered_trailing_headers.is_some() {
                    *this.state = PollingTrailers;
                    let mut zero_sized_chunk = BytesMut::from(&zero_sized_chunk[..]);
                    debug_assert!(zero_sized_chunk.ends_with(b"\r\n\r\n"));
                    // For trailing checksum, we do not want the second CRLF as the checksum is appended in-between two CRLFs
                    zero_sized_chunk.truncate(zero_sized_chunk.len() - 2);
                    let zero_sized_chunk = zero_sized_chunk.freeze();
                    tracing::trace!("writing zero sized signed chunk: {:#?}", zero_sized_chunk);
                    Poll::Ready(Some(Ok(http_body_1x::Frame::data(zero_sized_chunk))))
                } else {
                    *this.state = Closed;
                    tracing::trace!(
                        "writing zero sized signed chunk without trailer: {:#?}",
                        zero_sized_chunk
                    );
                    Poll::Ready(Some(Ok(http_body_1x::Frame::data(zero_sized_chunk))))
                }
            }
            PollingTrailers => match this.inner.as_mut().poll_frame(cx) {
                Poll::Ready(Some(Ok(frame))) => {
                    let trailers = frame.into_trailers().ok();
                    if let Some(trailers) = trailers {
                        match this.buffered_trailing_headers.as_mut().get_mut() {
                            Some(existing) => existing.extend(trailers),
                            None => {
                                *this.buffered_trailing_headers.as_mut().get_mut() = Some(trailers)
                            }
                        }
                    }
                    // Inner future has already returned `Ready` - no active waker.
                    // Wake explicitly to ensure the task gets polled again.
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
                Poll::Ready(Some(Err(err))) => {
                    tracing::error!(error = ?err, "error polling inner");
                    Poll::Ready(Some(Err(err)))
                }
                Poll::Ready(None) => {
                    *this.state = WritingTrailers;
                    // Inner future has already returned `Ready` - no active waker.
                    // Wake explicitly to ensure the task gets polled again.
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
                Poll::Pending => Poll::Pending,
            },
            WritingTrailers => {
                let mut final_chunk = if this.options.is_signed {
                    BytesMut::new()
                } else {
                    BytesMut::from(CHUNK_TERMINATOR_RAW)
                };

                let trailer_bytes = if let Some(mut trailer) = this.buffered_trailing_headers.take()
                {
                    let mut trailer_bytes = BytesMut::new();
                    let trailer = if this.options.is_signed && !trailer.is_empty() {
                        let signer = signer_mut!(this);
                        let signature = signer
                            .trailer_signature(&Headers::try_from(trailer.clone())?)
                            .map_err(|e| {
                                Box::new(AwsChunkedBodyError::FailedToSign { source: e })
                            })?;
                        trailer.insert(
                            http_1x::header::HeaderName::from_static(
                                header::X_AMZ_TRAILER_SIGNATURE,
                            ),
                            http_1x::header::HeaderValue::from_str(&signature).unwrap(),
                        );
                        trailer
                    } else {
                        trailer
                    };

                    let actual_length: u64 = total_rendered_length_of_trailers(Some(&trailer));
                    let expected_length = this.options.total_trailer_length();
                    if expected_length != actual_length {
                        let err = AwsChunkedBodyError::ReportedTrailerLengthMismatch {
                            actual: actual_length,
                            expected: expected_length,
                        };
                        return Poll::Ready(Some(Err(err.into())));
                    }

                    trailer_bytes = trailers_as_aws_chunked_bytes(Some(&trailer), trailer_bytes);
                    trailer_bytes.freeze()
                } else {
                    Bytes::new()
                };

                *this.state = Closed;

                if final_chunk.is_empty() && trailer_bytes.is_empty() {
                    // Case for signed aws-chunked encoding with no trailers
                    return Poll::Ready(None);
                }

                final_chunk.extend_from_slice(&trailer_bytes);
                final_chunk.extend_from_slice(CRLF_RAW);

                tracing::trace!("final chunk: {:#?}", final_chunk);
                Poll::Ready(Some(Ok(http_body_1x::Frame::data(final_chunk.freeze()))))
            }
            Closed => Poll::Ready(None),
            #[allow(unreachable_patterns)]
            // needed when cargo feature `http-02x` is enabled, bringing in an unused enum `WritingChunkData`
            ref otherwise => {
                unreachable!(
                    "invalid state {otherwise:?} for `poll_frame` in http-1x; this is a bug"
                )
            }
        }
    }
}

fn signed_encoded_chunk(
    signer: &mut (dyn SignChunk + Send + Sync),
    chunk_bytes: Bytes,
) -> Result<Bytes, SigningError> {
    let chunk_size = format!("{:X}", chunk_bytes.len());
    let mut chunk = BytesMut::new();
    chunk.extend_from_slice(chunk_size.as_bytes());
    chunk.extend_from_slice(CHUNK_SIGNATURE_BEGIN_RAW);
    chunk.extend_from_slice(signer.chunk_signature(&chunk_bytes)?.as_bytes());
    chunk.extend_from_slice(CRLF_RAW);
    chunk.extend_from_slice(&chunk_bytes);
    chunk.extend_from_slice(CRLF_RAW);
    Ok(chunk.freeze())
}

fn unsigned_encoded_chunk(chunk_bytes: Bytes) -> Bytes {
    let chunk_size = format!("{:X}", chunk_bytes.len());
    let mut chunk = BytesMut::new();
    chunk.extend_from_slice(chunk_size.as_bytes());
    chunk.extend_from_slice(CRLF_RAW);
    chunk.extend_from_slice(&chunk_bytes);
    chunk.extend_from_slice(CRLF_RAW);
    chunk.freeze()
}

/// Writes trailers out into a byte array `buffer`.
///
/// - Trailer names are separated by a single colon only, no space.
/// - Trailer names with multiple values will be written out one line per value, with the name
///   appearing on each line.
fn trailers_as_aws_chunked_bytes(
    trailer_map: Option<&http_1x::HeaderMap>,
    mut buffer: BytesMut,
) -> BytesMut {
    if let Some(trailer_map) = trailer_map {
        let mut current_header_name: Option<http_1x::header::HeaderName> = None;

        for (header_name, header_value) in trailer_map.clone().into_iter() {
            // When a header has multiple values, the name only comes up in iteration the first time
            // we see it. Therefore, we need to keep track of the last name we saw and fall back to
            // it when `header_name == None`.
            current_header_name = header_name.or(current_header_name);

            // In practice, this will always exist, but `if let` is nicer than unwrap
            if let Some(header_name) = current_header_name.as_ref() {
                buffer.extend_from_slice(header_name.as_ref());
                buffer.extend_from_slice(TRAILER_SEPARATOR);
                buffer.extend_from_slice(header_value.as_bytes());
                buffer.extend_from_slice(CRLF_RAW);
            }
        }

        buffer
    } else {
        buffer
    }
}

/// Given an optional `HeaderMap`, calculate the total number of bytes required to represent the
/// `HeaderMap`. If no `HeaderMap` is given as input, return 0.
///
/// - Trailer names are separated by a single colon only, no space.
/// - Trailer names with multiple values will be written out one line per value, with the name
///   appearing on each line.
fn total_rendered_length_of_trailers(trailer_map: Option<&http_1x::HeaderMap>) -> u64 {
    match trailer_map {
        Some(trailer_map) => trailer_map
            .iter()
            .map(|(trailer_name, trailer_value)| {
                trailer_name.as_str().len()
                    + TRAILER_SEPARATOR.len()
                    + trailer_value.len()
                    + CRLF_RAW.len()
            })
            .sum::<usize>() as u64,
        None => 0,
    }
}

/// This is an ugly return type, but in practice it just returns `Ok(())` if the values match
/// and `Err(Poll::Ready(Some(Err(AwsChunkedBodyError::StreamLengthMismatch))))` if they don't
#[allow(clippy::type_complexity)]
fn check_for_stream_length_mismatch(
    actual_stream_length: u64,
    expected_stream_length: u64,
) -> Result<(), Poll<Option<Result<http_body_1x::Frame<Bytes>, aws_smithy_types::body::Error>>>> {
    if actual_stream_length != expected_stream_length {
        let err = Box::new(AwsChunkedBodyError::StreamLengthMismatch {
            actual: actual_stream_length,
            expected: expected_stream_length,
        });
        return Err(Poll::Ready(Some(Err(err))));
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{total_rendered_length_of_trailers, trailers_as_aws_chunked_bytes};
    use crate::content_encoding::{
        AwsChunkedBody, AwsChunkedBodyOptions, CHUNK_TERMINATOR_RAW, CRLF_RAW,
        DEFAULT_CHUNK_SIZE_BYTE,
    };

    use aws_smithy_types::body::SdkBody;
    use bytes::{Buf, Bytes, BytesMut};
    use bytes_utils::SegmentedBuf;
    use http_1x::{HeaderMap, HeaderValue};
    use http_body_1x::{Body, Frame, SizeHint};
    use http_body_util::BodyExt;
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

        fn poll_frame(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
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
                    let frame = Frame::data(data);
                    Poll::Ready(Some(Ok(frame)))
                }
            }
        }

        fn is_end_stream(&self) -> bool {
            false
        }

        fn size_hint(&self) -> SizeHint {
            SizeHint::new()
        }
    }

    // Custom body that returns data and trailers
    pin_project! {
        struct TestBodyWithTrailers {
            data: Option<Bytes>,
            trailers: Option<HeaderMap>,
        }
    }

    impl Body for TestBodyWithTrailers {
        type Data = Bytes;
        type Error = aws_smithy_types::body::Error;

        fn poll_frame(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<http_body_1x::Frame<Self::Data>, Self::Error>>> {
            let this = self.project();

            if let Some(data) = this.data.take() {
                return Poll::Ready(Some(Ok(http_body_1x::Frame::data(data))));
            }

            if let Some(trailers) = this.trailers.take() {
                return Poll::Ready(Some(Ok(http_body_1x::Frame::trailers(trailers))));
            }

            Poll::Ready(None)
        }
    }

    #[tokio::test]
    async fn test_aws_chunked_encoding() {
        let test_fut = async {
            let input_str = "Hello world";
            let opts = AwsChunkedBodyOptions::new(input_str.len() as u64, vec![]);
            let mut body = AwsChunkedBody::new(SdkBody::from(input_str), opts);

            let mut output: SegmentedBuf<Bytes> = SegmentedBuf::new();
            while let Some(Ok(buf)) = body.frame().await {
                output.push(buf.into_data().unwrap());
            }

            let mut actual_output = String::new();
            output
                .reader()
                .read_to_string(&mut actual_output)
                .expect("Doesn't cause IO errors");

            let expected_output = "B\r\nHello world\r\n0\r\n\r\n";

            assert_eq!(expected_output, actual_output);

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
            let opts = AwsChunkedBodyOptions::new(input.len() as u64, vec![]);
            let mut body = AwsChunkedBody::new(input, opts);

            let mut output: SegmentedBuf<Bytes> = SegmentedBuf::new();
            while let Some(Ok(buf)) = body.frame().await {
                output.push(buf.into_data().unwrap());
            }

            let mut actual_output = String::new();
            output
                .reader()
                .read_to_string(&mut actual_output)
                .expect("Doesn't cause IO errors");

            let expected_output =
                "34\r\nchunk 1, chunk 2, chunk 3, chunk 4, chunk 5, chunk 6\r\n0\r\n\r\n";

            assert_eq!(expected_output, actual_output);
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
    async fn test_aws_chunked_encoding_incorrect_trailer_length_panic() {
        let input_str = "Hello world";
        // Test body has no trailers, so this length is incorrect and will trigger an assert panic
        // When the panic occurs, it will actually expect a length of 44. This is because, when using
        // aws-chunked encoding, each trailer will end with a CRLF which is 2 bytes long.
        let wrong_trailer_len = 42;
        let opts = AwsChunkedBodyOptions::new(input_str.len() as u64, vec![wrong_trailer_len]);
        let mut body = AwsChunkedBody::new(SdkBody::from(input_str), opts);

        // We don't care about the body contents but we have to read it all before checking for trailers
        while let Some(Ok(frame)) = body.frame().await {
            assert!(!frame.is_trailers());
        }
    }

    #[tokio::test]
    async fn test_aws_chunked_encoding_empty_body() {
        let input_str = "";
        let opts = AwsChunkedBodyOptions::new(input_str.len() as u64, vec![]);
        let mut body = AwsChunkedBody::new(SdkBody::from(input_str), opts);

        let mut output: SegmentedBuf<Bytes> = SegmentedBuf::new();
        while let Some(Ok(frame)) = body.frame().await {
            output.push(frame.into_data().unwrap());
        }

        let mut actual_output = String::new();
        output
            .reader()
            .read_to_string(&mut actual_output)
            .expect("Doesn't cause IO errors");

        let actual_output = std::str::from_utf8(actual_output.as_bytes()).unwrap();
        let expected_output = [CHUNK_TERMINATOR_RAW, CRLF_RAW].concat();
        let expected_output = std::str::from_utf8(&expected_output).unwrap();

        assert_eq!(expected_output, actual_output);
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

        let trailers = Some(&trailers);
        let actual_length = total_rendered_length_of_trailers(trailers);
        let buf = BytesMut::with_capacity(actual_length as usize);
        let expected_length = (trailers_as_aws_chunked_bytes(trailers, buf).len()) as u64;

        assert_eq!(expected_length, actual_length);
    }

    #[tokio::test]
    async fn test_total_rendered_length_of_empty_trailers() {
        let header_map = HeaderMap::new();
        let trailers = Some(&header_map);
        let actual_length = total_rendered_length_of_trailers(trailers);
        let buf = BytesMut::with_capacity(actual_length as usize);
        let expected_length = (trailers_as_aws_chunked_bytes(trailers, buf).len()) as u64;

        assert_eq!(expected_length, actual_length);
    }

    #[tokio::test]
    async fn test_poll_frame_with_default_chunk_size() {
        let test_data = Bytes::from("1234567890123456789012345");
        let body = SdkBody::from(test_data.clone());
        let options = AwsChunkedBodyOptions::new(test_data.len() as u64, vec![]);
        let mut chunked_body = AwsChunkedBody::new(body, options);

        let mut data_frames: Vec<Bytes> = Vec::new();
        while let Some(frame) = chunked_body.frame().await.transpose().unwrap() {
            if let Ok(data) = frame.into_data() {
                data_frames.push(data);
            }
        }

        assert_eq!(data_frames.len(), 2); // Data fits in one chunk, plus the final chunk
        assert_eq!(
            Bytes::from_static(b"19\r\n1234567890123456789012345\r\n"),
            data_frames[0]
        );
        assert_eq!(Bytes::from_static(b"0\r\n\r\n"), data_frames[1]);
    }

    #[tokio::test]
    async fn test_poll_frame_with_custom_chunk_size() {
        let test_data = Bytes::from("1234567890123456789012345");
        let body = SdkBody::from(test_data.clone());
        let options =
            AwsChunkedBodyOptions::new(test_data.len() as u64, vec![]).with_chunk_size(10);
        let mut chunked_body = AwsChunkedBody::new(body, options);

        let mut data_frames: Vec<Bytes> = Vec::new();
        while let Some(frame) = chunked_body.frame().await.transpose().unwrap() {
            if let Ok(data) = frame.into_data() {
                data_frames.push(data);
            }
        }

        assert_eq!(4, data_frames.len()); // 25 bytes / 10 = 2.5 so 3 chunks, plus the final chunk
        assert_eq!(Bytes::from_static(b"A\r\n1234567890\r\n"), data_frames[0]);
        assert_eq!(Bytes::from_static(b"A\r\n1234567890\r\n"), data_frames[1]);
        assert_eq!(Bytes::from_static(b"5\r\n12345\r\n"), data_frames[2]);
        assert_eq!(Bytes::from_static(b"0\r\n\r\n"), data_frames[3]);
    }

    #[tokio::test]
    async fn test_poll_frame_with_trailers() {
        let data = Bytes::from("1234567890123456789012345");
        let stream_len = data.len() as u64;
        let mut trailers = HeaderMap::new();
        trailers.insert("x-amz-checksum-crc32", HeaderValue::from_static("78DeVw=="));
        let body = TestBodyWithTrailers {
            data: Some(data),
            trailers: Some(trailers),
        };
        let options = AwsChunkedBodyOptions::new(stream_len, vec![29]).with_chunk_size(10);
        let mut chunked_body = AwsChunkedBody::new(body, options);

        let mut data_frames: Vec<Bytes> = Vec::new();
        while let Some(frame) = chunked_body.frame().await.transpose().unwrap() {
            if let Ok(data) = frame.into_data() {
                data_frames.push(data);
            }
        }

        assert_eq!(4, data_frames.len()); // 25 bytes / 10 = 2.5 so 3 chunks, plus the final chunk
        assert_eq!(Bytes::from_static(b"A\r\n1234567890\r\n"), data_frames[0]);
        assert_eq!(Bytes::from_static(b"A\r\n1234567890\r\n"), data_frames[1]);
        assert_eq!(Bytes::from_static(b"5\r\n12345\r\n"), data_frames[2]);
        assert_eq!(
            Bytes::from_static(b"0\r\nx-amz-checksum-crc32:78DeVw==\r\n\r\n"),
            data_frames[3]
        );
    }

    // Testing scenario derived from https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-streaming.html
    #[tokio::test]
    async fn test_aws_chunked_body_poll_frame_with_signer() {
        use crate::auth::sigv4::SigV4MessageSigner;
        use aws_credential_types::Credentials;
        use aws_sigv4::http_request::SigningSettings;
        use aws_smithy_async::time::{SharedTimeSource, StaticTimeSource};
        use aws_types::region::SigningRegion;
        use aws_types::SigningName;
        use std::time::{Duration, UNIX_EPOCH};

        // 65KB of 'a' characters
        let data = "a".repeat(65 * 1024);
        let stream_len = data.len() as u64;
        let inner_body = SdkBody::from(data);

        // `StaticTimeSource` for 20130524T000000Z
        let time = StaticTimeSource::new(UNIX_EPOCH + Duration::from_secs(1369353600));
        let shared_time = SharedTimeSource::from(time);

        let credentials = Credentials::new(
            "AKIAIOSFODNN7EXAMPLE",
            "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
            None,
            None,
            "test",
        );

        let seed_signature =
            "4f232c4386841ef735655705268965c44a0e4690baa4adea153f7db9fa80a0a9".to_owned();
        let signer = SigV4MessageSigner::new(
            seed_signature,
            credentials.into(),
            SigningRegion::from_static("us-east-1"),
            SigningName::from_static("s3"),
            shared_time,
            SigningSettings::default(),
        );

        let opt = AwsChunkedBodyOptions::new(stream_len, vec![]).signed_chunked_encoding(true);
        let mut chunked_body = AwsChunkedBody::new(inner_body, opt).with_signer(signer);

        let mut data_frames: Vec<Bytes> = Vec::new();
        while let Some(frame) = chunked_body.frame().await.transpose().unwrap() {
            if let Ok(data) = frame.into_data() {
                data_frames.push(data);
            }
        }

        assert_eq!(3, data_frames.len()); // 64 KB, 1 KB, and the final chunk with 0 bytes of chunk data.
        assert!(data_frames[0].starts_with(b"10000;chunk-signature=ad80c730a21e5b8d04586a2213dd63b9a0e99e0e2307b0ade35a65485a288648\r\n"));
        assert!(data_frames[1].starts_with(b"400;chunk-signature=0055627c9e194cb4542bae2aa5492e3c1575bbb81b612b7d234b86a503ef5497\r\n"));
        assert_eq!(data_frames[2], Bytes::from_static(b"0;chunk-signature=b6c6ea8a5354eaf15b3cb7646744f4275b71ea724fed81ceb9323e279d449df9\r\n\r\n"));
    }

    // Testing scenario derived from https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-streaming-trailers.html
    #[tokio::test]
    async fn test_aws_chunked_body_poll_frame_with_signer_and_trailers() {
        use crate::auth::sigv4::SigV4MessageSigner;
        use aws_credential_types::Credentials;
        use aws_sigv4::http_request::SigningSettings;
        use aws_smithy_async::time::{SharedTimeSource, StaticTimeSource};
        use aws_types::region::SigningRegion;
        use aws_types::SigningName;
        use std::time::{Duration, UNIX_EPOCH};

        // 65KB of 'a' characters
        let data = "a".repeat(65 * 1024);
        let stream_len = data.len() as u64;

        // Set trailers with x-amz-checksum-crc32c header
        let mut trailers = HeaderMap::new();
        trailers.insert(
            "x-amz-checksum-crc32c",
            HeaderValue::from_static("sOO8/Q=="),
        );

        let inner_body = TestBodyWithTrailers {
            data: Some(Bytes::from(data)),
            trailers: Some(trailers),
        };

        // `StaticTimeSource` for 20130524T000000Z
        let time = StaticTimeSource::new(UNIX_EPOCH + Duration::from_secs(1369353600));
        let shared_time = SharedTimeSource::from(time);

        let credentials = Credentials::new(
            "AKIAIOSFODNN7EXAMPLE",
            "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
            None,
            None,
            "test",
        );

        let seed_signature =
            "106e2a8a18243abcf37539882f36619c00e2dfc72633413f02d3b74544bfeb8e".to_owned();
        let signer = SigV4MessageSigner::new(
            seed_signature,
            credentials.into(),
            SigningRegion::from_static("us-east-1"),
            SigningName::from_static("s3"),
            shared_time,
            SigningSettings::default(),
        );

        let opt =
            AwsChunkedBodyOptions::new(stream_len, vec![30, 88]).signed_chunked_encoding(true);
        let mut chunked_body = AwsChunkedBody::new(inner_body, opt).with_signer(signer);

        let mut data_frames: Vec<Bytes> = Vec::new();
        while let Some(frame) = chunked_body.frame().await.transpose().unwrap() {
            if let Ok(data) = frame.into_data() {
                data_frames.push(data);
            }
        }

        assert_eq!(4, data_frames.len()); // 64 KB, 1 KB, 0 bytes of chunk data, and the trailer chunk.
        assert!(data_frames[0].starts_with(b"10000;chunk-signature=b474d8862b1487a5145d686f57f013e54db672cee1c953b3010fb58501ef5aa2\r\n"));
        assert!(data_frames[1].starts_with(b"400;chunk-signature=1c1344b170168f8e65b41376b44b20fe354e373826ccbbe2c1d40a8cae51e5c7\r\n"));
        assert_eq!(data_frames[2], Bytes::from_static(b"0;chunk-signature=2ca2aba2005185cf7159c6277faf83795951dd77a3a99e6e65d5c9f85863f992\r\n"));
        assert_eq!(data_frames[3], Bytes::from_static(b"x-amz-checksum-crc32c:sOO8/Q==\r\nx-amz-trailer-signature:d81f82fc3505edab99d459891051a732e8730629a2e4a59689829ca17fe2e435\r\n\r\n"));
    }

    #[test]
    fn test_unsigned_encoded_length_with_no_trailer() {
        {
            let options = AwsChunkedBodyOptions::new(10, vec![]);
            /*
             A\r\n
             10 bytes of data\r\n
             0\r\n
             \r\n
             -------------------------------------------------------------
             1 (A) + 2 (\r\n) +
             10 (data) + 2 (\r\n) +
             1 (0) + 2 (\r\n) +
             2 (\r\n)

                = 20 total bytes
            */
            assert_eq!(options.encoded_length(), 20);
        }
        {
            let options = AwsChunkedBodyOptions::new((DEFAULT_CHUNK_SIZE_BYTE + 10) as u64, vec![]);
            /*
             10000\r\n
             65536 bytes of data\r\n
             A\r\n
             10 bytes of data\r\n
             0\r\n
             \r\n
             -------------------------------------------------------------
             5 (10000) + 2 (\r\n) +
             65536 (data) + 2 (\r\n) +
             1 (A) + 2 (\r\n) +
             10 (data) + 2 (\r\n) +
             1 (0) + 2 (\r\n) +
             2 (\r\n)

                = 65565 total bytes
            */
            assert_eq!(options.encoded_length(), 65565);
        }
    }

    #[test]
    fn test_unsigned_encoded_length_with_trailer() {
        let options = AwsChunkedBodyOptions::new(10, vec![30]);
        /*
            A\r\n
            10 bytes of data\r\n
            0\r\n
            x-amz-checksum-crc32c:sOO8/Q==\r\n
            \r\n
            -------------------------------------------------------------
            1 (A) + 2 (\r\n) +
            10 (data) + 2 (\r\n) +
            1 (0) + 2 (\r\n) +
            21 (x-amz-checksum-crc32c) + 1 (:) + 8 (sOO8/Q==) + 2 (\r\n) +
            2 (\r\n)

                = 52 total bytes
        */
        assert_eq!(options.encoded_length(), 52);
    }

    #[test]
    fn test_signed_encoded_length_with_no_trailer() {
        {
            let options = AwsChunkedBodyOptions::new(10, vec![]).signed_chunked_encoding(true);
            /*
             A;chunk-signature=<signature>\r\n
             10 bytes of data\r\n
             0;chunk-signature=<signature>\r\n
             \r\n
             -------------------------------------------------------------
             1 (A) + 17 (;chunk-signature=) + 64 (signature) + 2 (\r\n) +
             10 (data) + 2 (\r\n) +
             1 (0) + 17 (;chunk-signature) + 64 (signature) + 2 (\r\n) +
             2 (\r\n)

                = 182 total bytes
            */
            assert_eq!(options.encoded_length(), 182);
        }
        {
            let options = AwsChunkedBodyOptions::new((DEFAULT_CHUNK_SIZE_BYTE + 10) as u64, vec![])
                .signed_chunked_encoding(true);
            /*
             10000;chunk-signature=<signature>\r\n
             65536 bytes of data\r\n
             A;chunk-signature=<signature>\r\n
             10 bytes of data\r\n
             0;chunk-signature=<signature>\r\n
             \r\n
             -------------------------------------------------------------
             5 (10000) + 17 (;chunk-signature=) + 64 (signature) + 2 (\r\n) +
             65536 (data) + 2 (\r\n) +
             1 (A) + 17 (;chunk-signature=) + 64 (signature) + 2 (\r\n) +
             10 (data) + 2 (\r\n) +
             1 (0) + 17 (;chunk-signature) + 64 (signature) + 2 (\r\n) +
             2 (\r\n)

                = 65808 total bytes
            */
            assert_eq!(options.encoded_length(), 65808);
        }
    }

    #[test]
    fn test_signed_encoded_length_with_trailer() {
        let options = AwsChunkedBodyOptions::new(10, vec![30, 88]).signed_chunked_encoding(true);
        /*
            A;chunk-signature=<signature>\r\n
            10 bytes of data\r\n
            0;chunk-signature=<signature>\r\n
            x-amz-checksum-crc32c:sOO8/Q==\r\n
            x-amz-trailer-signature:<signature>\r\n
            \r\n
            -------------------------------------------------------------
            1 (A) + 17 (;chunk-signature=) + 64 (signature) + 2 (\r\n) +
            10 (data) + 2 (\r\n) +
            1 (0) + 17 (;chunk-signature) + 64 (signature) + 2 (\r\n) +
            21 (x-amz-checksum-crc32c) + 1 (:) + 8 (sOO8/Q==) + 2 (\r\n) +
            23 (x-amz-trailer-signature) + 1 (:) + 64 (signature) + 2 (\r\n) +
            2 (\r\n)

                = 304 total bytes
        */
        assert_eq!(options.encoded_length(), 304);
    }
}
