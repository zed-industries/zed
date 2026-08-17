/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/* Automatically managed default lints */
#![cfg_attr(docsrs, feature(doc_cfg))]
/* End of automatically managed default lints */
#![warn(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    unreachable_pub,
    rust_2018_idioms
)]
#![allow(clippy::new_without_default)]

//! APIs needed to configure and customize the Smithy generated code.
//!
//! Most users will not need to use this crate directly as the most frequently used
//! APIs are re-exported in the generated clients. However, this crate will be useful
//! for anyone writing a library for others to use with their generated clients.
//!
//! If you're needing to depend on this and you're not writing a library for Smithy
//! generated clients, then please file an issue on [smithy-rs](https://github.com/smithy-lang/smithy-rs)
//! as we likely missed re-exporting one of the APIs.
//!
//! All client-specific code is in the [`client`] root level module
//! to leave room for smithy-rs server APIs in the future.

/// A boxed error that is `Send` and `Sync`.
pub mod box_error;

/// APIs for client orchestration.
#[cfg(feature = "client")]
pub mod client;

pub mod http;

pub mod shared;
