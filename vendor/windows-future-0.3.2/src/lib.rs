#![expect(
    missing_docs,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::all
)]
#![doc = include_str!("../readme.md")]
#![cfg_attr(all(not(feature = "std")), no_std)]

mod r#async;
mod bindings;
mod bindings_impl;
mod join;
mod waiter;
mod when;

pub use bindings::*;
use bindings_impl::*;
use r#async::*;
use waiter::*;
use windows_core::*;

#[cfg(feature = "std")]
mod async_ready;
#[cfg(feature = "std")]
mod async_spawn;
#[cfg(feature = "std")]
mod future;
