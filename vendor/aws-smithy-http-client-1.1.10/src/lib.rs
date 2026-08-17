/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/* Automatically managed default lints */
#![cfg_attr(docsrs, feature(doc_cfg))]
/* End of automatically managed default lints */

//! HTTP client implementation for smithy-rs generated code.
//!
//! # Crate Features
//!
//! - `default-client`: Enable default HTTP client implementation (based on hyper 1.x).
//! - `rustls-ring`: Enable TLS provider based on `rustls` using `ring` as the crypto provider
//! - `rustls-aws-lc`: Enable TLS provider based on `rustls` using `aws-lc` as the crypto provider
//! - `rustls-aws-lc-fips`: Same as `rustls-aws-lc` feature but using a FIPS compliant version of `aws-lc`
//! - `s2n-tls`: Enable TLS provider based on `s2n-tls` using `aws-lc` as the crypto provider.
//! - `hyper-014`: (Deprecated) HTTP client implementation based on hyper-0.14.x.
//! - `test-util`: Enables utilities for unit tests. DO NOT ENABLE IN PRODUCTION.

#![warn(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    unreachable_pub,
    rust_2018_idioms
)]

// ideally hyper_014 would just be exposed as is but due to
// https://github.com/rust-lang/rust/issues/47238 we get clippy warnings we can't suppress
#[cfg(feature = "hyper-014")]
pub(crate) mod hyper_legacy;

/// Legacy HTTP and TLS connectors that use hyper 0.14.x and rustls.
#[cfg(feature = "hyper-014")]
#[deprecated = "hyper 0.14.x support is deprecated, please migrate to 1.x client"]
pub mod hyper_014 {
    pub use crate::hyper_legacy::*;
}

/// Default HTTP and TLS connectors
#[cfg(feature = "default-client")]
pub(crate) mod client;
#[cfg(feature = "default-client")]
pub use client::{default_connector, proxy, tls, Builder, Connector, ConnectorBuilder};

#[cfg(feature = "test-util")]
pub mod test_util;

mod error;
pub use error::HttpClientError;

#[allow(unused_macros, unused_imports)]
#[macro_use]
pub(crate) mod cfg {
    /// Any TLS provider enabled
    macro_rules! cfg_tls {
        ($($item:item)*) => {
            $(
                #[cfg(any(
                    feature = "rustls-aws-lc",
                    feature = "rustls-aws-lc-fips",
                    feature = "rustls-ring",
                    feature = "s2n-tls",
                ))]
                #[cfg_attr(docsrs, doc(cfg(any(
                    feature = "rustls-aws-lc",
                    feature = "rustls-aws-lc-fips",
                    feature = "rustls-ring",
                    feature = "s2n-tls",
                ))))]
                $item
            )*
        }
    }

    /// Any rustls provider enabled
    macro_rules! cfg_rustls {
        ($($item:item)*) => {
            $(
                #[cfg(any(
                    feature = "rustls-aws-lc",
                    feature = "rustls-aws-lc-fips",
                    feature = "rustls-ring"
                ))]
                #[cfg_attr(docsrs, doc(cfg(any(feature = "rustls-aws-lc", feature = "rustls-aws-lc-fips", feature = "rustls-ring"))))]
                $item
            )*
        }
    }

    macro_rules! cfg_s2n_tls {
        ($($item:item)*) => {
            $(
                #[cfg(feature = "s2n-tls")]
                #[cfg_attr(docsrs, doc(cfg(feature = "s2n-tls")))]
                $item
            )*
        }
    }

    pub(crate) use cfg_rustls;
    pub(crate) use cfg_s2n_tls;
    pub(crate) use cfg_tls;
}
