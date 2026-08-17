//! A port of selected [`winapi_util`] code to windows-sys and io-lifetimes.
//!
//! I have submitted a PR to propose incorporating the main changes here
//! upstream in winapi_util, though there are some underlying logistical
//! constraints, so it may not be a desirable change, or may take a while.
//!
//! [`winapi_util`]: https://crates.io/crates/winapi_util

pub mod file;
