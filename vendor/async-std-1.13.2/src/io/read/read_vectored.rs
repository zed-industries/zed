use std::pin::Pin;
use std::future::Future;

use crate::io::{self, IoSliceMut, Read};
use crate::task::{Context, Poll};

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct ReadVectoredFuture<'a, T: Unpin + ?Sized> {
    pub(crate) reader: &'a mut T,
    pub(crate) bufs: &'a mut [IoSliceMut<'a>],
}

impl<T: Read + Unpin + ?Sized> Future for ReadVectoredFuture<'_, T> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self { reader, bufs } = &mut *self;
        Pin::new(reader).poll_read_vectored(cx, bufs)
    }
}
