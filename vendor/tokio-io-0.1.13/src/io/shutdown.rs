use std::io;

use futures::{Async, Future, Poll};

use AsyncWrite;

/// A future used to fully shutdown an I/O object.
///
/// Resolves to the underlying I/O object once the shutdown operation is
/// complete.
///
/// Created by the [`shutdown`] function.
///
/// [`shutdown`]: fn.shutdown.html
#[derive(Debug)]
pub struct Shutdown<A> {
    a: Option<A>,
}

/// Creates a future which will entirely shutdown an I/O object and then yield
/// the object itself.
///
/// This function will consume the object provided if an error happens, and
/// otherwise it will repeatedly call `shutdown` until it sees `Ok(())`,
/// scheduling a retry if `WouldBlock` is seen along the way.
pub fn shutdown<A>(a: A) -> Shutdown<A>
where
    A: AsyncWrite,
{
    Shutdown { a: Some(a) }
}

impl<A> Future for Shutdown<A>
where
    A: AsyncWrite,
{
    type Item = A;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<A, io::Error> {
        try_ready!(self.a.as_mut().unwrap().shutdown());
        Ok(Async::Ready(self.a.take().unwrap()))
    }
}
