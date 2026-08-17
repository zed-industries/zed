use std::fmt;
use std::pin::Pin;

use pin_project_lite::pin_project;

use crate::io::write::WriteExt;
use crate::io::{self, Seek, SeekFrom, Write, DEFAULT_BUF_SIZE};
use crate::task::{Context, Poll, ready};

pin_project! {
    /// Wraps a writer and buffers its output.
    ///
    /// It can be excessively inefficient to work directly with something that
    /// implements [`Write`]. For example, every call to
    /// [`write`][`TcpStream::write`] on [`TcpStream`] results in a system call. A
    /// `BufWriter` keeps an in-memory buffer of data and writes it to an underlying
    /// writer in large, infrequent batches.
    ///
    /// `BufWriter` can improve the speed of programs that make *small* and
    /// *repeated* write calls to the same file or network socket. It does not
    /// help when writing very large amounts at once, or writing just one or a few
    /// times. It also provides no advantage when writing to a destination that is
    /// in memory, like a `Vec<u8>`.
    ///
    /// Unlike the `BufWriter` type in `std`, this type does not write out the
    /// contents of its buffer when it is dropped. Therefore, it is absolutely
    /// critical that users explicitly flush the buffer before dropping a
    /// `BufWriter`.
    ///
    /// This type is an async version of [`std::io::BufWriter`].
    ///
    /// [`std::io::BufWriter`]: https://doc.rust-lang.org/std/io/struct.BufWriter.html
    ///
    /// # Examples
    ///
    /// Let's write the numbers one through ten to a [`TcpStream`]:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// use async_std::net::TcpStream;
    /// use async_std::prelude::*;
    ///
    /// let mut stream = TcpStream::connect("127.0.0.1:34254").await?;
    ///
    /// for i in 0..10 {
    ///     let arr = [i+1];
    ///     stream.write(&arr).await?;
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    ///
    /// Because we're not buffering, we write each one in turn, incurring the
    /// overhead of a system call per byte written. We can fix this with a
    /// `BufWriter`:
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// use async_std::io::BufWriter;
    /// use async_std::net::TcpStream;
    /// use async_std::prelude::*;
    ///
    /// let mut stream = BufWriter::new(TcpStream::connect("127.0.0.1:34254").await?);
    ///
    /// for i in 0..10 {
    ///     let arr = [i+1];
    ///     stream.write(&arr).await?;
    /// };
    ///
    /// stream.flush().await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    ///
    /// By wrapping the stream with a `BufWriter`, these ten writes are all grouped
    /// together by the buffer, and will all be written out in one system call when
    /// `stream.flush()` completes. (As mentioned above, dropping a `BufWriter`
    /// does not flush its buffers, so a `flush` call is essential.)
    ///
    /// [`Write`]: trait.Write.html
    /// [`TcpStream::write`]: ../net/struct.TcpStream.html#method.write
    /// [`TcpStream`]: ../net/struct.TcpStream.html
    /// [`flush`]: trait.Write.html#tymethod.flush
    pub struct BufWriter<W> {
        #[pin]
        inner: W,
        buf: Vec<u8>,
        written: usize,
    }
}

