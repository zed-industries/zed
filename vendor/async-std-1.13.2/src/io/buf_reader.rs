use std::io::{IoSliceMut, Read as _};
use std::pin::Pin;
use std::{cmp, fmt};

use pin_project_lite::pin_project;

use crate::io::{self, BufRead, Read, Seek, SeekFrom, DEFAULT_BUF_SIZE};
use crate::task::{Context, Poll};

pin_project! {
    /// Adds buffering to any reader.
    ///
    /// It can be excessively inefficient to work directly with a [`Read`] instance. A `BufReader`
    /// performs large, infrequent reads on the underlying [`Read`] and maintains an in-memory buffer
    /// of the incoming byte stream.
    ///
    /// `BufReader` can improve the speed of programs that make *small* and *repeated* read calls to
    /// the same file or network socket. It does not help when reading very large amounts at once, or
    /// reading just one or a few times. It also provides no advantage when reading from a source that
    /// is already in memory, like a `Vec<u8>`.
    ///
    /// When the `BufReader` is dropped, the contents of its buffer will be discarded. Creating
    /// multiple instances of a `BufReader` on the same stream can cause data loss.
    ///
    /// This type is an async version of [`std::io::BufReader`].
    ///
    /// [`Read`]: trait.Read.html
    /// [`std::io::BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    /// use async_std::io::BufReader;
    /// use async_std::prelude::*;
    ///
    /// let mut file = BufReader::new(File::open("a.txt").await?);
    ///
    /// let mut line = String::new();
    /// file.read_line(&mut line).await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub struct BufReader<R> {
        #[pin]
        inner: R,
        buf: Box<[u8]>,
        pos: usize,
        cap: usize,
    }
}

impl<R: io::Read> BufReader<R> {
    /// Creates a buffered reader with default buffer capacity.
    ///
    /// The default capacity is currently 8 KB, but may change in the future.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    /// use async_std::io::BufReader;
    ///
    /// let f = BufReader::new(File::open("a.txt").await?);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn new(inner: R) -> BufReader<R> {
        BufReader::with_capacity(DEFAULT_BUF_SIZE, inner)
    }

    /// Creates a new buffered reader with the specified capacity.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    /// use async_std::io::BufReader;
    ///
    /// let f = BufReader::with_capacity(1024, File::open("a.txt").await?);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn with_capacity(capacity: usize, inner: R) -> BufReader<R> {
        BufReader {
            inner,
            buf: vec![0; capacity].into_boxed_slice(),
            pos: 0,
            cap: 0,
        }
    }
}

impl<R> BufReader<R> {
    /// Gets a reference to the underlying reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    /// use async_std::io::BufReader;
    ///
    /// let f = BufReader::new(File::open("a.txt").await?);
    /// let inner = f.get_ref();
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    /// use async_std::io::BufReader;
    ///
    /// let mut file = BufReader::new(File::open("a.txt").await?);
    /// let inner = file.get_mut();
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Gets a pinned mutable reference to the underlying reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut R> {
        self.project().inner
    }

    /// Returns a reference to the internal buffer.
    ///
    /// This function will not attempt to fill the buffer if it is empty.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    /// use async_std::io::BufReader;
    ///
    /// let f = BufReader::new(File::open("a.txt").await?);
    /// let buffer = f.buffer();
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn buffer(&self) -> &[u8] {
        &self.buf[self.pos..self.cap]
    }

    /// Unwraps the buffered reader, returning the underlying reader.
    ///
    /// Note that any leftover data in the internal buffer is lost.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs::File;
    /// use async_std::io::BufReader;
    ///
    /// let f = BufReader::new(File::open("a.txt").await?);
    /// let inner = f.into_inner();
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Invalidates all data in the internal buffer.
    #[inline]
    fn discard_buffer(self: Pin<&mut Self>) {
        let this = self.project();
        *this.pos = 0;
        *this.cap = 0;
    }
}

