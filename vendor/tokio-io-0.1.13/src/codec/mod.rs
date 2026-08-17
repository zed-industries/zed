//! Utilities for encoding and decoding frames.
//!
//! Contains adapters to go from streams of bytes, [`AsyncRead`] and
//! [`AsyncWrite`], to framed streams implementing [`Sink`] and [`Stream`].
//! Framed streams are also known as [transports].
//!
//! [`AsyncRead`]: #
//! [`AsyncWrite`]: #
//! [`Sink`]: #
//! [`Stream`]: #
//! [transports]: #

// tokio_io::codec originally held all codec-related helpers. This is now intended to be in
// tokio_codec instead. However, for backward compatibility, this remains here. When the next major
// breaking change comes, `Encoder` and `Decoder` need to be moved to `tokio_codec`, and the rest
// of this module should be removed.

#![doc(hidden)]
#![allow(deprecated)]

mod bytes_codec;
mod decoder;
mod encoder;
mod lines_codec;

pub use self::bytes_codec::BytesCodec;
pub use self::decoder::Decoder;
pub use self::encoder::Encoder;
pub use self::lines_codec::LinesCodec;

pub use framed::{Framed, FramedParts};
pub use framed_read::FramedRead;
pub use framed_write::FramedWrite;

#[deprecated(since = "0.1.8", note = "Moved to tokio-codec")]
#[doc(hidden)]
pub mod length_delimited {
    //! Frame a stream of bytes based on a length prefix
    //!
    //! Many protocols delimit their frames by prefacing frame data with a
    //! frame head that specifies the length of the frame. The
    //! `length_delimited` module provides utilities for handling the length
    //! based framing. This allows the consumer to work with entire frames
    //! without having to worry about buffering or other framing logic.
    //!
    //! # Getting started
    //!
    //! If implementing a protocol from scratch, using length delimited framing
    //! is an easy way to get started. [`Framed::new()`](length_delimited::Framed::new) will adapt a
    //! full-duplex byte stream with a length delimited framer using default
    //! configuration values.
    //!
    //! ```
    //! use tokio_io::{AsyncRead, AsyncWrite};
    //! use tokio_io::codec::length_delimited;
    //!
    //! fn bind_transport<T: AsyncRead + AsyncWrite>(io: T)
    //!     -> length_delimited::Framed<T>
    //! {
    //!     length_delimited::Framed::new(io)
    //! }
    //! ```
    //!
    //! The returned transport implements `Sink + Stream` for `BytesMut`. It
    //! encodes the frame with a big-endian `u32` header denoting the frame
    //! payload length:
    //!
    //! ```text
    //! +----------+--------------------------------+
    //! | len: u32 |          frame payload         |
    //! +----------+--------------------------------+
    //! ```
    //!
    //! Specifically, given the following:
    //!
    //! ```
    //! # extern crate tokio_io;
    //! # extern crate bytes;
    //! # extern crate futures;
    //! #
    //! use tokio_io::{AsyncRead, AsyncWrite};
    //! use tokio_io::codec::length_delimited;
    //! use bytes::BytesMut;
    //! use futures::{Sink, Future};
    //!
    //! fn write_frame<T: AsyncRead + AsyncWrite>(io: T) {
    //!     let mut transport = length_delimited::Framed::new(io);
    //!     let frame = BytesMut::from("hello world");
    //!
    //!     transport.send(frame).wait().unwrap();
    //! }
    //! #
    //! # pub fn main() {}
    //! ```
    //!
    //! The encoded frame will look like this:
    //!
    //! ```text
    //! +---- len: u32 ----+---- data ----+
    //! | \x00\x00\x00\x0b |  hello world |
    //! +------------------+--------------+
    //! ```
    //!
    //! # Decoding
    //!
    //! [`FramedRead`] adapts an [`AsyncRead`] into a `Stream` of [`BytesMut`],
    //! such that each yielded [`BytesMut`] value contains the contents of an
    //! entire frame. There are many configuration parameters enabling
    //! [`FramedRead`] to handle a wide range of protocols. Here are some
    //! examples that will cover the various options at a high level.
    //!
    //! ## Example 1
    //!
    //! The following will parse a `u16` length field at offset 0, including the
    //! frame head in the yielded `BytesMut`.
    //!
    //! ```
    //! # use tokio_io::AsyncRead;
    //! # use tokio_io::codec::length_delimited;
    //! # fn bind_read<T: AsyncRead>(io: T) {
    //! length_delimited::Builder::new()
    //!     .length_field_offset(0) // default value
    //!     .length_field_length(2)
    //!     .length_adjustment(0)   // default value
    //!     .num_skip(0) // Do not strip frame header
    //!     .new_read(io);
    //! # }
    //! ```
    //!
    //! The following frame will be decoded as such:
    //!
    //! ```text
    //!          INPUT                           DECODED
    //! +-- len ---+--- Payload ---+     +-- len ---+--- Payload ---+
    //! | \x00\x0B |  Hello world  | --> | \x00\x0B |  Hello world  |
    //! +----------+---------------+     +----------+---------------+
    //! ```
    //!
    //! The value of the length field is 11 (`\x0B`) which represents the length
    //! of the payload, `hello world`. By default, [`FramedRead`] assumes that
    //! the length field represents the number of bytes that **follows** the
    //! length field. Thus, the entire frame has a length of 13: 2 bytes for the
    //! frame head + 11 bytes for the payload.
    //!
    //! ## Example 2
    //!
    //! The following will parse a `u16` length field at offset 0, omitting the
    //! frame head in the yielded `BytesMut`.
    //!
    //! ```
    //! # use tokio_io::AsyncRead;
    //! # use tokio_io::codec::length_delimited;
    //! # fn bind_read<T: AsyncRead>(io: T) {
    //! length_delimited::Builder::new()
    //!     .length_field_offset(0) // default value
    //!     .length_field_length(2)
    //!     .length_adjustment(0)   // default value
    //!     // `num_skip` is not needed, the default is to skip
    //!     .new_read(io);
    //! # }
    //! ```
    //!
    //! The following frame will be decoded as such:
    //!
    //! ```text
    //!          INPUT                        DECODED
    //! +-- len ---+--- Payload ---+     +--- Payload ---+
    //! | \x00\x0B |  Hello world  | --> |  Hello world  |
    //! +----------+---------------+     +---------------+
    //! ```
    //!
    //! This is similar to the first example, the only difference is that the
    //! frame head is **not** included in the yielded `BytesMut` value.
    //!
    //! ## Example 3
    //!
    //! The following will parse a `u16` length field at offset 0, including the
    //! frame head in the yielded `BytesMut`. In this case, the length field
    //! **includes** the frame head length.
    //!
    //! ```
    //! # use tokio_io::AsyncRead;
    //! # use tokio_io::codec::length_delimited;
    //! # fn bind_read<T: AsyncRead>(io: T) {
    //! length_delimited::Builder::new()
    //!     .length_field_offset(0) // default value
    //!     .length_field_length(2)
    //!     .length_adjustment(-2)  // size of head
    //!     .num_skip(0)
    //!     .new_read(io);
    //! # }
    //! ```
    //!
    //! The following frame will be decoded as such:
    //!
    //! ```text
    //!          INPUT                           DECODED
    //! +-- len ---+--- Payload ---+     +-- len ---+--- Payload ---+
    //! | \x00\x0D |  Hello world  | --> | \x00\x0D |  Hello world  |
    //! +----------+---------------+     +----------+---------------+
    //! ```
    //!
    //! In most cases, the length field represents the length of the payload
    //! only, as shown in the previous examples. However, in some protocols the
    //! length field represents the length of the whole frame, including the
    //! head. In such cases, we specify a negative `length_adjustment` to adjust
    //! the value provided in the frame head to represent the payload length.
    //!
    //! ## Example 4
    //!
    //! The following will parse a 3 byte length field at offset 0 in a 5 byte
    //! frame head, including the frame head in the yielded `BytesMut`.
    //!
    //! ```
    //! # use tokio_io::AsyncRead;
    //! # use tokio_io::codec::length_delimited;
    //! # fn bind_read<T: AsyncRead>(io: T) {
    //! length_delimited::Builder::new()
    //!     .length_field_offset(0) // default value
    //!     .length_field_length(3)
    //!     .length_adjustment(2)  // remaining head
    //!     .num_skip(0)
    //!     .new_read(io);
    //! # }
    //! ```
    //!
    //! The following frame will be decoded as such:
    //!
    //! ```text
    //!                  INPUT
    //! +---- len -----+- head -+--- Payload ---+
    //! | \x00\x00\x0B | \xCAFE |  Hello world  |
    //! +--------------+--------+---------------+
    //!
    //!                  DECODED
    //! +---- len -----+- head -+--- Payload ---+
    //! | \x00\x00\x0B | \xCAFE |  Hello world  |
    //! +--------------+--------+---------------+
    //! ```
    //!
    //! A more advanced example that shows a case where there is extra frame
    //! head data between the length field and the payload. In such cases, it is
    //! usually desirable to include the frame head as part of the yielded
    //! `BytesMut`. This lets consumers of the length delimited framer to
    //! process the frame head as needed.
    //!
    //! The positive `length_adjustment` value lets `FramedRead` factor in the
    //! additional head into the frame length calculation.
    //!
    //! ## Example 5
    //!
    //! The following will parse a `u16` length field at offset 1 of a 4 byte
    //! frame head. The first byte and the length field will be omitted from the
    //! yielded `BytesMut`, but the trailing 2 bytes of the frame head will be
    //! included.
    //!
    //! ```
    //! # use tokio_io::AsyncRead;
    //! # use tokio_io::codec::length_delimited;
    //! # fn bind_read<T: AsyncRead>(io: T) {
    //! length_delimited::Builder::new()
    //!     .length_field_offset(1) // length of hdr1
    //!     .length_field_length(2)
    //!     .length_adjustment(1)  // length of hdr2
    //!     .num_skip(3) // length of hdr1 + LEN
    //!     .new_read(io);
    //! # }
    //! ```
    //!
    //! The following frame will be decoded as such:
    //!
    //! ```text
    //!                  INPUT
    //! +- hdr1 -+-- len ---+- hdr2 -+--- Payload ---+
    //! |  \xCA  | \x00\x0B |  \xFE  |  Hello world  |
    //! +--------+----------+--------+---------------+
    //!
    //!          DECODED
    //! +- hdr2 -+--- Payload ---+
    //! |  \xFE  |  Hello world  |
    //! +--------+---------------+
    //! ```
    //!
    //! The length field is situated in the middle of the frame head. In this
    //! case, the first byte in the frame head could be a version or some other
    //! identifier that is not needed for processing. On the other hand, the
    //! second half of the head is needed.
    //!
    //! `length_field_offset` indicates how many bytes to skip before starting
    //! to read the length field.  `length_adjustment` is the number of bytes to
    //! skip starting at the end of the length field. In this case, it is the
    //! second half of the head.
    //!
    //! ## Example 6
    //!
    //! The following will parse a `u16` length field at offset 1 of a 4 byte
    //! frame head. The first byte and the length field will be omitted from the
    //! yielded `BytesMut`, but the trailing 2 bytes of the frame head will be
    //! included. In this case, the length field **includes** the frame head
    //! length.
    //!
    //! ```
    //! # use tokio_io::AsyncRead;
    //! # use tokio_io::codec::length_delimited;
    //! # fn bind_read<T: AsyncRead>(io: T) {
    //! length_delimited::Builder::new()
    //!     .length_field_offset(1) // length of hdr1
    //!     .length_field_length(2)
    //!     .length_adjustment(-3)  // length of hdr1 + LEN, negative
    //!     .num_skip(3)
    //!     .new_read(io);
    //! # }
    //! ```
    //!
    //! The following frame will be decoded as such:
    //!
    //! ```text
    //!                  INPUT
    //! +- hdr1 -+-- len ---+- hdr2 -+--- Payload ---+
    //! |  \xCA  | \x00\x0F |  \xFE  |  Hello world  |
    //! +--------+----------+--------+---------------+
    //!
    //!          DECODED
    //! +- hdr2 -+--- Payload ---+
    //! |  \xFE  |  Hello world  |
    //! +--------+---------------+
    //! ```
    //!
    //! Similar to the example above, the difference is that the length field
    //! represents the length of the entire frame instead of just the payload.
    //! The length of `hdr1` and `len` must be counted in `length_adjustment`.
    //! Note that the length of `hdr2` does **not** need to be explicitly set
    //! anywhere because it already is factored into the total frame length that
    //! is read from the byte stream.
    //!
    //! # Encoding
    //!
    //! [`FramedWrite`] adapts an [`AsyncWrite`] into a `Sink` of [`BytesMut`],
    //! such that each submitted [`BytesMut`] is prefaced by a length field.
    //! There are fewer configuration options than [`FramedRead`]. Given
    //! protocols that have more complex frame heads, an encoder should probably
    //! be written by hand using [`Encoder`].
    //!
    //! Here is a simple example, given a `FramedWrite` with the following
    //! configuration:
    //!
    //! ```
    //! # extern crate tokio_io;
    //! # extern crate bytes;
    //! # use tokio_io::AsyncWrite;
    //! # use tokio_io::codec::length_delimited;
    //! # use bytes::BytesMut;
    //! # fn write_frame<T: AsyncWrite>(io: T) {
    //! # let _: length_delimited::FramedWrite<T, BytesMut> =
    //! length_delimited::Builder::new()
    //!     .length_field_length(2)
    //!     .new_write(io);
    //! # }
    //! # pub fn main() {}
    //! ```
    //!
    //! A payload of `hello world` will be encoded as:
    //!
    //! ```text
    //! +- len: u16 -+---- data ----+
    //! |  \x00\x0b  |  hello world |
    //! +------------+--------------+
    //! ```
    //!
    //! [`FramedRead`]: struct.FramedRead.html
    //! [`FramedWrite`]: struct.FramedWrite.html
    //! [`AsyncRead`]: ../../trait.AsyncRead.html
    //! [`AsyncWrite`]: ../../trait.AsyncWrite.html
    //! [`Encoder`]: ../trait.Encoder.html
    //! [`BytesMut`]: https://docs.rs/bytes/0.4/bytes/struct.BytesMut.html

    pub use length_delimited::*;
}
