use bytes::BufMut;
use futures::{Async, Poll};
use std::io as std_io;

#[allow(deprecated)]
use codec::{Decoder, Encoder, Framed};
use split::{ReadHalf, WriteHalf};
use {framed, split, AsyncWrite};

/// Read bytes asynchronously.
///
/// This trait inherits from `std::io::Read` and indicates that an I/O object is
/// **non-blocking**. All non-blocking I/O objects must return an error when
/// bytes are unavailable instead of blocking the current thread.
///
/// Specifically, this means that the `poll_read` function will return one of
/// the following:
///
/// * `Ok(Async::Ready(n))` means that `n` bytes of data was immediately read
///   and placed into the output buffer, where `n` == 0 implies that EOF has
///   been reached.
///
/// * `Ok(Async::NotReady)` means that no data was read into the buffer
///   provided. The I/O object is not currently readable but may become readable
///   in the future. Most importantly, **the current future's task is scheduled
///   to get unparked when the object is readable**. This means that like
///   `Future::poll` you'll receive a notification when the I/O object is
///   readable again.
///
/// * `Err(e)` for other errors are standard I/O errors coming from the
///   underlying object.
///
/// This trait importantly means that the `read` method only works in the
/// context of a future's task. The object may panic if used outside of a task.
pub trait AsyncRead: std_io::Read {
    /// Prepares an uninitialized buffer to be safe to pass to `read`. Returns
    /// `true` if the supplied buffer was zeroed out.
    ///
    /// While it would be highly unusual, implementations of [`io::Read`] are
    /// able to read data from the buffer passed as an argument. Because of
    /// this, the buffer passed to [`io::Read`] must be initialized memory. In
    /// situations where large numbers of buffers are used, constantly having to
    /// zero out buffers can be expensive.
    ///
    /// This function does any necessary work to prepare an uninitialized buffer
    /// to be safe to pass to `read`. If `read` guarantees to never attempt to
    /// read data out of the supplied buffer, then `prepare_uninitialized_buffer`
    /// doesn't need to do any work.
    ///
    /// If this function returns `true`, then the memory has been zeroed out.
    /// This allows implementations of `AsyncRead` which are composed of
    /// multiple subimplementations to efficiently implement
    /// `prepare_uninitialized_buffer`.
    ///
    /// This function isn't actually `unsafe` to call but `unsafe` to implement.
    /// The implementer must ensure that either the whole `buf` has been zeroed
    /// or `read_buf()` overwrites the buffer without reading it and returns
    /// correct value.
    ///
    /// This function is called from [`read_buf`].
    ///
    /// [`io::Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
    /// [`read_buf`]: #method.read_buf
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        for i in 0..buf.len() {
            buf[i] = 0;
        }

        true
    }

    /// Attempt to read from the `AsyncRead` into `buf`.
    ///
    /// On success, returns `Ok(Async::Ready(num_bytes_read))`.
    ///
    /// If no data is available for reading, the method returns
    /// `Ok(Async::NotReady)` and arranges for the current task (via
    /// `cx.waker()`) to receive a notification when the object becomes
    /// readable or is closed.
    fn poll_read(&mut self, buf: &mut [u8]) -> Poll<usize, std_io::Error> {
        match self.read(buf) {
            Ok(t) => Ok(Async::Ready(t)),
            Err(ref e) if e.kind() == std_io::ErrorKind::WouldBlock => return Ok(Async::NotReady),
            Err(e) => return Err(e.into()),
        }
    }

    /// Pull some bytes from this source into the specified `BufMut`, returning
    /// how many bytes were read.
    ///
    /// The `buf` provided will have bytes read into it and the internal cursor
    /// will be advanced if any bytes were read. Note that this method typically
    /// will not reallocate the buffer provided.
    fn read_buf<B: BufMut>(&mut self, buf: &mut B) -> Poll<usize, std_io::Error>
    where
        Self: Sized,
    {
        if !buf.has_remaining_mut() {
            return Ok(Async::Ready(0));
        }

        unsafe {
            let n = {
                let b = buf.bytes_mut();

                self.prepare_uninitialized_buffer(b);

                try_ready!(self.poll_read(b))
            };

            buf.advance_mut(n);
            Ok(Async::Ready(n))
        }
    }

    /// Provides a `Stream` and `Sink` interface for reading and writing to this
    /// I/O object, using `Decode` and `Encode` to read and write the raw data.
    ///
    /// Raw I/O objects work with byte sequences, but higher-level code usually
    /// wants to batch these into meaningful chunks, called "frames". This
    /// method layers framing on top of an I/O object, by using the `Codec`
    /// traits to handle encoding and decoding of messages frames. Note that
    /// the incoming and outgoing frame types may be distinct.
    ///
    /// This function returns a *single* object that is both `Stream` and
    /// `Sink`; grouping this into a single object is often useful for layering
    /// things like gzip or TLS, which require both read and write access to the
    /// underlying object.
    ///
    /// If you want to work more directly with the streams and sink, consider
    /// calling `split` on the `Framed` returned by this method, which will
    /// break them into separate objects, allowing them to interact more easily.
    #[deprecated(since = "0.1.7", note = "Use tokio_codec::Decoder::framed instead")]
    #[allow(deprecated)]
    fn framed<T: Encoder + Decoder>(self, codec: T) -> Framed<Self, T>
    where
        Self: AsyncWrite + Sized,
    {
        framed::framed(self, codec)
    }

    /// Helper method for splitting this read/write object into two halves.
    ///
    /// The two halves returned implement the `Read` and `Write` traits,
    /// respectively.
    ///
    /// To restore this read/write object from its `ReadHalf` and `WriteHalf`
    /// use `unsplit`.
    fn split(self) -> (ReadHalf<Self>, WriteHalf<Self>)
    where
        Self: AsyncWrite + Sized,
    {
        split::split(self)
    }
}

impl<T: ?Sized + AsyncRead> AsyncRead for Box<T> {
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        (**self).prepare_uninitialized_buffer(buf)
    }
}

impl<'a, T: ?Sized + AsyncRead> AsyncRead for &'a mut T {
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        (**self).prepare_uninitialized_buffer(buf)
    }
}

impl<'a> AsyncRead for &'a [u8] {
    unsafe fn prepare_uninitialized_buffer(&self, _buf: &mut [u8]) -> bool {
        false
    }
}
