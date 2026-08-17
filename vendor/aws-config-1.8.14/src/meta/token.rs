/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Token providers that augment existing token providers to add functionality

use aws_credential_types::provider::{
    error::TokenError, future, token::ProvideToken, token::Result,
};
use aws_smithy_types::error::display::DisplayErrorContext;
use std::borrow::Cow;
use tracing::Instrument;

/// Access token provider that checks a series of inner providers.
///
/// Each provider will be evaluated in order:
/// * If a provider returns valid [`Token`](aws_credential_types::Token) they will be returned immediately.
///   No other token providers will be used.
/// * Otherwise, if a provider returns [`TokenError::TokenNotLoaded`], the next provider will be checked.
/// * Finally, if a provider returns any other error condition, an error will be returned immediately.
///
#[cfg_attr(
    feature = "sso",
    doc = r#"
# Examples

```no_run
# fn example() {
use aws_config::meta::token::TokenProviderChain;
use aws_config::profile::ProfileFileTokenProvider;
use aws_credential_types::Token;

let provider = TokenProviderChain::first_try("Profile", ProfileFileTokenProvider::builder().build())
    .or_else("Static", Token::new("example", None));
# }
```
"#
)]
#[derive(Debug)]
pub struct TokenProviderChain {
    providers: Vec<(Cow<'static, str>, Box<dyn ProvideToken>)>,
}

impl TokenProviderChain {
    /// Create a `TokenProviderChain` that begins by evaluating this provider
    pub fn first_try(
        name: impl Into<Cow<'static, str>>,
        provider: impl ProvideToken + 'static,
    ) -> Self {
        TokenProviderChain {
            providers: vec![(name.into(), Box::new(provider))],
        }
    }

    /// Add a fallback provider to the token provider chain
    pub fn or_else(
        mut self,
        name: impl Into<Cow<'static, str>>,
        provider: impl ProvideToken + 'static,
    ) -> Self {
        self.providers.push((name.into(), Box::new(provider)));
        self
    }

    /// Add a fallback to the default provider chain
    #[cfg(feature = "sso")]
    pub async fn or_default_provider(self) -> Self {
        self.or_else(
            "DefaultProviderChain",
            crate::default_provider::token::default_provider().await,
        )
    }

    /// Creates a token provider chain that starts with the default provider
    #[cfg(feature = "sso")]
    pub async fn default_provider() -> Self {
        Self::first_try(
            "DefaultProviderChain",
            crate::default_provider::token::default_provider().await,
        )
    }

    async fn token(&self) -> Result {
        for (name, provider) in &self.providers {
            let span = tracing::debug_span!("token_provider_chain", provider = %name);
            match provider.provide_token().instrument(span).await {
                Ok(credentials) => {
                    tracing::debug!(provider = %name, "loaded access token");
                    return Ok(credentials);
                }
                Err(err @ TokenError::TokenNotLoaded(_)) => {
                    tracing::debug!(provider = %name, context = %DisplayErrorContext(&err), "provider in chain did not provide an access token");
                }
                Err(err) => {
                    tracing::warn!(provider = %name, error = %DisplayErrorContext(&err), "provider failed to provide an access token");
                    return Err(err);
                }
            }
        }
        Err(TokenError::not_loaded(
            "no providers in chain provided tokens",
        ))
    }
}

impl ProvideToken for TokenProviderChain {
    fn provide_token<'a>(&'a self) -> future::ProvideToken<'a>
    where
        Self: 'a,
    {
        future::ProvideToken::new(self.token())
    }
}
