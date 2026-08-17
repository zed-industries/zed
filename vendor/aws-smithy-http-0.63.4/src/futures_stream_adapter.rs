/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_types::body::SdkBody;
use aws_smithy_types::byte_stream::error::Error as ByteStreamError;
use aws_smithy_types::byte_stream::ByteStream;
use bytes::Bytes;
use futures_core::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A new-type wrapper to enable the impl of the `futures_core::stream::Stream` trait
///
/// [`ByteStream`] no longer implements `futures_core::stream::Stream` so we wrap it in the
/// new-type to enable the trait when it is required.
///
/// This is meant to be used by codegen code, and users should not need to use it directly.
#[derive(Debug)]
pub struct FuturesStreamCompatByteStream(ByteStream);

impl FuturesStreamCompatByteStream {
    /// Creates a new `FuturesStreamCompatByteStream` by wrapping `stream`.
    pub fn new(stream: ByteStream) -> Self {
        Self(stream)
    }

    /// Returns [`SdkBody`] of the wrapped [`ByteStream`].
    pub fn into_inner(self) -> SdkBody {
        self.0.into_inner()
    }
}

impl Stream for FuturesStreamCompatByteStream {
    type Item = Result<Bytes, ByteStreamError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_core::stream::Stream;

    fn check_compatible_with_hyper_wrap_stream<S, O, E>(stream: S) -> S
    where
        S: Stream<Item = Result<O, E>> + Send + 'static,
        O: Into<Bytes> + 'static,
        E: Into<Box<dyn std::error::Error + Send + Sync + 'static>> + 'static,
    {
        stream
    }

    #[test]
    fn test_byte_stream_stream_can_be_made_compatible_with_hyper_wrap_stream() {
        let stream = ByteStream::from_static(b"Hello world");
        check_compatible_with_hyper_wrap_stream(FuturesStreamCompatByteStream::new(stream));
    }
}
