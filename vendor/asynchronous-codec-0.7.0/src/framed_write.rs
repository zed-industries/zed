use super::fuse::Fuse;
use super::Encoder;
use bytes::{Buf, BytesMut};
use futures_sink::Sink;
use futures_util::io::{AsyncRead, AsyncWrite};
use futures_util::ready;
use pin_project_lite::pin_project;
use std::io::{Error, ErrorKind};
use std::marker::Unpin;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    /// A `Sink` of frames encoded to an `AsyncWrite`.
    ///
    /// # Example
    /// ```
    /// use bytes::Bytes;
    /// use asynchronous_codec::{FramedWrite, BytesCodec};
    /// use futures::SinkExt;
    ///
    /// # futures::executor::block_on(async move {
    /// let mut buf = Vec::new();
    /// let mut framed = FramedWrite::new(&mut buf, BytesCodec {});
    ///
    /// let bytes = Bytes::from("Hello World!");
    /// framed.send(bytes.clone()).await?;
    ///
    /// assert_eq!(&buf[..], &bytes[..]);
    /// # Ok::<_, std::io::Error>(())
    /// # }).unwrap();
    /// ```
    #[derive(Debug)]
    pub struct FramedWrite<T, E> {
        #[pin]
        inner: FramedWrite2<Fuse<T, E>>,
    }
}

impl<T, E> FramedWrite<T, E>
where
    T: AsyncWrite,
    E: Encoder,
{
    /// Creates a new `FramedWrite` transport with the given `Encoder`.
    pub fn new(inner: T, encoder: E) -> Self {
        Self {
            inner: framed_write_2(Fuse::new(inner, encoder), None),
        }
    }

    /// Creates a new `FramedWrite` from [`FramedWriteParts`].
    ///
    /// See also [`FramedWrite::into_parts`].
    pub fn from_parts(
        FramedWriteParts {
            io,
            encoder,
            buffer,
            ..
        }: FramedWriteParts<T, E>,
    ) -> Self {
        Self {
            inner: framed_write_2(Fuse::new(io, encoder), Some(buffer)),
        }
    }

    /// High-water mark for writes, in bytes
    ///
    /// The send *high-water mark* prevents the `FramedWrite`
    /// from accepting additional messages to send when its
    /// buffer exceeds this length, in bytes. Attempts to enqueue
    /// additional messages will be deferred until progress is
    /// made on the underlying `AsyncWrite`. This applies
    /// back-pressure on fast senders and prevents unbounded
    /// buffer growth.
    ///
    /// See [`set_send_high_water_mark()`](#method.set_send_high_water_mark).
    pub fn send_high_water_mark(&self) -> usize {
        self.inner.high_water_mark
    }

    /// Sets high-water mark for writes, in bytes
    ///
    /// The send *high-water mark* prevents the `FramedWrite`
    /// from accepting additional messages to send when its
    /// buffer exceeds this length, in bytes. Attempts to enqueue
    /// additional messages will be deferred until progress is
    /// made on the underlying `AsyncWrite`. This applies
    /// back-pressure on fast senders and prevents unbounded
    /// buffer growth.
    ///
    /// The default high-water mark is 2^17 bytes. Applications
    /// which desire low latency may wish to reduce this value.
    /// There is little point to increasing this value beyond
    /// your socket's `SO_SNDBUF` size. On linux, this defaults
    /// to 212992 bytes but is user-adjustable.
    pub fn set_send_high_water_mark(&mut self, hwm: usize) {
        self.inner.high_water_mark = hwm;
    }

    /// Consumes the `FramedWrite`, returning its parts such that
    /// a new `FramedWrite` may be constructed, possibly with a different encoder.
    ///
    /// See also [`FramedWrite::from_parts`].
    pub fn into_parts(self) -> FramedWriteParts<T, E> {
        let (fuse, buffer) = self.inner.into_parts();
        FramedWriteParts {
            io: fuse.t,
            encoder: fuse.u,
            buffer,
            _priv: (),
        }
    }

    /// Consumes the `FramedWrite`, returning its underlying I/O stream.
    ///
    /// Note that data that has already been written but not yet flushed
    /// is dropped. To retain any such potentially buffered data, use
    /// [`FramedWrite::into_parts()`].
    pub fn into_inner(self) -> T {
        self.into_parts().io
    }

    /// Returns a reference to the underlying encoder.
    ///
    /// Note that care should be taken to not tamper with the underlying encoder
    /// as it may corrupt the stream of frames otherwise being worked with.
    pub fn encoder(&self) -> &E {
        &self.inner.u
    }

    /// Returns a mutable reference to the underlying encoder.
    ///
    /// Note that care should be taken to not tamper with the underlying encoder
    /// as it may corrupt the stream of frames otherwise being worked with.
    pub fn encoder_mut(&mut self) -> &mut E {
        &mut self.inner.u
    }
}

