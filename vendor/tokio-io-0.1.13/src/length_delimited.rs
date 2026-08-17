#![allow(deprecated)]

use {codec, AsyncRead, AsyncWrite};

use bytes::buf::Chain;
use bytes::{Buf, BufMut, BytesMut, IntoBuf};

use futures::{Async, AsyncSink, Poll, Sink, StartSend, Stream};

use std::error::Error as StdError;
use std::io::{self, Cursor};
use std::{cmp, fmt};

/// Configure length delimited `FramedRead`, `FramedWrite`, and `Framed` values.
///
/// `Builder` enables constructing configured length delimited framers. Note
/// that not all configuration settings apply to both encoding and decoding. See
/// the documentation for specific methods for more detail.
#[deprecated(since = "0.1.8", note = "Moved to tokio-codec")]
#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct Builder {
    // Maximum frame length
    max_frame_len: usize,

    // Number of bytes representing the field length
    length_field_len: usize,

    // Number of bytes in the header before the length field
    length_field_offset: usize,

    // Adjust the length specified in the header field by this amount
    length_adjustment: isize,

    // Total number of bytes to skip before reading the payload, if not set,
    // `length_field_len + length_field_offset`
    num_skip: Option<usize>,

    // Length field byte order (little or big endian)
    length_field_is_big_endian: bool,
}

/// Adapts a byte stream into a unified `Stream` and `Sink` that works over
/// entire frame values.
///
/// See [module level] documentation for more detail.
///
/// [module level]: index.html
#[deprecated(since = "0.1.8", note = "Moved to tokio-codec")]
#[doc(hidden)]
pub struct Framed<T, B: IntoBuf = BytesMut> {
    inner: FramedRead<FramedWrite<T, B>>,
}

/// Adapts a byte stream to a `Stream` yielding entire frame values.
///
/// See [module level] documentation for more detail.
///
/// [module level]: index.html
#[deprecated(since = "0.1.8", note = "Moved to tokio-codec")]
#[doc(hidden)]
#[derive(Debug)]
pub struct FramedRead<T> {
    inner: codec::FramedRead<T, Decoder>,
}

/// An error when the number of bytes read is more than max frame length.
#[deprecated(since = "0.1.8", note = "Moved to tokio-codec")]
#[doc(hidden)]
pub struct FrameTooBig {
    _priv: (),
}

#[derive(Debug)]
struct Decoder {
    // Configuration values
    builder: Builder,

    // Read state
    state: DecodeState,
}

#[derive(Debug, Clone, Copy)]
enum DecodeState {
    Head,
    Data(usize),
}

/// Adapts a byte stream to a `Sink` accepting entire frame values.
///
/// See [module level] documentation for more detail.
///
/// [module level]: index.html
#[deprecated(since = "0.1.8", note = "Moved to tokio-codec")]
#[doc(hidden)]
pub struct FramedWrite<T, B: IntoBuf = BytesMut> {
    // I/O type
    inner: T,

    // Configuration values
    builder: Builder,

    // Current frame being written
    frame: Option<Chain<Cursor<BytesMut>, B::Buf>>,
}

// ===== impl Framed =====

impl<T: AsyncRead + AsyncWrite, B: IntoBuf> Framed<T, B> {
    /// Creates a new `Framed` with default configuration values.
    pub fn new(inner: T) -> Framed<T, B> {
        Builder::new().new_framed(inner)
    }
}

impl<T, B: IntoBuf> Framed<T, B> {
    /// Returns a reference to the underlying I/O stream wrapped by `Framed`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref().get_ref()
    }

    /// Returns a mutable reference to the underlying I/O stream wrapped by
    /// `Framed`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut().get_mut()
    }

    /// Consumes the `Framed`, returning its underlying I/O stream.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn into_inner(self) -> T {
        self.inner.into_inner().into_inner()
    }
}

impl<T: AsyncRead, B: IntoBuf> Stream for Framed<T, B> {
    type Item = BytesMut;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<BytesMut>, io::Error> {
        self.inner.poll()
    }
}

impl<T: AsyncWrite, B: IntoBuf> Sink for Framed<T, B> {
    type SinkItem = B;
    type SinkError = io::Error;

