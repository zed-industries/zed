/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/* Automatically managed default lints */
#![cfg_attr(docsrs, feature(doc_cfg))]
/* End of automatically managed default lints */
//! `aws-credential-types` provides types concerned with AWS SDK credentials including:
//! * Traits for credentials providers and for credentials caching
//! * An opaque struct representing credentials
//! * Concrete implementations of credentials caching

#![allow(clippy::derive_partial_eq_without_eq)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rustdoc::missing_crate_level_docs,
    unreachable_pub
)]

pub mod attributes;
#[doc(hidden)]
pub mod credential_feature;
pub mod credential_fn;
mod credentials_impl;
pub mod provider;
pub mod token_fn;

pub use credentials_impl::{Credentials, CredentialsBuilder};

/// AWS Access Token
///
/// This access token type is used to authenticate to AWS services that use HTTP Bearer
/// Auth with an AWS Builder ID such as CodeCatalyst.
///
/// For more details on tokens, see: <https://oauth.net/2/access-tokens>
pub type Token = aws_smithy_runtime_api::client::identity::http::Token;
