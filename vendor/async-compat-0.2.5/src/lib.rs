//! Compatibility adapter between tokio and futures.
//!
//! There are two kinds of compatibility issues between [tokio] and [futures]:
//!
//! 1. Tokio's types cannot be used outside tokio context, so any attempt to use them will panic.
//!     - Solution: If you apply the [`Compat`] adapter to a future, the future will manually
//!       enter the context of a global tokio runtime. If a runtime is already available via tokio
//!       thread-locals, then it will be used. Otherwise, a new single-threaded runtime will be
//!       created on demand. That does *not* mean the future is polled by the tokio runtime - it
//!       only means the future sets a thread-local variable pointing to the global tokio runtime so
//!       that tokio's types can be used inside it.
//! 2. Tokio and futures have similar but different I/O traits `AsyncRead`, `AsyncWrite`,
//!    `AsyncBufRead`, and `AsyncSeek`.
//!     - Solution: When the [`Compat`] adapter is applied to an I/O type, it will implement traits
//!       of the opposite kind. That's how you can use tokio-based types wherever futures-based
//!       types are expected, and the other way around.
//!
//! You can apply the [`Compat`] adapter using the [`Compat::new()`] constructor or using any
//! method from the [`CompatExt`] trait.
//!
//! # Examples
//!
//! This program reads lines from stdin and echoes them into stdout, except it's not going to work:
//!
//! ```compile_fail
//! fn main() -> std::io::Result<()> {
//!     futures::executor::block_on(async {
//!         let stdin = tokio::io::stdin();
//!         let mut stdout = tokio::io::stdout();
//!
//!         // The following line will not work for two reasons:
//!         // 1. Runtime error because stdin and stdout are used outside tokio context.
//!         // 2. Compilation error due to mismatched `AsyncRead` and `AsyncWrite` traits.
//!         futures::io::copy(stdin, &mut stdout).await?;
//!         Ok(())
//!     })
//! }
//! ```
//!
//! To get around the compatibility issues, apply the [`Compat`] adapter to `stdin`, `stdout`, and
//! [`futures::io::copy()`]:
//!
//! ```
//! use async_compat::CompatExt;
//!
//! fn main() -> std::io::Result<()> {
//!     futures::executor::block_on(async {
//!         let stdin = tokio::io::stdin();
//!         let mut stdout = tokio::io::stdout();
//!
//!         futures::io::copy(stdin.compat(), &mut stdout.compat_mut()).compat().await?;
//!         Ok(())
//!     })
//! }
//! ```
//!
//! It is also possible to apply [`Compat`] to the outer future passed to
//! [`futures::executor::block_on()`] rather than [`futures::io::copy()`] itself.
//! When applied to the outer future, individual inner futures don't need the adapter because
//! they're all now inside tokio context:
//!
//! ```no_run
//! use async_compat::{Compat, CompatExt};
//!
//! fn main() -> std::io::Result<()> {
//!     futures::executor::block_on(Compat::new(async {
//!         let stdin = tokio::io::stdin();
//!         let mut stdout = tokio::io::stdout();
//!
//!         futures::io::copy(stdin.compat(), &mut stdout.compat_mut()).await?;
//!         Ok(())
//!     }))
//! }
//! ```
//!
//! The compatibility adapter converts between tokio-based and futures-based I/O types in any
//! direction. Here's how we can write the same program by using futures-based I/O types inside
//! tokio:
//!
//! ```no_run
//! use async_compat::CompatExt;
//! use blocking::Unblock;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let mut stdin = Unblock::new(std::io::stdin());
//!     let mut stdout = Unblock::new(std::io::stdout());
//!
//!     tokio::io::copy(&mut stdin.compat_mut(), &mut stdout.compat_mut()).await?;
//!     Ok(())
//! }
//! ```
//!
//! Finally, we can use any tokio-based crate from any other async runtime.
//! Here are [reqwest] and [warp] as an example:
//!
//! ```no_run
//! use async_compat::{Compat, CompatExt};
//! use warp::Filter;
//!
//! fn main() {
//!     futures::executor::block_on(Compat::new(async {
//!         // Make an HTTP GET request.
//!         let response = reqwest::get("https://www.rust-lang.org").await.unwrap();
//!         println!("{}", response.text().await.unwrap());
//!
//!         // Start an HTTP server.
//!         let routes = warp::any().map(|| "Hello from warp!");
//!         warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
//!     }))
//! }
//! ```
//!
//! [blocking]: https://docs.rs/blocking
//! [futures]: https://docs.rs/futures
//! [reqwest]: https://docs.rs/reqwest
//! [tokio]: https://docs.rs/tokio
//! [warp]: https://docs.rs/warp
//! [`futures::io::copy()`]: https://docs.rs/futures/0.3/futures/io/fn.copy.html
//! [`futures::executor::block_on()`]: https://docs.rs/futures/0.3/futures/executor/fn.block_on.html