    fn start_send(&mut self, item: B) -> StartSend<B, io::Error> {
        self.inner.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), io::Error> {
        self.inner.poll_complete()
    }

    fn close(&mut self) -> Poll<(), io::Error> {
        self.inner.close()
    }
}

impl<T, B: IntoBuf> fmt::Debug for Framed<T, B>
where
    T: fmt::Debug,
    B::Buf: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Framed")
            .field("inner", &self.inner)
            .finish()
    }
}

// ===== impl FramedRead =====

impl<T: AsyncRead> FramedRead<T> {
    /// Creates a new `FramedRead` with default configuration values.
    pub fn new(inner: T) -> FramedRead<T> {
        Builder::new().new_read(inner)
    }
}

impl<T> FramedRead<T> {
    /// Returns the current max frame setting
    ///
    /// This is the largest size this codec will accept from the wire. Larger
    /// frames will be rejected.
    pub fn max_frame_length(&self) -> usize {
        self.inner.decoder().builder.max_frame_len
    }

    /// Updates the max frame setting.
    ///
    /// The change takes effect the next time a frame is decoded. In other
    /// words, if a frame is currently in process of being decoded with a frame
    /// size greater than `val` but less than the max frame length in effect
    /// before calling this function, then the frame will be allowed.
    pub fn set_max_frame_length(&mut self, val: usize) {
        self.inner.decoder_mut().builder.max_frame_length(val);
    }

    /// Returns a reference to the underlying I/O stream wrapped by `FramedRead`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref()
    }

    /// Returns a mutable reference to the underlying I/O stream wrapped by
    /// `FramedRead`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Consumes the `FramedRead`, returning its underlying I/O stream.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T: AsyncRead> Stream for FramedRead<T> {
    type Item = BytesMut;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<BytesMut>, io::Error> {
        self.inner.poll()
    }
}

impl<T: Sink> Sink for FramedRead<T> {
    type SinkItem = T::SinkItem;
    type SinkError = T::SinkError;

    fn start_send(&mut self, item: T::SinkItem) -> StartSend<T::SinkItem, T::SinkError> {
        self.inner.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), T::SinkError> {
        self.inner.poll_complete()
    }

    fn close(&mut self) -> Poll<(), T::SinkError> {
        self.inner.close()
    }
}

impl<T: io::Write> io::Write for FramedRead<T> {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        self.inner.get_mut().write(src)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.get_mut().flush()
    }
}

impl<T: AsyncWrite> AsyncWrite for FramedRead<T> {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.inner.get_mut().shutdown()
    }

    fn write_buf<B: Buf>(&mut self, buf: &mut B) -> Poll<usize, io::Error> {
        self.inner.get_mut().write_buf(buf)
    }
}

// ===== impl Decoder ======

impl Decoder {
    fn decode_head(&mut self, src: &mut BytesMut) -> io::Result<Option<usize>> {
        let head_len = self.builder.num_head_bytes();
        let field_len = self.builder.length_field_len;

        if src.len() < head_len {
            // Not enough data
            return Ok(None);
        }

        let n = {
            let mut src = Cursor::new(&mut *src);

            // Skip the required bytes
            src.advance(self.builder.length_field_offset);

            // match endianess
            let n = if self.builder.length_field_is_big_endian {
                src.get_uint_be(field_len)
            } else {
                src.get_uint_le(field_len)
            };

            if n > self.builder.max_frame_len as u64 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    FrameTooBig { _priv: () },
                ));
            }

            // The check above ensures there is no overflow
            let n = n as usize;

            // Adjust `n` with bounds checking
            let n = if self.builder.length_adjustment < 0 {
                n.checked_sub(-self.builder.length_adjustment as usize)
            } else {
                n.checked_add(self.builder.length_adjustment as usize)
            };

            // Error handling
            match n {
                Some(n) => n,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "provided length would overflow after adjustment",
                    ));
                }
            }
        };

        let num_skip = self.builder.get_num_skip();

        if num_skip > 0 {
            let _ = src.split_to(num_skip);
        }

        // Ensure that the buffer has enough space to read the incoming
        // payload
        src.reserve(n);

        return Ok(Some(n));
    }

    fn decode_data(&self, n: usize, src: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        // At this point, the buffer has already had the required capacity
        // reserved. All there is to do is read.
        if src.len() < n {
            return Ok(None);
        }

        Ok(Some(src.split_to(n)))
    }
}

