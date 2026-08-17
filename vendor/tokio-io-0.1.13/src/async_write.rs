use bytes::Buf;
use futures::{Async, Poll};
use std::io as std_io;

use AsyncRead;

/// Writes bytes asynchronously.
///
/// The trait inherits from `std::io::Write` and indicates that an I/O object is
/// **nonblocking**. All non-blocking I/O objects must return an error when
/// bytes cannot be written instead of blocking the current thread.
///
/// Specifically, this means that the `poll_write` function will return one of
/// the following:
///
/// * `Ok(Async::Ready(n))` means that `n` bytes of data was immediately
///   written.
///
/// * `Ok(Async::NotReady)` means that no data was written from the buffer
///   provided. The I/O object is not currently writable but may become writable
///   in the future. Most importantly, **the current future's task is scheduled
///   to get unparked when the object is writable**. This means that like
///   `Future::poll` you'll receive a notification when the I/O object is
///   writable again.
///
/// * `Err(e)` for other errors are standard I/O errors coming from the
///   underlying object.
///
/// This trait importantly means that the `write` method only works in the
/// context of a future's task. The object may panic if used outside of a task.
///
/// Note that this trait also represents that the  `Write::flush` method works
/// very similarly to the `write` method, notably that `Ok(())` means that the
/// writer has successfully been flushed, a "would block" error means that the
/// current task is ready to receive a notification when flushing can make more
/// progress, and otherwise normal errors can happen as well.
pub trait AsyncWrite: std_io::Write {
    /// Attempt to write bytes from `buf` into the object.
    ///
    /// On success, returns `Ok(Async::Ready(num_bytes_written))`.
    ///
    /// If the object is not ready for writing, the method returns
    /// `Ok(Async::NotReady)` and arranges for the current task (via
    /// `cx.waker()`) to receive a notification when the object becomes
    /// readable or is closed.
    fn poll_write(&mut self, buf: &[u8]) -> Poll<usize, std_io::Error> {
        match self.write(buf) {
            Ok(t) => Ok(Async::Ready(t)),
            Err(ref e) if e.kind() == std_io::ErrorKind::WouldBlock => return Ok(Async::NotReady),
            Err(e) => return Err(e.into()),
        }
    }

    /// Attempt to flush the object, ensuring that any buffered data reach
    /// their destination.
    ///
    /// On success, returns `Ok(Async::Ready(()))`.
    ///
    /// If flushing cannot immediately complete, this method returns
    /// `Ok(Async::NotReady)` and arranges for the current task (via
    /// `cx.waker()`) to receive a notification when the object can make
    /// progress towards flushing.
    fn poll_flush(&mut self) -> Poll<(), std_io::Error> {
        match self.flush() {
            Ok(t) => Ok(Async::Ready(t)),
            Err(ref e) if e.kind() == std_io::ErrorKind::WouldBlock => return Ok(Async::NotReady),
            Err(e) => return Err(e.into()),
        }
    }

