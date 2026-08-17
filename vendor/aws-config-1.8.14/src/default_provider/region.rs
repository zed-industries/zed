/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_types::region::Region;

use crate::environment::region::EnvironmentVariableRegionProvider;
use crate::meta::region::{ProvideRegion, RegionProviderChain};
use crate::provider_config::ProviderConfig;
use crate::{imds, profile};

/// Default Region Provider chain
///
/// This provider will check the following sources in order:
/// 1. [Environment variables](EnvironmentVariableRegionProvider)
/// 2. [Profile file](crate::profile::region::ProfileFileRegionProvider)
/// 3. [EC2 IMDSv2](crate::imds::region)
pub fn default_provider() -> impl ProvideRegion {
    Builder::default().build()
}

/// Default region provider chain
#[derive(Debug)]
pub struct DefaultRegionChain(RegionProviderChain);

impl DefaultRegionChain {
    /// Load a region from this chain
    pub async fn region(&self) -> Option<Region> {
        self.0.region().await
    }

    /// Builder for [`DefaultRegionChain`]
    pub fn builder() -> Builder {
        Builder::default()
    }
}

/// Builder for [DefaultRegionChain]
#[derive(Debug, Default)]
pub struct Builder {
    env_provider: EnvironmentVariableRegionProvider,
    profile_file: profile::region::Builder,
    imds: imds::region::Builder,
}

impl Builder {
    /// Configure the default chain
    ///
    /// Exposed for overriding the environment when unit-testing providers
    pub(crate) fn configure(mut self, configuration: &ProviderConfig) -> Self {
        self.env_provider = EnvironmentVariableRegionProvider::new_with_env(configuration.env());
        self.profile_file = self.profile_file.configure(configuration);
        self.imds = self.imds.configure(configuration);
        self
    }

    /// Override the profile name used by this provider
    pub fn profile_name(mut self, name: &str) -> Self {
        self.profile_file = self.profile_file.profile_name(name);
        self
    }

    /// Build a [DefaultRegionChain]
    pub fn build(self) -> DefaultRegionChain {
        DefaultRegionChain(
            RegionProviderChain::first_try(self.env_provider)
                .or_else(self.profile_file.build())
                .or_else(self.imds.build()),
        )
    }
}

impl ProvideRegion for DefaultRegionChain {
    fn region(&self) -> crate::meta::region::future::ProvideRegion<'_> {
        ProvideRegion::region(&self.0)
    }
}