impl codec::Decoder for Decoder {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        let n = match self.state {
            DecodeState::Head => match self.decode_head(src)? {
                Some(n) => {
                    self.state = DecodeState::Data(n);
                    n
                }
                None => return Ok(None),
            },
            DecodeState::Data(n) => n,
        };

        match self.decode_data(n, src)? {
            Some(data) => {
                // Update the decode state
                self.state = DecodeState::Head;

                // Make sure the buffer has enough space to read the next head
                src.reserve(self.builder.num_head_bytes());

                Ok(Some(data))
            }
            None => Ok(None),
        }
    }
}

// ===== impl FramedWrite =====

impl<T: AsyncWrite, B: IntoBuf> FramedWrite<T, B> {
    /// Creates a new `FramedWrite` with default configuration values.
    pub fn new(inner: T) -> FramedWrite<T, B> {
        Builder::new().new_write(inner)
    }
}

impl<T, B: IntoBuf> FramedWrite<T, B> {
    /// Returns the current max frame setting
    ///
    /// This is the largest size this codec will write to the wire. Larger
    /// frames will be rejected.
    pub fn max_frame_length(&self) -> usize {
        self.builder.max_frame_len
    }

    /// Updates the max frame setting.
    ///
    /// The change takes effect the next time a frame is encoded. In other
    /// words, if a frame is currently in process of being encoded with a frame
    /// size greater than `val` but less than the max frame length in effect
    /// before calling this function, then the frame will be allowed.
    pub fn set_max_frame_length(&mut self, val: usize) {
        self.builder.max_frame_length(val);
    }

    /// Returns a reference to the underlying I/O stream wrapped by
    /// `FramedWrite`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Returns a mutable reference to the underlying I/O stream wrapped by
    /// `FramedWrite`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Consumes the `FramedWrite`, returning its underlying I/O stream.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: AsyncWrite, B: IntoBuf> FramedWrite<T, B> {
    // If there is a buffered frame, try to write it to `T`
    fn do_write(&mut self) -> Poll<(), io::Error> {
        if self.frame.is_none() {
            return Ok(Async::Ready(()));
        }

        loop {
            let frame = self.frame.as_mut().unwrap();
            if try_ready!(self.inner.write_buf(frame)) == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write frame to transport",
                ));
            }

            if !frame.has_remaining() {
                break;
            }
        }

        self.frame = None;

        Ok(Async::Ready(()))
    }

    fn set_frame(&mut self, buf: B::Buf) -> io::Result<()> {
        let mut head = BytesMut::with_capacity(8);
        let n = buf.remaining();

        if n > self.builder.max_frame_len {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                FrameTooBig { _priv: () },
            ));
        }

        // Adjust `n` with bounds checking
        let n = if self.builder.length_adjustment < 0 {
            n.checked_add(-self.builder.length_adjustment as usize)
        } else {
            n.checked_sub(self.builder.length_adjustment as usize)
        };

        // Error handling
        let n = match n {
            Some(n) => n,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "provided length would overflow after adjustment",
                ));
            }
        };

        if self.builder.length_field_is_big_endian {
            head.put_uint_be(n as u64, self.builder.length_field_len);
        } else {
            head.put_uint_le(n as u64, self.builder.length_field_len);
        }

        debug_assert!(self.frame.is_none());

        self.frame = Some(head.into_buf().chain(buf));

        Ok(())
    }
}

impl<T: AsyncWrite, B: IntoBuf> Sink for FramedWrite<T, B> {
    type SinkItem = B;
    type SinkError = io::Error;

    fn start_send(&mut self, item: B) -> StartSend<B, io::Error> {
        if !self.do_write()?.is_ready() {
            return Ok(AsyncSink::NotReady(item));
        }

        self.set_frame(item.into_buf())?;

        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), io::Error> {
        // Write any buffered frame to T
        try_ready!(self.do_write());

        // Try flushing the underlying IO
        try_ready!(self.inner.poll_flush());

        return Ok(Async::Ready(()));
    }

    fn close(&mut self) -> Poll<(), io::Error> {
        try_ready!(self.poll_complete());
        self.inner.shutdown()
    }
}

