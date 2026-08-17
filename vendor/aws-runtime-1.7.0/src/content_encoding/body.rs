/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#[cfg(feature = "http-02x")]
mod http_body_0_x;

mod http_body_1_x;

use crate::content_encoding::{AwsChunkedBodyOptions, SignChunk};
use aws_sigv4::http_request::SigningError;
use bytes::{Buf, Bytes};
use bytes_utils::SegmentedBuf;
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug)]
pub(super) enum ChunkBuf {
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

impl ChunkBuf {
    /// Return true if there's more buffered data.
    pub(super) fn remaining(&self) -> usize {
        match self {
            ChunkBuf::Empty | ChunkBuf::Terminated => 0,
            ChunkBuf::Partial(segments) | ChunkBuf::EosPartial(segments) => segments.remaining(),
        }
    }

    /// Return true if the stream has ended.
    pub(super) fn is_eos(&self) -> bool {
        matches!(self, ChunkBuf::EosPartial(_) | ChunkBuf::Terminated)
    }

    /// Return a mutable reference to the underlying buffered data.
    pub(super) fn buffered(&mut self) -> &mut SegmentedBuf<Bytes> {
        match self {
            ChunkBuf::Empty => panic!("buffer must be populated before reading; this is a bug"),
            ChunkBuf::Partial(segmented) => segmented,
            ChunkBuf::EosPartial(segmented) => segmented,
            ChunkBuf::Terminated => panic!("buffer has been terminated; this is a bug"),
        }
    }

