use std::io;

use futures::{Async, Future, Poll};

use AsyncWrite;

/// A future used to fully flush an I/O object.
///
/// Resolves to the underlying I/O object once the flush operation is complete.
///
/// Created by the [`flush`] function.
///
/// [`flush`]: fn.flush.html
#[derive(Debug)]
pub struct Flush<A> {
    a: Option<A>,
}

/// Creates a future which will entirely flush an I/O object and then yield the
/// object itself.
///
/// This function will consume the object provided if an error happens, and
/// otherwise it will repeatedly call `flush` until it sees `Ok(())`, scheduling
/// a retry if `WouldBlock` is seen along the way.
pub fn flush<A>(a: A) -> Flush<A>
where
    A: AsyncWrite,
{
    Flush { a: Some(a) }
}

impl<A> Future for Flush<A>
where
    A: AsyncWrite,
{
    type Item = A;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<A, io::Error> {
        try_ready!(self.a.as_mut().unwrap().poll_flush());
        Ok(Async::Ready(self.a.take().unwrap()))
    }
}
