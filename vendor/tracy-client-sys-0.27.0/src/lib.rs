//! The Tracy Client and its low level API
//!
//! This crate embeds the C++ Tracy client library and exposes its API. For a higher-level API
//! consider the `tracy-client` crate.
//!
//! # Important note
//!
//! Depending on the configuration Tracy may broadcast discovery packets to the local network and
//! expose the data it collects in the background to that same network. Traces collected by Tracy
//! may include source and assembly code as well.
//!
//! As thus, you may want make sure to only enable the `tracy-client-sys` crate conditionally, via
//! the `enable` feature flag provided by this crate.
//!
//! In order to start tracing it is important that you first call the [`___tracy_startup_profiler`]
//! function first to initialize the client. The [`___tracy_shutdown_profiler`] must not be called
//! until it is guaranteed that there will be no more calls to any other Tracy APIs. This can be
//! especially difficult to ensure if you have detached threads.
//!
//! # Features
//!
//! The following crate features are provided to customize the functionality of the Tracy client:
//!
//! * `manual-lifetime` – disables Tracy’s life-before-main initialization, requiring manual
//!   initialization. Corresponds to the `TRACY_MANUAL_LIFETIME` define.
//! * `delayed-init` – profiler data is gathered into one structure and initialized on the first
//!   request rather than statically at the DLL load at the expense of atomic load on each request
//!   to the profiler data. Corresponds to the `TRACY_DELAYED_INIT` define.
#![doc = include_str!("../FEATURES.mkd")]
#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_variables,
    deref_nullptr
)]

#[cfg(feature = "enable")]
mod generated;
#[cfg(feature = "enable")]
pub use generated::*;

#[cfg(all(feature = "enable", feature = "manual-lifetime"))]
mod generated_manual_lifetime;
#[cfg(all(feature = "enable", feature = "manual-lifetime"))]
pub use generated_manual_lifetime::*;

#[cfg(all(feature = "enable", feature = "fibers"))]
mod generated_fibers;
#[cfg(all(feature = "enable", feature = "fibers"))]
pub use generated_fibers::{___tracy_fiber_enter, ___tracy_fiber_leave};

#[cfg(all(feature = "enable", target_os = "windows"))]
mod dbghelp;
