/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Convenience `ProvideCredentials` struct that implements the `ProvideCredentials` trait.

use crate::provider::token::Result as TokenResult;
use crate::provider::Result as CredsResult;
use aws_smithy_async::future::now_or_later::NowOrLater;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Future new-type that `ProvideCredentials::provide_credentials` must return.
#[derive(Debug)]
pub struct ProvideCredentials<'a>(NowOrLater<CredsResult, BoxFuture<'a, CredsResult>>);

impl<'a> ProvideCredentials<'a> {
    /// Creates a `ProvideCredentials` struct from a future.
    pub fn new(future: impl Future<Output = CredsResult> + Send + 'a) -> Self {
        ProvideCredentials(NowOrLater::new(Box::pin(future)))
    }

    /// Creates a `ProvideCredentials` struct from a resolved credentials value.
    pub fn ready(credentials: CredsResult) -> Self {
        ProvideCredentials(NowOrLater::ready(credentials))
    }
}

impl Future for ProvideCredentials<'_> {
    type Output = CredsResult;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}

/// Future new-type that `ProvideToken::provide_token` must return.
#[derive(Debug)]
pub struct ProvideToken<'a>(NowOrLater<TokenResult, BoxFuture<'a, TokenResult>>);

impl<'a> ProvideToken<'a> {
    /// Creates a `ProvideToken` struct from a future.
    pub fn new(future: impl Future<Output = TokenResult> + Send + 'a) -> Self {
        ProvideToken(NowOrLater::new(Box::pin(future)))
    }

    /// Creates a `ProvideToken` struct from a resolved credentials value.
    pub fn ready(credentials: TokenResult) -> Self {
        ProvideToken(NowOrLater::ready(credentials))
    }
}

impl Future for ProvideToken<'_> {
    type Output = TokenResult;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}
