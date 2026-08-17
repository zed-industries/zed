use std::pin::Pin;
use std::future::Future;

use super::read_until_internal;
use crate::io::{self, BufRead};
use crate::task::{Context, Poll};

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct ReadUntilFuture<'a, T: Unpin + ?Sized> {
    pub(crate) reader: &'a mut T,
    pub(crate) byte: u8,
    pub(crate) buf: &'a mut Vec<u8>,
    pub(crate) read: usize,
}

impl<T: BufRead + Unpin + ?Sized> Future for ReadUntilFuture<'_, T> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self {
            reader,
            byte,
            buf,
            read,
        } = &mut *self;
        read_until_internal(Pin::new(reader), cx, *byte, buf, read)
    }
}
