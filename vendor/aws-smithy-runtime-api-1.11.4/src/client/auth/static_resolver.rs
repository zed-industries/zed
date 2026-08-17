/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::box_error::BoxError;
use crate::client::auth::{AuthSchemeId, AuthSchemeOptionResolverParams, ResolveAuthSchemeOptions};
use std::borrow::Cow;

/// New-type around a `Vec<AuthSchemeId>` that implements `ResolveAuthSchemeOptions`.
#[derive(Debug)]
pub struct StaticAuthSchemeOptionResolver {
    auth_scheme_options: Vec<AuthSchemeId>,
}

impl StaticAuthSchemeOptionResolver {
    /// Creates a new instance of `StaticAuthSchemeOptionResolver`.
    pub fn new(auth_scheme_options: Vec<AuthSchemeId>) -> Self {
        Self {
            auth_scheme_options,
        }
    }
}

impl ResolveAuthSchemeOptions for StaticAuthSchemeOptionResolver {
    fn resolve_auth_scheme_options(
        &self,
        _params: &AuthSchemeOptionResolverParams,
    ) -> Result<Cow<'_, [AuthSchemeId]>, BoxError> {
        Ok(Cow::Borrowed(&self.auth_scheme_options))
    }
}

/// Empty params to be used with [`StaticAuthSchemeOptionResolver`].
#[derive(Debug)]
pub struct StaticAuthSchemeOptionResolverParams;

impl StaticAuthSchemeOptionResolverParams {
    /// Creates a new `StaticAuthSchemeOptionResolverParams`.
    pub fn new() -> Self {
        Self
    }
}

impl From<StaticAuthSchemeOptionResolverParams> for AuthSchemeOptionResolverParams {
    fn from(params: StaticAuthSchemeOptionResolverParams) -> Self {
        AuthSchemeOptionResolverParams::new(params)
    }
}
