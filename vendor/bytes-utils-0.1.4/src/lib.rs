#![doc(test(attr(deny(warnings))))]
#![warn(missing_docs)]
#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

//! # Extra utilities for the [bytes] crate.
//!
//! The [bytes] crate defines few traits and types to help with high-performance manipulation of
//! byte arrays. Nevertheless, it is more of an interface-level of library (many other crates
//! expose its types and traits in their own public interfaces) and therefore tries to be on the
//! lean side.
//!
//! One often wishes for some more auxiliary functionality „around“ these types and that's what
//! this crate aims to provide.
//!
//! ## The content
//!
//! * [SegmentedBuf] and [SegmentedSlice] for concatenating multiple buffers into a large one
//!   without copying the bytes.
//! * [Str] and [StrMut] are wrappers around [Bytes][bytes::Bytes] and [BytesMut]
//!   respectively, providing a [String]-like interface. They allow splitting into owned
//!   sub-slices, similar to how the [Bytes] and [BytesMut] work.
//!
//! [Bytes]: bytes::Bytes
//! [BytesMut]: bytes::BytesMut

extern crate alloc;

mod segmented;
pub mod string;

pub use segmented::{SegmentedBuf, SegmentedSlice};
pub use string::{Str, StrMut};
