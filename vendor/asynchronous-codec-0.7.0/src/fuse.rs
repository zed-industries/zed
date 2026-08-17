use futures_util::io::{AsyncRead, AsyncWrite};
use pin_project_lite::pin_project;
use std::io::Error;
use std::marker::Unpin;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    #[derive(Debug)]
    pub(crate) struct Fuse<T, U> {
        #[pin]
        pub t: T,
        pub u: U,
    }
}

impl<T, U> Fuse<T, U> {
    pub(crate) fn new(t: T, u: U) -> Self {
        Self { t, u }
    }
}

impl<T, U> Deref for Fuse<T, U> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.t
    }
}

impl<T, U> DerefMut for Fuse<T, U> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.t
    }
}

impl<T: AsyncRead + Unpin, U> AsyncRead for Fuse<T, U> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        self.project().t.poll_read(cx, buf)
    }
}

impl<T: AsyncWrite + Unpin, U> AsyncWrite for Fuse<T, U> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        self.project().t.poll_write(cx, buf)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Error>> {
        self.project().t.poll_flush(cx)
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Error>> {
        self.project().t.poll_close(cx)
    }
}
