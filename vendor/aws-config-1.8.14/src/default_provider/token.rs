/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::{
    meta::{region::ProvideRegion, token::TokenProviderChain},
    provider_config::ProviderConfig,
};
use aws_credential_types::provider::{future, token::ProvideToken};

/// Default access token provider chain
///
/// The region from the default region provider will be used
pub async fn default_provider() -> impl ProvideToken {
    DefaultTokenChain::builder().build().await
}

/// Default access token provider chain
///
/// Currently, the default chain only examines the shared config
/// (`~/.aws/config`) file and the SSO token cache to resolve an
/// access token.
///
/// The AWS CLI can be used to retrieve the initial access token into
/// the SSO token cache. Once it's there, the SDK can refresh automatically
/// long as the it remains refreshable (it will eventually expire).
///
/// # Examples
/// Create a default chain with a custom region:
/// ```no_run
/// use aws_types::region::Region;
/// use aws_config::default_provider::token::DefaultTokenChain;
/// let token_provider = DefaultTokenChain::builder()
///     .region(Region::new("us-west-1"))
///     .build();
/// ```
///
/// Create a default chain with no overrides:
/// ```no_run
/// use aws_config::default_provider::token::DefaultTokenChain;
/// let token_provider = DefaultTokenChain::builder().build();
/// ```
///
/// Create a default chain that uses a different profile:
/// ```no_run
/// use aws_config::default_provider::token::DefaultTokenChain;
/// let token_provider = DefaultTokenChain::builder()
///     .profile_name("otherprofile")
///     .build();
/// ```
#[derive(Debug)]
pub struct DefaultTokenChain {
    provider_chain: TokenProviderChain,
}

impl DefaultTokenChain {
    /// Builder for `DefaultTokenChain`.
    pub fn builder() -> Builder {
        Builder::default()
    }
}

impl ProvideToken for DefaultTokenChain {
    fn provide_token<'a>(&'a self) -> future::ProvideToken<'a>
    where
        Self: 'a,
    {
        self.provider_chain.provide_token()
    }
}

/// Builder for [`DefaultTokenChain`].
#[derive(Debug, Default)]
pub struct Builder {
    profile_file_builder: crate::profile::token::Builder,
    region_override: Option<Box<dyn ProvideRegion>>,
    region_chain: crate::default_provider::region::Builder,
    conf: Option<ProviderConfig>,
}

impl Builder {
    /// Sets the region used when making requests to AWS services
    ///
    /// When unset, the default region resolver chain will be used.
    pub fn region(mut self, region: impl ProvideRegion + 'static) -> Self {
        self.set_region(Some(region));
        self
    }

    /// Sets the region used when making requests to AWS services
    ///
    /// When unset, the default region resolver chain will be used.
    pub fn set_region(&mut self, region: Option<impl ProvideRegion + 'static>) -> &mut Self {
        self.region_override = region.map(|provider| Box::new(provider) as _);
        self
    }

    /// Override the profile name used by this provider
    ///
    /// When unset, the value of the `AWS_PROFILE` environment variable will be used.
    pub fn profile_name(mut self, name: &str) -> Self {
        self.profile_file_builder = self.profile_file_builder.profile_name(name);
        self.region_chain = self.region_chain.profile_name(name);
        self
    }

    /// Override the configuration used for this provider
    pub(crate) fn configure(mut self, config: ProviderConfig) -> Self {
        self.region_chain = self.region_chain.configure(&config);
        self.conf = Some(config);
        self
    }

    /// Creates a [`DefaultTokenChain`].
    pub async fn build(self) -> DefaultTokenChain {
        let region = match self.region_override {
            Some(provider) => provider.region().await,
            None => self.region_chain.build().region().await,
        };
        let conf = self.conf.unwrap_or_default().with_region(region);

        let provider_chain = TokenProviderChain::first_try(
            "Profile",
            self.profile_file_builder.configure(&conf).build(),
        );
        DefaultTokenChain { provider_chain }
    }
}
