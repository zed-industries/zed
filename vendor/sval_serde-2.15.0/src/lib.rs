/*!
Treat any [`sval::Value`] as a [`serde::Serialize`].

This crate provides `ToSerialize`, a wrapper around any `sval::Value`
that forwards it through `serde`.

# Buffering

Add the `alloc` feature to enable buffering for values that need it.

Types that derive [`sval::Value`] automatically can be streamed through
`serde` without requiring any buffering. Types that manually stream
text across multiple fragments, or nested fields without recursing through
`Stream::value` will need to be buffered.

Without the `alloc` feature, any values that require buffering will instead
produce errors during serialization.
*/

#![no_std]
#![deny(missing_docs)]

mod to_serialize;
mod to_value;

pub use self::{to_serialize::*, to_value::*};
