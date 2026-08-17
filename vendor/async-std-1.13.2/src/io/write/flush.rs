use std::pin::Pin;
use std::future::Future;

use crate::io::{self, Write};
use crate::task::{Context, Poll};

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct FlushFuture<'a, T: Unpin + ?Sized> {
    pub(crate) writer: &'a mut T,
}

impl<T: Write + Unpin + ?Sized> Future for FlushFuture<'_, T> {
    type Output = io::Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut *self.writer).poll_flush(cx)
    }
}
