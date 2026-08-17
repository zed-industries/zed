use std::mem;
use std::pin::Pin;
use std::str;
use std::future::Future;

use super::read_to_end_internal;
use crate::io::{self, Read};
use crate::task::{Context, Poll};

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct ReadToStringFuture<'a, T: Unpin + ?Sized> {
    pub(crate) reader: &'a mut T,
    pub(crate) buf: &'a mut String,
    pub(crate) bytes: Vec<u8>,
    pub(crate) start_len: usize,
}

impl<T: Read + Unpin + ?Sized> Future for ReadToStringFuture<'_, T> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self {
            reader,
            buf,
            bytes,
            start_len,
        } = &mut *self;
        let reader = Pin::new(reader);

        let ret = futures_core::ready!(read_to_end_internal(reader, cx, bytes, *start_len));
        if str::from_utf8(&bytes).is_err() {
            Poll::Ready(ret.and_then(|_| {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "stream did not contain valid UTF-8",
                ))
            }))
        } else {
            #[allow(clippy::debug_assert_with_mut_call)]
            {
                debug_assert!(buf.is_empty());
            }

            // Safety: `bytes` is a valid UTF-8 because `str::from_utf8` returned `Ok`.
            mem::swap(unsafe { buf.as_mut_vec() }, bytes);
            Poll::Ready(ret)
        }
    }
}
