/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Providers that implement the default AWS provider chain
//!
//! Default Provider chains for [`region`], [`credentials`],
//! [retries](crate::default_provider::retry_config), [timeouts](crate::default_provider::timeout_config) and
//! [app name](crate::default_provider::app_name).
//!
//! Typically, this module is used via [`load_from_env`](crate::load_from_env) or [`from_env`](crate::from_env). It should only be used directly
//! if you need to set custom configuration options to override the default resolution chain.

/// Default [region](aws_types::region::Region) provider chain
///
/// Typically, this module is used via [`load_from_env`](crate::load_from_env) or [`from_env`](crate::from_env). It should only be used directly
/// if you need to set custom configuration options to override the default resolution chain.
pub mod region;

/// Default retry behavior configuration provider chain
///
/// Typically, this module is used via [`load_from_env`](crate::load_from_env) or [`from_env`](crate::from_env). It should only be used directly
/// if you need to set custom configuration options to override the default resolution chain.
pub mod retry_config;

/// Default app name provider chain
///
/// Typically, this module is used via [`load_from_env`](crate::load_from_env) or [`from_env`](crate::from_env). It should only be used directly
/// if you need to set custom configuration options to override the default resolution chain.
pub mod app_name;

/// Default timeout configuration provider chain
///
/// Typically, this module is used via [`load_from_env`](crate::load_from_env) or [`from_env`](crate::from_env). It should only be used directly
/// if you need to set custom configuration options to override the default resolution chain.
pub mod timeout_config;

/// Default credentials provider chain
///
/// Typically, this module is used via [`load_from_env`](crate::load_from_env) or [`from_env`](crate::from_env). It should only be used directly
/// if you need to set custom configuration options like [`region`](credentials::Builder::region) or [`profile_name`](credentials::Builder::profile_name).
pub mod credentials;

/// Default FIPS provider chain
pub mod use_fips;

/// Default dual-stack provider chain
pub mod use_dual_stack;

/// Default access token provider chain
#[cfg(feature = "sso")]
pub mod token;

/// Default "ignore configured endpoint URLs" provider chain
pub mod ignore_configured_endpoint_urls;

/// Default endpoint URL provider chain
pub mod endpoint_url;

/// Default "disable request compression" provider chain
pub mod disable_request_compression;

/// Default "request minimum compression size bytes" provider chain
pub mod request_min_compression_size_bytes;

/// Default provider chains for request/response checksum configuration
pub mod checksums;

/// Default provider chain for account-based endpoint mode
pub mod account_id_endpoint_mode;

/// Default provider chain for auth scheme preference list
pub mod auth_scheme_preference;
