mod flush;
mod write;
mod write_all;
mod write_fmt;
mod write_vectored;

use flush::FlushFuture;
use write::WriteFuture;
use write_all::WriteAllFuture;
use write_fmt::WriteFmtFuture;
use write_vectored::WriteVectoredFuture;

use crate::io::{self, IoSlice};

pub use futures_io::AsyncWrite as Write;

#[doc = r#"
    Extension methods for [`Write`].

    [`Write`]: ../trait.Write.html
"#]
pub trait WriteExt: Write {
    #[doc = r#"
        Writes some bytes into the byte stream.

        Returns the number of bytes written from the start of the buffer.

        If the return value is `Ok(n)` then it must be guaranteed that
        `0 <= n <= buf.len()`. A return value of `0` typically means that the underlying
        object is no longer able to accept bytes and will likely not be able to in the
        future as well, or that the buffer provided is empty.

        # Examples

        ```no_run
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::fs::File;
        use async_std::prelude::*;

        let mut file = File::create("a.txt").await?;

        let n = file.write(b"hello world").await?;
        #
        # Ok(()) }) }
        ```
    "#]
    fn write<'a>(
        &'a mut self,
        buf: &'a [u8],
    ) -> WriteFuture<'a, Self>
    where
        Self: Unpin,
    {
        WriteFuture { writer: self, buf }
    }

    #[doc = r#"
        Flushes the stream to ensure that all buffered contents reach their destination.

        # Examples

        ```no_run
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::fs::File;
        use async_std::prelude::*;

        let mut file = File::create("a.txt").await?;

        file.write_all(b"hello world").await?;
        file.flush().await?;
        #
        # Ok(()) }) }
        ```
    "#]
    fn flush(&mut self) -> FlushFuture<'_, Self>
    where
        Self: Unpin,
    {
        FlushFuture { writer: self }
    }

    #[doc = r#"
        Like [`write`], except that it writes from a slice of buffers.

        Data is copied from each buffer in order, with the final buffer read from possibly
        being only partially consumed. This method must behave as a call to [`write`] with
        the buffers concatenated would.

        The default implementation calls [`write`] with either the first nonempty buffer
        provided, or an empty one if none exists.

        [`write`]: #tymethod.write
    "#]
    fn write_vectored<'a>(
        &'a mut self,
        bufs: &'a [IoSlice<'a>],
    ) -> WriteVectoredFuture<'a, Self>
    where
        Self: Unpin,
    {
        WriteVectoredFuture { writer: self, bufs }
    }

    #[doc = r#"
        Writes an entire buffer into the byte stream.

        This method will continuously call [`write`] until there is no more data to be
        written or an error is returned. This method will not return until the entire
        buffer has been successfully written or such an error occurs.

        [`write`]: #tymethod.write

        # Examples

        ```no_run
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::fs::File;
        use async_std::prelude::*;

        let mut file = File::create("a.txt").await?;

        file.write_all(b"hello world").await?;
        #
        # Ok(()) }) }
        ```

        [`write`]: #tymethod.write
    "#]
    fn write_all<'a>(
        &'a mut self,
        buf: &'a [u8],
    ) -> WriteAllFuture<'a, Self>
    where
        Self: Unpin,
    {
        WriteAllFuture { writer: self, buf }
    }

    #[doc = r#"
        Writes a formatted string into this writer, returning any error encountered.

        This method will continuously call [`write`] until there is no more data to be
        written or an error is returned. This future will not resolve until the entire
        buffer has been successfully written or such an error occurs.

        [`write`]: #tymethod.write

        # Examples

        ```no_run
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::io::prelude::*;
        use async_std::fs::File;

        let mut buffer = File::create("foo.txt").await?;

        // this call
        write!(buffer, "{:.*}", 2, 1.234567).await?;
        // turns into this:
        buffer.write_fmt(format_args!("{:.*}", 2, 1.234567)).await?;
        #
        # Ok(()) }) }
        ```
    "#]
    fn write_fmt<'a>(
        &'a mut self,
        fmt: std::fmt::Arguments<'_>,
    ) -> WriteFmtFuture<'a, Self>
    where
        Self: Unpin,
    {
        // In order to not have to implement an async version of `fmt` including private types
        // and all, we convert `Arguments` to a `Result<Vec<u8>>` and pass that to the Future.
        // Doing an owned conversion saves us from juggling references.
        let mut string = String::new();
        let res = std::fmt::write(&mut string, fmt)
            .map(|_| string.into_bytes())
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "formatter error"));
        WriteFmtFuture { writer: self, res: Some(res), buffer: None, amt: 0 }
    }
}

impl<T: Write + ?Sized> WriteExt for T {}
