/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::provider_config::ProviderConfig;
use aws_smithy_types::timeout::TimeoutConfig;
use std::time::Duration;

const SDK_DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_millis(3100);

/// Default [`TimeoutConfig`] provider chain
///
/// Unlike other credentials and region, [`TimeoutConfig`] has no related `TimeoutConfigProvider` trait. Instead,
/// a builder struct is returned which has a similar API.
///
pub fn default_provider() -> Builder {
    Builder::default()
}

/// Builder for [`TimeoutConfig`] that resolves the default timeout configuration
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct Builder;

impl Builder {
    /// Configure the default chain
    ///
    /// Exposed for overriding the environment when unit-testing providers
    pub fn configure(self, _configuration: &ProviderConfig) -> Self {
        self
    }

    /// Resolve default timeout configuration
    pub async fn timeout_config(self) -> TimeoutConfig {
        // TODO(https://github.com/smithy-lang/smithy-rs/issues/1732): Implement complete timeout defaults specification
        TimeoutConfig::builder()
            .connect_timeout(SDK_DEFAULT_CONNECT_TIMEOUT)
            .build()
    }
}
