use std::future::Future;
use std::pin::Pin;

use pin_project_lite::pin_project;

use crate::io::{self, BufRead, BufReader, Read, Write};
use crate::task::{Context, Poll};
use crate::utils::Context as _;

// Note: There are two otherwise-identical implementations of this
// function because unstable has removed the `?Sized` bound for the
// reader and writer and accepts `R` and `W` instead of `&mut R` and
// `&mut W`. If making a change to either of the implementations,
// ensure that you copy it into the other.

/// Copies the entire contents of a reader into a writer.
///
/// This function will continuously read data from `reader` and then
/// write it into `writer` in a streaming fashion until `reader`
/// returns EOF.
///
/// On success, the total number of bytes that were copied from
/// `reader` to `writer` is returned.
///
/// If you’re wanting to copy the contents of one file to another and you’re
/// working with filesystem paths, see the [`fs::copy`] function.
///
/// This function is an async version of [`std::io::copy`].
///
/// [`std::io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
/// [`fs::copy`]: ../fs/fn.copy.html
///
/// # Errors
///
/// This function will return an error immediately if any call to `read` or
/// `write` returns an error. All instances of `ErrorKind::Interrupted` are
/// handled by this function and the underlying operation is retried.
///
/// # Examples
///
/// ```
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::io;
///
/// let mut reader: &[u8] = b"hello";
/// let mut writer = io::stdout();
///
/// io::copy(&mut reader, &mut writer).await?;
/// #
/// # Ok(()) }) }
/// ```
#[cfg(any(feature = "docs", not(feature = "unstable")))]
pub async fn copy<R, W>(reader: &mut R, writer: &mut W) -> io::Result<u64>
where
    R: Read + Unpin + ?Sized,
    W: Write + Unpin + ?Sized,
{
    pin_project! {
        struct CopyFuture<R, W> {
            #[pin]
            reader: R,
            #[pin]
            writer: W,
            amt: u64,
            reader_eof: bool
        }
    }

    impl<R, W> Future for CopyFuture<R, W>
    where
        R: BufRead,
        W: Write + Unpin,
    {
        type Output = io::Result<u64>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let mut this = self.project();

            loop {
                if *this.reader_eof {
                    futures_core::ready!(this.writer.as_mut().poll_flush(cx))?;
                    return Poll::Ready(Ok(*this.amt));
                }

                let buffer = futures_core::ready!(this.reader.as_mut().poll_fill_buf(cx))?;

                if buffer.is_empty() {
                    *this.reader_eof = true;
                    continue;
                }

                let i = futures_core::ready!(this.writer.as_mut().poll_write(cx, buffer))?;
                if i == 0 {
                    return Poll::Ready(Err(io::ErrorKind::WriteZero.into()));
                }
                *this.amt += i as u64;
                this.reader.as_mut().consume(i);
            }
        }
    }

    let future = CopyFuture {
        reader: BufReader::new(reader),
        writer,
        reader_eof: false,
        amt: 0,
    };
    future.await.context(|| String::from("io::copy failed"))
}

/// Copies the entire contents of a reader into a writer.
///
/// This function will continuously read data from `reader` and then
/// write it into `writer` in a streaming fashion until `reader`
/// returns EOF.
///
/// On success, the total number of bytes that were copied from
/// `reader` to `writer` is returned.
///
/// If you’re wanting to copy the contents of one file to another and you’re
/// working with filesystem paths, see the [`fs::copy`] function.
///
/// This function is an async version of [`std::io::copy`].
///
/// [`std::io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
/// [`fs::copy`]: ../fs/fn.copy.html
///
/// # Errors
///
/// This function will return an error immediately if any call to `read` or
/// `write` returns an error. All instances of `ErrorKind::Interrupted` are
/// handled by this function and the underlying operation is retried.
///
/// # Examples
///
/// ```
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::io;
///
/// let mut reader: &[u8] = b"hello";
/// let mut writer = io::stdout();
///
/// io::copy(&mut reader, &mut writer).await?;
/// #
/// # Ok(()) }) }
/// ```
#[cfg(all(feature = "unstable", not(feature = "docs")))]
pub async fn copy<R, W>(reader: R, writer: W) -> io::Result<u64>
where
    R: Read + Unpin,
    W: Write + Unpin,
{
    pin_project! {
        struct CopyFuture<R, W> {
            #[pin]
            reader: R,
            #[pin]
            writer: W,
            amt: u64,
            reader_eof: bool
        }
    }

    impl<R, W> Future for CopyFuture<R, W>
    where
        R: BufRead,
        W: Write + Unpin,
    {
        type Output = io::Result<u64>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let mut this = self.project();

            loop {
                if *this.reader_eof {
                    futures_core::ready!(this.writer.as_mut().poll_flush(cx))?;
                    return Poll::Ready(Ok(*this.amt));
                }

                let buffer = futures_core::ready!(this.reader.as_mut().poll_fill_buf(cx))?;

                if buffer.is_empty() {
                    *this.reader_eof = true;
                    continue;
                }

                let i = futures_core::ready!(this.writer.as_mut().poll_write(cx, buffer))?;
                if i == 0 {
                    return Poll::Ready(Err(io::ErrorKind::WriteZero.into()));
                }
                *this.amt += i as u64;
                this.reader.as_mut().consume(i);
            }
        }
    }

    let future = CopyFuture {
        reader: BufReader::new(reader),
        writer,
        reader_eof: false,
        amt: 0,
    };
    future.await.context(|| String::from("io::copy failed"))
}
