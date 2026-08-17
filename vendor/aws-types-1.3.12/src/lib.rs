/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/* Automatically managed default lints */
#![cfg_attr(docsrs, feature(doc_cfg))]
/* End of automatically managed default lints */
//! Cross-service types for the AWS SDK.

#![allow(clippy::derive_partial_eq_without_eq)]
#![warn(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]

pub mod app_name;
pub mod build_metadata;
pub mod endpoint_config;
pub mod origin;
pub mod os_shim_internal;
pub mod region;
pub mod request_id;
pub mod sdk_config;
pub mod service_config;

pub use sdk_config::SdkConfig;

use aws_smithy_types::config_bag::{Storable, StoreReplace};
use std::borrow::Cow;

/// The name of the service used to sign this request
///
/// Generally, user code should never interact with `SigningName` directly
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SigningName(Cow<'static, str>);
impl AsRef<str> for SigningName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SigningName {
    /// Creates a `SigningName` from a static str.
    pub fn from_static(name: &'static str) -> Self {
        SigningName(Cow::Borrowed(name))
    }
}

impl From<String> for SigningName {
    fn from(name: String) -> Self {
        SigningName(Cow::Owned(name))
    }
}

impl From<&'static str> for SigningName {
    fn from(name: &'static str) -> Self {
        Self::from_static(name)
    }
}

impl Storable for SigningName {
    type Storer = StoreReplace<Self>;
}
