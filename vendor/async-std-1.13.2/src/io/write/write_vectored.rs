use std::pin::Pin;
use std::future::Future;

use crate::io::{self, IoSlice, Write};
use crate::task::{Context, Poll};

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct WriteVectoredFuture<'a, T: Unpin + ?Sized> {
    pub(crate) writer: &'a mut T,
    pub(crate) bufs: &'a [IoSlice<'a>],
}

impl<T: Write + Unpin + ?Sized> Future for WriteVectoredFuture<'_, T> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let bufs = self.bufs;
        Pin::new(&mut *self.writer).poll_write_vectored(cx, bufs)
    }
}
