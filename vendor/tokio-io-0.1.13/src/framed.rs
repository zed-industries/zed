#![allow(deprecated)]

use std::fmt;
use std::io::{self, Read, Write};

use codec::{Decoder, Encoder};
use framed_read::{framed_read2, framed_read2_with_buffer, FramedRead2};
use framed_write::{framed_write2, framed_write2_with_buffer, FramedWrite2};
use {AsyncRead, AsyncWrite};

use bytes::BytesMut;
use futures::{Poll, Sink, StartSend, Stream};

/// A unified `Stream` and `Sink` interface to an underlying I/O object, using
/// the `Encoder` and `Decoder` traits to encode and decode frames.
///
/// You can create a `Framed` instance by using the `AsyncRead::framed` adapter.
#[deprecated(since = "0.1.7", note = "Moved to tokio-codec")]
#[doc(hidden)]
pub struct Framed<T, U> {
    inner: FramedRead2<FramedWrite2<Fuse<T, U>>>,
}

#[deprecated(since = "0.1.7", note = "Moved to tokio-codec")]
#[doc(hidden)]
pub struct Fuse<T, U>(pub T, pub U);

pub fn framed<T, U>(inner: T, codec: U) -> Framed<T, U>
where
    T: AsyncRead + AsyncWrite,
    U: Decoder + Encoder,
{
    Framed {
        inner: framed_read2(framed_write2(Fuse(inner, codec))),
    }
}

impl<T, U> Framed<T, U> {
    /// Provides a `Stream` and `Sink` interface for reading and writing to this
    /// `Io` object, using `Decode` and `Encode` to read and write the raw data.
    ///
    /// Raw I/O objects work with byte sequences, but higher-level code usually
    /// wants to batch these into meaningful chunks, called "frames". This
    /// method layers framing on top of an I/O object, by using the `Codec`
    /// traits to handle encoding and decoding of messages frames. Note that
    /// the incoming and outgoing frame types may be distinct.
    ///
    /// This function returns a *single* object that is both `Stream` and
    /// `Sink`; grouping this into a single object is often useful for layering
    /// things like gzip or TLS, which require both read and write access to the
    /// underlying object.
    ///
    /// This objects takes a stream and a readbuffer and a writebuffer. These field
    /// can be obtained from an existing `Framed` with the `into_parts` method.
    ///
    /// If you want to work more directly with the streams and sink, consider
    /// calling `split` on the `Framed` returned by this method, which will
    /// break them into separate objects, allowing them to interact more easily.
    pub fn from_parts(parts: FramedParts<T>, codec: U) -> Framed<T, U> {
        Framed {
            inner: framed_read2_with_buffer(
                framed_write2_with_buffer(Fuse(parts.inner, codec), parts.writebuf),
                parts.readbuf,
            ),
        }
    }

    /// Returns a reference to the underlying I/O stream wrapped by
    /// `Frame`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_ref(&self) -> &T {
        &self.inner.get_ref().get_ref().0
    }

    /// Returns a mutable reference to the underlying I/O stream wrapped by
    /// `Frame`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner.get_mut().get_mut().0
    }

    /// Consumes the `Frame`, returning its underlying I/O stream.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn into_inner(self) -> T {
        self.inner.into_inner().into_inner().0
    }

    /// Consumes the `Frame`, returning its underlying I/O stream and the buffer
    /// with unprocessed data.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn into_parts(self) -> FramedParts<T> {
        let (inner, readbuf) = self.inner.into_parts();
        let (inner, writebuf) = inner.into_parts();
        FramedParts {
            inner: inner.0,
            readbuf: readbuf,
            writebuf: writebuf,
        }
    }

    /// Consumes the `Frame`, returning its underlying I/O stream and the buffer
    /// with unprocessed data, and also the current codec state.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    ///
    /// Note that this function will be removed once the codec has been
    /// integrated into `FramedParts` in a new version (see
    /// [#53](https://github.com/tokio-rs/tokio-io/pull/53)).
    pub fn into_parts_and_codec(self) -> (FramedParts<T>, U) {
        let (inner, readbuf) = self.inner.into_parts();
        let (inner, writebuf) = inner.into_parts();
        (
            FramedParts {
                inner: inner.0,
                readbuf: readbuf,
                writebuf: writebuf,
            },
            inner.1,
        )
    }
}

impl<T, U> Stream for Framed<T, U>
where
    T: AsyncRead,
    U: Decoder,
{
    type Item = U::Item;
    type Error = U::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll()
    }
}

impl<T, U> Sink for Framed<T, U>
where
    T: AsyncWrite,
    U: Encoder,
    U::Error: From<io::Error>,
{
    type SinkItem = U::Item;
    type SinkError = U::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.inner.get_mut().start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.inner.get_mut().poll_complete()
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        self.inner.get_mut().close()
    }
}

impl<T, U> fmt::Debug for Framed<T, U>
where
    T: fmt::Debug,
    U: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Framed")
            .field("io", &self.inner.get_ref().get_ref().0)
            .field("codec", &self.inner.get_ref().get_ref().1)
            .finish()
    }
}

// ===== impl Fuse =====

impl<T: Read, U> Read for Fuse<T, U> {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        self.0.read(dst)
    }
}

impl<T: AsyncRead, U> AsyncRead for Fuse<T, U> {
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        self.0.prepare_uninitialized_buffer(buf)
    }
}

impl<T: Write, U> Write for Fuse<T, U> {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        self.0.write(src)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl<T: AsyncWrite, U> AsyncWrite for Fuse<T, U> {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.0.shutdown()
    }
}

impl<T, U: Decoder> Decoder for Fuse<T, U> {
    type Item = U::Item;
    type Error = U::Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.1.decode(buffer)
    }

    fn decode_eof(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.1.decode_eof(buffer)
    }
}

impl<T, U: Encoder> Encoder for Fuse<T, U> {
    type Item = U::Item;
    type Error = U::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.1.encode(item, dst)
    }
}

/// `FramedParts` contains an export of the data of a Framed transport.
/// It can be used to construct a new `Framed` with a different codec.
/// It contains all current buffers and the inner transport.
#[derive(Debug)]
pub struct FramedParts<T> {
    /// The inner transport used to read bytes to and write bytes to
    pub inner: T,
    /// The buffer with read but unprocessed data.
    pub readbuf: BytesMut,
    /// A buffer with unprocessed data which are not written yet.
    pub writebuf: BytesMut,
}
