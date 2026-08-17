/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/* Automatically managed default lints */
#![cfg_attr(docsrs, feature(doc_cfg))]
/* End of automatically managed default lints */
//! Runtime support code for the AWS SDK. This crate isn't intended to be used directly.

#![warn(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]

/// Supporting code for authentication in the AWS SDK.
pub mod auth;

/// AWS-specific content-encoding tools for http-02x and http-1x
pub mod content_encoding;

/// Supporting code for recursion detection in the AWS SDK.
pub mod recursion_detection;

/// Supporting code for user agent headers in the AWS SDK.
pub mod user_agent;

/// Supporting code for retry behavior specific to the AWS SDK.
pub mod retries;

/// Supporting code for invocation ID headers in the AWS SDK.
pub mod invocation_id;

/// Supporting code for request metadata headers in the AWS SDK.
pub mod request_info;

/// AWS SDK feature identifies.
#[doc(hidden)]
pub mod sdk_feature;

/// Interceptor that determines the clock skew between the client and service.
pub mod service_clock_skew;

/// Filesystem utilities
pub mod fs_util;

/// Supporting code for parsing AWS config values set in a user's environment or
/// in a shared config file.
pub mod env_config;