    /// Initiates or attempts to shut down this writer, returning success when
    /// the I/O connection has completely shut down.
    ///
    /// This method is intended to be used for asynchronous shutdown of I/O
    /// connections. For example this is suitable for implementing shutdown of a
    /// TLS connection or calling `TcpStream::shutdown` on a proxied connection.
    /// Protocols sometimes need to flush out final pieces of data or otherwise
    /// perform a graceful shutdown handshake, reading/writing more data as
    /// appropriate. This method is the hook for such protocols to implement the
    /// graceful shutdown logic.
    ///
    /// This `shutdown` method is required by implementers of the
    /// `AsyncWrite` trait. Wrappers typically just want to proxy this call
    /// through to the wrapped type, and base types will typically implement
    /// shutdown logic here or just return `Ok(().into())`. Note that if you're
    /// wrapping an underlying `AsyncWrite` a call to `shutdown` implies that
    /// transitively the entire stream has been shut down. After your wrapper's
    /// shutdown logic has been executed you should shut down the underlying
    /// stream.
    ///
    /// Invocation of a `shutdown` implies an invocation of `flush`. Once this
    /// method returns `Ready` it implies that a flush successfully happened
    /// before the shutdown happened. That is, callers don't need to call
    /// `flush` before calling `shutdown`. They can rely that by calling
    /// `shutdown` any pending buffered data will be written out.
    ///
    /// # Return value
    ///
    /// This function returns a `Poll<(), io::Error>` classified as such:
    ///
    /// * `Ok(Async::Ready(()))` - indicates that the connection was
    ///   successfully shut down and is now safe to deallocate/drop/close
    ///   resources associated with it. This method means that the current task
    ///   will no longer receive any notifications due to this method and the
    ///   I/O object itself is likely no longer usable.
    ///
    /// * `Ok(Async::NotReady)` - indicates that shutdown is initiated but could
    ///   not complete just yet. This may mean that more I/O needs to happen to
    ///   continue this shutdown operation. The current task is scheduled to
    ///   receive a notification when it's otherwise ready to continue the
    ///   shutdown operation. When woken up this method should be called again.
    ///
    /// * `Err(e)` - indicates a fatal error has happened with shutdown,
    ///   indicating that the shutdown operation did not complete successfully.
    ///   This typically means that the I/O object is no longer usable.
    ///
    /// # Errors
    ///
    /// This function can return normal I/O errors through `Err`, described
    /// above. Additionally this method may also render the underlying
    /// `Write::write` method no longer usable (e.g. will return errors in the
    /// future). It's recommended that once `shutdown` is called the
    /// `write` method is no longer called.
    ///
    /// # Panics
    ///
    /// This function will panic if not called within the context of a future's
    /// task.
    fn shutdown(&mut self) -> Poll<(), std_io::Error>;

    /// Write a `Buf` into this value, returning how many bytes were written.
    ///
    /// Note that this method will advance the `buf` provided automatically by
    /// the number of bytes written.
    fn write_buf<B: Buf>(&mut self, buf: &mut B) -> Poll<usize, std_io::Error>
    where
        Self: Sized,
    {
        if !buf.has_remaining() {
            return Ok(Async::Ready(0));
        }

        let n = try_ready!(self.poll_write(buf.bytes()));
        buf.advance(n);
        Ok(Async::Ready(n))
    }
}

impl<T: ?Sized + AsyncWrite> AsyncWrite for Box<T> {
    fn shutdown(&mut self) -> Poll<(), std_io::Error> {
        (**self).shutdown()
    }
}
impl<'a, T: ?Sized + AsyncWrite> AsyncWrite for &'a mut T {
    fn shutdown(&mut self) -> Poll<(), std_io::Error> {
        (**self).shutdown()
    }
}

impl AsyncRead for std_io::Repeat {
    unsafe fn prepare_uninitialized_buffer(&self, _: &mut [u8]) -> bool {
        false
    }
}

impl AsyncWrite for std_io::Sink {
    fn shutdown(&mut self) -> Poll<(), std_io::Error> {
        Ok(().into())
    }
}

impl<T: AsyncRead> AsyncRead for std_io::Take<T> {
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        self.get_ref().prepare_uninitialized_buffer(buf)
    }
}

impl<T, U> AsyncRead for std_io::Chain<T, U>
where
    T: AsyncRead,
    U: AsyncRead,
{
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        let (t, u) = self.get_ref();
        // We don't need to execute the second initializer if the first one
        // already zeroed the buffer out.
        t.prepare_uninitialized_buffer(buf) || u.prepare_uninitialized_buffer(buf)
    }
}

impl<T: AsyncWrite> AsyncWrite for std_io::BufWriter<T> {
    fn shutdown(&mut self) -> Poll<(), std_io::Error> {
        try_ready!(self.poll_flush());
        self.get_mut().shutdown()
    }
}

impl<T: AsyncRead> AsyncRead for std_io::BufReader<T> {
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        self.get_ref().prepare_uninitialized_buffer(buf)
    }
}

impl<T: AsRef<[u8]>> AsyncRead for std_io::Cursor<T> {}

impl<'a> AsyncWrite for std_io::Cursor<&'a mut [u8]> {
    fn shutdown(&mut self) -> Poll<(), std_io::Error> {
        Ok(().into())
    }
}

impl AsyncWrite for std_io::Cursor<Vec<u8>> {
    fn shutdown(&mut self) -> Poll<(), std_io::Error> {
        Ok(().into())
    }
}

impl AsyncWrite for std_io::Cursor<Box<[u8]>> {
    fn shutdown(&mut self) -> Poll<(), std_io::Error> {
        Ok(().into())
    }
}
