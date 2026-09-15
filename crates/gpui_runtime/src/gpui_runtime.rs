//! GPUI's process harness: how an application is built, configured and started.
//!
//! [`Application`] is what a host program holds in order to bring GPUI up — the
//! `main` function of a desktop app, an embedder driving its own run loop, or a
//! test binary. Everything an application *is* once it is running lives in
//! [`gpui_authoring`], which this crate drives rather than contains.
//!
//! The `gpui` crate is a thin facade over both, so applications keep importing
//! `gpui`.

#![warn(missing_docs)]

mod application;

pub use application::{Application, ApplicationHandle};