#![allow(clippy::needless_doctest_main)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png"
)]

use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;

use futures_core::ready;
use once_cell::sync::Lazy;
use pin_project_lite::pin_project;

/// Applies the [`Compat`] adapter to futures and I/O types.
pub trait CompatExt {
    /// Applies the [`Compat`] adapter by value.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_compat::CompatExt;
    ///
    /// let stdout = tokio::io::stdout().compat();
    /// ```
    fn compat(self) -> Compat<Self>
    where
        Self: Sized;

    /// Applies the [`Compat`] adapter by shared reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_compat::CompatExt;
    ///
    /// let original = tokio::io::stdout();
    /// let stdout = original.compat_ref();
    /// ```
    fn compat_ref(&self) -> Compat<&Self>;

    /// Applies the [`Compat`] adapter by mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_compat::CompatExt;
    ///
    /// let mut original = tokio::io::stdout();
    /// let stdout = original.compat_mut();
    /// ```
    fn compat_mut(&mut self) -> Compat<&mut Self>;
}

impl<T> CompatExt for T {
    fn compat(self) -> Compat<Self>
    where
        Self: Sized,
    {
        Compat::new(self)
    }

    fn compat_ref(&self) -> Compat<&Self> {
        Compat::new(self)
    }

    fn compat_mut(&mut self) -> Compat<&mut Self> {
        Compat::new(self)
    }
}

pin_project! {
    /// Compatibility adapter for futures and I/O types.
    #[derive(Clone)]
    pub struct Compat<T> {
        #[pin]
        inner: Option<T>,
        seek_pos: Option<io::SeekFrom>,
    }

    impl<T> PinnedDrop for Compat<T> {
        fn drop(this: Pin<&mut Self>) {
            if this.inner.is_some() {
                // If the inner future wasn't moved out using into_inner,
                // enter the tokio context while the inner value is dropped.
                let _guard = get_runtime_handle().enter();
                this.project().inner.set(None);
            }
        }
    }
}

impl<T> Compat<T> {
    /// Applies the compatibility adapter to a future or an I/O type.
    ///
    /// # Examples
    ///
    /// Apply it to a future:
    ///
    /// ```
    /// use async_compat::Compat;
    /// use std::time::Duration;
    ///
    /// futures::executor::block_on(Compat::new(async {
    ///     // We can use tokio's timers because we're inside tokio context.
    ///     tokio::time::sleep(Duration::from_secs(1)).await;
    /// }));
    /// ```
    ///
    /// Apply it to an I/O type:
    ///
    /// ```
    /// use async_compat::{Compat, CompatExt};
    /// use futures::prelude::*;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// futures::executor::block_on(Compat::new(async {
    ///     // The `write_all` method comes from `futures::io::AsyncWriteExt`.
    ///     Compat::new(tokio::io::stdout()).write_all(b"hello\n").await?;
    ///     Ok(())
    /// }))
    /// # }
    /// ```
    pub fn new(t: T) -> Compat<T> {
        Compat {
            inner: Some(t),
            seek_pos: None,
        }
    }

    /// Gets a shared reference to the inner value.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_compat::Compat;
    /// use tokio::net::UdpSocket;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// futures::executor::block_on(Compat::new(async {
    ///     let socket = Compat::new(UdpSocket::bind("127.0.0.1:0").await?);
    ///     let addr = socket.get_ref().local_addr()?;
    ///     Ok(())
    /// }))
    /// # }
    /// ```
    pub fn get_ref(&self) -> &T {
        self.inner
            .as_ref()
            .expect("inner is only None when Compat is about to drop")
    }

    /// Gets a mutable reference to the inner value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_compat::Compat;
    /// use tokio::net::TcpListener;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// futures::executor::block_on(Compat::new(async {
    ///     let mut listener = Compat::new(TcpListener::bind("127.0.0.1:0").await?);
    ///     let (stream, addr) = listener.get_mut().accept().await?;
    ///     let stream = Compat::new(stream);
    ///     Ok(())
    /// }))
    /// # }
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        self.inner
            .as_mut()
            .expect("inner is only None when Compat is about to drop")
    }

    fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T> {
        self.project()
            .inner
            .as_pin_mut()
            .expect("inner is only None when Compat is about to drop")
    }

    /// Unwraps the compatibility adapter.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_compat::Compat;
    ///
    /// let stdout = Compat::new(tokio::io::stdout());
    /// let original = stdout.into_inner();
    /// ```
    pub fn into_inner(mut self) -> T {
        self.inner
            .take()
            .expect("inner is only None when Compat is about to drop")
    }
}

