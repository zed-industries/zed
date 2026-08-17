// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use std::pin::Pin;
use std::task::{Context, Poll};

use futures_lite::io::{AsyncBufRead, AsyncRead};
use pin_project::pin_project;

/// A wrapping reader which holds an owned R or a mutable borrow to R.
///
/// This is used to represent whether the supplied reader can be acted on concurrently or not (with an owned value
/// suggesting that R implements some method of synchronisation & cloning).
#[pin_project(project = OwnedReaderProj)]
pub(crate) enum OwnedReader<'a, R> {
    Owned(#[pin] R),
    Borrow(#[pin] &'a mut R),
}

impl<'a, R> OwnedReader<'a, R>
where
    R: AsyncBufRead + Unpin,
{
    /// Consumes an owned reader and returns the inner value.
    pub(crate) fn owned_into_inner(self) -> R {
        match self {
            OwnedReader::Owned(inner) => inner,
            OwnedReader::Borrow(_) => panic!("not OwnedReader::Owned value"),
        }
    }
}

impl<'a, R> AsyncBufRead for OwnedReader<'a, R>
where
    R: AsyncBufRead + Unpin,
{
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<&[u8]>> {
        match self.project() {
            OwnedReaderProj::Owned(inner) => inner.poll_fill_buf(cx),
            OwnedReaderProj::Borrow(inner) => inner.poll_fill_buf(cx),
        }
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        match self.project() {
            OwnedReaderProj::Owned(inner) => inner.consume(amt),
            OwnedReaderProj::Borrow(inner) => inner.consume(amt),
        }
    }
}

impl<'a, R> AsyncRead for OwnedReader<'a, R>
where
    R: AsyncBufRead + Unpin,
{
    fn poll_read(self: Pin<&mut Self>, c: &mut Context<'_>, b: &mut [u8]) -> Poll<std::io::Result<usize>> {
        match self.project() {
            OwnedReaderProj::Owned(inner) => inner.poll_read(c, b),
            OwnedReaderProj::Borrow(inner) => inner.poll_read(c, b),
        }
    }
}
