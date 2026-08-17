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

#![deny(missing_docs, missing_debug_implementations)]
#![doc(hidden, html_root_url = "https://docs.rs/tokio-codec/0.1.0")]

// _tokio_codec are the items that belong in the `tokio_codec` crate. However, because we need to
// maintain backward compatibility until the next major breaking change, they are defined here.
// When the next breaking change comes, they should be moved to the `tokio_codec` crate and become
// independent.
//
// The primary reason we can't move these to `tokio-codec` now is because, again for backward
// compatibility reasons, we need to keep `Decoder` and `Encoder` in tokio_io::codec. And `Decoder`
// and `Encoder` needs to reference `Framed`. So they all still need to still be in the same
// module.

mod decoder;
mod encoder;
mod framed;
mod framed_read;
mod framed_write;

pub use self::decoder::Decoder;
pub use self::encoder::Encoder;
pub use self::framed::{Framed, FramedParts};
pub use self::framed_read::FramedRead;
pub use self::framed_write::FramedWrite;
