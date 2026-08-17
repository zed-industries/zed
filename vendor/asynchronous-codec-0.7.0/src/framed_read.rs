use super::fuse::Fuse;
use super::Decoder;

use bytes::BytesMut;
use futures_sink::Sink;
use futures_util::io::AsyncRead;
use futures_util::ready;
use futures_util::stream::{Stream, TryStreamExt};
use pin_project_lite::pin_project;
use std::io;
use std::marker::Unpin;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll};

/// A `Stream` of messages decoded from an `AsyncRead`.
///
/// # Example
/// ```
/// use asynchronous_codec::{BytesCodec, FramedRead};
/// use futures::TryStreamExt;
/// use bytes::{Bytes};
///
/// let buf = [3u8; 3];
/// let mut framed = FramedRead::new(&buf[..], BytesCodec);
///
/// # futures::executor::block_on(async move {
/// if let Some(bytes) = framed.try_next().await? {
///     assert_eq!(bytes, Bytes::copy_from_slice(&buf[..]));
/// }
/// # Ok::<_, std::io::Error>(())
/// # }).unwrap();
/// ```
#[derive(Debug)]
pub struct FramedRead<T, D> {
    inner: FramedRead2<Fuse<T, D>>,
}

impl<T, D> Deref for FramedRead<T, D> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T, D> DerefMut for FramedRead<T, D> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T, D> FramedRead<T, D>
where
    T: AsyncRead,
    D: Decoder,
{
    /// Creates a new `FramedRead` transport with the given `Decoder`.
    pub fn new(inner: T, decoder: D) -> Self {
        Self {
            inner: framed_read_2(Fuse::new(inner, decoder), None),
        }
    }

    /// Creates a new `FramedRead` from [`FramedReadParts`].
    ///
    /// See also [`FramedRead::into_parts`].
    pub fn from_parts(
        FramedReadParts {
            io,
            decoder,
            buffer,
            ..
        }: FramedReadParts<T, D>,
    ) -> Self {
        Self {
            inner: framed_read_2(Fuse::new(io, decoder), Some(buffer)),
        }
    }

    /// Consumes the `FramedRead`, returning its parts such that a
    /// new `FramedRead` may be constructed, possibly with a different decoder.
    ///
    /// See also [`FramedRead::from_parts`].
    pub fn into_parts(self) -> FramedReadParts<T, D> {
        let (fuse, buffer) = self.inner.into_parts();
        FramedReadParts {
            io: fuse.t,
            decoder: fuse.u,
            buffer,
            _priv: (),
        }
    }

    /// Consumes the `FramedRead`, returning its underlying I/O stream.
    ///
    /// Note that data that has already been read but not yet consumed
    /// by the decoder is dropped. To retain any such potentially
    /// buffered data, use [`FramedRead::into_parts()`].
    pub fn into_inner(self) -> T {
        self.into_parts().io
    }

    /// Returns a reference to the underlying decoder.
    ///
    /// Note that care should be taken to not tamper with the underlying decoder
    /// as it may corrupt the stream of frames otherwise being worked with.
    pub fn decoder(&self) -> &D {
        &self.inner.u
    }

    /// Returns a mutable reference to the underlying decoder.
    ///
    /// Note that care should be taken to not tamper with the underlying decoder
    /// as it may corrupt the stream of frames otherwise being worked with.
    pub fn decoder_mut(&mut self) -> &mut D {
        &mut self.inner.u
    }

    /// Returns a reference to the read buffer.
    pub fn read_buffer(&self) -> &BytesMut {
        &self.inner.buffer
    }
}

impl<T, D> Stream for FramedRead<T, D>
where
    T: AsyncRead + Unpin,
    D: Decoder,
{
    type Item = Result<D::Item, D::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.try_poll_next_unpin(cx)
    }
}

pin_project! {
    #[derive(Debug)]
    pub struct FramedRead2<T> {
        #[pin]
        inner: T,
        buffer: BytesMut,
    }
}

impl<T> Deref for FramedRead2<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for FramedRead2<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

const INITIAL_CAPACITY: usize = 8 * 1024;

pub fn framed_read_2<T>(inner: T, buffer: Option<BytesMut>) -> FramedRead2<T> {
    FramedRead2 {
        inner,
        buffer: buffer.unwrap_or_else(|| BytesMut::with_capacity(INITIAL_CAPACITY)),
    }
}

impl<T> Stream for FramedRead2<T>
where
    T: AsyncRead + Decoder + Unpin,
{
    type Item = Result<T::Item, T::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = &mut *self;

        if let Some(item) = this.inner.decode(&mut this.buffer)? {
            return Poll::Ready(Some(Ok(item)));
        }

        let mut buf = [0u8; INITIAL_CAPACITY];

        loop {
            let n = ready!(Pin::new(&mut this.inner).poll_read(cx, &mut buf))?;
            this.buffer.extend_from_slice(&buf[..n]);

            let ended = n == 0;

            match this.inner.decode(&mut this.buffer)? {
                Some(item) => return Poll::Ready(Some(Ok(item))),
                None if ended => {
                    if this.buffer.is_empty() {
                        return Poll::Ready(None);
                    } else {
                        match this.inner.decode_eof(&mut this.buffer)? {
                            Some(item) => return Poll::Ready(Some(Ok(item))),
                            None if this.buffer.is_empty() => return Poll::Ready(None),
                            None => {
                                return Poll::Ready(Some(Err(io::Error::new(
                                    io::ErrorKind::UnexpectedEof,
                                    "bytes remaining in stream",
                                )
                                .into())));
                            }
                        }
                    }
                }
                _ => continue,
            }
        }
    }
}

impl<T, I> Sink<I> for FramedRead2<T>
where
    T: Sink<I> + Unpin,
{
    type Error = T::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx)
    }
    fn start_send(self: Pin<&mut Self>, item: I) -> Result<(), Self::Error> {
        self.project().inner.start_send(item)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx)
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx)
    }
}

impl<T> FramedRead2<T> {
    pub fn into_parts(self) -> (T, BytesMut) {
        (self.inner, self.buffer)
    }

    pub fn buffer(&self) -> &BytesMut {
        &self.buffer
    }
}

/// The parts obtained from (FramedRead::into_parts).
pub struct FramedReadParts<T, D> {
    /// The underlying I/O stream.
    pub io: T,
    /// The frame decoder.
    pub decoder: D,
    /// The buffer of data that has been read from `io` but not
    /// yet consumed by `decoder`.
    pub buffer: BytesMut,
    /// Keep the constructor private.
    _priv: (),
}

impl<T, D> FramedReadParts<T, D> {
    /// Changes the decoder in `FramedReadParts`.
    pub fn map_decoder<E, F>(self, f: F) -> FramedReadParts<T, E>
    where
        E: Decoder,
        F: FnOnce(D) -> E,
    {
        FramedReadParts {
            io: self.io,
            decoder: f(self.decoder),
            buffer: self.buffer,
            _priv: (),
        }
    }
}
