// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

/// ## Compatibility with async runtimes
///
/// While this crate's API is purely blocking, it internally spawns asynchronous tasks on an executor.
///
/// - If you use tokio, make sure to enable the `tokio` feature of this crate.
/// - If you use another async runtime or if you don't use one at all, the default feature will suit your needs.

#[cfg(all(not(feature = "async-io"), not(feature = "tokio")))]
compile_error!("Either \"async-io\" (default) or \"tokio\" feature must be enabled.");

#[cfg(all(feature = "async-io", feature = "tokio"))]
compile_error!(
    "Both \"async-io\" (default) and \"tokio\" features cannot be enabled at the same time."
);

mod adapter;
mod atspi;
mod context;
mod executor;
mod util;

pub use adapter::Adapter;
