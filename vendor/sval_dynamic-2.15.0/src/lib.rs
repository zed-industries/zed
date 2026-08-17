/*!
Object-safe wrappers for `sval::Stream` and `sval::Value`.

This crate makes it possible to erase a concrete `sval::Value` or `sval::Stream`
as a `dyn Value` or `dyn Stream`. It doesn't require any allocator,
so it's possible to use in no-std environments.
*/

#![no_std]
#![deny(missing_docs)]

mod stream;
mod value;

mod private {
    pub struct Erased<T>(pub(crate) T);
}

pub use self::{stream::Stream, value::Value};

// NOTE: Tests for forwarding through dynamic traits is in `sval_test`
