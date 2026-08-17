//! # async-recursion macro
//!
//! [![Latest version](https://img.shields.io/crates/v/async-recursion)](https://crates.io/crates/async-recursion)
//! [![crates.io downloads](https://img.shields.io/crates/d/async_recursion)](https://crates.io/crates/async-recursion)
//! [![Build Status](https://img.shields.io/github/actions/workflow/status/dcchut/async-recursion/ci.yml?branch=master)](https://github.com/dcchut/async-recursion/actions)
//! ![Apache/MIT2.0 License](https://img.shields.io/crates/l/async-recursion)
//!
//! Procedural macro for recursive async functions.
//!
//! * [Documentation](https://docs.rs/async-recursion/)
//! * Cargo package: [async-recursion](https://crates.io/crates/async-recursion)
//!
//! ## Motivation
//! Consider the following recursive implementation of the fibonacci numbers:
//!
//! ```rust,compile_fail
//! async fn fib(n : u32) -> u32 {
//!    match n {
//!        0 | 1 => 1,
//!        _ => fib(n-1).await + fib(n-2).await
//!    }
//! }
//! ```
//!
//! The compiler helpfully tells us that:
//!
//! ```console
//! error[E0733]: recursion in an `async fn` requires boxing
//! --> src/main.rs:1:26
//!   |
//! 1 | async fn fib(n : u32) -> u32 {
//!   |                          ^^^ recursive `async fn`
//!   |
//!   = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`.
//!   = note: consider using the `async_recursion` crate: https://crates.io/crates/async_recursion
//! ```
//!
//! This crate provides an attribute macro to automatically convert an async function
//! to one returning a boxed [`Future`](core::future::Future).
//!
//! ## Example
//!
//! ```rust
//! use async_recursion::async_recursion;
//!
//! #[async_recursion]
//! async fn fib(n : u32) -> u32 {
//!    match n {
//!        0 | 1 => 1,
//!        _ => fib(n-1).await + fib(n-2).await
//!    }
//! }
//! ```
//!
//! ## ?Send option
//!
//! The returned [`Future`] has a [`Send`] bound to make sure it can be sent between threads.
//! If this is undesirable you can mark that the bound should be left out like so:
//!
//! ```rust
//! # use async_recursion::async_recursion;
//!
//! #[async_recursion(?Send)]
//! async fn returned_future_is_not_send() {
//!     // ...
//! }
//! ```
//!
//! ## Sync option
//!
//! The returned [`Future`] doesn't have a [`Sync`] bound as it is usually not required.
//! You can include a [`Sync`] bound as follows:
//!
//! ```rust
//! # use async_recursion::async_recursion;
//!
//! #[async_recursion(Sync)]
//! async fn returned_future_is_send_and_sync() {
//!     // ...
//! }
//! ```
//!
//! In detail:
//!
//!
//! - `#[async_recursion]` modifies your function to return a boxed [`Future`] with a [`Send`] bound.
//! - `#[async_recursion(?Send)]` modifies your function to return a boxed [`Future`] _without_ a [`Send`] bound.
//! - `#[async_recursion(Sync)]` modifies your function to return a boxed [`Future`] with [`Send`] and [`Sync`] bounds.
//!
//! ### License
//!
//! Licensed under either of
//!  * Apache License, Version 2.0 (<http://www.apache.org/licenses/LICENSE-2.0>)
//!  * MIT license (<http://opensource.org/licenses/MIT>)
//!
//! at your option.

extern crate proc_macro;

mod expand;
mod parse;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn async_recursion(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as parse::AsyncItem);
    let args = parse_macro_input!(args as parse::RecursionArgs);

    expand::expand(&mut item, &args);

    TokenStream::from(quote!(#item))
}
