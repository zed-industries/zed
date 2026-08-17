/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! AWS SDK Credentials
//!
//! ## Implementing your own credentials provider
//!
//! While for many use cases, using a built in credentials provider is sufficient, you may want to
//! implement your own credential provider.
//!
//! ### With static credentials
//!
//! _Note: In general, you should prefer to use the credential providers that come
//! with the AWS SDK to get credentials. It is __NOT__ secure to hardcode credentials
//! into your application. Only use this approach if you really know what you're doing._
//!
#![cfg_attr(
    feature = "hardcoded-credentials",
    doc = r##"
See [`Credentials::from_keys`] for an example on how to use static credentials.
    "##
)]
#![cfg_attr(
    not(feature = "hardcoded-credentials"),
    doc = r##"
Enable the `hardcoded-credentials` feature to be able to use `Credentials::from_keys` to
construct credentials from hardcoded values.
    "##
)]

//!
//! ### With dynamically loaded credentials
//! If you are loading credentials dynamically, you can provide your own implementation of
//! [`ProvideCredentials`]. Generally, this is best done by
//! defining an inherent `async fn` on your structure, then calling that method directly from
//! the trait implementation.
//! ```rust
//! use aws_credential_types::{
//!     provider::{self, future, error::CredentialsError, ProvideCredentials},
//!     Credentials,
//! };
//! #[derive(Debug)]
//! struct SubprocessCredentialProvider;
//!
//! async fn invoke_command(command: &str) -> String {
//!     // implementation elided...
//!     # String::from("some credentials")
//! }
//!
//! /// Parse access key and secret from the first two lines of a string
//! fn parse_credentials(creds: &str) -> provider::Result {
//!     let mut lines = creds.lines();
//!     let akid = lines.next().ok_or(CredentialsError::provider_error("invalid credentials"))?;
//!     let secret = lines.next().ok_or(CredentialsError::provider_error("invalid credentials"))?;
//!     Ok(Credentials::new(akid, secret, None, None, "CustomCommand"))
//! }
//!
//! impl SubprocessCredentialProvider {
//!     async fn load_credentials(&self) -> provider::Result {
//!         let creds = invoke_command("load-credentials.py").await;
//!         parse_credentials(&creds)
//!     }
//! }
//!
//! impl ProvideCredentials for SubprocessCredentialProvider {
//!     fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials<'a> where Self: 'a {
//!         future::ProvideCredentials::new(self.load_credentials())
//!     }
//! }
//! ```

use crate::Credentials;
use aws_smithy_runtime_api::client::identity::{
    Identity, IdentityCachePartition, IdentityFuture, ResolveIdentity,
};
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::{ConfigBag, Storable, StoreReplace};
use std::sync::Arc;

/// Result type for credential providers.
pub type Result = std::result::Result<Credentials, super::error::CredentialsError>;

/// Asynchronous Credentials Provider
pub trait ProvideCredentials: Send + Sync + std::fmt::Debug {
    /// Returns a future that provides credentials.
    fn provide_credentials<'a>(&'a self) -> super::future::ProvideCredentials<'a>
    where
        Self: 'a;

    /// Returns fallback credentials.
    ///
    /// This method should be used as a fallback plan, i.e., when
    /// a call to `provide_credentials` is interrupted and its future
    /// fails to complete.
    ///
    /// The fallback credentials should be set aside and ready to be returned
    /// immediately. Therefore, the user should NOT go fetch new credentials
    /// within this method, which might cause a long-running operation.
    fn fallback_on_interrupt(&self) -> Option<Credentials> {
        None
    }
}

impl ProvideCredentials for Credentials {
    fn provide_credentials<'a>(&'a self) -> super::future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        super::future::ProvideCredentials::ready(Ok(self.clone()))
    }
}

impl ProvideCredentials for Arc<dyn ProvideCredentials> {
    fn provide_credentials<'a>(&'a self) -> super::future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        self.as_ref().provide_credentials()
    }
}

/// Credentials Provider wrapper that may be shared
///
/// Newtype wrapper around ProvideCredentials that implements Clone using an internal
/// Arc.
#[derive(Clone, Debug)]
pub struct SharedCredentialsProvider(Arc<dyn ProvideCredentials>, IdentityCachePartition);

impl SharedCredentialsProvider {
    /// Create a new SharedCredentials provider from `ProvideCredentials`
    ///
    /// The given provider will be wrapped in an internal `Arc`. If your
    /// provider is already in an `Arc`, use `SharedCredentialsProvider::from(provider)` instead.
    pub fn new(provider: impl ProvideCredentials + 'static) -> Self {
        Self(Arc::new(provider), IdentityCachePartition::new())
    }
}

impl AsRef<dyn ProvideCredentials> for SharedCredentialsProvider {
    fn as_ref(&self) -> &(dyn ProvideCredentials + 'static) {
        self.0.as_ref()
    }
}

impl From<Arc<dyn ProvideCredentials>> for SharedCredentialsProvider {
    fn from(provider: Arc<dyn ProvideCredentials>) -> Self {
        SharedCredentialsProvider(provider, IdentityCachePartition::new())
    }
}

impl ProvideCredentials for SharedCredentialsProvider {
    fn provide_credentials<'a>(&'a self) -> super::future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        self.0.provide_credentials()
    }
}

impl Storable for SharedCredentialsProvider {
    type Storer = StoreReplace<SharedCredentialsProvider>;
}

impl ResolveIdentity for SharedCredentialsProvider {
    fn resolve_identity<'a>(
        &'a self,
        _runtime_components: &'a RuntimeComponents,
        _config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a> {
        IdentityFuture::new(async move { Ok(self.provide_credentials().await?.into()) })
    }

    fn fallback_on_interrupt(&self) -> Option<Identity> {
        ProvideCredentials::fallback_on_interrupt(self).map(|creds| creds.into())
    }

    fn cache_partition(&self) -> Option<IdentityCachePartition> {
        Some(self.1)
    }
}

#[cfg(test)]
mod tests {
    use aws_smithy_runtime_api::client::{
        identity::SharedIdentityResolver, runtime_components::RuntimeComponentsBuilder,
    };

    use crate::attributes::AccountId;

    use super::*;

    #[test]
    fn reuses_cache_partition() {
        let creds = Credentials::new("AKID", "SECRET", None, None, "test");
        let provider = SharedCredentialsProvider::new(creds);
        let partition = provider.cache_partition();
        assert!(partition.is_some());

        let identity_resolver = SharedIdentityResolver::new(provider);
        let identity_partition = identity_resolver.cache_partition();

        assert!(partition.unwrap() == identity_partition);
    }

    #[tokio::test]
    async fn account_id_can_be_retrieved_from_identity() {
        let expected_account_id = "012345678901";
        let creds = Credentials::builder()
            .access_key_id("AKID")
            .secret_access_key("SECRET")
            .account_id(expected_account_id)
            .provider_name("test")
            .build();
        let provider = SharedCredentialsProvider::new(creds);
        let identity = provider
            .resolve_identity(
                &RuntimeComponentsBuilder::for_tests().build().unwrap(),
                &ConfigBag::base(),
            )
            .await
            .unwrap();
        let actual = identity.property::<AccountId>().unwrap();
        assert_eq!(expected_account_id, actual.as_str());
    }
}
