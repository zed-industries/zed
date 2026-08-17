//! # Rust library for Neovim clients
//!
//! Implements support for rust plugins for
//! [Neovim](https://github.com/neovim/neovim) through its msgpack-rpc API.
//!
//! ### Origins
//!
//! This library started as a fork of
//! [neovim-lib](https://github.com/daa84/neovim-lib) with the goal to utilize
//! Rust's `async/await` to allow requests/notification to/from neovim to be
//! arbitrarily nested. After the fork, I started implementing more ideas I had
//! for this library.
//!
//! ### Status
//!
//! As of the end of 2019, I'm somewhat confident to recommend starting to use
//! this library. The overall handling should not change anymore. A breaking
//! change I kind of expect is adding error variants to
//! [`CallError`](crate::error::CallError) when I start working on the API
//! (right now, it panics when messages don't have the right format, I'll want
//! to return proper errors in that case).
//!
//! I've not yet worked through the details of what-to-export, but I'm quite
//! willing to consider what people need or want.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
extern crate rmp;
extern crate rmpv;
#[macro_use]
extern crate log;

pub mod rpc;
#[macro_use]
pub mod neovim;
pub mod error;
pub mod examples;
pub mod exttypes;
pub mod neovim_api;
pub mod neovim_api_manual;
pub mod uioptions;

pub mod create;

pub use crate::{
  exttypes::{Buffer, Tabpage, Window},
  neovim::Neovim,
  rpc::handler::Handler,
  uioptions::{UiAttachOptions, UiOption},
};

#[cfg(feature = "use_tokio")]
pub mod compat {
  //! A re-export of tokio-util's [`Compat`](tokio_util::compat::Compat)
  pub mod tokio {
    pub use tokio_util::compat::Compat;
  }
}

pub use rmpv::Value;