impl<R: Read> Read for BufReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        // If we don't have any buffered data and we're doing a massive read
        // (larger than our internal buffer), bypass our internal buffer
        // entirely.
        if self.pos == self.cap && buf.len() >= self.buf.len() {
            let res = futures_core::ready!(self.as_mut().get_pin_mut().poll_read(cx, buf));
            self.discard_buffer();
            return Poll::Ready(res);
        }
        let mut rem = futures_core::ready!(self.as_mut().poll_fill_buf(cx))?;
        let nread = rem.read(buf)?;
        self.consume(nread);
        Poll::Ready(Ok(nread))
    }

    fn poll_read_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [IoSliceMut<'_>],
    ) -> Poll<io::Result<usize>> {
        let total_len = bufs.iter().map(|b| b.len()).sum::<usize>();
        if self.pos == self.cap && total_len >= self.buf.len() {
            let res =
                futures_core::ready!(self.as_mut().get_pin_mut().poll_read_vectored(cx, bufs));
            self.discard_buffer();
            return Poll::Ready(res);
        }
        let mut rem = futures_core::ready!(self.as_mut().poll_fill_buf(cx))?;
        let nread = rem.read_vectored(bufs)?;
        self.consume(nread);
        Poll::Ready(Ok(nread))
    }
}

impl<R: Read> BufRead for BufReader<R> {
    fn poll_fill_buf<'a>(
        self: Pin<&'a mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<&'a [u8]>> {
        let mut this = self.project();

        // If we've reached the end of our internal buffer then we need to fetch
        // some more data from the underlying reader.
        // Branch using `>=` instead of the more correct `==`
        // to tell the compiler that the pos..cap slice is always valid.
        if *this.pos >= *this.cap {
            debug_assert!(*this.pos == *this.cap);
            *this.cap = futures_core::ready!(this.inner.as_mut().poll_read(cx, this.buf))?;
            *this.pos = 0;
        }
        Poll::Ready(Ok(&this.buf[*this.pos..*this.cap]))
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        let this = self.project();
        *this.pos = cmp::min(*this.pos + amt, *this.cap);
    }
}

impl<R: io::Read + fmt::Debug> fmt::Debug for BufReader<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufReader")
            .field("reader", &self.inner)
            .field(
                "buffer",
                &format_args!("{}/{}", self.cap - self.pos, self.buf.len()),
            )
            .finish()
    }
}

impl<R: Seek> Seek for BufReader<R> {
    /// Seeks to an offset, in bytes, in the underlying reader.
    ///
    /// The position used for seeking with `SeekFrom::Current(_)` is the position the underlying
    /// reader would be at if the `BufReader` had no internal buffer.
    ///
    /// Seeking always discards the internal buffer, even if the seek position would otherwise fall
    /// within it. This guarantees that calling `.into_inner()` immediately after a seek yields the
    /// underlying reader at the same position.
    ///
    /// See [`Seek`] for more details.
    ///
    /// Note: In the edge case where you're seeking with `SeekFrom::Current(n)` where `n` minus the
    /// internal buffer length overflows an `i64`, two seeks will be performed instead of one. If
    /// the second seek returns `Err`, the underlying reader will be left at the same position it
    /// would have if you called `seek` with `SeekFrom::Current(0)`.
    ///
    /// [`Seek`]: trait.Seek.html
    fn poll_seek(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: SeekFrom,
    ) -> Poll<io::Result<u64>> {
        let result: u64;
        if let SeekFrom::Current(n) = pos {
            let remainder = (self.cap - self.pos) as i64;
            // it should be safe to assume that remainder fits within an i64 as the alternative
            // means we managed to allocate 8 exbibytes and that's absurd.
            // But it's not out of the realm of possibility for some weird underlying reader to
            // support seeking by i64::min_value() so we need to handle underflow when subtracting
            // remainder.
            if let Some(offset) = n.checked_sub(remainder) {
                result = futures_core::ready!(
                    self.as_mut()
                        .get_pin_mut()
                        .poll_seek(cx, SeekFrom::Current(offset))
                )?;
            } else {
                // seek backwards by our remainder, and then by the offset
                futures_core::ready!(
                    self.as_mut()
                        .get_pin_mut()
                        .poll_seek(cx, SeekFrom::Current(-remainder))
                )?;
                self.as_mut().discard_buffer();
                result = futures_core::ready!(
                    self.as_mut()
                        .get_pin_mut()
                        .poll_seek(cx, SeekFrom::Current(n))
                )?;
            }
        } else {
            // Seeking with Start/End doesn't care about our buffer length.
            result = futures_core::ready!(self.as_mut().get_pin_mut().poll_seek(cx, pos))?;
        }
        self.discard_buffer();
        Poll::Ready(Ok(result))
    }
}