impl<T: Stream, B: IntoBuf> Stream for FramedWrite<T, B> {
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<T::Item>, T::Error> {
        self.inner.poll()
    }
}

impl<T: io::Read, B: IntoBuf> io::Read for FramedWrite<T, B> {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        self.get_mut().read(dst)
    }
}

impl<T: AsyncRead, U: IntoBuf> AsyncRead for FramedWrite<T, U> {
    fn read_buf<B: BufMut>(&mut self, buf: &mut B) -> Poll<usize, io::Error> {
        self.get_mut().read_buf(buf)
    }

    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        self.get_ref().prepare_uninitialized_buffer(buf)
    }
}

impl<T, B: IntoBuf> fmt::Debug for FramedWrite<T, B>
where
    T: fmt::Debug,
    B::Buf: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FramedWrite")
            .field("inner", &self.inner)
            .field("builder", &self.builder)
            .field("frame", &self.frame)
            .finish()
    }
}

// ===== impl Builder =====

impl Builder {
    /// Creates a new length delimited framer builder with default configuration
    /// values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio_io::AsyncRead;
    /// use tokio_io::codec::length_delimited::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .length_field_offset(0)
    ///     .length_field_length(2)
    ///     .length_adjustment(0)
    ///     .num_skip(0)
    ///     .new_read(io);
    /// # }
    /// ```
    pub fn new() -> Builder {
        Builder {
            // Default max frame length of 8MB
            max_frame_len: 8 * 1_024 * 1_024,

            // Default byte length of 4
            length_field_len: 4,

            // Default to the header field being at the start of the header.
            length_field_offset: 0,

            length_adjustment: 0,

            // Total number of bytes to skip before reading the payload, if not set,
            // `length_field_len + length_field_offset`
            num_skip: None,

            // Default to reading the length field in network (big) endian.
            length_field_is_big_endian: true,
        }
    }

    /// Read the length field as a big endian integer
    ///
    /// This is the default setting.
    ///
    /// This configuration option applies to both encoding and decoding.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio_io::AsyncRead;
    /// use tokio_io::codec::length_delimited::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .big_endian()
    ///     .new_read(io);
    /// # }
    /// ```
    pub fn big_endian(&mut self) -> &mut Self {
        self.length_field_is_big_endian = true;
        self
    }

    /// Read the length field as a little endian integer
    ///
    /// The default setting is big endian.
    ///
    /// This configuration option applies to both encoding and decoding.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio_io::AsyncRead;
    /// use tokio_io::codec::length_delimited::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .little_endian()
    ///     .new_read(io);
    /// # }
    /// ```
    pub fn little_endian(&mut self) -> &mut Self {
        self.length_field_is_big_endian = false;
        self
    }

    /// Read the length field as a native endian integer
    ///
    /// The default setting is big endian.
    ///
    /// This configuration option applies to both encoding and decoding.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio_io::AsyncRead;
    /// use tokio_io::codec::length_delimited::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .native_endian()
    ///     .new_read(io);
    /// # }
    /// ```
    pub fn native_endian(&mut self) -> &mut Self {
        if cfg!(target_endian = "big") {
            self.big_endian()
        } else {
            self.little_endian()
        }
    }

    /// Sets the max frame length
    ///
    /// This configuration option applies to both encoding and decoding. The
    /// default value is 8MB.
    ///
    /// When decoding, the length field read from the byte stream is checked
    /// against this setting **before** any adjustments are applied. When
    /// encoding, the length of the submitted payload is checked against this
    /// setting.
    ///
    /// When frames exceed the max length, an `io::Error` with the custom value
    /// of the `FrameTooBig` type will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio_io::AsyncRead;
    /// use tokio_io::codec::length_delimited::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .max_frame_length(8 * 1024)
    ///     .new_read(io);
    /// # }
    /// ```
    pub fn max_frame_length(&mut self, val: usize) -> &mut Self {
        self.max_frame_len = val;
        self
    }

    /// Sets the number of bytes used to represent the length field
    ///
    /// The default value is `4`. The max value is `8`.
    ///
    /// This configuration option applies to both encoding and decoding.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio_io::AsyncRead;
    /// use tokio_io::codec::length_delimited::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .length_field_length(4)
    ///     .new_read(io);
    /// # }
    /// ```
    pub fn length_field_length(&mut self, val: usize) -> &mut Self {
        assert!(val > 0 && val <= 8, "invalid length field length");
        self.length_field_len = val;
        self
    }

