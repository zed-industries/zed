/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::context::InterceptorContext;
use aws_smithy_runtime_api::client::retries::{RetryStrategy, ShouldAttempt};
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::ConfigBag;

/// A retry strategy that never retries.
#[non_exhaustive]
#[derive(Debug, Clone, Default)]
pub struct NeverRetryStrategy;

impl NeverRetryStrategy {
    /// Creates a new `NeverRetryStrategy`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl RetryStrategy for NeverRetryStrategy {
    fn should_attempt_initial_request(
        &self,
        _runtime_components: &RuntimeComponents,
        _cfg: &ConfigBag,
    ) -> Result<ShouldAttempt, BoxError> {
        Ok(ShouldAttempt::Yes)
    }

    fn should_attempt_retry(
        &self,
        _context: &InterceptorContext,
        _runtime_components: &RuntimeComponents,
        _cfg: &ConfigBag,
    ) -> Result<ShouldAttempt, BoxError> {
        Ok(ShouldAttempt::No)
    }
}
