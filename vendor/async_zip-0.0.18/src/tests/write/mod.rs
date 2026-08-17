// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use futures_lite::io::AsyncWrite;
use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};

pub(crate) mod offset;
mod zip64;

/// /dev/null for AsyncWrite.
/// Useful for tests that involve writing, but not reading, large amounts of data.
pub(crate) struct AsyncSink;

// AsyncSink is always ready to receive bytes and throw them away.
impl AsyncWrite for AsyncSink {
    fn poll_write(self: Pin<&mut Self>, _: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, Error>> {
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }
}
