/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Identity types for HTTP auth

use crate::client::identity::{Identity, IdentityFuture, ResolveIdentity};
use crate::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::ConfigBag;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::SystemTime;
use zeroize::Zeroizing;

/// Identity type required to sign requests using Smithy's token-based HTTP auth schemes
///
/// This `Token` type is used with Smithy's `@httpApiKeyAuth` and `@httpBearerAuth`
/// auth traits.
#[derive(Clone, Eq, PartialEq)]
pub struct Token(Arc<TokenInner>);

#[derive(Eq, PartialEq)]
struct TokenInner {
    token: Zeroizing<String>,
    expiration: Option<SystemTime>,
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("token", &"** redacted **")
            .finish()
    }
}

impl Token {
    /// Constructs a new identity token for HTTP auth.
    pub fn new(token: impl Into<String>, expiration: Option<SystemTime>) -> Self {
        Self(Arc::new(TokenInner {
            token: Zeroizing::new(token.into()),
            expiration,
        }))
    }

    /// Returns the underlying identity token.
    pub fn token(&self) -> &str {
        &self.0.token
    }

    /// Returns the expiration time (if any) for this token.
    pub fn expiration(&self) -> Option<SystemTime> {
        self.0.expiration
    }

    /// Creates a `Token` for tests.
    #[cfg(feature = "test-util")]
    pub fn for_tests() -> Self {
        Self::new("test-token", None)
    }
}

impl From<&str> for Token {
    fn from(token: &str) -> Self {
        Self::from(token.to_owned())
    }
}

impl From<String> for Token {
    fn from(api_key: String) -> Self {
        Self(Arc::new(TokenInner {
            token: Zeroizing::new(api_key),
            expiration: None,
        }))
    }
}

impl ResolveIdentity for Token {
    fn resolve_identity<'a>(
        &'a self,
        _runtime_components: &'a RuntimeComponents,
        _config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a> {
        IdentityFuture::ready(Ok(self.into()))
    }
}

impl From<&Token> for Identity {
    fn from(value: &Token) -> Self {
        Identity::new(value.clone(), value.0.expiration)
    }
}
impl From<Token> for Identity {
    fn from(value: Token) -> Self {
        let expiration = value.0.expiration;
        Identity::new(value, expiration)
    }
}

/// Identity type required to sign requests using Smithy's login-based HTTP auth schemes
///
/// This `Login` type is used with Smithy's `@httpBasicAuth` and `@httpDigestAuth`
/// auth traits.
#[derive(Clone, Eq, PartialEq)]
pub struct Login(Arc<LoginInner>);

#[derive(Eq, PartialEq)]
struct LoginInner {
    user: String,
    password: Zeroizing<String>,
    expiration: Option<SystemTime>,
}

impl Debug for Login {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Login")
            .field("user", &self.0.user)
            .field("password", &"** redacted **")
            .finish()
    }
}

impl Login {
    /// Constructs a new identity login for HTTP auth.
    pub fn new(
        user: impl Into<String>,
        password: impl Into<String>,
        expiration: Option<SystemTime>,
    ) -> Self {
        Self(Arc::new(LoginInner {
            user: user.into(),
            password: Zeroizing::new(password.into()),
            expiration,
        }))
    }

    /// Returns the login user.
    pub fn user(&self) -> &str {
        &self.0.user
    }

    /// Returns the login password.
    pub fn password(&self) -> &str {
        &self.0.password
    }

    /// Returns the expiration time of this login (if any)
    pub fn expiration(&self) -> Option<SystemTime> {
        self.0.expiration
    }
}

impl ResolveIdentity for Login {
    fn resolve_identity<'a>(
        &'a self,
        _runtime_components: &'a RuntimeComponents,
        _config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a> {
        IdentityFuture::ready(Ok(Identity::new(self.clone(), self.0.expiration)))
    }
}
