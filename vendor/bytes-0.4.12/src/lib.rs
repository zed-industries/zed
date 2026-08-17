//! Provides abstractions for working with bytes.
//!
//! The `bytes` crate provides an efficient byte buffer structure
//! ([`Bytes`](struct.Bytes.html)) and traits for working with buffer
//! implementations ([`Buf`], [`BufMut`]).
//!
//! [`Buf`]: trait.Buf.html
//! [`BufMut`]: trait.BufMut.html
//!
//! # `Bytes`
//!
//! `Bytes` is an efficient container for storing and operating on continguous
//! slices of memory. It is intended for use primarily in networking code, but
//! could have applications elsewhere as well.
//!
//! `Bytes` values facilitate zero-copy network programming by allowing multiple
//! `Bytes` objects to point to the same underlying memory. This is managed by
//! using a reference count to track when the memory is no longer needed and can
//! be freed.
//!
//! A `Bytes` handle can be created directly from an existing byte store (such as `&[u8]`
//! or `Vec<u8>`), but usually a `BytesMut` is used first and written to. For
//! example:
//!
//! ```rust
//! use bytes::{BytesMut, BufMut, BigEndian};
//!
//! let mut buf = BytesMut::with_capacity(1024);
//! buf.put(&b"hello world"[..]);
//! buf.put_u16::<BigEndian>(1234);
//!
//! let a = buf.take();
//! assert_eq!(a, b"hello world\x04\xD2"[..]);
//!
//! buf.put(&b"goodbye world"[..]);
//!
//! let b = buf.take();
//! assert_eq!(b, b"goodbye world"[..]);
//!
//! assert_eq!(buf.capacity(), 998);
//! ```
//!
//! In the above example, only a single buffer of 1024 is allocated. The handles
//! `a` and `b` will share the underlying buffer and maintain indices tracking
//! the view into the buffer represented by the handle.
//!
//! See the [struct docs] for more details.
//!
//! [struct docs]: struct.Bytes.html
//!
//! # `Buf`, `BufMut`
//!
//! These two traits provide read and write access to buffers. The underlying
//! storage may or may not be in contiguous memory. For example, `Bytes` is a
//! buffer that guarantees contiguous memory, but a [rope] stores the bytes in
//! disjoint chunks. `Buf` and `BufMut` maintain cursors tracking the current
//! position in the underlying byte storage. When bytes are read or written, the
//! cursor is advanced.
//!
//! [rope]: https://en.wikipedia.org/wiki/Rope_(data_structure)
//!
//! ## Relation with `Read` and `Write`
//!
//! At first glance, it may seem that `Buf` and `BufMut` overlap in
//! functionality with `std::io::Read` and `std::io::Write`. However, they
//! serve different purposes. A buffer is the value that is provided as an
//! argument to `Read::read` and `Write::write`. `Read` and `Write` may then
//! perform a syscall, which has the potential of failing. Operations on `Buf`
//! and `BufMut` are infallible.

#![deny(warnings, missing_docs, missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/bytes/0.4.12")]

extern crate byteorder;
extern crate iovec;

pub mod buf;
pub use buf::{
    Buf,
    BufMut,
    IntoBuf,
};
#[deprecated(since = "0.4.1", note = "moved to `buf` module")]
#[doc(hidden)]
pub use buf::{
    Reader,
    Writer,
    Take,
};

mod bytes;
mod debug;
pub use bytes::{Bytes, BytesMut};

#[deprecated]
pub use byteorder::{ByteOrder, BigEndian, LittleEndian};

// Optional Serde support
#[cfg(feature = "serde")]
#[doc(hidden)]
pub mod serde;

// Optional `Either` support
#[cfg(feature = "either")]
mod either;
