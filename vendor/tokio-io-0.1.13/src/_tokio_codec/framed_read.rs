#![allow(deprecated)]

use std::fmt;

use super::framed::Fuse;
use codec::Decoder;
use AsyncRead;

use bytes::BytesMut;
use futures::{Async, Poll, Sink, StartSend, Stream};

/// A `Stream` of messages decoded from an `AsyncRead`.
pub struct FramedRead<T, D> {
    inner: FramedRead2<Fuse<T, D>>,
}

pub struct FramedRead2<T> {
    inner: T,
    eof: bool,
    is_readable: bool,
    buffer: BytesMut,
}

const INITIAL_CAPACITY: usize = 8 * 1024;

// ===== impl FramedRead =====

impl<T, D> FramedRead<T, D>
where
    T: AsyncRead,
    D: Decoder,
{
    /// Creates a new `FramedRead` with the given `decoder`.
    pub fn new(inner: T, decoder: D) -> FramedRead<T, D> {
        FramedRead {
            inner: framed_read2(Fuse(inner, decoder)),
        }
    }
}

impl<T, D> FramedRead<T, D> {
    /// Returns a reference to the underlying I/O stream wrapped by
    /// `FramedRead`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_ref(&self) -> &T {
        &self.inner.inner.0
    }

    /// Returns a mutable reference to the underlying I/O stream wrapped by
    /// `FramedRead`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner.inner.0
    }

    /// Consumes the `FramedRead`, returning its underlying I/O stream.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn into_inner(self) -> T {
        self.inner.inner.0
    }

    /// Returns a reference to the underlying decoder.
    pub fn decoder(&self) -> &D {
        &self.inner.inner.1
    }

    /// Returns a mutable reference to the underlying decoder.
    pub fn decoder_mut(&mut self) -> &mut D {
        &mut self.inner.inner.1
    }
}

impl<T, D> Stream for FramedRead<T, D>
where
    T: AsyncRead,
    D: Decoder,
{
    type Item = D::Item;
    type Error = D::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll()
    }
}

impl<T, D> Sink for FramedRead<T, D>
where
    T: Sink,
{
    type SinkItem = T::SinkItem;
    type SinkError = T::SinkError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.inner.inner.0.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.inner.inner.0.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        self.inner.inner.0.close()
    }
}

impl<T, D> fmt::Debug for FramedRead<T, D>
where
    T: fmt::Debug,
    D: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FramedRead")
            .field("inner", &self.inner.inner.0)
            .field("decoder", &self.inner.inner.1)
            .field("eof", &self.inner.eof)
            .field("is_readable", &self.inner.is_readable)
            .field("buffer", &self.inner.buffer)
            .finish()
    }
}

// ===== impl FramedRead2 =====

pub fn framed_read2<T>(inner: T) -> FramedRead2<T> {
    FramedRead2 {
        inner: inner,
        eof: false,
        is_readable: false,
        buffer: BytesMut::with_capacity(INITIAL_CAPACITY),
    }
}

pub fn framed_read2_with_buffer<T>(inner: T, mut buf: BytesMut) -> FramedRead2<T> {
    if buf.capacity() < INITIAL_CAPACITY {
        let bytes_to_reserve = INITIAL_CAPACITY - buf.capacity();
        buf.reserve(bytes_to_reserve);
    }
    FramedRead2 {
        inner: inner,
        eof: false,
        is_readable: buf.len() > 0,
        buffer: buf,
    }
}

impl<T> FramedRead2<T> {
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn into_parts(self) -> (T, BytesMut) {
        (self.inner, self.buffer)
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> Stream for FramedRead2<T>
where
    T: AsyncRead + Decoder,
{
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            // Repeatedly call `decode` or `decode_eof` as long as it is
            // "readable". Readable is defined as not having returned `None`. If
            // the upstream has returned EOF, and the decoder is no longer
            // readable, it can be assumed that the decoder will never become
            // readable again, at which point the stream is terminated.
            if self.is_readable {
                if self.eof {
                    let frame = self.inner.decode_eof(&mut self.buffer)?;
                    return Ok(Async::Ready(frame));
                }

                trace!("attempting to decode a frame");

                if let Some(frame) = self.inner.decode(&mut self.buffer)? {
                    trace!("frame decoded from buffer");
                    return Ok(Async::Ready(Some(frame)));
                }

                self.is_readable = false;
            }

            assert!(!self.eof);

            // Otherwise, try to read more data and try again. Make sure we've
            // got room for at least one byte to read to ensure that we don't
            // get a spurious 0 that looks like EOF
            self.buffer.reserve(1);
            if 0 == try_ready!(self.inner.read_buf(&mut self.buffer)) {
                self.eof = true;
            }

            self.is_readable = true;
        }
    }
}