    /// Return a `ChunkBuf` that has reached end of stream.
    pub(super) fn ended(self) -> Self {
        match self {
            ChunkBuf::Empty => ChunkBuf::EosPartial(SegmentedBuf::new()),
            ChunkBuf::Partial(segmented) => ChunkBuf::EosPartial(segmented),
            ChunkBuf::EosPartial(_) => panic!("already end of stream; this is a bug"),
            ChunkBuf::Terminated => panic!("stream terminated; this is a bug"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum AwsChunkedBodyState {
    /// Write out the chunk data.
    WritingChunk,
    #[cfg(feature = "http-02x")]
    /// Write out the next chunk of data. Multiple polls of the inner body may need to occur before
    /// all data is written out.
    WritingChunkData,
    /// Write out a zero-sized signed chunk.
    WritingZeroSizedSignedChunk,
    /// Buffer all trailers from the inner body, which avoids assuming trailing headers fit in a single frame.
    PollingTrailers,
    /// Write out all trailers associated with this `AwsChunkedBody` and then transition into the
    /// `Closed` state.
    WritingTrailers,
    /// This is the final state. Write out the body terminator and then remain in this state.
    Closed,
}

pin_project! {
    /// A request body compatible with `Content-Encoding: aws-chunked`.
    ///
    /// See [SigV4 streaming](https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-streaming.html)
    /// and [streaming trailers](https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-streaming-trailers.html).
    #[derive(Debug)]
    pub struct AwsChunkedBody<InnerBody> {
        #[pin]
        pub(super) inner: InnerBody,
        #[pin]
        pub(super) state: AwsChunkedBodyState,
        pub(super) options: AwsChunkedBodyOptions,
        pub(super) inner_body_bytes_read_so_far: usize,
        #[pin]
        pub(super) chunk_buffer: ChunkBuf,
        #[pin]
        pub(super) buffered_trailing_headers: Option<http_1x::HeaderMap>,
        #[pin]
        pub(super) signer: Option<std::panic::AssertUnwindSafe<Box<dyn SignChunk + Send + Sync>>>,
    }
}

impl<Inner> AwsChunkedBody<Inner> {
    /// Wrap the given body in an outer body compatible with `Content-Encoding: aws-chunked`
    pub fn new(body: Inner, options: AwsChunkedBodyOptions) -> Self {
        Self {
            inner: body,
            state: AwsChunkedBodyState::WritingChunk,
            options,
            inner_body_bytes_read_so_far: 0,
            chunk_buffer: ChunkBuf::Empty,
            buffered_trailing_headers: None,
            signer: None,
        }
    }

    /// Set signer for signing chunks and trailers.
    #[allow(private_bounds)] // Until we support chunk signing for a custom signer, the trait does not need to be public
    pub fn with_signer<S>(mut self, signer: S) -> Self
    where
        S: SignChunk + Send + Sync + 'static,
    {
        self.signer = Some(std::panic::AssertUnwindSafe(Box::new(signer)));
        self
    }

    // Buffer the next chunk from the inner body into the provided `chunk_buffer`, and return
    // whether or not it should continue reading from `inner`.
    //
    // If it has exhausted data frames and started polling trailers, the buffered trailer will be
    // pushed into `buffered_trailing_headers`, immediately marking the `chunk_buffer` as `eos`.
    pub(super) fn buffer_next_chunk(
        inner: Pin<&mut Inner>,
        mut chunk_buffer: Pin<&mut ChunkBuf>,
        mut buffered_trailing_headers: Pin<&mut Option<http_1x::HeaderMap>>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<bool, aws_smithy_types::body::Error>>
    where
        Inner: http_body_1x::Body<Data = Bytes, Error = aws_smithy_types::body::Error>,
    {
        match inner.poll_frame(cx) {
            Poll::Ready(Some(Ok(frame))) => {
                if frame.is_data() {
                    let data = frame.into_data().expect("just checked to be data");
                    match chunk_buffer.as_mut().get_mut() {
                        ChunkBuf::Empty => {
                            let mut buf = SegmentedBuf::new();
                            buf.push(data);
                            *chunk_buffer.as_mut().get_mut() = ChunkBuf::Partial(buf);
                        }
                        ChunkBuf::Partial(buf) => buf.push(data),
                        ChunkBuf::EosPartial(_) | ChunkBuf::Terminated => {
                            panic!("cannot buffer more data after the stream has ended or been terminated; this is a bug")
                        }
                    }
                    Poll::Ready(Ok(true))
                } else {
                    let buf = chunk_buffer.as_mut().get_mut();
                    *buf = std::mem::replace(buf, ChunkBuf::Empty).ended();
                    *buffered_trailing_headers.as_mut().get_mut() = frame.into_trailers().ok();
                    Poll::Ready(Ok(false))
                }
            }
            Poll::Ready(Some(Err(e))) => {
                *chunk_buffer.as_mut().get_mut() = ChunkBuf::Terminated;
                Poll::Ready(Err(e))
            }
            Poll::Ready(None) => Poll::Ready(Ok(false)),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[derive(Debug)]
pub(super) enum AwsChunkedBodyError {
    /// Error that occurs when the sum of `trailer_lengths` set when creating an `AwsChunkedBody` is
    /// not equal to the actual length of the trailers returned by the inner `http_body::Body`
    /// implementor. These trailer lengths are necessary in order to correctly calculate the total
    /// size of the body for setting the content length header.
    ReportedTrailerLengthMismatch { actual: u64, expected: u64 },
    /// Error that occurs when the `stream_length` set when creating an `AwsChunkedBody` is not
    /// equal to the actual length of the body returned by the inner `http_body::Body` implementor.
    /// `stream_length` must be correct in order to set an accurate content length header.
    StreamLengthMismatch { actual: u64, expected: u64 },
    /// Error that occurs when signing a chunk fails.
    FailedToSign { source: SigningError },
}

impl std::fmt::Display for AwsChunkedBodyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReportedTrailerLengthMismatch { actual, expected } => {
                write!(f, "When creating this AwsChunkedBody, length of trailers was reported as {expected}. However, when double checking during trailer encoding, length was found to be {actual} instead.")
            }
            Self::StreamLengthMismatch { actual, expected } => {
                write!(f, "When creating this AwsChunkedBody, stream length was reported as {expected}. However, when double checking during body encoding, length was found to be {actual} instead.")
            }
            Self::FailedToSign { source } => {
                write!(f, "Signing error during aws-chunked encoding: {source}")
            }
        }
    }
}

impl std::error::Error for AwsChunkedBodyError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aws_chunked_body_is_unwind_safe_and_ref_unwind_safe() {
        fn assert_unwind_safe<T: std::panic::UnwindSafe>() {}
        fn assert_ref_unwind_safe<T: std::panic::RefUnwindSafe>() {}

        assert_unwind_safe::<AwsChunkedBody<()>>();
        assert_ref_unwind_safe::<AwsChunkedBody<()>>();
    }
}