impl<T: Future> Future for Compat<T> {
    type Output = T::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let _guard = get_runtime_handle().enter();
        self.get_pin_mut().poll(cx)
    }
}

impl<T: tokio::io::AsyncRead> futures_io::AsyncRead for Compat<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let mut buf = tokio::io::ReadBuf::new(buf);
        ready!(self.get_pin_mut().poll_read(cx, &mut buf))?;
        Poll::Ready(Ok(buf.filled().len()))
    }
}

impl<T: futures_io::AsyncRead> tokio::io::AsyncRead for Compat<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let unfilled = buf.initialize_unfilled();
        let poll = self.get_pin_mut().poll_read(cx, unfilled);
        if let Poll::Ready(Ok(num)) = &poll {
            buf.advance(*num);
        }
        poll.map_ok(|_| ())
    }
}

impl<T: tokio::io::AsyncBufRead> futures_io::AsyncBufRead for Compat<T> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        self.get_pin_mut().poll_fill_buf(cx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        self.get_pin_mut().consume(amt)
    }
}

impl<T: futures_io::AsyncBufRead> tokio::io::AsyncBufRead for Compat<T> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        self.get_pin_mut().poll_fill_buf(cx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        self.get_pin_mut().consume(amt)
    }
}

impl<T: tokio::io::AsyncWrite> futures_io::AsyncWrite for Compat<T> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.get_pin_mut().poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_shutdown(cx)
    }
}

impl<T: futures_io::AsyncWrite> tokio::io::AsyncWrite for Compat<T> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.get_pin_mut().poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_close(cx)
    }
}

impl<T: tokio::io::AsyncSeek> futures_io::AsyncSeek for Compat<T> {
    fn poll_seek(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        pos: io::SeekFrom,
    ) -> Poll<io::Result<u64>> {
        if self.seek_pos != Some(pos) {
            self.as_mut().get_pin_mut().start_seek(pos)?;
            *self.as_mut().project().seek_pos = Some(pos);
        }
        let res = ready!(self.as_mut().get_pin_mut().poll_complete(cx));
        *self.as_mut().project().seek_pos = None;
        Poll::Ready(res)
    }
}

impl<T: futures_io::AsyncSeek> tokio::io::AsyncSeek for Compat<T> {
    fn start_seek(mut self: Pin<&mut Self>, pos: io::SeekFrom) -> io::Result<()> {
        *self.as_mut().project().seek_pos = Some(pos);
        Ok(())
    }

    fn poll_complete(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<u64>> {
        let pos = match self.seek_pos {
            None => {
                // tokio 1.x AsyncSeek recommends calling poll_complete before start_seek.
                // We don't have to guarantee that the value returned by
                // poll_complete called without start_seek is correct,
                // so we'll return 0.
                return Poll::Ready(Ok(0));
            }
            Some(pos) => pos,
        };
        let res = ready!(self.as_mut().get_pin_mut().poll_seek(cx, pos));
        *self.as_mut().project().seek_pos = None;
        Poll::Ready(res)
    }
}

fn get_runtime_handle() -> tokio::runtime::Handle {
    tokio::runtime::Handle::try_current().unwrap_or_else(|_| TOKIO1.handle().clone())
}

static TOKIO1: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    thread::Builder::new()
        .name("async-compat/tokio-1".into())
        .spawn(|| TOKIO1.block_on(Pending))
        .unwrap();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("cannot start tokio-1 runtime")
});

struct Pending;

impl Future for Pending {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::Lazy;
    use crate::{CompatExt, TOKIO1};

    #[test]
    fn fallback_runtime_is_created_if_and_only_if_outside_tokio_context() {
        // Use compat inside of a tokio context.
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(use_tokio().compat());

        // We didn't need to create the fallback runtime, because we used compat
        // inside of an existing tokio context.
        assert!(Lazy::get(&TOKIO1).is_none());

        // Use compat outside of a tokio context.
        futures::executor::block_on(use_tokio().compat());

        // We must have created the fallback runtime, because we used compat
        // outside of a tokio context.
        assert!(Lazy::get(&TOKIO1).is_some());
    }

    async fn use_tokio() {
        tokio::time::sleep(std::time::Duration::from_micros(1)).await
    }
}
