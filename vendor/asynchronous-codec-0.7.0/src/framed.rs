use super::framed_read::{framed_read_2, FramedRead2};
use super::framed_write::{framed_write_2, FramedWrite2};
use super::fuse::Fuse;
use super::{Decoder, Encoder};
use bytes::BytesMut;
use futures_sink::Sink;
use futures_util::io::{AsyncRead, AsyncWrite};
use futures_util::stream::{Stream, TryStreamExt};
use pin_project_lite::pin_project;
use std::marker::Unpin;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    /// A unified `Stream` and `Sink` interface to an underlying I/O object,
    /// using the `Encoder` and `Decoder` traits to encode and decode frames.
    ///
    /// # Example
    /// ```
    /// use bytes::Bytes;
    /// use futures::{SinkExt, TryStreamExt};
    /// use futures::io::Cursor;
    /// use asynchronous_codec::{BytesCodec, Framed};
    ///
    /// # futures::executor::block_on(async move {
    /// let cur = Cursor::new(vec![0u8; 12]);
    /// let mut framed = Framed::new(cur, BytesCodec {});
    ///
    /// // Send bytes to `buf` through the `BytesCodec`
    /// let bytes = Bytes::from("Hello world!");
    /// framed.send(bytes).await?;
    ///
    /// // Drop down to the underlying I/O stream.
    /// let cur = framed.into_inner();
    /// assert_eq!(cur.get_ref(), b"Hello world!");
    /// # Ok::<_, std::io::Error>(())
    /// # }).unwrap();
    /// ```
    #[derive(Debug)]
    pub struct Framed<T, U> {
        #[pin]
        inner: FramedRead2<FramedWrite2<Fuse<T, U>>>,
    }
}

impl<T, U> Deref for Framed<T, U> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T, U> DerefMut for Framed<T, U> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T, U> Framed<T, U>
where
    T: AsyncRead + AsyncWrite,
    U: Decoder + Encoder,
{
    /// Creates a new `Framed` transport with the given codec.
    /// A codec is a type which implements `Decoder` and `Encoder`.
    pub fn new(inner: T, codec: U) -> Self {
        Self {
            inner: framed_read_2(framed_write_2(Fuse::new(inner, codec), None), None),
        }
    }

    /// Creates a new `Framed` from [`FramedParts`].
    ///
    /// See also [`Framed::into_parts`].
    pub fn from_parts(
        FramedParts {
            io,
            codec,
            write_buffer,
            read_buffer,
            ..
        }: FramedParts<T, U>,
    ) -> Self {
        let framed_write = framed_write_2(Fuse::new(io, codec), Some(write_buffer));
        let framed_read = framed_read_2(framed_write, Some(read_buffer));
        Self { inner: framed_read }
    }

    /// Consumes the `Framed`, returning its parts, such that a new
    /// `Framed` may be constructed, possibly with a different codec.
    ///
    /// See also [`Framed::from_parts`].
    pub fn into_parts(self) -> FramedParts<T, U> {
        let (framed_write, read_buffer) = self.inner.into_parts();
        let (fuse, write_buffer) = framed_write.into_parts();
        FramedParts {
            io: fuse.t,
            codec: fuse.u,
            read_buffer,
            write_buffer,
            _priv: (),
        }
    }

    /// Consumes the `Framed`, returning its underlying I/O stream.
    ///
    /// Note that data that has already been read or written but not yet
    /// consumed by the decoder or flushed, respectively, is dropped.
    /// To retain any such potentially buffered data, use [`Framed::into_parts()`].
    pub fn into_inner(self) -> T {
        self.into_parts().io
    }

    /// Returns a reference to the underlying codec wrapped by
    /// `Framed`.
    ///
    /// Note that care should be taken to not tamper with the underlying codec
    /// as it may corrupt the stream of frames otherwise being worked with.
    pub fn codec(&self) -> &U {
        &self.inner.u
    }

    /// Returns a mutable reference to the underlying codec wrapped by
    /// `Framed`.
    ///
    /// Note that care should be taken to not tamper with the underlying codec
    /// as it may corrupt the stream of frames otherwise being worked with.
    pub fn codec_mut(&mut self) -> &mut U {
        &mut self.inner.u
    }

    /// Returns a reference to the read buffer.
    pub fn read_buffer(&self) -> &BytesMut {
        self.inner.buffer()
    }

    /// High-water mark for writes, in bytes
    ///
    /// See [`FramedWrite::send_high_water_mark`].
    pub fn send_high_water_mark(&self) -> usize {
        self.inner.high_water_mark
    }

    /// Sets high-water mark for writes, in bytes
    ///
    /// See [`FramedWrite::set_send_high_water_mark`].
    pub fn set_send_high_water_mark(&mut self, hwm: usize) {
        self.inner.high_water_mark = hwm;
    }
}

impl<T, U> Stream for Framed<T, U>
where
    T: AsyncRead + Unpin,
    U: Decoder,
{
    type Item = Result<U::Item, U::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.try_poll_next_unpin(cx)
    }
}

impl<T, U> Sink<U::Item<'_>> for Framed<T, U>
where
    T: AsyncWrite + Unpin,
    U: Encoder,
{
    type Error = U::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx)
    }
    fn start_send(self: Pin<&mut Self>, item: U::Item<'_>) -> Result<(), Self::Error> {
        self.project().inner.start_send(item)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx)
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx)
    }
}

/// The parts obtained from [`Framed::into_parts`].
pub struct FramedParts<T, U> {
    /// The underlying I/O stream.
    pub io: T,
    /// The codec used for encoding and decoding frames.
    pub codec: U,
    /// The remaining read buffer, containing data that has been
    /// read from `io` but not yet consumed by the codec's decoder.
    pub read_buffer: BytesMut,
    /// The remaining write buffer, containing framed data that has been
    /// buffered but not yet flushed to `io`.
    pub write_buffer: BytesMut,
    /// Keep the constructor private.
    _priv: (),
}

impl<T, U> FramedParts<T, U> {
    /// Changes the codec used in this `FramedParts`.
    pub fn map_codec<V, F>(self, f: F) -> FramedParts<T, V>
    where
        F: FnOnce(U) -> V,
    {
        FramedParts {
            io: self.io,
            codec: f(self.codec),
            read_buffer: self.read_buffer,
            write_buffer: self.write_buffer,
            _priv: (),
        }
    }
}
