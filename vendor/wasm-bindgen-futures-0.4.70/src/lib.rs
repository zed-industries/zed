//! Bridging the gap between Rust Futures and JavaScript Promises.
//!
//! This crate is now a thin shim re-exporting from [`js_sys::futures`].
//! The implementation has been moved into `js-sys` so that
//! [`js_sys::Promise`] can implement [`core::future::IntoFuture`] directly,
//! enabling `promise.await` without any wrapper type.
//!
//! All public items (`JsFuture`, `spawn_local`, `future_to_promise`,
//! `future_to_promise_typed`) are re-exported unchanged for backwards
//! compatibility.

#![cfg_attr(not(feature = "std"), no_std)]

pub use js_sys::futures::{future_to_promise, future_to_promise_typed, spawn_local, JsFuture};

#[cfg(feature = "futures-core-03-stream")]
pub use js_sys::futures::stream;

pub use js_sys;
pub use wasm_bindgen;
