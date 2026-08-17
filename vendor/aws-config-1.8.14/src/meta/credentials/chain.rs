/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_credential_types::{
    provider::{self, error::CredentialsError, future, ProvideCredentials},
    Credentials,
};
use aws_smithy_types::error::display::DisplayErrorContext;
use std::borrow::Cow;
use std::fmt::Debug;
use tracing::Instrument;

/// Credentials provider that checks a series of inner providers
///
/// Each provider will be evaluated in order:
/// * If a provider returns valid [`Credentials`] they will be returned immediately.
///   No other credential providers will be used.
/// * Otherwise, if a provider returns [`CredentialsError::CredentialsNotLoaded`], the next provider will be checked.
/// * Finally, if a provider returns any other error condition, an error will be returned immediately.
///
/// # Examples
///
/// ```no_run
/// # fn example() {
/// use aws_config::meta::credentials::CredentialsProviderChain;
/// use aws_config::environment::credentials::EnvironmentVariableCredentialsProvider;
/// use aws_config::profile::ProfileFileCredentialsProvider;
///
/// let provider = CredentialsProviderChain::first_try("Environment", EnvironmentVariableCredentialsProvider::new())
///     .or_else("Profile", ProfileFileCredentialsProvider::builder().build());
/// # }
/// ```
pub struct CredentialsProviderChain {
    providers: Vec<(Cow<'static, str>, Box<dyn ProvideCredentials>)>,
}

impl Debug for CredentialsProviderChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CredentialsProviderChain")
            .field(
                "providers",
                &self
                    .providers
                    .iter()
                    .map(|provider| &provider.0)
                    .collect::<Vec<&Cow<'static, str>>>(),
            )
            .finish()
    }
}

impl CredentialsProviderChain {
    /// Create a `CredentialsProviderChain` that begins by evaluating this provider
    pub fn first_try(
        name: impl Into<Cow<'static, str>>,
        provider: impl ProvideCredentials + 'static,
    ) -> Self {
        CredentialsProviderChain {
            providers: vec![(name.into(), Box::new(provider))],
        }
    }

    /// Add a fallback provider to the credentials provider chain
    pub fn or_else(
        mut self,
        name: impl Into<Cow<'static, str>>,
        provider: impl ProvideCredentials + 'static,
    ) -> Self {
        self.providers.push((name.into(), Box::new(provider)));
        self
    }

    /// Add a fallback to the default provider chain
    #[cfg(any(feature = "default-https-client", feature = "rustls"))]
    pub async fn or_default_provider(self) -> Self {
        self.or_else(
            "DefaultProviderChain",
            crate::default_provider::credentials::default_provider().await,
        )
    }

    /// Creates a credential provider chain that starts with the default provider
    #[cfg(any(feature = "default-https-client", feature = "rustls"))]
    pub async fn default_provider() -> Self {
        Self::first_try(
            "DefaultProviderChain",
            crate::default_provider::credentials::default_provider().await,
        )
    }

    async fn credentials(&self) -> provider::Result {
        for (name, provider) in &self.providers {
            let span = tracing::debug_span!("credentials_provider_chain", provider = %name);
            match provider.provide_credentials().instrument(span).await {
                Ok(credentials) => {
                    tracing::debug!(provider = %name, "loaded credentials");
                    return Ok(credentials);
                }
                Err(err @ CredentialsError::CredentialsNotLoaded(_)) => {
                    tracing::debug!(provider = %name, context = %DisplayErrorContext(&err), "provider in chain did not provide credentials");
                }
                Err(err) => {
                    tracing::warn!(provider = %name, error = %DisplayErrorContext(&err), "provider failed to provide credentials");
                    return Err(err);
                }
            }
        }
        Err(CredentialsError::not_loaded(
            "no providers in chain provided credentials",
        ))
    }
}

impl ProvideCredentials for CredentialsProviderChain {
    fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        future::ProvideCredentials::new(self.credentials())
    }

    fn fallback_on_interrupt(&self) -> Option<Credentials> {
        for (_, provider) in &self.providers {
            if let creds @ Some(_) = provider.fallback_on_interrupt() {
                return creds;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use aws_credential_types::{
        credential_fn::provide_credentials_fn,
        provider::{error::CredentialsError, future, ProvideCredentials},
        Credentials,
    };
    use aws_smithy_async::future::timeout::Timeout;

    use crate::meta::credentials::CredentialsProviderChain;

    #[derive(Debug)]
    struct FallbackCredentials(Credentials);

    impl ProvideCredentials for FallbackCredentials {
        fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials<'a>
        where
            Self: 'a,
        {
            future::ProvideCredentials::new(async {
                tokio::time::sleep(Duration::from_millis(200)).await;
                Ok(self.0.clone())
            })
        }

        fn fallback_on_interrupt(&self) -> Option<Credentials> {
            Some(self.0.clone())
        }
    }

    #[tokio::test]
    async fn fallback_credentials_should_be_returned_from_provider2_on_timeout_while_provider2_was_providing_credentials(
    ) {
        let chain = CredentialsProviderChain::first_try(
            "provider1",
            provide_credentials_fn(|| async {
                tokio::time::sleep(Duration::from_millis(200)).await;
                Err(CredentialsError::not_loaded(
                    "no providers in chain provided credentials",
                ))
            }),
        )
        .or_else("provider2", FallbackCredentials(Credentials::for_tests()));

        // Let the first call to `provide_credentials` succeed.
        let expected = chain.provide_credentials().await.unwrap();

        // Let the second call fail with an external timeout.
        let timeout = Timeout::new(
            chain.provide_credentials(),
            tokio::time::sleep(Duration::from_millis(300)),
        );
        match timeout.await {
            Ok(_) => panic!("provide_credentials completed before timeout future"),
            Err(_err) => match chain.fallback_on_interrupt() {
                Some(actual) => assert_eq!(actual, expected),
                None => panic!(
                    "provide_credentials timed out and no credentials returned from fallback_on_interrupt"
                ),
            },
        };
    }

    #[tokio::test]
    async fn fallback_credentials_should_be_returned_from_provider2_on_timeout_while_provider1_was_providing_credentials(
    ) {
        let chain = CredentialsProviderChain::first_try(
            "provider1",
            provide_credentials_fn(|| async {
                tokio::time::sleep(Duration::from_millis(200)).await;
                Err(CredentialsError::not_loaded(
                    "no providers in chain provided credentials",
                ))
            }),
        )
        .or_else("provider2", FallbackCredentials(Credentials::for_tests()));

        // Let the first call to `provide_credentials` succeed.
        let expected = chain.provide_credentials().await.unwrap();

        // Let the second call fail with an external timeout.
        let timeout = Timeout::new(
            chain.provide_credentials(),
            tokio::time::sleep(Duration::from_millis(100)),
        );
        match timeout.await {
            Ok(_) => panic!("provide_credentials completed before timeout future"),
            Err(_err) => match chain.fallback_on_interrupt() {
                Some(actual) => assert_eq!(actual, expected),
                None => panic!(
                    "provide_credentials timed out and no credentials returned from fallback_on_interrupt"
                ),
            },
        };
    }
}
