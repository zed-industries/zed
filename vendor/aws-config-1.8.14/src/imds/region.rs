/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! IMDS Region Provider
//!
//! Load region from IMDS from `/latest/meta-data/placement/region`
//! This provider has a 5 second timeout.

use crate::imds::{self, Client};
use crate::meta::region::{future, ProvideRegion};
use crate::provider_config::ProviderConfig;
use aws_smithy_types::error::display::DisplayErrorContext;
use aws_types::os_shim_internal::Env;
use aws_types::region::Region;
use std::fmt::Debug;
use tracing::Instrument;

/// IMDSv2 Region Provider
///
/// This provider is included in the default region chain, so it does not need to be used manually.
pub struct ImdsRegionProvider {
    client: Client,
    env: Env,
}

impl Debug for ImdsRegionProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImdsRegionProvider")
            .field("client", &"IMDS client truncated for readability")
            .field("env", &self.env)
            .finish()
    }
}

const REGION_PATH: &str = "/latest/meta-data/placement/region";

impl ImdsRegionProvider {
    /// Builder for [`ImdsRegionProvider`]
    pub fn builder() -> Builder {
        Builder::default()
    }

    fn imds_disabled(&self) -> bool {
        match self.env.get(super::env::EC2_METADATA_DISABLED) {
            Ok(value) => value.eq_ignore_ascii_case("true"),
            _ => false,
        }
    }

    /// Load a region from IMDS
    ///
    /// This provider uses the API `/latest/meta-data/placement/region`
    pub async fn region(&self) -> Option<Region> {
        if self.imds_disabled() {
            tracing::debug!("not using IMDS to load region, IMDS is disabled");
            return None;
        }
        match self.client.get(REGION_PATH).await {
            Ok(region) => {
                tracing::debug!(region = %region.as_ref(), "loaded region from IMDS");
                Some(Region::new(String::from(region)))
            }
            Err(err) => {
                tracing::warn!(err = %DisplayErrorContext(&err), "failed to load region from IMDS");
                None
            }
        }
    }
}

impl ProvideRegion for ImdsRegionProvider {
    fn region(&self) -> future::ProvideRegion<'_> {
        future::ProvideRegion::new(
            self.region()
                .instrument(tracing::debug_span!("imds_load_region")),
        )
    }
}

/// Builder for [`ImdsRegionProvider`]
#[derive(Debug, Default)]
pub struct Builder {
    provider_config: Option<ProviderConfig>,
    imds_client_override: Option<imds::Client>,
}

impl Builder {
    /// Set configuration options of the [`Builder`]
    pub fn configure(self, provider_config: &ProviderConfig) -> Self {
        Self {
            provider_config: Some(provider_config.clone()),
            ..self
        }
    }

    /// Override the IMDS client used to load the region
    pub fn imds_client(mut self, imds_client: imds::Client) -> Self {
        self.imds_client_override = Some(imds_client);
        self
    }

    /// Create an [`ImdsRegionProvider`] from this builder
    pub fn build(self) -> ImdsRegionProvider {
        let provider_config = self.provider_config.unwrap_or_default();
        let client = self
            .imds_client_override
            .unwrap_or_else(|| imds::Client::builder().configure(&provider_config).build());
        ImdsRegionProvider {
            client,
            env: provider_config.env(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::imds::client::test::{imds_request, imds_response, token_request, token_response};
    use crate::imds::region::ImdsRegionProvider;
    use crate::provider_config::ProviderConfig;
    use aws_smithy_async::rt::sleep::TokioSleep;
    use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
    use aws_smithy_types::body::SdkBody;
    use aws_types::region::Region;
    use tracing_test::traced_test;

    #[tokio::test]
    async fn load_region() {
        let http_client = StaticReplayClient::new(vec![
            ReplayEvent::new(
                token_request("http://169.254.169.254", 21600),
                token_response(21600, "token"),
            ),
            ReplayEvent::new(
                imds_request(
                    "http://169.254.169.254/latest/meta-data/placement/region",
                    "token",
                ),
                imds_response("eu-west-1"),
            ),
        ]);
        let provider = ImdsRegionProvider::builder()
            .configure(
                &ProviderConfig::no_configuration()
                    .with_http_client(http_client)
                    .with_sleep_impl(TokioSleep::new()),
            )
            .build();
        assert_eq!(
            provider.region().await.expect("returns region"),
            Region::new("eu-west-1")
        );
    }

    #[traced_test]
    #[tokio::test]
    async fn no_region_imds_disabled() {
        let http_client = StaticReplayClient::new(vec![ReplayEvent::new(
            token_request("http://169.254.169.254", 21600),
            http::Response::builder()
                .status(403)
                .body(SdkBody::empty())
                .unwrap(),
        )]);
        let provider = ImdsRegionProvider::builder()
            .configure(
                &ProviderConfig::no_configuration()
                    .with_http_client(http_client)
                    .with_sleep_impl(TokioSleep::new()),
            )
            .build();
        assert_eq!(provider.region().await, None);
        assert!(logs_contain("failed to load region from IMDS"));
        assert!(logs_contain("IMDS is disabled"));
    }
}
