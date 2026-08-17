//! Asynchronous I/O abstractions for sockets.

#[cfg(feature = "tokio")]
mod tokio;

use std::{
    future::Future,
    io::{Error, ErrorKind},
    mem,
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::ready;

#[cfg(feature = "futures-io")]
mod compat;
#[cfg(feature = "futures-io")]
pub use compat::Compat;

/// A trait for asynchronous socket I/O.
///
/// Any type that implements tokio's `AsyncRead` and `AsyncWrite` traits
/// has implemented `AsyncSocket` trait.
///
/// Use `FuturesIoCompatExt` to wrap `futures-io` types as `AsyncSocket` types.
pub trait AsyncSocket {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<Result<usize, Error>>;

    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, Error>>;
}

pub(crate) trait AsyncSocketExt {
    fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadExact<'a, Self>
    where Self: Sized;

    fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> WriteAll<'a, Self>
    where Self: Sized;
}

impl<S: AsyncSocket> AsyncSocketExt for S {
    fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadExact<'a, Self>
    where Self: Sized {
        let capacity = buf.len();
        ReadExact {
            reader: self,
            buf,
            capacity,
        }
    }

    fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> WriteAll<'a, Self>
    where Self: Sized {
        WriteAll { writer: self, buf }
    }
}

pub(crate) struct ReadExact<'a, R> {
    reader: &'a mut R,
    buf: &'a mut [u8],
    capacity: usize,
}

impl<R: AsyncSocket + Unpin> Future for ReadExact<'_, R> {
    type Output = Result<usize, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        while !this.buf.is_empty() {
            let n = ready!(Pin::new(&mut *this.reader).poll_read(cx, this.buf))?;
            {
                let (_, rest) = mem::take(&mut this.buf).split_at_mut(n);
                this.buf = rest;
            }
            if n == 0 {
                return Poll::Ready(Err(ErrorKind::UnexpectedEof.into()));
            }
        }
        Poll::Ready(Ok(this.capacity))
    }
}

pub(crate) struct WriteAll<'a, W> {
    writer: &'a mut W,
    buf: &'a [u8],
}

impl<W: AsyncSocket + Unpin> Future for WriteAll<'_, W> {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        while !this.buf.is_empty() {
            let n = ready!(Pin::new(&mut *this.writer).poll_write(cx, this.buf))?;
            {
                let (_, rest) = mem::take(&mut this.buf).split_at(n);
                this.buf = rest;
            }
            if n == 0 {
                return Poll::Ready(Err(ErrorKind::WriteZero.into()));
            }
        }

        Poll::Ready(Ok(()))
    }
}
