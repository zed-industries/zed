#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::all
)]
#![doc = include_str!("../readme.md")]
#![allow(missing_docs)]
#![cfg_attr(all(not(feature = "std")), no_std)]

mod bindings;
mod bindings_impl;
mod get;
mod waiter;

pub use bindings::*;
use bindings_impl::*;
use waiter::*;
use windows_core::*;

#[cfg(feature = "std")]
mod async_ready;
#[cfg(feature = "std")]
mod async_spawn;
#[cfg(feature = "std")]
mod future;
#[cfg(feature = "std")]
use future::*;
