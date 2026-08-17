/*!
Treat any [`sval::Value`] as a [`core::fmt::Debug`].

This crate provides [`ToFmt`], a wrapper around any [`sval::Value`]
that formats it using the same output that you'd get if you
derived [`core::fmt::Debug`].
*/

#![no_std]
#![deny(missing_docs)]

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

mod writer;

mod to_fmt;
mod to_value;
mod to_write;

pub mod tags;
mod token_write;

pub use self::{to_fmt::*, to_value::*, to_write::*, token_write::*};

#[cfg(feature = "alloc")]
mod to_string;

#[cfg(feature = "alloc")]
pub use self::to_string::stream_to_string;
