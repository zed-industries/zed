/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![cfg(any(
    feature = "rustls-ring",
    feature = "rustls-aws-lc",
    feature = "rustls-aws-lc-fips",
    feature = "s2n-tls",
))]

use aws_smithy_async::time::SystemTimeSource;
use aws_smithy_http_client::{tls, Builder};
use aws_smithy_runtime_api::client::dns::{DnsFuture, ResolveDns, ResolveDnsError};
use aws_smithy_runtime_api::client::http::{HttpClient, HttpConnector, HttpConnectorSettings};
use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
use hyper_util::client::legacy::connect::dns::{GaiResolver, Name};
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use tower::Service;

#[cfg(feature = "rustls-ring")]
#[tokio::test]
async fn ring_client() {
    let client = Builder::new()
        .tls_provider(tls::Provider::Rustls(
            tls::rustls_provider::CryptoMode::Ring,
        ))
        .build_https();
    smoke_test_client(&client).await.unwrap();
}

#[cfg(feature = "rustls-aws-lc-fips")]
#[tokio::test]
async fn aws_lc_fips_client() {
    let client = Builder::new()
        .tls_provider(tls::Provider::Rustls(
            tls::rustls_provider::CryptoMode::AwsLcFips,
        ))
        .build_https();
    smoke_test_client(&client).await.unwrap();
}

#[cfg(feature = "rustls-aws-lc")]
#[tokio::test]
async fn aws_lc_client() {
    let client = Builder::new()
        .tls_provider(tls::Provider::Rustls(
            tls::rustls_provider::CryptoMode::AwsLc,
        ))
        .build_https();
    smoke_test_client(&client).await.unwrap();
}

#[cfg(feature = "s2n-tls")]
#[tokio::test]
async fn s2n_tls_client() {
    let client = Builder::new()
        .tls_provider(tls::Provider::S2nTls)
        .build_https();
    smoke_test_client(&client).await.unwrap();
}

#[cfg(any(feature = "rustls-ring", feature = "s2n-tls"))]
#[tokio::test]
async fn custom_dns_client() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    #[derive(Debug, Clone)]
    struct PassThroughResolver {
        inner: GaiResolver,
        count: Arc<AtomicUsize>,
    }
    impl ResolveDns for PassThroughResolver {
        fn resolve_dns<'a>(&'a self, _name: &'a str) -> DnsFuture<'a> {
            let mut inner = self.inner.clone();
            let name = Name::from_str(_name).unwrap();
            let count = self.count.clone();
            DnsFuture::new(async move {
                count.fetch_add(1, Ordering::Relaxed);
                let result = inner.call(name).await.map_err(ResolveDnsError::new)?;
                Ok(result.map(|addr| addr.ip()).collect::<Vec<_>>())
            })
        }
    }

    let providers = [
        #[cfg(feature = "rustls-ring")]
        tls::Provider::Rustls(tls::rustls_provider::CryptoMode::Ring),
        #[cfg(feature = "s2n-tls")]
        tls::Provider::S2nTls,
    ];

    for provider in providers {
        let resolver = PassThroughResolver {
            inner: GaiResolver::new(),
            count: Default::default(),
        };
        let client = Builder::new()
            .tls_provider(provider)
            .build_with_resolver(resolver.clone());
        smoke_test_client(&client).await.unwrap();
        assert_eq!(resolver.count.load(Ordering::Relaxed), 1);
    }
}

async fn smoke_test_client(client: &dyn HttpClient) -> Result<(), Box<dyn Error>> {
    let connector_settings = HttpConnectorSettings::builder().build();
    let runtime_components = RuntimeComponentsBuilder::for_tests()
        .with_time_source(Some(SystemTimeSource::new()))
        .build()
        .unwrap();
    let connector = client.http_connector(&connector_settings, &runtime_components);
    let _response = connector
        .call(HttpRequest::get("https://amazon.com").unwrap())
        .await?;
    Ok(())
}