impl<T, E> Sink<E::Item<'_>> for FramedWrite<T, E>
where
    T: AsyncWrite + Unpin,
    E: Encoder,
{
    type Error = E::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx)
    }
    fn start_send(self: Pin<&mut Self>, item: E::Item<'_>) -> Result<(), Self::Error> {
        self.project().inner.start_send(item)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx)
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx)
    }
}

impl<T, E> Deref for FramedWrite<T, E> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T, E> DerefMut for FramedWrite<T, E> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

pin_project! {
    #[derive(Debug)]
    pub struct FramedWrite2<T> {
        #[pin]
        pub inner: T,
        pub high_water_mark: usize,
        buffer: BytesMut,
    }
}

impl<T> Deref for FramedWrite2<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for FramedWrite2<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

// 2^17 bytes, which is slightly over 60% of the default
// TCP send buffer size (SO_SNDBUF)
const DEFAULT_SEND_HIGH_WATER_MARK: usize = 131072;

pub fn framed_write_2<T>(inner: T, buffer: Option<BytesMut>) -> FramedWrite2<T> {
    FramedWrite2 {
        inner,
        high_water_mark: DEFAULT_SEND_HIGH_WATER_MARK,
        buffer: buffer.unwrap_or_else(|| BytesMut::with_capacity(1028 * 8)),
    }
}

impl<T: AsyncRead + Unpin> AsyncRead for FramedWrite2<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        self.project().inner.poll_read(cx, buf)
    }
}

impl<T> Sink<T::Item<'_>> for FramedWrite2<T>
where
    T: AsyncWrite + Encoder + Unpin,
{
    type Error = T::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let this = &mut *self;
        while this.buffer.len() >= this.high_water_mark {
            let num_write = ready!(Pin::new(&mut this.inner).poll_write(cx, &this.buffer))?;

            if num_write == 0 {
                return Poll::Ready(Err(err_eof().into()));
            }

            this.buffer.advance(num_write);
        }

        Poll::Ready(Ok(()))
    }
    fn start_send(mut self: Pin<&mut Self>, item: T::Item<'_>) -> Result<(), Self::Error> {
        let this = &mut *self;
        this.inner.encode(item, &mut this.buffer)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let mut this = self.project();

        while !this.buffer.is_empty() {
            let num_write = ready!(Pin::new(&mut this.inner).poll_write(cx, &this.buffer))?;

            if num_write == 0 {
                return Poll::Ready(Err(err_eof().into()));
            }

            this.buffer.advance(num_write);
        }

        this.inner.poll_flush(cx).map_err(Into::into)
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        ready!(self.as_mut().poll_flush(cx))?;
        self.project().inner.poll_close(cx).map_err(Into::into)
    }
}

impl<T> FramedWrite2<T> {
    pub fn into_parts(self) -> (T, BytesMut) {
        (self.inner, self.buffer)
    }
}

fn err_eof() -> Error {
    Error::new(ErrorKind::UnexpectedEof, "End of file")
}

/// The parts obtained from [`FramedWrite::into_parts`].
pub struct FramedWriteParts<T, E> {
    /// The underlying I/O stream.
    pub io: T,
    /// The frame encoder.
    pub encoder: E,
    /// The framed data that has been buffered but not yet flushed to `io`.
    pub buffer: BytesMut,
    /// Keep the constructor private.
    _priv: (),
}

impl<T, E> FramedWriteParts<T, E> {
    /// Changes the encoder used in `FramedWriteParts`.
    pub fn map_encoder<G, F>(self, f: F) -> FramedWriteParts<T, G>
    where
        G: Encoder,
        F: FnOnce(E) -> G,
    {
        FramedWriteParts {
            io: self.io,
            encoder: f(self.encoder),
            buffer: self.buffer,
            _priv: (),
        }
    }
}