/// An error returned by `into_inner` which combines an error that
/// happened while writing out the buffer, and the buffered writer object
/// which may be used to recover from the condition.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// use async_std::io::BufWriter;
/// use async_std::net::TcpStream;
///
/// let buf_writer = BufWriter::new(TcpStream::connect("127.0.0.1:34251").await?);
///
/// // unwrap the TcpStream and flush the buffer
/// let stream = match buf_writer.into_inner().await {
///     Ok(s) => s,
///     Err(e) => {
///         // Here, e is an IntoInnerError
///         panic!("An error occurred");
///     }
/// };
/// #
/// # Ok(()) }) }
///```
#[derive(Debug)]
pub struct IntoInnerError<W>(W, #[allow(dead_code)] crate::io::Error);

impl<W: Write> BufWriter<W> {
    /// Creates a new `BufWriter` with a default buffer capacity. The default is currently 8 KB,
    /// but may change in the future.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #![allow(unused_mut)]
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// use async_std::io::BufWriter;
    /// use async_std::net::TcpStream;
    ///
    /// let mut buffer = BufWriter::new(TcpStream::connect("127.0.0.1:34254").await?);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn new(inner: W) -> BufWriter<W> {
        BufWriter::with_capacity(DEFAULT_BUF_SIZE, inner)
    }

    /// Creates a new `BufWriter` with the specified buffer capacity.
    ///
    /// # Examples
    ///
    /// Creating a buffer with a buffer of a hundred bytes.
    ///
    /// ```no_run
    /// # #![allow(unused_mut)]
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// use async_std::io::BufWriter;
    /// use async_std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:34254").await?;
    /// let mut buffer = BufWriter::with_capacity(100, stream);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn with_capacity(capacity: usize, inner: W) -> BufWriter<W> {
        BufWriter {
            inner,
            buf: Vec::with_capacity(capacity),
            written: 0,
        }
    }

    /// Gets a reference to the underlying writer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #![allow(unused_mut)]
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// use async_std::io::BufWriter;
    /// use async_std::net::TcpStream;
    ///
    /// let mut buffer = BufWriter::new(TcpStream::connect("127.0.0.1:34254").await?);
    ///
    /// // We can use reference just like buffer
    /// let reference = buffer.get_ref();
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Gets a mutable reference to the underlying writer.
    ///
    /// It is inadvisable to directly write to the underlying writer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// use async_std::io::BufWriter;
    /// use async_std::net::TcpStream;
    ///
    /// let mut buffer = BufWriter::new(TcpStream::connect("127.0.0.1:34254").await?);
    ///
    /// // We can use reference just like buffer
    /// let reference = buffer.get_mut();
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    /// Gets a pinned mutable reference to the underlying writer.
    ///
    /// It is inadvisable to directly write to the underlying writer.
    fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut W> {
        self.project().inner
    }

    /// Consumes BufWriter, returning the underlying writer
    ///
    /// This method will not write leftover data, it will be lost.
    /// For method that will attempt to write before returning the writer see [`poll_into_inner`]
    ///
    /// [`poll_into_inner`]: #method.poll_into_inner
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// use async_std::io::BufWriter;
    /// use async_std::net::TcpStream;
    ///
    /// let buf_writer = BufWriter::new(TcpStream::connect("127.0.0.1:34251").await?);
    ///
    /// // unwrap the TcpStream and flush the buffer
    /// let stream = buf_writer.into_inner().await.unwrap();
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn into_inner(mut self) -> Result<W, IntoInnerError<BufWriter<W>>>
    where
        Self: Unpin,
    {
        match self.flush().await {
            Err(e) => Err(IntoInnerError(self, e)),
            Ok(()) => Ok(self.inner),
        }
    }

    /// Returns a reference to the internally buffered data.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// use async_std::io::BufWriter;
    /// use async_std::net::TcpStream;
    ///
    /// let buf_writer = BufWriter::new(TcpStream::connect("127.0.0.1:34251").await?);
    ///
    /// // See how many bytes are currently buffered
    /// let bytes_buffered = buf_writer.buffer().len();
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn buffer(&self) -> &[u8] {
        &self.buf
    }

    /// Poll buffer flushing until completion
    ///
    /// This is used in types that wrap around BufWrite, one such example: [`LineWriter`]
    ///
    /// [`LineWriter`]: struct.LineWriter.html
    fn poll_flush_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut this = self.project();
        let len = this.buf.len();
        let mut ret = Ok(());
        while *this.written < len {
            match this
                .inner
                .as_mut()
                .poll_write(cx, &this.buf[*this.written..])
            {
                Poll::Ready(Ok(0)) => {
                    ret = Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "Failed to write buffered data",
                    ));
                    break;
                }
                Poll::Ready(Ok(n)) => *this.written += n,
                Poll::Ready(Err(ref e)) if e.kind() == io::ErrorKind::Interrupted => {}
                Poll::Ready(Err(e)) => {
                    ret = Err(e);
                    break;
                }
                Poll::Pending => return Poll::Pending,
            }
        }
        if *this.written > 0 {
            this.buf.drain(..*this.written);
        }
        *this.written = 0;
        Poll::Ready(ret)
    }
}

impl<W: Write> Write for BufWriter<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        if self.buf.len() + buf.len() > self.buf.capacity() {
            ready!(self.as_mut().poll_flush_buf(cx))?;
        }
        if buf.len() >= self.buf.capacity() {
            self.get_pin_mut().poll_write(cx, buf)
        } else {
            Pin::new(&mut *self.project().buf).poll_write(cx, buf)
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        ready!(self.as_mut().poll_flush_buf(cx))?;
        self.get_pin_mut().poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        ready!(self.as_mut().poll_flush_buf(cx))?;
        self.get_pin_mut().poll_close(cx)
    }
}

impl<W: Write + fmt::Debug> fmt::Debug for BufWriter<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufWriter")
            .field("writer", &self.inner)
            .field("buf", &self.buf)
            .finish()
    }
}

impl<W: Write + Seek> Seek for BufWriter<W> {
    /// Seek to the offset, in bytes, in the underlying writer.
    ///
    /// Seeking always writes out the internal buffer before seeking.
    fn poll_seek(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: SeekFrom,
    ) -> Poll<io::Result<u64>> {
        ready!(self.as_mut().poll_flush_buf(cx))?;
        self.get_pin_mut().poll_seek(cx, pos)
    }
}
