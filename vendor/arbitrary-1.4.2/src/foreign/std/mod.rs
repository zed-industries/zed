//! Implementations of [`Arbitrary`] for [`std`] types,
//!   excluding those in [`core`] and [`alloc`].
//!
//! [`Arbitrary`]: crate::Arbitrary

mod collections;
mod ffi;
mod net;
mod path;
mod sync;