    /// Sets the number of bytes in the header before the length field
    ///
    /// This configuration option only applies to decoding.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio_io::AsyncRead;
    /// use tokio_io::codec::length_delimited::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .length_field_offset(1)
    ///     .new_read(io);
    /// # }
    /// ```
    pub fn length_field_offset(&mut self, val: usize) -> &mut Self {
        self.length_field_offset = val;
        self
    }

    /// Delta between the payload length specified in the header and the real
    /// payload length
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio_io::AsyncRead;
    /// use tokio_io::codec::length_delimited::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .length_adjustment(-2)
    ///     .new_read(io);
    /// # }
    /// ```
    pub fn length_adjustment(&mut self, val: isize) -> &mut Self {
        self.length_adjustment = val;
        self
    }

    /// Sets the number of bytes to skip before reading the payload
    ///
    /// Default value is `length_field_len + length_field_offset`
    ///
    /// This configuration option only applies to decoding
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio_io::AsyncRead;
    /// use tokio_io::codec::length_delimited::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .num_skip(4)
    ///     .new_read(io);
    /// # }
    /// ```
    pub fn num_skip(&mut self, val: usize) -> &mut Self {
        self.num_skip = Some(val);
        self
    }

    /// Create a configured length delimited `FramedRead`
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio_io::AsyncRead;
    /// use tokio_io::codec::length_delimited::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .length_field_offset(0)
    ///     .length_field_length(2)
    ///     .length_adjustment(0)
    ///     .num_skip(0)
    ///     .new_read(io);
    /// # }
    /// ```
    pub fn new_read<T>(&self, upstream: T) -> FramedRead<T>
    where
        T: AsyncRead,
    {
        FramedRead {
            inner: codec::FramedRead::new(
                upstream,
                Decoder {
                    builder: *self,
                    state: DecodeState::Head,
                },
            ),
        }
    }

    /// Create a configured length delimited `FramedWrite`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tokio_io;
    /// # extern crate bytes;
    /// # use tokio_io::AsyncWrite;
    /// # use tokio_io::codec::length_delimited;
    /// # use bytes::BytesMut;
    /// # fn write_frame<T: AsyncWrite>(io: T) {
    /// # let _: length_delimited::FramedWrite<T, BytesMut> =
    /// length_delimited::Builder::new()
    ///     .length_field_length(2)
    ///     .new_write(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn new_write<T, B>(&self, inner: T) -> FramedWrite<T, B>
    where
        T: AsyncWrite,
        B: IntoBuf,
    {
        FramedWrite {
            inner: inner,
            builder: *self,
            frame: None,
        }
    }

    /// Create a configured length delimited `Framed`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tokio_io;
    /// # extern crate bytes;
    /// # use tokio_io::{AsyncRead, AsyncWrite};
    /// # use tokio_io::codec::length_delimited;
    /// # use bytes::BytesMut;
    /// # fn write_frame<T: AsyncRead + AsyncWrite>(io: T) {
    /// # let _: length_delimited::Framed<T, BytesMut> =
    /// length_delimited::Builder::new()
    ///     .length_field_length(2)
    ///     .new_framed(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn new_framed<T, B>(&self, inner: T) -> Framed<T, B>
    where
        T: AsyncRead + AsyncWrite,
        B: IntoBuf,
    {
        let inner = self.new_read(self.new_write(inner));
        Framed { inner: inner }
    }

    fn num_head_bytes(&self) -> usize {
        let num = self.length_field_offset + self.length_field_len;
        cmp::max(num, self.num_skip.unwrap_or(0))
    }

    fn get_num_skip(&self) -> usize {
        self.num_skip
            .unwrap_or(self.length_field_offset + self.length_field_len)
    }
}

// ===== impl FrameTooBig =====

impl fmt::Debug for FrameTooBig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FrameTooBig").finish()
    }
}

impl fmt::Display for FrameTooBig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for FrameTooBig {
    fn description(&self) -> &str {
        "frame size too big"
    }
}
