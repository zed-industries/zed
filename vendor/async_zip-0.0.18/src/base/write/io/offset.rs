// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use std::io::{Error, IoSlice};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_lite::io::AsyncWrite;
use pin_project::pin_project;

/// A wrapper around an [`AsyncWrite`] implementation which tracks the current byte offset.
#[pin_project(project = OffsetWriterProj)]
pub struct AsyncOffsetWriter<W> {
    #[pin]
    inner: W,
    offset: u64,
}

impl<W> AsyncOffsetWriter<W>
where
    W: AsyncWrite + Unpin,
{
    /// Constructs a new wrapper from an inner [`AsyncWrite`] writer.
    pub fn new(inner: W) -> Self {
        Self { inner, offset: 0 }
    }

    /// Returns the current byte offset.
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// Consumes this wrapper and returns the inner [`AsyncWrite`] writer.
    pub fn into_inner(self) -> W {
        self.inner
    }

    pub fn inner_mut(&mut self) -> &mut W {
        &mut self.inner
    }
}

impl<W> AsyncWrite for AsyncOffsetWriter<W>
where
    W: AsyncWrite + Unpin,
{
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<Result<usize, Error>> {
        let this = self.project();
        let poll = this.inner.poll_write(cx, buf);

        if let Poll::Ready(Ok(inner)) = &poll {
            *this.offset += *inner as u64;
        }

        poll
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Error>> {
        self.project().inner.poll_close(cx)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize, Error>> {
        self.project().inner.poll_write_vectored(cx, bufs)
    }
}
