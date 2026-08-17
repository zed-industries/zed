/*!
Buffering support for `sval`.

This crate provides the [`ValueBuf`] type, which can buffer a flat
stream of data into a tree of borrowed values.

Some functionality requires the `alloc` Cargo feature to be enabled.
Rather than conditionally compile these methods, this library stubs
out functionality when an allocator isn't available.
*/

#![no_std]
#![deny(missing_docs)]

mod error;

#[cfg(feature = "std")]
#[macro_use]
#[allow(unused_imports)]
extern crate std as libstd;

#[cfg(not(feature = "alloc"))]
extern crate core as std;

#[cfg(any(test, feature = "alloc"))]
#[macro_use]
#[allow(unused_imports)]
extern crate alloc;
#[cfg(feature = "alloc")]
extern crate core;

#[cfg(feature = "alloc")]
mod std {
    #[allow(unused_imports)]
    pub use crate::{
        alloc::{borrow, boxed, collections, string, vec},
        core::{convert, fmt, hash, marker, mem, ops, result, str},
    };

    #[cfg(feature = "std")]
    pub use libstd::error;
}

mod fragments;
mod value;

#[cfg(feature = "alloc")]
fn assert_static<T: 'static>(_: &mut T) {}

pub use self::{error::*, fragments::*, value::*};
