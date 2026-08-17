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

//! Smithy Observability
// TODO(smithyobservability): once we have finalized everything and integrated metrics with our runtime
// libraries update this with detailed usage docs and examples

mod attributes;
pub use attributes::{AttributeValue, Attributes};
mod context;
pub use context::{Context, ContextManager, Scope};
mod error;
pub use error::{ErrorKind, GlobalTelemetryProviderError, ObservabilityError};
pub mod global;
pub mod meter;
mod noop;
mod provider;
pub use provider::{TelemetryProvider, TelemetryProviderBuilder};
pub mod instruments;
