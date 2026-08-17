use std::cmp;
use std::pin::Pin;

use pin_project_lite::pin_project;

use crate::io::{self, BufRead, Read};
use crate::task::{Context, Poll};

pin_project! {
    /// Reader adaptor which limits the bytes read from an underlying reader.
    ///
    /// This struct is generally created by calling [`take`] on a reader.
    /// Please see the documentation of [`take`] for more details.
    ///
    /// [`take`]: trait.Read.html#method.take
    #[derive(Debug)]
    pub struct Take<T> {
        #[pin]
        pub(crate) inner: T,
        pub(crate) limit: u64,
    }
}

impl<T> Take<T> {
    /// Returns the number of bytes that can be read before this instance will
    /// return EOF.
    ///
    /// # Note
    ///
    /// This instance may reach `EOF` after reading fewer bytes than indicated by
    /// this method if the underlying [`Read`] instance reaches EOF.
    ///
    /// [`Read`]: trait.Read.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> async_std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::prelude::*;
    /// use async_std::fs::File;
    ///
    /// let f = File::open("foo.txt").await?;
    ///
    /// // read at most five bytes
    /// let handle = f.take(5);
    ///
    /// println!("limit: {}", handle.limit());
    /// #
    /// #     Ok(()) }) }
    /// ```
    pub fn limit(&self) -> u64 {
        self.limit
    }

    /// Sets the number of bytes that can be read before this instance will
    /// return EOF. This is the same as constructing a new `Take` instance, so
    /// the amount of bytes read and the previous limit value don't matter when
    /// calling this method.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> async_std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::prelude::*;
    /// use async_std::fs::File;
    ///
    /// let f = File::open("foo.txt").await?;
    ///
    /// // read at most five bytes
    /// let mut handle = f.take(5);
    /// handle.set_limit(10);
    ///
    /// assert_eq!(handle.limit(), 10);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn set_limit(&mut self, limit: u64) {
        self.limit = limit;
    }

    /// Consumes the `Take`, returning the wrapped reader.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> async_std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::prelude::*;
    /// use async_std::fs::File;
    ///
    /// let file = File::open("foo.txt").await?;
    ///
    /// let mut buffer = [0; 5];
    /// let mut handle = file.take(5);
    /// handle.read(&mut buffer).await?;
    ///
    /// let file = handle.into_inner();
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets a reference to the underlying reader.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> async_std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::prelude::*;
    /// use async_std::fs::File;
    ///
    /// let file = File::open("foo.txt").await?;
    ///
    /// let mut buffer = [0; 5];
    /// let mut handle = file.take(5);
    /// handle.read(&mut buffer).await?;
    ///
    /// let file = handle.get_ref();
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader.
    ///
    /// Care should be taken to avoid modifying the internal I/O state of the
    /// underlying reader as doing so may corrupt the internal limit of this
    /// `Take`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> async_std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::prelude::*;
    /// use async_std::fs::File;
    ///
    /// let file = File::open("foo.txt").await?;
    ///
    /// let mut buffer = [0; 5];
    /// let mut handle = file.take(5);
    /// handle.read(&mut buffer).await?;
    ///
    /// let file = handle.get_mut();
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: Read> Read for Take<T> {
    /// Attempt to read from the `AsyncRead` into `buf`.
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.project();
        take_read_internal(this.inner, cx, buf, this.limit)
    }
}

pub fn take_read_internal<R: Read + ?Sized>(
    mut rd: Pin<&mut R>,
    cx: &mut Context<'_>,
    buf: &mut [u8],
    limit: &mut u64,
) -> Poll<io::Result<usize>> {
    // Don't call into inner reader at all at EOF because it may still block
    if *limit == 0 {
        return Poll::Ready(Ok(0));
    }

    let max = cmp::min(buf.len() as u64, *limit) as usize;

    match futures_core::ready!(rd.as_mut().poll_read(cx, &mut buf[..max])) {
        Ok(n) => {
            *limit -= n as u64;
            Poll::Ready(Ok(n))
        }
        Err(e) => Poll::Ready(Err(e)),
    }
}

impl<T: BufRead> BufRead for Take<T> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        let this = self.project();

        if *this.limit == 0 {
            return Poll::Ready(Ok(&[]));
        }

        match futures_core::ready!(this.inner.poll_fill_buf(cx)) {
            Ok(buf) => {
                let cap = cmp::min(buf.len() as u64, *this.limit) as usize;
                Poll::Ready(Ok(&buf[..cap]))
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        let this = self.project();
        // Don't let callers reset the limit by passing an overlarge value
        let amt = cmp::min(amt as u64, *this.limit) as usize;
        *this.limit -= amt as u64;

        this.inner.consume(amt);
    }
}

#[cfg(all(test, not(target_os = "unknown")))]
mod tests {
    use crate::io;
    use crate::prelude::*;
    use crate::task;

    #[test]
    fn test_take_basics() -> std::io::Result<()> {
        let source: io::Cursor<Vec<u8>> = io::Cursor::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);

        task::block_on(async move {
            let mut buffer = [0u8; 5];

            // read at most five bytes
            let mut handle = source.take(5);

            handle.read(&mut buffer).await?;
            assert_eq!(buffer, [0, 1, 2, 3, 4]);

            // check that the we are actually at the end
            assert_eq!(handle.read(&mut buffer).await.unwrap(), 0);

            Ok(())
        })
    }
}
